// Copyright (c) Spotify AB

#pragma once

// Description:
// span is a wrapper around pointer/size. It references a fixed size array.
// Use this instead of passing say: int *items, int nitems
// It does not copy/allocate any memory so it should be about as fast as
// the old style way.

#include <cstddef>
#include <cstring>
#include <boost/assert.hpp>

namespace spotify::tl {

    template <typename T>
    class span;
    template <typename T>
    constexpr span<T> as_span(T *a, size_t n);

    template <typename T>
    class span {
    protected:
        T *a;
        size_t n;

    public:
        using value_type = T;
        using element_type = T;
        typedef T *iterator;
        typedef const T *const_iterator;

        span() : a(NULL), n(0) {}
        constexpr span(T *a, size_t n) : a(a), n(n) {}
        template <int N>
        constexpr span(T (&a)[N]) : a(a), n(N) {}

        template <typename U>
        explicit span(const span<U> u) : a(u.data()), n(u.size()) {}

        [[nodiscard]] size_t inline count() const { return n; }
        [[nodiscard]] size_t inline size() const { return n; }
        [[nodiscard]] bool inline empty() const { return n == 0; }

        T *data() { return a; }
        const T *data() const { return a; }

        inline T &operator[](size_t offset) {
            BOOST_ASSERT(offset < n);
            return a[offset];
        }
        inline const T &operator[](size_t offset) const {
            BOOST_ASSERT(offset < n);
            return a[offset];
        }

        inline operator span<const T>() const { return span<const T>(a, n); }

        inline span<T> subspan(int p, int l) const {
            BOOST_ASSERT(p >= 0 && l >= 0 && p + l <= n);
            return as_span(a + p, l);
        }

        inline span<T> sub_span(size_t p, size_t l) const {
            BOOST_ASSERT(p + l <= n);
            return as_span(a + p, l);
        }

        inline void shift(size_t l) {
            BOOST_ASSERT(l <= n);
            a += l;
            n -= l;
        }

        inline bool operator==(const span<const T> arr) const {
            return n == arr.size() && memcmp(a, arr.data(), sizeof(T) * n) == 0;
        }
        inline bool operator!=(const span<const T> arr) const {
            return n != arr.size() || memcmp(a, arr.data(), sizeof(T) * n) != 0;
        }

        iterator begin() { return data(); }
        iterator end() { return data() + size(); }
        const_iterator begin() const { return data(); }
        const_iterator end() const { return data() + size(); }
        const_iterator cbegin() const { return begin(); }
        const_iterator cend() const { return end(); }

        template <typename U>
        static span<T> from(const U &u) {
            static_assert(sizeof(T) == sizeof(typename U::value_type), "size mismatch");
            return as_span((T *)u.data(), u.size());
        }
    };

    template <typename T>
    constexpr span<T> as_span(T *a, size_t n) {
        return span<T>(a, n);
    }
    template <typename T, size_t N>
    constexpr span<T> as_span(T (&arr)[N]) {
        return span<T>(arr, N);
    }
    template <typename Cont>
    constexpr span<typename Cont::value_type> as_span(Cont &a) {
        return span<typename Cont::value_type>::from(a);
    }

}  // namespace spotify::tl