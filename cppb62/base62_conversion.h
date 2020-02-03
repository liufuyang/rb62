// Copyright (c) Spotify AB

#pragma once

#include <array>
#include <cstdint>

namespace spotify::tl::detail {

char *convert_to_base62(char base62[23], const std::array<uint8_t, 16> &id);

/**
 * @param base62 22 base62 character data. It doesn't have to be NUL-terminated,
 *        but reading will stop if a NUL is found prematurely.
 */
bool convert_from_base62(std::array<uint8_t, 16> &id, const char *base62);

}  // namespace spotify::tl::detail
