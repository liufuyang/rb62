// Copyright (c) Spotify AB

#pragma once

#include <stdbool.h>

char *convert_to_base62(char base62[23], char id[16]);

/**
 * @param base62 22 base62 character data. It doesn't have to be NUL-terminated,
 *        but reading will stop if a NUL is found prematurely.
 */
bool convert_from_base62(char id[16], const char *base62);

