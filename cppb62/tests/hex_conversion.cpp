// Copyright (c) Spotify AB

#include "hex_conversion.h"

namespace spotify::tl::detail {

    inline bool is_char_inside(char v, char mi, char ma) {
        return (unsigned char)((v) - (mi)) < ((ma) - (mi));
    }

    char *convert_to_hex_impl(char *hex, const uint8_t *id, size_t N) {
        static const char kBaseChars[] =
                "0123456789abcdefghijklmnopqrstuvwxyz"
                "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        size_t i;
        for (i = 0; i < N; i++) {
            uint8_t x = id[i];
            hex[i * 2 + 0] = kBaseChars[x >> 4];
            hex[i * 2 + 1] = kBaseChars[x & 0xF];
        }
        hex[i * 2] = 0;
        return hex;
    }

#ifdef __GNUC__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wsign-conversion"
#endif  // __GNUC__
/**
 * @param hex A pointer to 32 valid hexadecimal characters. Doesn't have to be
 *            NUL-terminated, but reading will stop if a NUL is found
 *            prematurely.
 */
    bool convert_from_hex_impl(uint8_t *id, size_t N, const char *hex) {
        for (size_t i = 0; i < N; i++) {
            uint8_t c, d;

            c = hex[i * 2 + 0];
            if (is_char_inside(c, '0', '9' + 1))
                c -= '0';
            else if (is_char_inside(c & ~32, 'A', 'F' + 1))
                c = (c & ~32) - 'A' + 10;
            else
                return false;

            d = hex[i * 2 + 1];
            if (is_char_inside(d, '0', '9' + 1))
                d -= '0';
            else if (is_char_inside(d & ~32, 'A', 'F' + 1))
                d = (d & ~32) - 'A' + 10;
            else
                return false;

            id[i] = c * 16 + d;
        }
        return true;
    }
#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif  // __GNUC__

}  // namespace spotify::tl::detail