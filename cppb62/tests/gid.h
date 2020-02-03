// Copyright (c) Spotify AB

#pragma once

#include <algorithm>  // for all_of
#include <array>
#include <cassert>
#include <cstdint>
#include <functional>  // for hash<>
#include <optional>
#include <string>
#include <type_traits>  // for enable_if<>
#include "base62_conversion.h"
#include "hex_conversion.h"
#include "span.h"

namespace spotify::tl {

/**
 * A gid is a POD type meant to be used for an identifier for a resource.
 * It can be assumed to be convertible to std::array<uint8_t, N>.
 *
 * To distinguish IDs of the same size but for different types of resources,
 * the user specifies a tag type.
 */
    template <size_t N, typename Tag = void>
    struct gid : public std::array<uint8_t, N> {
    private:
        using super_t = std::array<uint8_t, N>;

    public:
        /**
         * Internal type meant for efficiently returning a fixed size string
         * representing the ID.
         */
        template <ptrdiff_t Extent>
        struct string_span {
            char raw[Extent + 1];
            char *c_str() { return raw; }
            std::string string() const { return std::string(raw, Extent); }
            operator std::string() const { return string(); }
        };

        /**
         * Check if the gid consists of just zeroes.
         */
        bool all_zeroes() const {
            return std::all_of(super_t::cbegin(), super_t::cend(), [](uint8_t c) { return c == 0; });
        }

        /**
         * Creates a gid with only zeroes in it.
         */
        static gid from_all_zeroes() {
            gid g;
            g.fill(0);
            return g;
        }

        /**
         * Create a gid from an array of the correct size.
         */
        static const gid &from(const std::array<uint8_t, N> &a) { return *static_cast<const gid *>(&a); }

        /**
         * Convert the gid to a null-terminated hex string.
         */
        string_span<N * 2> to_hex() const {
            string_span<N * 2> res;
            detail::convert_to_hex(res.raw, *this);
            return res;
        }

        /**
         * Convert the gid to a null-terminated base62 string.
         * TODO(hammar): Support base62 for all gid sizes, but
         * use the optimized version for N == 16.
         */
        template <bool Enable = true, typename std::enable_if<Enable && N == 16, int>::type * = nullptr>
        string_span<22> to_base62() const {
            string_span<22> res;
            detail::convert_to_base62(res.raw, *this);
            return res;
        }

        string_span<N> to_raw() const {
            string_span<N> res;
            memcpy(res.raw, this->data(), N);
            res.raw[N] = '\0';
            return res;
        }

        static const gid *from_raw(const char *raw_bytes) {
            if (strlen(raw_bytes) != N) {
                assert(false && "raw byte string should match gid size exactly");
                return nullptr;
            }
            return reinterpret_cast<const gid<N, Tag> *>(raw_bytes);
        }

        static const gid *from_raw(const std::string &raw_bytes) {
            assert(raw_bytes.size() == N && "raw byte string should match gid size exactly");
            if (raw_bytes.size() != N) {
                assert(false && "raw byte string should match gid size exactly");
                return nullptr;
            }
            return reinterpret_cast<const gid<N, Tag> *>(raw_bytes.data());
        }

        /**
         * Populate the gid from the bytes represented by a hex string.
         * If the hex string fails parsing, this returns 'false' and the gid
         * is left in an undefined state.
         */
        bool from_hex(const char *hex) {
            return detail::convert_from_hex(*this, hex) && hex[2 * N] == '\0';
        }

        bool from_hex(const tl::span<const char> &hex) {
            if (hex.size() != 2 * N) {
                return false;
            }
            return detail::convert_from_hex(*this, hex.data());
        }

        /**
         * Populate the gid from the bytes represented by a base62 string.
         * If the base62 string fails parsing, this returns 'false' and the gid
         * is left in an undefined state.
         */
        bool from_base62(const char *base62) {
            return detail::convert_from_base62(*this, base62) && base62[22] == '\0';
        }

        /**
         * @see from_hex.
         */
        bool from_hex(const std::string &hex) { return from_hex(hex.c_str()); }

        bool from_base62(const tl::span<const char> &base62) {
            if (base62.size() != 22) {
                return false;
            }
            return detail::convert_from_base62(*this, base62.data());
        }

        /**
         * @see from_base62.
         */
        bool from_base62(const std::string &base62) { return from_base62(base62.c_str()); }

        /**
         * Create a gid from a hex string.
         */
        static std::optional<const gid> from_hex_string(const std::string &s) {
            gid id;
            if (id.from_hex(s)) {
                return id;
            }
            return {};
        }

        /**
         * Create a gid from a base62 string.
         */
        static std::optional<const gid> from_base62_string(const std::string &s) {
            gid id;
            if (id.from_base62(s)) {
                return id;
            }
            return {};
        }

    private:
        /**
         * This is almost never what you want, let's hide it.
         * See @all_zeroes for checking if a gid is unset.
         */
        bool empty() const;
    };

    static_assert(sizeof(gid<7>) == 7, "size mismatch");

    namespace detail {
        size_t hashGid(const uint8_t *buffer, size_t size);
    }

}  // namespace spotify::tl

namespace std {

    template <size_t N, typename Tag>
    struct hash<spotify::tl::gid<N, Tag>> {
        size_t operator()(const spotify::tl::gid<N, Tag> &id) const {
            return spotify::tl::detail::hashGid(id.data(), N);
        }
    };

}  // namespace std
