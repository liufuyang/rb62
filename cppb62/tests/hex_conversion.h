// Copyright (c) Spotify AB

#pragma once

#include <array>
#include <cstdint>

namespace spotify::tl::detail {

    char *convert_to_hex_impl(char *hex, const uint8_t *id, size_t N);
    bool convert_from_hex_impl(uint8_t *id, size_t N, const char *hex);

    template <size_t N>
    inline char *convert_to_hex(char *hex, const std::array<uint8_t, N> &id) {
        return convert_to_hex_impl(hex, id.data(), N);
    }

    template <size_t N>
    inline bool convert_from_hex(std::array<uint8_t, N> &id, const char *hex) {
        return convert_from_hex_impl(id.data(), N, hex);
    }

}  // namespace spotify::tl::detail