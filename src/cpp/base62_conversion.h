// Copyright (c) Spotify AB

// #pragma once

#include <cstdint>

extern "C" {
    char *convert_to_base62(char base62[23], uint8_t id[16]);

    /**
     * @param base62 22 base62 character data. It doesn't have to be NUL-terminated,
     *        but reading will stop if a NUL is found prematurely.
     */
    bool convert_from_base62(uint8_t id[16], const char *base62);
}

