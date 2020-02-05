#![feature(test)]

extern crate test;

extern "C" {
    #[allow(dead_code)]
    fn convert_to_base62(
        base62: *mut ::std::os::raw::c_char,
        id: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}

extern "C" {
    #[allow(dead_code)]
    fn convert_from_base62(
        id: *mut ::std::os::raw::c_char,
        base62: *const ::std::os::raw::c_char,
    ) -> bool;
}

use test::Bencher;
use rb62::{get_integer, get_b62};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::str;

struct Base62TestData(&'static str, &'static str);

#[bench]
fn bench_cpp_hex_to_b62(b: &mut Bencher) {
    b.iter(|| {
        for test in TEST_DATA {
            let mut base62 = vec![0i8; 23];
            let mut id = hex::decode(test.1).unwrap();
            unsafe {
                let b62 = convert_to_base62(base62.as_mut_ptr(), id.as_mut_ptr() as *mut c_char);
                let b62 = CStr::from_ptr(b62).to_string_lossy();
                assert_eq!(b62, test.0)
            }
        }
    });
}

#[bench]
fn bench_cpp_b62_to_hex(b: &mut Bencher) {
    b.iter(|| {
        for test in TEST_DATA {
            let base62 = CString::new(test.0).expect("Create CString from test data.0");
            let mut id = vec![0u8; 16];
            unsafe {
                let _bool = convert_from_base62(id.as_mut_ptr() as *mut c_char, base62.as_ptr() as *const c_char);
            }
            let hex = hex::encode(id);
            assert_eq!(hex, test.1)
        }
    });
}

#[bench]
fn bench_rust_hex_to_b62(b: &mut Bencher) {
    b.iter(|| {
        for test in TEST_DATA {
            let b62 = get_b62(test.1).expect("get_b62 can parse test data");
            let b62 = str::from_utf8(&b62).unwrap();
            assert_eq!(b62, test.0,
                       "we are testing hex {} to b62 {}, but got b62 {}", test.1, test.0, b62
            );
        }
    });
}

#[bench]
fn bench_rust_b62_to_hex(b: &mut Bencher) {
    b.iter(|| {
        for test in TEST_DATA {
            let i = get_integer(test.0).expect("get_integer can parse test data");
            let hex = format! {"{:032x}", i};
            assert_eq!(hex, test.1,
                       "we are testing b62 {} to hex {}, but got hex {}", test.0, test.1, hex
            );
        }
    });
}

#[bench]
fn bench_single_operation_cpp_hex_to_b62(b: &mut Bencher) {
    b.iter(|| {
        let mut base62 = vec![0i8; 23];
        let mut id = hex::decode("dbc3d5ebe344484da3e2448712a02213").unwrap();
        unsafe {
            let b62 = convert_to_base62(base62.as_mut_ptr(), id.as_mut_ptr() as *mut c_char);
            let b62 = CStr::from_ptr(b62).to_string_lossy();
            assert_eq!(b62, "6GGODyP2LIdbxIfYxy5UbN")
        }
    });
}

#[bench]
fn bench_single_operation_cpp_b62_to_hex(b: &mut Bencher) {
    b.iter(|| {
        let base62 = CString::new("6GGODyP2LIdbxIfYxy5UbN").expect("Create CString from test data.0");
        let mut id = vec![0u8; 16];
        unsafe {
            let _bool = convert_from_base62(id.as_mut_ptr() as *mut c_char, base62.as_ptr() as *const c_char);
        }
        let hex = hex::encode(id);
        assert_eq!(hex, "dbc3d5ebe344484da3e2448712a02213")
    });
}

#[bench]
fn bench_single_operation_rust_hex_to_b62(b: &mut Bencher) {
    b.iter(|| {
        let b62 = get_b62("dbc3d5ebe344484da3e2448712a02213").expect("get_b62 can parse test data");
        let b62 = str::from_utf8(&b62).unwrap();
        assert_eq!(b62, "6GGODyP2LIdbxIfYxy5UbN",
                   "we are testing hex {} to b62 {}, but got b62 {}", "dbc3d5ebe344484da3e2448712a02213", "6GGODyP2LIdbxIfYxy5UbN", b62
        );
    });
}

#[bench]
fn bench_single_operation_rust_b62_to_hex(b: &mut Bencher) {
    b.iter(|| {
        let i = get_integer("6GGODyP2LIdbxIfYxy5UbN").expect("get_integer can parse test data");
        let hex = format! {"{:032x}", i};
        assert_eq!(hex, "dbc3d5ebe344484da3e2448712a02213",
                   "we are testing b62 {} to hex {}, but got hex {}", "6GGODyP2LIdbxIfYxy5UbN", "dbc3d5ebe344484da3e2448712a02213", hex
        );
    });
}

#[test]
fn cpp_convert_to_base62_works_for_all() {
    for test in TEST_DATA {
        let mut base62 = vec![0i8; 23];
        let mut id = hex::decode(test.1).unwrap();
        unsafe {
            let b62 = convert_to_base62(base62.as_mut_ptr(), id.as_mut_ptr() as *mut c_char);
            let b62 = CStr::from_ptr(b62).to_string_lossy();
            assert_eq!(b62, test.0)
        }
    }
}

#[test]
fn cpp_convert_from_base62_works_for_all() {
    for test in TEST_DATA {
        let base62 = CString::new(test.0).expect("Create CString from test data.0");
        let mut id = vec![0u8; 16];
        unsafe {
            let _bool = convert_from_base62(id.as_mut_ptr() as *mut c_char, base62.as_ptr() as *const c_char);
        }
        let hex = hex::encode(id);
        assert_eq!(hex, test.1)
    }
}

#[test]
fn cpp_convert_from_base62_should_return_false_when_input_invalid() {
    let invalid_inputs = vec![
        "000000000000000000000+",  // Invalid characters (+)
        "000000000000000000001",   // String is too short (should be at least 22 characters long)
        "7N42dgm5tFLK9N8MT7fHC8",  // Too large (max is 7N42dgm5tFLK9N8MT7fHC7)
        "ZZZZZZZZZZZZZZZZZZZZZZ",  // Definately too large to fit in 128 bits
    ];

    for invalid in invalid_inputs {
        let base62 = CString::new(invalid).expect("Create CString from test data");
        let mut id = vec![0u8; 16];

        unsafe {
            let bool = convert_from_base62(id.as_mut_ptr() as *mut c_char, base62.as_ptr() as *const c_char);
            assert_eq!(bool, false);
        }
    }
}

const TEST_DATA: &'static [Base62TestData] = &[
    // Base62TestData("0000000000000000000001", "00000000000000000000000000000001"),
    Base62TestData("0000000000000000000002", "00000000000000000000000000000002"),
    Base62TestData("0000000000000000000004", "00000000000000000000000000000004"),
    Base62TestData("0000000000000000000008", "00000000000000000000000000000008"),
    Base62TestData("000000000000000000000g", "00000000000000000000000000000010"),
    Base62TestData("000000000000000000000w", "00000000000000000000000000000020"),
    Base62TestData("0000000000000000000012", "00000000000000000000000000000040"),
    Base62TestData("0000000000000000000024", "00000000000000000000000000000080"),
    Base62TestData("0000000000000000000048", "00000000000000000000000000000100"),
    Base62TestData("000000000000000000008g", "00000000000000000000000000000200"),
    Base62TestData("00000000000000000000gw", "00000000000000000000000000000400"),
    Base62TestData("00000000000000000000x2", "00000000000000000000000000000800"),
    Base62TestData("0000000000000000000144", "00000000000000000000000000001000"),
    Base62TestData("0000000000000000000288", "00000000000000000000000000002000"),
    Base62TestData("00000000000000000004gg", "00000000000000000000000000004000"),
    Base62TestData("00000000000000000008ww", "00000000000000000000000000008000"),
    Base62TestData("0000000000000000000h32", "00000000000000000000000000010000"),
    Base62TestData("0000000000000000000y64", "00000000000000000000000000020000"),
    Base62TestData("00000000000000000016c8", "00000000000000000000000000040000"),
    Base62TestData("0000000000000000002cog", "00000000000000000000000000080000"),
    Base62TestData("0000000000000000004oMw", "00000000000000000000000000100000"),
    Base62TestData("0000000000000000008Nz2", "00000000000000000000000000200000"),
    Base62TestData("000000000000000000hB84", "00000000000000000000000000400000"),
    Base62TestData("000000000000000000zcg8", "00000000000000000000000000800000"),
    Base62TestData("0000000000000000018owg", "00000000000000000000000001000000"),
    Base62TestData("000000000000000002gN2w", "00000000000000000000000002000000"),
    Base62TestData("000000000000000004xA52", "00000000000000000000000004000000"),
    Base62TestData("0000000000000000095aa4", "00000000000000000000000008000000"),
    Base62TestData("00000000000000000iakk8", "00000000000000000000000010000000"),
    Base62TestData("00000000000000000AkEEg", "00000000000000000000000020000000"),
    Base62TestData("00000000000000001aFjiw", "00000000000000000000000040000000"),
    Base62TestData("00000000000000002lkCB2", "00000000000000000000000080000000"),
    Base62TestData("00000000000000004GFfc4", "00000000000000000000000100000000"),
    Base62TestData("00000000000000009nkuo8", "00000000000000000000000200000000"),
    Base62TestData("0000000000000000iKEYMg", "00000000000000000000000400000000"),
    Base62TestData("0000000000000000BvjXyw", "00000000000000000000000800000000"),
    Base62TestData("0000000000000001d0DV72", "00000000000000000000001000000000"),
    Base62TestData("0000000000000002q1hQe4", "00000000000000000000002000000000"),
    Base62TestData("0000000000000004Q2zGs8", "00000000000000000000004000000000"),
    Base62TestData("0000000000000009G59mUg", "00000000000000000000008000000000"),
    Base62TestData("000000000000000jmaiJOw", "00000000000000000000010000000000"),
    Base62TestData("000000000000000CIkBtD2", "00000000000000000000020000000000"),
    Base62TestData("000000000000001fqFcXg4", "00000000000000000000040000000000"),
    Base62TestData("000000000000002uRkpUw8", "00000000000000000000080000000000"),
    Base62TestData("000000000000004ZIEPP2g", "00000000000000000000100000000000"),
    Base62TestData("000000000000009ZrjFE4w", "00000000000000000000200000000000"),
    Base62TestData("00000000000000jYSDli92", "00000000000000000000400000000000"),
    Base62TestData("00000000000000DXLgGAi4", "00000000000000000000800000000000"),
    Base62TestData("00000000000001hVwxnaA8", "00000000000000000001000000000000"),
    Base62TestData("00000000000002zR34Klag", "00000000000000000002000000000000"),
    Base62TestData("000000000000059I69uGkw", "00000000000000000004000000000000"),
    Base62TestData("0000000000000ajqciZmF2", "00000000000000000008000000000000"),
    Base62TestData("0000000000000kCQoBYJk4", "00000000000000000010000000000000"),
    Base62TestData("0000000000000FfGNdXsE8", "00000000000000000020000000000000"),
    Base62TestData("0000000000001kvnArUVig", "00000000000000000040000000000000"),
    Base62TestData("0000000000002F0LaTPQAw", "00000000000000000080000000000000"),
    Base62TestData("0000000000005k1wlNFHb2", "00000000000000000100000000000000"),
    Base62TestData("000000000000aE32HBlom4", "00000000000000000200000000000000"),
    Base62TestData("000000000000li65pcGMI8", "00000000000000000400000000000000"),
    Base62TestData("000000000000GAcaOpnzqg", "00000000000000000800000000000000"),
    Base62TestData("000000000001naolCOL8Qw", "00000000000000001000000000000000"),
    Base62TestData("000000000002KkMHfDwhH2", "00000000000000002000000000000000"),
    Base62TestData("000000000005uFzovh2zo4", "00000000000000004000000000000000"),
    Base62TestData("00000000000aZl8N0y58M8", "00000000000000008000000000000000"),
    Base62TestData("00000000000lYGhA16ahyg", "00000000000000010000000000000000"),
    Base62TestData("00000000000HXmza2ckz6w", "00000000000000020000000000000000"),
    Base62TestData("00000000001pUJ8k4oF8d2", "00000000000000040000000000000000"),
    Base62TestData("00000000002PPsgE8Nkgq4", "00000000000000080000000000000000"),
    Base62TestData("00000000005FEUxihAEwQ8", "00000000000000100000000000000000"),
    Base62TestData("0000000000bljP4Azbj3Gg", "00000000000000200000000000000000"),
    Base62TestData("0000000000mGDE9b8mC7mw", "00000000000000400000000000000000"),
    Base62TestData("0000000000JnhiimgJeeJ2", "00000000000000800000000000000000"),
    Base62TestData("0000000001sKyAAIxssts4", "00000000000001000000000000000000"),
    Base62TestData("0000000002Vv7bbr4UUWU8", "00000000000002000000000000000000"),
    Base62TestData("0000000005R0emmS9PPTOg", "00000000000004000000000000000000"),
    Base62TestData("000000000bI0sIJKjFFNCw", "00000000000008000000000000000000"),
    Base62TestData("000000000nq0VrtuDllBf2", "00000000000010000000000000000000"),
    Base62TestData("000000000KQ1QSWZgGHcu4", "00000000000020000000000000000000"),
    Base62TestData("000000001vG3HLTYxnooY8", "00000000000040000000000000000000"),
    Base62TestData("0000000031m7pxNX4KMNWg", "00000000000080000000000000000000"),
    Base62TestData("0000000062IeP5BU9vzBSw", "00000000000100000000000000000000"),
    Base62TestData("00000000c5qtEbdOj19dL2", "00000000000200000000000000000000"),
    Base62TestData("00000000oaQXimrCC2irw4", "00000000000400000000000000000000"),
    Base62TestData("00000000MlHUAITfe4AT28", "00000000000800000000000000000000"),
    Base62TestData("00000001yHpPbrMus9bM4g", "00000000001000000000000000000000"),
    Base62TestData("000000037oPEmTyYUiny8w", "00000000002000000000000000000000"),
    Base62TestData("00000006eNFiJN7XOAL6h2", "00000000004000000000000000000000"),
    Base62TestData("0000000ctBkBtAfVDbwcy4", "00000000008000000000000000000000"),
    Base62TestData("0000000oXcFcXavRgn2p68", "00000000010000000000000000000000"),
    Base62TestData("0000000NUpkpUl1IwK4Ocg", "00000000020000000000000000000000"),
    Base62TestData("0000001BOOEPOG3r3u9Cow", "00000000040000000000000000000000"),
    Base62TestData("0000003dDDjFDm6S6YjeN2", "00000000080000000000000000000000"),
    Base62TestData("0000006rhgDlgIdKdWCtA4", "00000000100000000000000000000000"),
    Base62TestData("000000cSyxgGxqrurTeXa8", "00000000200000000000000000000000"),
    Base62TestData("000000pL74xn4QSYTMtUkg", "00000000400000000000000000000000"),
    Base62TestData("000000Pwe94K9HLXNyXOEw", "00000000800000000000000000000000"),
    Base62TestData("000001F2si9ujpxVB7VDj2", "00000001000000000000000000000000"),
    Base62TestData("000003k4UAiYCP5RcfRgC4", "00000002000000000000000000000000"),
    Base62TestData("000006E9PaBXfEbIovIxe8", "00000004000000000000000000000000"),
    Base62TestData("00000dijEldUvinqN1r4sg", "00000008000000000000000000000000"),
    Base62TestData("00000qADiGrP0AKRA2S8Uw", "00000010000000000000000000000000"),
    Base62TestData("00000RbgBmTE1bvJa5KhP2", "00000020000000000000000000000000"),
    Base62TestData("00001ImxcJNi2n1skbuzE4", "00000040000000000000000000000000"),
    Base62TestData("00003qJ4ptAA4K2UEmZ9i8", "00000080000000000000000000000000"),
    Base62TestData("00006Rs8OXba9u5PiJYiAg", "00000100000000000000000000000000"),
    Base62TestData("0000dIUhDUmkiYbEBtWBaw", "00000200000000000000000000000000"),
    Base62TestData("0000rrOzhOIEBWnjcXTcl2", "00000400000000000000000000000000"),
    Base62TestData("0000STD8zDrjdSKCpVMoG4", "00000800000000000000000000000000"),
    Base62TestData("0001LNgh9gSCrLvePRyNm8", "00001000000000000000000000000000"),
    Base62TestData("0003xAwyixLeTx0tFJ7AIg", "00002000000000000000000000000000"),
    Base62TestData("00075b36B5wtN40Xlsfbqw", "00004000000000000000000000000000"),
    Base62TestData("000eam6dcb2XA81UGUumR2", "00008000000000000000000000000000"),
    Base62TestData("000skIcqom5Vag3PnOYJI4", "00010000000000000000000000000000"),
    Base62TestData("000UFqoQMIbQkw7ELDXtq8", "00020000000000000000000000000000"),
    Base62TestData("001PkQNHzqnGF2fjxhUWQg", "00040000000000000000000000000000"),
    Base62TestData("003EFHBp8QLnk4uD4zPTGw", "00080000000000000000000000000000"),
    Base62TestData("007jlpcOhHwKE8Zg99FNn2", "00100000000000000000000000000000"),
    Base62TestData("00eCGOpCzp3vihYwijlAK4", "00200000000000000000000000000000"),
    Base62TestData("00tfnCPf8O70AzX2ACHbu8", "00400000000000000000000000000000"),
    Base62TestData("00WuLfEuhCe1b9U5bfomYg", "00800000000000000000000000000000"),
    Base62TestData("01SZwviYzes2mjOamuMJWw", "01000000000000000000000000000000"),
    Base62TestData("03LZ30BX8sU4IDCkIZztT2", "02000000000000000000000000000000"),
    Base62TestData("07xY61dUgVO9rheFrZ8XM4", "04000000000000000000000000000000"),
    Base62TestData("0f5Wc2rOxRCiSytkTYhVy8", "08000000000000000000000000000000"),
    Base62TestData("0ubSo4TD5JeBL6WFNWzR6g", "10000000000000000000000000000000"),
    Base62TestData("0YnKM9NgbstdwdTlBT9Icw", "20000000000000000000000000000000"),
    Base62TestData("1WLvyjAwmUWr2rMHdMjqp2", "40000000000000000000000000000000"),
    Base62TestData("3Tx16Db2JPSS4TzoryCQO4", "80000000000000000000000000000000"),
    // All but one bit set
    Base62TestData("7N42dgm5tFLK9N8MT7fHC6", "fffffffffffffffffffffffffffffffe"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHC5", "fffffffffffffffffffffffffffffffd"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHC3", "fffffffffffffffffffffffffffffffb"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHBZ", "fffffffffffffffffffffffffffffff7"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHBR", "ffffffffffffffffffffffffffffffef"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHBB", "ffffffffffffffffffffffffffffffdf"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHB5", "ffffffffffffffffffffffffffffffbf"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHA3", "ffffffffffffffffffffffffffffff7f"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHxZ", "fffffffffffffffffffffffffffffeff"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHtR", "fffffffffffffffffffffffffffffdff"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHlB", "fffffffffffffffffffffffffffffbff"),
    Base62TestData("7N42dgm5tFLK9N8MT7fH55", "fffffffffffffffffffffffffffff7ff"),
    Base62TestData("7N42dgm5tFLK9N8MT7fGy3", "ffffffffffffffffffffffffffffefff"),
    Base62TestData("7N42dgm5tFLK9N8MT7fFtZ", "ffffffffffffffffffffffffffffdfff"),
    Base62TestData("7N42dgm5tFLK9N8MT7fDlR", "ffffffffffffffffffffffffffffbfff"),
    Base62TestData("7N42dgm5tFLK9N8MT7fz5B", "ffffffffffffffffffffffffffff7fff"),
    Base62TestData("7N42dgm5tFLK9N8MT7fqz5", "fffffffffffffffffffffffffffeffff"),
    Base62TestData("7N42dgm5tFLK9N8MT7f9w3", "fffffffffffffffffffffffffffdffff"),
    Base62TestData("7N42dgm5tFLK9N8MT7eBpZ", "fffffffffffffffffffffffffffbffff"),
    Base62TestData("7N42dgm5tFLK9N8MT7dvdR", "fffffffffffffffffffffffffff7ffff"),
    Base62TestData("7N42dgm5tFLK9N8MT7biPB", "ffffffffffffffffffffffffffefffff"),
    Base62TestData("7N42dgm5tFLK9N8MT76U35", "ffffffffffffffffffffffffffdfffff"),
    Base62TestData("7N42dgm5tFLK9N8MT6Y6u3", "ffffffffffffffffffffffffffbfffff"),
    Base62TestData("7N42dgm5tFLK9N8MT6GvlZ", "ffffffffffffffffffffffffff7fffff"),
    Base62TestData("7N42dgm5tFLK9N8MT67j5R", "fffffffffffffffffffffffffeffffff"),
    Base62TestData("7N42dgm5tFLK9N8MT4YUzB", "fffffffffffffffffffffffffdffffff"),
    Base62TestData("7N42dgm5tFLK9N8MT2I7x5", "fffffffffffffffffffffffffbffffff"),
    Base62TestData("7N42dgm5tFLK9N8MSYaxs3", "fffffffffffffffffffffffff7ffffff"),
    Base62TestData("7N42dgm5tFLK9N8MSP5nhZ", "ffffffffffffffffffffffffefffffff"),
    Base62TestData("7N42dgm5tFLK9N8MSwV2XR", "ffffffffffffffffffffffffdfffffff"),
    Base62TestData("7N42dgm5tFLK9N8MRWAojB", "ffffffffffffffffffffffffbfffffff"),
    Base62TestData("7N42dgm5tFLK9N8MQLV515", "ffffffffffffffffffffffff7fffffff"),
    Base62TestData("7N42dgm5tFLK9N8MOqAsq3", "fffffffffffffffffffffffeffffffff"),
    Base62TestData("7N42dgm5tFLK9N8MJJVddZ", "fffffffffffffffffffffffdffffffff"),
    Base62TestData("7N42dgm5tFLK9N8MAmAIPR", "fffffffffffffffffffffffbffffffff"),
    Base62TestData("7N42dgm5tFLK9N8MhBVK3B", "fffffffffffffffffffffff7ffffffff"),
    Base62TestData("7N42dgm5tFLK9N8LG6BMv5", "ffffffffffffffffffffffefffffffff"),
    Base62TestData("7N42dgm5tFLK9N8Kt5XRo3", "ffffffffffffffffffffffdfffffffff"),
    Base62TestData("7N42dgm5tFLK9N8I34G19Z", "ffffffffffffffffffffffbfffffffff"),
    Base62TestData("7N42dgm5tFLK9N8Dd26kHR", "ffffffffffffffffffffff7fffffffff"),
    Base62TestData("7N42dgm5tFLK9N8twWWXNB", "fffffffffffffffffffffeffffffffff"),
    Base62TestData("7N42dgm5tFLK9N8aaMEdZ5", "fffffffffffffffffffffdffffffffff"),
    Base62TestData("7N42dgm5tFLK9N7xss2Km3", "fffffffffffffffffffffbffffffffff"),
    Base62TestData("7N42dgm5tFLK9N6i1MPN5Z", "fffffffffffffffffffff7ffffffffff"),
    Base62TestData("7N42dgm5tFLK9N3NaspSzR", "ffffffffffffffffffffefffffffffff"),
    Base62TestData("7N42dgm5tFLK9MYNrNA3xB", "ffffffffffffffffffffdfffffffffff"),
    Base62TestData("7N42dgm5tFLK9MOO0tUpt5", "ffffffffffffffffffffbfffffffffff"),
    Base62TestData("7N42dgm5tFLK9MuP7Qz7k3", "ffffffffffffffffffff7fffffffffff"),
    Base62TestData("7N42dgm5tFLK9LQRmzSx1Z", "fffffffffffffffffffeffffffffffff"),
    Base62TestData("7N42dgm5tFLK9KyVQ2vmrR", "fffffffffffffffffffdffffffffffff"),
    Base62TestData("7N42dgm5tFLK9HZ4MXL1hB", "fffffffffffffffffffbffffffffffff"),
    Base62TestData("7N42dgm5tFLK9CPmGOgkX5", "fffffffffffffffffff7ffffffffffff"),
    Base62TestData("7N42dgm5tFLK9svWuvgYi3", "ffffffffffffffffffefffffffffffff"),
    Base62TestData("7N42dgm5tFLK97T65TieXZ", "ffffffffffffffffffdfffffffffffff"),
    Base62TestData("7N42dgm5tFLK8sDpiFkMjR", "ffffffffffffffffffbfffffffffffff"),
    Base62TestData("7N42dgm5tFLK7881IdpR1B", "ffffffffffffffffff7fffffffffffff"),
    Base62TestData("7N42dgm5tFLK4t7gxjA0r5", "fffffffffffffffffeffffffffffffff"),
    Base62TestData("7N42dgm5tFLJZ95KbvUjg3", "fffffffffffffffffdffffffffffffff"),
    Base62TestData("7N42dgm5tFLJOv2HtUyUTZ", "fffffffffffffffffbffffffffffffff"),
    Base62TestData("7N42dgm5tFLJtcWC4HS8bR", "fffffffffffffffff7ffffffffffffff"),
    Base62TestData("7N42dgm5tFLIMCKrgiuyLB", "ffffffffffffffffefffffffffffffff"),
    Base62TestData("7N42dgm5tFLHpsm5DtJpV5", "ffffffffffffffffdfffffffffffffff"),
    Base62TestData("7N42dgm5tFLEF7zonQd8e3", "ffffffffffffffffbfffffffffffffff"),
    Base62TestData("7N42dgm5tFLzarZZSzayPZ", "ffffffffffffffff7fffffffffffffff"),
    Base62TestData("7N42dgm5tFLob6RcS15q3R", "fffffffffffffffeffffffffffffffff"),
    Base62TestData("7N42dgm5tFL2cqzCQUV8vB", "fffffffffffffffdffffffffffffffff"),
    Base62TestData("7N42dgm5tFKkf40sOIAzp5", "fffffffffffffffbffffffffffffffff"),
    Base62TestData("7N42dgm5tFIUkkS8KjVrc3", "fffffffffffffff7ffffffffffffffff"),
    Base62TestData("7N42dgm5tFG4uSBuBwBaLZ", "ffffffffffffffefffffffffffffffff"),
    Base62TestData("7N42dgm5tFAoPY4cjVWDVR", "ffffffffffffffdfffffffffffffffff"),
    Base62TestData("7N42dgm5tFp3w8ZBKKDAfB", "ffffffffffffffbfffffffffffffffff"),
    Base62TestData("7N42dgm5tF2mSuQqCo1sT5", "ffffffffffffff7fffffffffffffffff"),
    Base62TestData("7N42dgm5tEiZBcy4lENea3", "fffffffffffffeffffffffffffffffff"),
    Base62TestData("7N42dgm5tCQf2BXlOckKHZ", "fffffffffffffdffffffffffffffffff"),
    Base62TestData("7N42dgm5tzUJVqLUJhpNNR", "fffffffffffffbffffffffffffffffff"),
    Base62TestData("7N42dgm5tu3JH4p2zrzTZB", "fffffffffffff7ffffffffffffffffff"),
    Base62TestData("7N42dgm5tilJelFifLU6n5", "ffffffffffffefffffffffffffffffff"),
    Base62TestData("7N42dgm5sUVIiUbNCqyv83", "ffffffffffffdfffffffffffffffffff"),
    Base62TestData("7N42dgm5sa5Gs1eOlJRiDZ", "ffffffffffffbfffffffffffffffffff"),
    Base62TestData("7N42dgm5qEpCKfkPOmsTFR", "ffffffffffff7fffffffffffffffffff"),
    Base62TestData("7N42dgm5nD3vkHwSJBG5JB", "fffffffffffeffffffffffffffffffff"),
    Base62TestData("7N42dgm5hAlgvBUYA66tR5", "fffffffffffdffffffffffffffffffff"),
    Base62TestData("7N42dgm55uUMRqHah4Xg63", "fffffffffffbffffffffffffffffffff"),
    Base62TestData("7N42dgm4Hk3Pz4fxF2EOzZ", "fffffffffff7ffffffffffffffffffff"),
    Base62TestData("7N42dgm3UYlUYlmiqY3VxR", "ffffffffffefffffffffffffffffffff"),
    Base62TestData("7N42dgm2mgW5MTzNYOS9tB", "ffffffffffdfffffffffffffffffffff"),
    Base62TestData("7N42dglZeS6rq00P4wuBl5", "ffffffffffbfffffffffffffffffffff"),
    Base62TestData("7N42dglT04r8GcSRfVJv43", "ffffffffff7fffffffffffffffffffff"),
    Base62TestData("7N42dglGwt6xcCCVCKdivZ", "fffffffffeffffffffffffffffffffff"),
    Base62TestData("7N42dglhzgrkfs74mnaTpR", "fffffffffdffffffffffffffffffffff"),
    Base62TestData("7N42dgktER6Ul75lPD65dB", "fffffffffbffffffffffffffffffffff"),
    Base62TestData("7N42dgiRQ2s4wr1UM8WsP5", "fffffffff7ffffffffffffffffffffff"),
    Base62TestData("7N42dgfEcp8oT4V2FaDe23", "ffffffffefffffffffffffffffffffff"),
    Base62TestData("7N42dg9cV8v3CmHire0KrZ", "ffffffffdfffffffffffffffffffffff"),
    Base62TestData("7N42dfWkmBen4WfNZkLNhR", "ffffffffbfffffffffffffffffffffff"),
    Base62TestData("7N42dfwzfwH005mP5yhSXB", "ffffffff7fffffffffffffffffffffff"),
    Base62TestData("7N42deH31nCfQnARhZk4j5", "fffffffeffffffffffffffffffffffff"),
    Base62TestData("7N42dd20z5sLwY2VGRor03", "fffffffdffffffffffffffffffffffff"),
    Base62TestData("7N42d9HVEv9MU8X4uBxanZ", "fffffffbffffffffffffffffffffffff"),
    Base62TestData("7N42d33LPkxPEuLm65OD9R", "fffffff7ffffffffffffffffffffffff"),
    Base62TestData("7N42cPLsaZjV9cnVj4nyHB", "ffffffefffffffffffffffffffffffff"),
    Base62TestData("7N42cpaOSiS68BD3J1vpN5", "ffffffdfffffffffffffffffffffffff"),
    Base62TestData("7N42bxZygVYs7q7kyVL7Y3", "ffffffbfffffffffffffffffffffffff"),
    Base62TestData("7N429PD14cba535SeKgyjZ", "ffffff7fffffffffffffffffffffffff"),
    Base62TestData("7N426oTWEIAA0j2XAnhp1R", "fffffeffffffffffffffffffffffffff"),
    Base62TestData("7N41ZxrNPLppQOX8hDj6rB", "fffffdffffffffffffffffffffffffff"),
    Base62TestData("7N41LOxwbR35xQLtG9mvh5", "fffffbffffffffffffffffffffffffff"),
    Base62TestData("7N41kmIWU2kqVUoatbtiW3", "fffff7ffffffffffffffffffffffffff"),
    Base62TestData("7N40rt5OkoT7I1Dy3fGUfZ", "ffffefffffffffffffffffffffffffff"),
    Base62TestData("7N3YFFPxb80vgg8jdo86TR", "ffffdfffffffffffffffffffffffffff"),
    Base62TestData("7N3V85iYSAfgmJ7PxF0wbB", "ffffbfffffffffffffffffffffffffff"),
    Base62TestData("7N3O2UfShuIMzF6SccLkL5", "ffff7fffffffffffffffffffffffffff"),
    Base62TestData("7N3zSy9F5jFOZx4XvigXU3", "fffeffffffffffffffffffffffffffff"),
    Base62TestData("7N37xPXeGXzTPh187tiebZ", "fffdffffffffffffffffffffffffffff"),
    Base62TestData("7N2cSpynUfo3uKTtlPkKLR", "fffbffffffffffffffffffffffffffff"),
    Base62TestData("7N0nxyKGkP0mPIE9OxpNVB", "fff7ffffffffffffffffffffffffffff"),
    Base62TestData("7MWIRR9hbYeZvE9wJXzUf5", "ffefffffffffffffffffffffffffffff"),
    Base62TestData("7MPpwrWsUgIeRvagANU6S3", "ffdfffffffffffffffffffffffffffff"),
    Base62TestData("7MAMPDwQkREJzdbKiuyw7Z", "ffbfffffffffffffffffffffffffffff"),
    Base62TestData("7M7xs0HBc3xIYDeHHRRkDR", "ff7fffffffffffffffffffffffffffff"),
    Base62TestData("7Lb2GL36UrjHNtkCwCsXFB", "feffffffffffffffffffffffffffffff"),
    Base62TestData("7Ji3afK8lcRFr9wsa7GdJ5", "fdffffffffffffffffffffffffffffff"),
    Base62TestData("7Fw47f8bcJXAIvU7r86JQ3", "fbffffffffffffffffffffffffffffff"),
    Base62TestData("7xY61dUgVO9rheFrZ8XM3Z", "f7ffffffffffffffffffffffffffffff"),
    Base62TestData("7iS9PbssnWx8oGc75aFQvR", "efffffffffffffffffffffffffffffff"),
    Base62TestData("6OGhr6yPidiwDzfrhe5ZpB", "dfffffffffffffffffffffffffffffff"),
    Base62TestData("5QiwEWLz6KPj7lm5FkWhd5", "bfffffffffffffffffffffffffffffff"),
    Base62TestData("3Tx16Db2JPSS4TzoryCQO3", "7fffffffffffffffffffffffffffffff"),
    // One bit set in each 32 bit dword
    Base62TestData("000001F2si9Qi5PvGULa3n", "00000001000000010000000100000001"),
    Base62TestData("000003k4UAjGAbF1nPwk6K", "00000002000000020000000200000002"),
    Base62TestData("000006E9PaDnank2LF2Edu", "00000004000000040000000400000004"),
    Base62TestData("00000dijElgKkKE5xk5iqY", "00000008000000080000000800000008"),
    Base62TestData("00000qADiGxuFvib4EaARW", "00000010000000100000001000000010"),
    Base62TestData("00000RbgBn4Zl0Am9ilbJS", "00000020000000200000002000000020"),
    Base62TestData("00001ImxcK9YG1aIiAGntK", "00000040000000400000004000000040"),
    Base62TestData("00003qJ4pujXm2lqBbmKXu", "00000080000000800000008000000080"),
    Base62TestData("00006Rs8OYDUI4GRcmJvUY", "00000100000001000000010000000100"),
    Base62TestData("0000dIUhDXhPq9nIoJt1PW", "00000200000002000000020000000200"),
    Base62TestData("0000rrOzhUzEQiLqNsW3FS", "00000400000004000000040000000400"),
    Base62TestData("0000STD8zP9jGBwRAVS7lK", "00000800000008000000080000000800"),
    Base62TestData("0001LNgh9EiDnd3JbRKeHu", "00001000000010000000100000001000"),
    Base62TestData("0003xAwyjiBgKq7snJutoY", "00002000000020000000200000002000"),
    Base62TestData("00075b36CBcxuQeULsYWNW", "00004000000040000000400000004000"),
    Base62TestData("000eam6dfcp4ZGtPwVXTBS", "00008000000080000000800000008000"),
    Base62TestData("000skIcquoO9ZmXF3RVNdK", "00010000000100000001000000010000"),
    Base62TestData("000UFqoQYNCjYJVk7JRAru", "00020000000200000002000000020000"),
    Base62TestData("001PkQNHXBeDXtQEftJaSY", "00040000000400000004000000040000"),
    Base62TestData("003EFHBpVcthUXHiuXslLW", "00080000000800000008000000080000"),
    Base62TestData("007jlpcPQoWzPVoAZUUHxS", "00100000001000000010000000100000"),
    Base62TestData("00eCGOpFGNT9FQNbZPPp5K", "00200000002000000020000000200000"),
    Base62TestData("00tfnCPlnBMjlHAnZFEObu", "00400000004000000040000000400000"),
    Base62TestData("00WuLfEGLdyCHpaLZljCmY", "00800000008000000080000000800000"),
    Base62TestData("01SZwvjnwr7foOlxYGDeJW", "01000000010000000100000001000000"),
    Base62TestData("03LZ30CL2SeuNCH5XngttS", "02000000020000000200000002000000"),
    Base62TestData("07xY61fw5KsZBfobUKwWXK", "04000000040000000400000004000000"),
    Base62TestData("0f5Wc2v2buVZcuMnPv3TVu", "08000000080000000800000008000000"),
    Base62TestData("0ubSo504mZRYoZyLF07NQY", "10000000100000001000000010000000"),
    Base62TestData("0YnKMa08JZJWNZ7xk0fBHW", "20000000200000002000000020000000"),
    Base62TestData("1WLvyk0htZtTBYf4E0vdpS", "40000000400000004000000040000000"),
    Base62TestData("3Tx16E0yXYXNdWu9i10qPK", "80000000800000008000000080000000"),
    // All but one bit set in each DWORD
    Base62TestData("7N42deH31nBTRHjhccuxyK", "fffffffefffffffefffffffefffffffe"),
    Base62TestData("7N42dd20z5s3zBtLvhJnvn", "fffffffdfffffffdfffffffdfffffffd"),
    Base62TestData("7N42d9HVEv8mZpOK7sd3oD", "fffffffbfffffffbfffffffbfffffffb"),
    Base62TestData("7N42d33LPkuZP2uHlNapb9", "fffffff7fffffff7fffffff7fffffff7"),
    Base62TestData("7N42cPLsaZefuhQBOt56Kb", "ffffffefffffffefffffffefffffffef"),
    Base62TestData("7N42cpaOSiGKOMyqJOUvSf", "ffffffdfffffffdfffffffdfffffffdf"),
    Base62TestData("7N42bxZygVBLtLY4Awzk8n", "ffffffbfffffffbfffffffbfffffffbf"),
    Base62TestData("7N429PD14brMNKNmhVSWED", "ffffff7fffffff7fffffff7fffffff7f"),
    Base62TestData("7N426oTWEH7PrIrVGKwbH9", "fffffefffffffefffffffefffffffeff"),
    Base62TestData("7N41ZxrNPItUJDL4unMFMb", "fffffdfffffffdfffffffdfffffffdff"),
    Base62TestData("7N41LOxwbLc5junm5EjDWf", "fffffbfffffffbfffffffbfffffffbff"),
    Base62TestData("7N41kmIWTQCqtbBVibnAgn", "fffff7fffffff7fffffff7fffffff7ff"),
    Base62TestData("7N40rt5Ok1t6MA53HfvsUD", "ffffefffffffefffffffefffffffefff"),
    Base62TestData("7N3YFFPxanatpn1kvnLed9", "ffffdfffffffdfffffffdfffffffdfff"),
    Base62TestData("7N3V85iYR4zcEWTS7EgKOb", "ffffbfffffffbfffffffbfffffffbfff"),
    Base62TestData("7N3O2UfSetmFa6EXmbhO0f", "ffff7fffffff7fffffff7fffffff7fff"),
    Base62TestData("7N3zSy9EZgXAaqb7PfjUon", "fffefffffffefffffffefffffffeffff"),
    Base62TestData("7N37xPXeuS9qb3dsLno7aD", "fffdfffffffdfffffffdfffffffdffff"),
    Base62TestData("7N2cSpynw4x6cji8DDwwJ9", "fffbfffffffbfffffffbfffffffbffff"),
    Base62TestData("7N0nxyKFytisePruo9NlQb", "fff7fffffff7fffffff7fffffff7ffff"),
    Base62TestData("7MWIRR9fDgPajRKbTcl04f", "ffefffffffefffffffefffffffefffff"),
    Base62TestData("7MPpwrWpMRSAtWlAThqiwn", "ffdfffffffdfffffffdfffffffdfffff"),
    Base62TestData("7MAMPDwK63ZqO5yoTrATqD", "ffbfffffffbfffffffbfffffffbfffff"),
    Base62TestData("7M7xs0HoIsd7snY0TLW5f9", "ff7fffffff7fffffff7fffffff7fffff"),
    Base62TestData("7Lb2GL2HXeEuKYNeUqCsSb", "fefffffffefffffffefffffffeffffff"),
    Base62TestData("7Ji3afJkqNxfmarGVJZe8f", "fdfffffffdfffffffdfffffffdffffff"),
    Base62TestData("7Fw47f6znViKyxKAYmIKEn", "fbfffffffbfffffffbfffffffbffffff"),
    Base62TestData("7xY61dR3iaPKXimp3CbNGD", "f7fffffff7fffffff7fffffff7ffffff"),
    Base62TestData("7iS9Pbm16FTLKNA1e77TL9", "efffffffefffffffefffffffefffffff"),
    Base62TestData("6OGhr6lWJG1NlO1fz705Ub", "dfffffffdfffffffdfffffffdfffffff"),
    Base62TestData("5QiwEWlNZGhQxOTIf6Kucf", "bfffffffbfffffffbfffffffbfffffff"),
    Base62TestData("3Tx16ClwvGNWVQEDB6fgMn", "7fffffff7fffffff7fffffff7fffffff"),
    // One bit set in each byte
    Base62TestData("01TrY6CZi6JaolPzVQ3HW1", "01010101010101010101010101010101"),
    Base62TestData("03MTWdfYAdskMHF9RG7pS2", "02020202020202020202020202020202"),
    Base62TestData("07zNSqvXaqUFzpkjJmePK4", "04040404040404040404040404040404"),
    Base62TestData("0f9BKR1UkRPl8OEDsItFu8", "08080808080808080808080808080808"),
    Base62TestData("0ujdvI3OFJEGhDjgVqXkYg", "10101010101010101010101010101010"),
    Base62TestData("0YCr1q7DltjmzgCxQRUFWw", "20202020202020202020202020202020"),
    Base62TestData("1XeS2QfgGWCJ8xf5HJPlT2", "40404040404040404040404040404040"),
    Base62TestData("3UtK5GuxnTfsh4ubptEHM4", "80808080808080808080808080808080"),
    // All but one bit set in each byte
    Base62TestData("7LaAf9J6bz2zLrjcXhbZG6", "fefefefefefefefefefefefefefefefe"),
    Base62TestData("7Jh8h366Tsjpn5tD1r8hK5", "fdfdfdfdfdfdfdfdfdfdfdfdfdfdfdfd"),
    Base62TestData("7FuekPQ8jeR4AnOt9L0RS3", "fbfbfbfbfbfbfbfbfbfbfbfbfbfbfbfb"),
    Base62TestData("7xUqspkb8NWp0Yu9qoM27Z", "f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7"),
    Base62TestData("7iKOHyigNW73S9PvXGimDR", "efefefefefefefefefefefefefefefef"),
    Base62TestData("6OrBbQes8csnAwwf2fl1FB", "dfdfdfdfdfdfdfdfdfdfdfdfdfdfdfdf"),
    Base62TestData("5PPaaq6OMJ911fTHbnqlJ5", "bfbfbfbfbfbfbfbfbfbfbfbfbfbfbfbf"),
    Base62TestData("3SAi7zRy5MwhSIEBtDAZQ3", "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f"),
    // Shift in bits from the left
    Base62TestData("0000000000000000000001", "00000000000000000000000000000001"),
    Base62TestData("0000000000000000000003", "00000000000000000000000000000003"),
    Base62TestData("0000000000000000000007", "00000000000000000000000000000007"),
    Base62TestData("000000000000000000000f", "0000000000000000000000000000000f"),
    Base62TestData("000000000000000000000v", "0000000000000000000000000000001f"),
    Base62TestData("0000000000000000000011", "0000000000000000000000000000003f"),
    Base62TestData("0000000000000000000023", "0000000000000000000000000000007f"),
    Base62TestData("0000000000000000000047", "000000000000000000000000000000ff"),
    Base62TestData("000000000000000000008f", "000000000000000000000000000001ff"),
    Base62TestData("00000000000000000000gv", "000000000000000000000000000003ff"),
    Base62TestData("00000000000000000000x1", "000000000000000000000000000007ff"),
    Base62TestData("0000000000000000000143", "00000000000000000000000000000fff"),
    Base62TestData("0000000000000000000287", "00000000000000000000000000001fff"),
    Base62TestData("00000000000000000004gf", "00000000000000000000000000003fff"),
    Base62TestData("00000000000000000008wv", "00000000000000000000000000007fff"),
    Base62TestData("0000000000000000000h31", "0000000000000000000000000000ffff"),
    Base62TestData("0000000000000000000y63", "0000000000000000000000000001ffff"),
    Base62TestData("00000000000000000016c7", "0000000000000000000000000003ffff"),
    Base62TestData("0000000000000000002cof", "0000000000000000000000000007ffff"),
    Base62TestData("0000000000000000004oMv", "000000000000000000000000000fffff"),
    Base62TestData("0000000000000000008Nz1", "000000000000000000000000001fffff"),
    Base62TestData("000000000000000000hB83", "000000000000000000000000003fffff"),
    Base62TestData("000000000000000000zcg7", "000000000000000000000000007fffff"),
    Base62TestData("0000000000000000018owf", "00000000000000000000000000ffffff"),
    Base62TestData("000000000000000002gN2v", "00000000000000000000000001ffffff"),
    Base62TestData("000000000000000004xA51", "00000000000000000000000003ffffff"),
    Base62TestData("0000000000000000095aa3", "00000000000000000000000007ffffff"),
    Base62TestData("00000000000000000iakk7", "0000000000000000000000000fffffff"),
    Base62TestData("00000000000000000AkEEf", "0000000000000000000000001fffffff"),
    Base62TestData("00000000000000001aFjiv", "0000000000000000000000003fffffff"),
    Base62TestData("00000000000000002lkCB1", "0000000000000000000000007fffffff"),
    Base62TestData("00000000000000004GFfc3", "000000000000000000000000ffffffff"),
    Base62TestData("00000000000000009nkuo7", "000000000000000000000001ffffffff"),
    Base62TestData("0000000000000000iKEYMf", "000000000000000000000003ffffffff"),
    Base62TestData("0000000000000000BvjXyv", "000000000000000000000007ffffffff"),
    Base62TestData("0000000000000001d0DV71", "00000000000000000000000fffffffff"),
    Base62TestData("0000000000000002q1hQe3", "00000000000000000000001fffffffff"),
    Base62TestData("0000000000000004Q2zGs7", "00000000000000000000003fffffffff"),
    Base62TestData("0000000000000009G59mUf", "00000000000000000000007fffffffff"),
    Base62TestData("000000000000000jmaiJOv", "0000000000000000000000ffffffffff"),
    Base62TestData("000000000000000CIkBtD1", "0000000000000000000001ffffffffff"),
    Base62TestData("000000000000001fqFcXg3", "0000000000000000000003ffffffffff"),
    Base62TestData("000000000000002uRkpUw7", "0000000000000000000007ffffffffff"),
    Base62TestData("000000000000004ZIEPP2f", "000000000000000000000fffffffffff"),
    Base62TestData("000000000000009ZrjFE4v", "000000000000000000001fffffffffff"),
    Base62TestData("00000000000000jYSDli91", "000000000000000000003fffffffffff"),
    Base62TestData("00000000000000DXLgGAi3", "000000000000000000007fffffffffff"),
    Base62TestData("00000000000001hVwxnaA7", "00000000000000000000ffffffffffff"),
    Base62TestData("00000000000002zR34Klaf", "00000000000000000001ffffffffffff"),
    Base62TestData("000000000000059I69uGkv", "00000000000000000003ffffffffffff"),
    Base62TestData("0000000000000ajqciZmF1", "00000000000000000007ffffffffffff"),
    Base62TestData("0000000000000kCQoBYJk3", "0000000000000000000fffffffffffff"),
    Base62TestData("0000000000000FfGNdXsE7", "0000000000000000001fffffffffffff"),
    Base62TestData("0000000000001kvnArUVif", "0000000000000000003fffffffffffff"),
    Base62TestData("0000000000002F0LaTPQAv", "0000000000000000007fffffffffffff"),
    Base62TestData("0000000000005k1wlNFHb1", "000000000000000000ffffffffffffff"),
    Base62TestData("000000000000aE32HBlom3", "000000000000000001ffffffffffffff"),
    Base62TestData("000000000000li65pcGMI7", "000000000000000003ffffffffffffff"),
    Base62TestData("000000000000GAcaOpnzqf", "000000000000000007ffffffffffffff"),
    Base62TestData("000000000001naolCOL8Qv", "00000000000000000fffffffffffffff"),
    Base62TestData("000000000002KkMHfDwhH1", "00000000000000001fffffffffffffff"),
    Base62TestData("000000000005uFzovh2zo3", "00000000000000003fffffffffffffff"),
    Base62TestData("00000000000aZl8N0y58M7", "00000000000000007fffffffffffffff"),
    Base62TestData("00000000000lYGhA16ahyf", "0000000000000000ffffffffffffffff"),
    Base62TestData("00000000000HXmza2ckz6v", "0000000000000001ffffffffffffffff"),
    Base62TestData("00000000001pUJ8k4oF8d1", "0000000000000003ffffffffffffffff"),
    Base62TestData("00000000002PPsgE8Nkgq3", "0000000000000007ffffffffffffffff"),
    Base62TestData("00000000005FEUxihAEwQ7", "000000000000000fffffffffffffffff"),
    Base62TestData("0000000000bljP4Azbj3Gf", "000000000000001fffffffffffffffff"),
    Base62TestData("0000000000mGDE9b8mC7mv", "000000000000003fffffffffffffffff"),
    Base62TestData("0000000000JnhiimgJeeJ1", "000000000000007fffffffffffffffff"),
    Base62TestData("0000000001sKyAAIxssts3", "00000000000000ffffffffffffffffff"),
    Base62TestData("0000000002Vv7bbr4UUWU7", "00000000000001ffffffffffffffffff"),
    Base62TestData("0000000005R0emmS9PPTOf", "00000000000003ffffffffffffffffff"),
    Base62TestData("000000000bI0sIJKjFFNCv", "00000000000007ffffffffffffffffff"),
    Base62TestData("000000000nq0VrtuDllBf1", "0000000000000fffffffffffffffffff"),
    Base62TestData("000000000KQ1QSWZgGHcu3", "0000000000001fffffffffffffffffff"),
    Base62TestData("000000001vG3HLTYxnooY7", "0000000000003fffffffffffffffffff"),
    Base62TestData("0000000031m7pxNX4KMNWf", "0000000000007fffffffffffffffffff"),
    Base62TestData("0000000062IeP5BU9vzBSv", "000000000000ffffffffffffffffffff"),
    Base62TestData("00000000c5qtEbdOj19dL1", "000000000001ffffffffffffffffffff"),
    Base62TestData("00000000oaQXimrCC2irw3", "000000000003ffffffffffffffffffff"),
    Base62TestData("00000000MlHUAITfe4AT27", "000000000007ffffffffffffffffffff"),
    Base62TestData("00000001yHpPbrMus9bM4f", "00000000000fffffffffffffffffffff"),
    Base62TestData("000000037oPEmTyYUiny8v", "00000000001fffffffffffffffffffff"),
    Base62TestData("00000006eNFiJN7XOAL6h1", "00000000003fffffffffffffffffffff"),
    Base62TestData("0000000ctBkBtAfVDbwcy3", "00000000007fffffffffffffffffffff"),
    Base62TestData("0000000oXcFcXavRgn2p67", "0000000000ffffffffffffffffffffff"),
    Base62TestData("0000000NUpkpUl1IwK4Ocf", "0000000001ffffffffffffffffffffff"),
    Base62TestData("0000001BOOEPOG3r3u9Cov", "0000000003ffffffffffffffffffffff"),
    Base62TestData("0000003dDDjFDm6S6YjeN1", "0000000007ffffffffffffffffffffff"),
    Base62TestData("0000006rhgDlgIdKdWCtA3", "000000000fffffffffffffffffffffff"),
    Base62TestData("000000cSyxgGxqrurTeXa7", "000000001fffffffffffffffffffffff"),
    Base62TestData("000000pL74xn4QSYTMtUkf", "000000003fffffffffffffffffffffff"),
    Base62TestData("000000Pwe94K9HLXNyXOEv", "000000007fffffffffffffffffffffff"),
    Base62TestData("000001F2si9ujpxVB7VDj1", "00000000ffffffffffffffffffffffff"),
    Base62TestData("000003k4UAiYCP5RcfRgC3", "00000001ffffffffffffffffffffffff"),
    Base62TestData("000006E9PaBXfEbIovIxe7", "00000003ffffffffffffffffffffffff"),
    Base62TestData("00000dijEldUvinqN1r4sf", "00000007ffffffffffffffffffffffff"),
    Base62TestData("00000qADiGrP0AKRA2S8Uv", "0000000fffffffffffffffffffffffff"),
    Base62TestData("00000RbgBmTE1bvJa5KhP1", "0000001fffffffffffffffffffffffff"),
    Base62TestData("00001ImxcJNi2n1skbuzE3", "0000003fffffffffffffffffffffffff"),
    Base62TestData("00003qJ4ptAA4K2UEmZ9i7", "0000007fffffffffffffffffffffffff"),
    Base62TestData("00006Rs8OXba9u5PiJYiAf", "000000ffffffffffffffffffffffffff"),
    Base62TestData("0000dIUhDUmkiYbEBtWBav", "000001ffffffffffffffffffffffffff"),
    Base62TestData("0000rrOzhOIEBWnjcXTcl1", "000003ffffffffffffffffffffffffff"),
    Base62TestData("0000STD8zDrjdSKCpVMoG3", "000007ffffffffffffffffffffffffff"),
    Base62TestData("0001LNgh9gSCrLvePRyNm7", "00000fffffffffffffffffffffffffff"),
    Base62TestData("0003xAwyixLeTx0tFJ7AIf", "00001fffffffffffffffffffffffffff"),
    Base62TestData("00075b36B5wtN40Xlsfbqv", "00003fffffffffffffffffffffffffff"),
    Base62TestData("000eam6dcb2XA81UGUumR1", "00007fffffffffffffffffffffffffff"),
    Base62TestData("000skIcqom5Vag3PnOYJI3", "0000ffffffffffffffffffffffffffff"),
    Base62TestData("000UFqoQMIbQkw7ELDXtq7", "0001ffffffffffffffffffffffffffff"),
    Base62TestData("001PkQNHzqnGF2fjxhUWQf", "0003ffffffffffffffffffffffffffff"),
    Base62TestData("003EFHBp8QLnk4uD4zPTGv", "0007ffffffffffffffffffffffffffff"),
    Base62TestData("007jlpcOhHwKE8Zg99FNn1", "000fffffffffffffffffffffffffffff"),
    Base62TestData("00eCGOpCzp3vihYwijlAK3", "001fffffffffffffffffffffffffffff"),
    Base62TestData("00tfnCPf8O70AzX2ACHbu7", "003fffffffffffffffffffffffffffff"),
    Base62TestData("00WuLfEuhCe1b9U5bfomYf", "007fffffffffffffffffffffffffffff"),
    Base62TestData("01SZwviYzes2mjOamuMJWv", "00ffffffffffffffffffffffffffffff"),
    Base62TestData("03LZ30BX8sU4IDCkIZztT1", "01ffffffffffffffffffffffffffffff"),
    Base62TestData("07xY61dUgVO9rheFrZ8XM3", "03ffffffffffffffffffffffffffffff"),
    Base62TestData("0f5Wc2rOxRCiSytkTYhVy7", "07ffffffffffffffffffffffffffffff"),
    Base62TestData("0ubSo4TD5JeBL6WFNWzR6f", "0fffffffffffffffffffffffffffffff"),
    Base62TestData("0YnKM9NgbstdwdTlBT9Icv", "1fffffffffffffffffffffffffffffff"),
    Base62TestData("1WLvyjAwmUWr2rMHdMjqp1", "3fffffffffffffffffffffffffffffff"),
    Base62TestData("3Tx16Db2JPSS4TzoryCQO3", "7fffffffffffffffffffffffffffffff"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHC7", "ffffffffffffffffffffffffffffffff"),
    // Shift in bits from the right
    Base62TestData("3Tx16Db2JPSS4TzoryCQO4", "80000000000000000000000000000000"),
    Base62TestData("5QiwEWLz6KPj7lm5FkWhd6", "c0000000000000000000000000000000"),
    Base62TestData("6OGhr6yPidiwDzfrhe5ZpC", "e0000000000000000000000000000000"),
    Base62TestData("7iS9PbssnWx8oGc75aFQvS", "f0000000000000000000000000000000"),
    Base62TestData("7xY61dUgVO9rheFrZ8XM40", "f8000000000000000000000000000000"),
    Base62TestData("7Fw47f8bcJXAIvU7r86JQ4", "fc000000000000000000000000000000"),
    Base62TestData("7Ji3afK8lcRFr9wsa7GdJ6", "fe000000000000000000000000000000"),
    Base62TestData("7Lb2GL36UrjHNtkCwCsXFC", "ff000000000000000000000000000000"),
    Base62TestData("7M7xs0HBc3xIYDeHHRRkDS", "ff800000000000000000000000000000"),
    Base62TestData("7MAMPDwQkREJzdbKiuyw80", "ffc00000000000000000000000000000"),
    Base62TestData("7MPpwrWsUgIeRvagANU6S4", "ffe00000000000000000000000000000"),
    Base62TestData("7MWIRR9hbYeZvE9wJXzUf6", "fff00000000000000000000000000000"),
    Base62TestData("7N0nxyKGkP0mPIE9OxpNVC", "fff80000000000000000000000000000"),
    Base62TestData("7N2cSpynUfo3uKTtlPkKLS", "fffc0000000000000000000000000000"),
    Base62TestData("7N37xPXeGXzTPh187tiec0", "fffe0000000000000000000000000000"),
    Base62TestData("7N3zSy9F5jFOZx4XvigXU4", "ffff0000000000000000000000000000"),
    Base62TestData("7N3O2UfShuIMzF6SccLkL6", "ffff8000000000000000000000000000"),
    Base62TestData("7N3V85iYSAfgmJ7PxF0wbC", "ffffc000000000000000000000000000"),
    Base62TestData("7N3YFFPxb80vgg8jdo86TS", "ffffe000000000000000000000000000"),
    Base62TestData("7N40rt5OkoT7I1Dy3fGUg0", "fffff000000000000000000000000000"),
    Base62TestData("7N41kmIWU2kqVUoatbtiW4", "fffff800000000000000000000000000"),
    Base62TestData("7N41LOxwbR35xQLtG9mvh6", "fffffc00000000000000000000000000"),
    Base62TestData("7N41ZxrNPLppQOX8hDj6rC", "fffffe00000000000000000000000000"),
    Base62TestData("7N426oTWEIAA0j2XAnhp1S", "ffffff00000000000000000000000000"),
    Base62TestData("7N429PD14cba535SeKgyk0", "ffffff80000000000000000000000000"),
    Base62TestData("7N42bxZygVYs7q7kyVL7Y4", "ffffffc0000000000000000000000000"),
    Base62TestData("7N42cpaOSiS68BD3J1vpN6", "ffffffe0000000000000000000000000"),
    Base62TestData("7N42cPLsaZjV9cnVj4nyHC", "fffffff0000000000000000000000000"),
    Base62TestData("7N42d33LPkxPEuLm65OD9S", "fffffff8000000000000000000000000"),
    Base62TestData("7N42d9HVEv9MU8X4uBxao0", "fffffffc000000000000000000000000"),
    Base62TestData("7N42dd20z5sLwY2VGRor04", "fffffffe000000000000000000000000"),
    Base62TestData("7N42deH31nCfQnARhZk4j6", "ffffffff000000000000000000000000"),
    Base62TestData("7N42dfwzfwH005mP5yhSXC", "ffffffff800000000000000000000000"),
    Base62TestData("7N42dfWkmBen4WfNZkLNhS", "ffffffffc00000000000000000000000"),
    Base62TestData("7N42dg9cV8v3CmHire0Ks0", "ffffffffe00000000000000000000000"),
    Base62TestData("7N42dgfEcp8oT4V2FaDe24", "fffffffff00000000000000000000000"),
    Base62TestData("7N42dgiRQ2s4wr1UM8WsP6", "fffffffff80000000000000000000000"),
    Base62TestData("7N42dgktER6Ul75lPD65dC", "fffffffffc0000000000000000000000"),
    Base62TestData("7N42dglhzgrkfs74mnaTpS", "fffffffffe0000000000000000000000"),
    Base62TestData("7N42dglGwt6xcCCVCKdiw0", "ffffffffff0000000000000000000000"),
    Base62TestData("7N42dglT04r8GcSRfVJv44", "ffffffffff8000000000000000000000"),
    Base62TestData("7N42dglZeS6rq00P4wuBl6", "ffffffffffc000000000000000000000"),
    Base62TestData("7N42dgm2mgW5MTzNYOS9tC", "ffffffffffe000000000000000000000"),
    Base62TestData("7N42dgm3UYlUYlmiqY3VxS", "fffffffffff000000000000000000000"),
    Base62TestData("7N42dgm4Hk3Pz4fxF2EOA0", "fffffffffff800000000000000000000"),
    Base62TestData("7N42dgm55uUMRqHah4Xg64", "fffffffffffc00000000000000000000"),
    Base62TestData("7N42dgm5hAlgvBUYA66tR6", "fffffffffffe00000000000000000000"),
    Base62TestData("7N42dgm5nD3vkHwSJBG5JC", "ffffffffffff00000000000000000000"),
    Base62TestData("7N42dgm5qEpCKfkPOmsTFS", "ffffffffffff80000000000000000000"),
    Base62TestData("7N42dgm5sa5Gs1eOlJRiE0", "ffffffffffffc0000000000000000000"),
    Base62TestData("7N42dgm5sUVIiUbNCqyv84", "ffffffffffffe0000000000000000000"),
    Base62TestData("7N42dgm5tilJelFifLU6n6", "fffffffffffff0000000000000000000"),
    Base62TestData("7N42dgm5tu3JH4p2zrzTZC", "fffffffffffff8000000000000000000"),
    Base62TestData("7N42dgm5tzUJVqLUJhpNNS", "fffffffffffffc000000000000000000"),
    Base62TestData("7N42dgm5tCQf2BXlOckKI0", "fffffffffffffe000000000000000000"),
    Base62TestData("7N42dgm5tEiZBcy4lENea4", "ffffffffffffff000000000000000000"),
    Base62TestData("7N42dgm5tF2mSuQqCo1sT6", "ffffffffffffff800000000000000000"),
    Base62TestData("7N42dgm5tFp3w8ZBKKDAfC", "ffffffffffffffc00000000000000000"),
    Base62TestData("7N42dgm5tFAoPY4cjVWDVS", "ffffffffffffffe00000000000000000"),
    Base62TestData("7N42dgm5tFG4uSBuBwBaM0", "fffffffffffffff00000000000000000"),
    Base62TestData("7N42dgm5tFIUkkS8KjVrc4", "fffffffffffffff80000000000000000"),
    Base62TestData("7N42dgm5tFKkf40sOIAzp6", "fffffffffffffffc0000000000000000"),
    Base62TestData("7N42dgm5tFL2cqzCQUV8vC", "fffffffffffffffe0000000000000000"),
    Base62TestData("7N42dgm5tFLob6RcS15q3S", "ffffffffffffffff0000000000000000"),
    Base62TestData("7N42dgm5tFLzarZZSzayQ0", "ffffffffffffffff8000000000000000"),
    Base62TestData("7N42dgm5tFLEF7zonQd8e4", "ffffffffffffffffc000000000000000"),
    Base62TestData("7N42dgm5tFLHpsm5DtJpV6", "ffffffffffffffffe000000000000000"),
    Base62TestData("7N42dgm5tFLIMCKrgiuyLC", "fffffffffffffffff000000000000000"),
    Base62TestData("7N42dgm5tFLJtcWC4HS8bS", "fffffffffffffffff800000000000000"),
    Base62TestData("7N42dgm5tFLJOv2HtUyUU0", "fffffffffffffffffc00000000000000"),
    Base62TestData("7N42dgm5tFLJZ95KbvUjg4", "fffffffffffffffffe00000000000000"),
    Base62TestData("7N42dgm5tFLK4t7gxjA0r6", "ffffffffffffffffff00000000000000"),
    Base62TestData("7N42dgm5tFLK7881IdpR1C", "ffffffffffffffffff80000000000000"),
    Base62TestData("7N42dgm5tFLK8sDpiFkMjS", "ffffffffffffffffffc0000000000000"),
    Base62TestData("7N42dgm5tFLK97T65TieY0", "ffffffffffffffffffe0000000000000"),
    Base62TestData("7N42dgm5tFLK9svWuvgYi4", "fffffffffffffffffff0000000000000"),
    Base62TestData("7N42dgm5tFLK9CPmGOgkX6", "fffffffffffffffffff8000000000000"),
    Base62TestData("7N42dgm5tFLK9HZ4MXL1hC", "fffffffffffffffffffc000000000000"),
    Base62TestData("7N42dgm5tFLK9KyVQ2vmrS", "fffffffffffffffffffe000000000000"),
    Base62TestData("7N42dgm5tFLK9LQRmzSx20", "ffffffffffffffffffff000000000000"),
    Base62TestData("7N42dgm5tFLK9MuP7Qz7k4", "ffffffffffffffffffff800000000000"),
    Base62TestData("7N42dgm5tFLK9MOO0tUpt6", "ffffffffffffffffffffc00000000000"),
    Base62TestData("7N42dgm5tFLK9MYNrNA3xC", "ffffffffffffffffffffe00000000000"),
    Base62TestData("7N42dgm5tFLK9N3NaspSzS", "fffffffffffffffffffff00000000000"),
    Base62TestData("7N42dgm5tFLK9N6i1MPN60", "fffffffffffffffffffff80000000000"),
    Base62TestData("7N42dgm5tFLK9N7xss2Km4", "fffffffffffffffffffffc0000000000"),
    Base62TestData("7N42dgm5tFLK9N8aaMEdZ6", "fffffffffffffffffffffe0000000000"),
    Base62TestData("7N42dgm5tFLK9N8twWWXNC", "ffffffffffffffffffffff0000000000"),
    Base62TestData("7N42dgm5tFLK9N8Dd26kHS", "ffffffffffffffffffffff8000000000"),
    Base62TestData("7N42dgm5tFLK9N8I34G1a0", "ffffffffffffffffffffffc000000000"),
    Base62TestData("7N42dgm5tFLK9N8Kt5XRo4", "ffffffffffffffffffffffe000000000"),
    Base62TestData("7N42dgm5tFLK9N8LG6BMv6", "fffffffffffffffffffffff000000000"),
    Base62TestData("7N42dgm5tFLK9N8MhBVK3C", "fffffffffffffffffffffff800000000"),
    Base62TestData("7N42dgm5tFLK9N8MAmAIPS", "fffffffffffffffffffffffc00000000"),
    Base62TestData("7N42dgm5tFLK9N8MJJVde0", "fffffffffffffffffffffffe00000000"),
    Base62TestData("7N42dgm5tFLK9N8MOqAsq4", "ffffffffffffffffffffffff00000000"),
    Base62TestData("7N42dgm5tFLK9N8MQLV516", "ffffffffffffffffffffffff80000000"),
    Base62TestData("7N42dgm5tFLK9N8MRWAojC", "ffffffffffffffffffffffffc0000000"),
    Base62TestData("7N42dgm5tFLK9N8MSwV2XS", "ffffffffffffffffffffffffe0000000"),
    Base62TestData("7N42dgm5tFLK9N8MSP5ni0", "fffffffffffffffffffffffff0000000"),
    Base62TestData("7N42dgm5tFLK9N8MSYaxs4", "fffffffffffffffffffffffff8000000"),
    Base62TestData("7N42dgm5tFLK9N8MT2I7x6", "fffffffffffffffffffffffffc000000"),
    Base62TestData("7N42dgm5tFLK9N8MT4YUzC", "fffffffffffffffffffffffffe000000"),
    Base62TestData("7N42dgm5tFLK9N8MT67j5S", "ffffffffffffffffffffffffff000000"),
    Base62TestData("7N42dgm5tFLK9N8MT6Gvm0", "ffffffffffffffffffffffffff800000"),
    Base62TestData("7N42dgm5tFLK9N8MT6Y6u4", "ffffffffffffffffffffffffffc00000"),
    Base62TestData("7N42dgm5tFLK9N8MT76U36", "ffffffffffffffffffffffffffe00000"),
    Base62TestData("7N42dgm5tFLK9N8MT7biPC", "fffffffffffffffffffffffffff00000"),
    Base62TestData("7N42dgm5tFLK9N8MT7dvdS", "fffffffffffffffffffffffffff80000"),
    Base62TestData("7N42dgm5tFLK9N8MT7eBq0", "fffffffffffffffffffffffffffc0000"),
    Base62TestData("7N42dgm5tFLK9N8MT7f9w4", "fffffffffffffffffffffffffffe0000"),
    Base62TestData("7N42dgm5tFLK9N8MT7fqz6", "ffffffffffffffffffffffffffff0000"),
    Base62TestData("7N42dgm5tFLK9N8MT7fz5C", "ffffffffffffffffffffffffffff8000"),
    Base62TestData("7N42dgm5tFLK9N8MT7fDlS", "ffffffffffffffffffffffffffffc000"),
    Base62TestData("7N42dgm5tFLK9N8MT7fFu0", "ffffffffffffffffffffffffffffe000"),
    Base62TestData("7N42dgm5tFLK9N8MT7fGy4", "fffffffffffffffffffffffffffff000"),
    Base62TestData("7N42dgm5tFLK9N8MT7fH56", "fffffffffffffffffffffffffffff800"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHlC", "fffffffffffffffffffffffffffffc00"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHtS", "fffffffffffffffffffffffffffffe00"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHy0", "ffffffffffffffffffffffffffffff00"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHA4", "ffffffffffffffffffffffffffffff80"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHB6", "ffffffffffffffffffffffffffffffc0"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHBC", "ffffffffffffffffffffffffffffffe0"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHBS", "fffffffffffffffffffffffffffffff0"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHC0", "fffffffffffffffffffffffffffffff8"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHC4", "fffffffffffffffffffffffffffffffc"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHC6", "fffffffffffffffffffffffffffffffe"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHC7", "ffffffffffffffffffffffffffffffff"),
    // Misc patterns
    Base62TestData("0000000000000000000000", "00000000000000000000000000000000"),
    Base62TestData("7MWcKsjukco48au9ic33u8", "ffeeddccbbaa99887766554433221100"),
    Base62TestData("007PsO2B9tnG1CEDAVcE7Z", "00112233445566778899aabbccddeeff"),
    // Shift in max base62 number ("7N42dgm5tFLK9N8MT7fHC7") from the right
    Base62TestData("0000000000000000000007", "00000000000000000000000000000007"),
    Base62TestData("000000000000000000007N", "000000000000000000000000000001e3"),
    Base62TestData("00000000000000000007N4", "000000000000000000000000000074fe"),
    Base62TestData("0000000000000000007N42", "000000000000000000000000001c5586"),
    Base62TestData("000000000000000007N42d", "00000000000000000000000006dcb681"),
    Base62TestData("00000000000000007N42dg", "000000000000000000000001a974334e"),
    Base62TestData("0000000000000007N42dgm", "0000000000000000000000670a246cfa"),
    Base62TestData("000000000000007N42dgm5", "0000000000000000000018f474d26491"),
    Base62TestData("00000000000007N42dgm5t", "000000000000000000060b344af45b3b"),
    Base62TestData("0000000000007N42dgm5tF", "00000000000000000176b6aa272e1873"),
    Base62TestData("000000000007N42dgm5tFL", "00000000000000005ac03d357d29ec09"),
    Base62TestData("00000000007N42dgm5tFLK", "0000000000000015fa8ed2f450272a5c"),
    Base62TestData("0000000007N42dgm5tFLK9", "0000000000000552ae97172b697c4251"),
    Base62TestData("000000007N42dgm5tFLK9N", "0000000000014a0648979c838c180fcf"),
    Base62TestData("00000007N42dgm5tFLK9N8", "00000000004fed8594b7e7dbedd3d42a"),
    Base62TestData("0000007N42dgm5tFLK9N8M", "00000000135b865a048a2743994d625c"),
    Base62TestData("000007N42dgm5tFLK9N8MT", "00000004b02a89cd1975825f20bdd27f"),
    Base62TestData("00007N42dgm5tFLK9N8MT7", "00000122aa4d5fac2a759309edf8fac9"),
    Base62TestData("0007N42dgm5tFLK9N8MT7f", "000046653ebd2bb248799c67a24cbcbd"),
    Base62TestData("007N42dgm5tFLK9N8MT7fH", "00110c8531d0952d8d73e1194e95b5f1"),
    Base62TestData("07N42dgm5tFLK9N8MT7fHC", "04210842108421084210842108421084"),
    Base62TestData("7N42dgm5tFLK9N8MT7fHC7", "ffffffffffffffffffffffffffffffff"),
];
