// Copyright (c) Spotify AB

// clang base62_conversion.cpp -c -std=c++11

#include "base62_conversion.h"

extern "C" {

char *convert_to_base62(char base62[23], uint8_t id[16]) {
  static const char kBaseChars[62 + 1] =
      "0123456789"
      "abcdefghijklmnopqrstuvwxyz"
      "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

  int i;
  uint32_t bi4;
  uint32_t bi3 = id[0] << 24 | id[1] << 16 | id[2] << 8 | id[3];
  uint32_t bi2 = id[4] << 24 | id[5] << 16 | id[6] << 8 | id[7];
  uint32_t bi1 = id[8] << 24 | id[9] << 16 | id[10] << 8 | id[11];
  uint32_t bi0 = id[12] << 24 | id[13] << 16 | id[14] << 8 | id[15];

  bi4 = bi3 >> 8;
  bi3 = (bi3 << (26 - 8)) | (bi2 >> 14);
  bi2 = (bi2 << (26 - 14)) | (bi1 >> 20);
  bi1 = (bi1 << (26 - 20)) | (bi0 >> 26);

  bi3 &= 0x3FFFFFF;
  bi2 &= 0x3FFFFFF;
  bi1 &= 0x3FFFFFF;
  bi0 &= 0x3FFFFFF;

  // after some iterations, the upper bits are guaranteed to be < 62,
  // so shorten the loop each time
  for (i = 0; i < 4; ++i) {
    uint32_t acc = bi4;
    bi4 = (acc / 62);
    acc = ((acc % 62) << 26) | bi3;
    bi3 = (acc / 62);
    acc = ((acc % 62) << 26) | bi2;
    bi2 = (acc / 62);
    acc = ((acc % 62) << 26) | bi1;
    bi1 = (acc / 62);
    acc = ((acc % 62) << 26) | bi0;
    bi0 = (acc / 62);
    base62[21 - i] = kBaseChars[(acc % 62)];
  }
  bi3 |= bi4 << 26;
  for (; i < 8; ++i) {
    uint32_t acc = bi3;
    bi3 = (acc / 62);
    acc = ((acc % 62) << 26) | bi2;
    bi2 = (acc / 62);
    acc = ((acc % 62) << 26) | bi1;
    bi1 = (acc / 62);
    acc = ((acc % 62) << 26) | bi0;
    bi0 = (acc / 62);
    base62[21 - i] = kBaseChars[(acc % 62)];
  }
  bi2 |= bi3 << 26;
  for (; i < 12; ++i) {
    uint32_t acc = bi2;
    bi2 = (acc / 62);
    acc = ((acc % 62) << 26) | bi1;
    bi1 = (acc / 62);
    acc = ((acc % 62) << 26) | bi0;
    bi0 = (acc / 62);
    base62[21 - i] = kBaseChars[(acc % 62)];
  }
  bi1 |= bi2 << 26;
  for (; i < 17; ++i) {
    uint32_t acc = bi1;
    bi1 = (acc / 62);
    acc = ((acc % 62) << 26) | bi0;
    bi0 = (acc / 62);
    base62[21 - i] = kBaseChars[(acc % 62)];
  }
  bi0 |= bi1 << 26;
  for (; i < 21; ++i) {
    uint32_t acc = bi0;
    bi0 = (acc / 62);
    base62[21 - i] = kBaseChars[(acc % 62)];
  }
  base62[0] = '0' + bi0;
  base62[22] = 0;
  return base62;
}

/**
 * @param base62 A pointer to 22 valid base62 characters. Doesn't have to be
 *               NUL-terminated, but reading will stop if a NUL is found
 *               prematurely.
 */
inline bool get_base62_values(char values[22], const char *base62) {
  for (int it = 0; it < 22; ++it) {
    char value_char = base62[it];
    char &value = values[it];
    if ('0' <= value_char && value_char <= '9') {
      value = value_char - '0';
    } else if ('a' <= value_char && value_char <= 'z') {
      value = value_char - 'a' + ('9' - '0' + 1);
    } else if ('A' <= value_char && value_char <= 'Z') {
      value = value_char - 'A' + ('9' - '0' + 1) + ('z' - 'a' + 1);
    } else {
      return false;
    }
  }
  return true;
}
#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif  // __GNUC__

inline bool base62_number_fits_in_id(const char values[22]) {
  static char max_values[22];
  if (max_values[0] == 0) {
    const char *kMaxBase62 = "7N42dgm5tFLK9N8MT7fHC7";  // All 128 bits set
    get_base62_values(max_values, kMaxBase62);
  }
  for (int it = 0; it < 22; ++it) {
    if (values[it] > max_values[it]) {
      return false;
    } else if (values[it] < max_values[it]) {
      return true;
    }
  }
  return true;
}

/**
 * @param base62 22 base62 character data. It doesn't have to be NUL-terminated,
 *        but reading will stop if a NUL is found prematurely.
 */
bool convert_from_base62(uint8_t id[16], const char *base62) {
  /**
   * This function might look daunting, but don't despair, at a higher level it's actually
   * quite simple.
   *
   * In words it goes something like this:
   * Loop through all 62 based values, most significant value first.
   * Add them to a big integer, while constantly "shifting" previously added
   * values upwards by multiplying by the base (62).
   *
   * And in pseudo code:
   * uint128_t bi = 0; // bi stands for "big int"
   * for (int i = 0; i < 22; ++i) {
   *   bi *= 62;
   *   bi += get_value_of_base62_digit(base62[i]); // Returns 0-61
   * }
   *
   * Unfortunately C++ isn't equipped to do 128 bit integer calculations natively, it can however
   * do efficient 32 bit calculations.
   * A gid of 16 bytes, that's 128 bits, which can be represented as four 32 bit
   * integers. Let's call these bi0 - bi3 where bi3 stores most significant bits in the resulting
   * id.
   * Now, we need to keep track of overflow when multiplying and adding in new digits from the
   * base62 number. The overflow from bi0 should be added to bi1 and bi1 should in turn overflow
   * into bi2 and so on.
   * There is no way to get the overflow once a multiplication/add has been done. This is
   * solved by only using the first 24 bits in each integer, the upper 8 bits is used to store and
   * detect overflow for previous multiplication/add. We need to add another int (bi4) to replace
   * the lost bits.
   * If we validate that the base62 number fits into 128 bits before we start we don't need to worry
   * about overflow for bi4.
   *
   * The code would then be:
   * uint32_t bi[5] = {0, 0, 0, 0, 0};
   * int overflow = 0;
   * for (int i = 0; i < 22; ++i) {
   *   bi[0] *= 62;
   *   bi[0] += get_value_of_base62_digit(base62[i]);
   *   overflow = (bi[0] & 0xff000000) >> 24;
   *   bi[0] &= 0x00ffffff;
   *   for (int j = 1; j < 4; ++j) {
   *     bi[j] *= 62;
   *     bi[j] += overflow;
   *     overflow = (bi[j] & 0xff000000) >> 24;
   *     bi[j] &= 0x00ffffff;
   *   }
   *   bi[4] *= 62;
   *   bi[4] += overflow;
   * }
   *
   * Good enough? Nope.
   * If you think about it, in the first few iterations, only bi0 will be affected. It's not until
   * we have inserted and shifted enough digits to overflow the lower 24 bits that anything can
   * reach bi1. We can then keep going with until the lower 24 bits of bi1 is full, and so on.
   * To figure out the 24 bit cutoff points we use the largest base62 number that can fix
   * in 128 bits and shift in one digit at a time. The relevant iterations , where we overflow
   * bit-boundaries 24, 48, 72, and 96 is 5, 9, 13 and 17 respecively.
   * These numbers is listed in tests/detail/base62_test_data.cpp under the label:
   * // Shift in max base62 number ("7N42dgm5tFLK9N8MT7fHC7") from the right
   *
   * By expanding the "simple" code above, while removing all unnecessary calculations we end up
   * with this code.
   */
  char values[22];  // Contains values for each of the "digit" in base62 string

  // We do string to value conversions and overflow validations first.
  // This way we can chew away in a fast and branch-less fashion later.
  if (!get_base62_values(values, base62)) {
    return false;
  }
  if (!base62_number_fits_in_id(values)) {
    return false;
  }

  const uint32_t kOverflowMask = 0xff000000;
  uint32_t bi0 = 0x00000000;
  uint32_t bi1 = 0x00000000;
  uint32_t bi2 = 0x00000000;
  uint32_t bi3 = 0x00000000;
  uint32_t bi4 = 0x00000000;
  uint32_t overflow = 0x00000000;
  int i = 0;

  // Consume the first 5 base62 digits into bi0
  for (; i < 5; ++i) {
    bi0 *= 62;
    bi0 += values[i];
  }

  // Transfer overflow from bi0 into bi1.
  // On the last iteration we could overflow outside the first 24 bits of bi0.
  overflow = (bi0 & kOverflowMask) >> 24;
  bi0 &= ~kOverflowMask;
  bi1 = overflow;

  // Consume base62 digits 5 through 8 into bi0 and bi1
  for (; i < 9; ++i) {
    bi0 *= 62;
    bi0 += values[i];
    overflow = (bi0 & kOverflowMask) >> 24;
    bi0 &= ~kOverflowMask;
    bi1 *= 62;
    bi1 += overflow;
  }

  // Transfer overflow from bi1 into bi2.
  // On the last iteration we could overflow outside the first 24 bits of bi1.
  overflow = (bi1 & kOverflowMask) >> 24;
  bi1 &= ~kOverflowMask;
  bi2 = overflow;

  // Consume base62 digits 9 through 12 into bi0, bi1 and bi2
  for (; i < 13; ++i) {
    bi0 *= 62;
    bi0 += values[i];
    overflow = (bi0 & kOverflowMask) >> 24;
    bi0 &= ~kOverflowMask;

    bi1 *= 62;
    bi1 += overflow;
    overflow = (bi1 & kOverflowMask) >> 24;
    bi1 &= ~kOverflowMask;

    bi2 *= 62;
    bi2 += overflow;
  }

  // Transfer overflow from bi2 into bi3.
  // On the last iteration we could overflow outside the first 24 bits of bi2.
  overflow = (bi2 & kOverflowMask) >> 24;
  bi2 &= ~kOverflowMask;
  bi3 = overflow;

  // Consume base62 digits 13 through 16 into bi0, bi1, bi2 and bi3
  for (; i < 17; ++i) {
    bi0 *= 62;
    bi0 += values[i];
    overflow = (bi0 & kOverflowMask) >> 24;
    bi0 &= ~kOverflowMask;

    bi1 *= 62;
    bi1 += overflow;
    overflow = (bi1 & kOverflowMask) >> 24;
    bi1 &= ~kOverflowMask;

    bi2 *= 62;
    bi2 += overflow;
    overflow = (bi2 & kOverflowMask) >> 24;
    bi2 &= ~kOverflowMask;

    bi3 *= 62;
    bi3 += overflow;
  }

  // Transfer overflow from bi3 into bi4.
  // On the last iteration we could overflow outside the first 24 bits of bi3.
  overflow = (bi3 & kOverflowMask) >> 24;
  bi3 &= ~kOverflowMask;
  bi4 = overflow;

  // Consume base62 digits 17 through 21 into bi0, bi1, bi2, bi3 and bi4
  // We know that the number won't overflow 128 bits, so we don't need to consider overflow
  // for bi4.
  for (; i < 22; ++i) {
    bi0 *= 62;
    bi0 += values[i];
    overflow = (bi0 & kOverflowMask) >> 24;
    bi0 &= ~kOverflowMask;

    bi1 *= 62;
    bi1 += overflow;
    overflow = (bi1 & kOverflowMask) >> 24;
    bi1 &= ~kOverflowMask;

    bi2 *= 62;
    bi2 += overflow;
    overflow = (bi2 & kOverflowMask) >> 24;
    bi2 &= ~kOverflowMask;

    bi3 *= 62;
    bi3 += overflow;
    overflow = (bi3 & kOverflowMask) >> 24;
    bi3 &= ~kOverflowMask;

    bi4 *= 62;
    bi4 += overflow;
  }

  // Extract bytes from bi* into the result
  id[0] = bi4 >> 24 & 0xff;
  id[1] = bi4 >> 16 & 0xff;
  id[2] = bi4 >> 8 & 0xff;
  id[3] = bi4 & 0xff;

  id[4] = bi3 >> 16 & 0xff;
  id[5] = bi3 >> 8 & 0xff;
  id[6] = bi3 & 0xff;

  id[7] = bi2 >> 16 & 0xff;
  id[8] = bi2 >> 8 & 0xff;
  id[9] = bi2 & 0xff;

  id[10] = bi1 >> 16 & 0xff;
  id[11] = bi1 >> 8 & 0xff;
  id[12] = bi1 & 0xff;

  id[13] = bi0 >> 16 & 0xff;
  id[14] = bi0 >> 8 & 0xff;
  id[15] = bi0 & 0xff;

  return true;
}

}
