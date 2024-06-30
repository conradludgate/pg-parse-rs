#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
// #![feature(linkage)]
// extern "C" {
//     fn abort() -> !;
//     static mut _DefaultRuneLocale: _RuneLocale;
//     fn __maskrune(_: __darwin_ct_rune_t, _: libc::c_ulong) -> libc::c_int;
//     fn __tolower(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;
//     fn palloc(size: usize) -> *mut libc::c_void;
//     fn errmsg(fmt: *const libc::c_char, _: ...) -> libc::c_int;
//     fn errcode(sqlerrcode: libc::c_int) -> libc::c_int;
//     fn errfinish(
//         filename: *const libc::c_char,
//         lineno: libc::c_int,
//         funcname: *const libc::c_char,
//     );
//     fn errstart(elevel: libc::c_int, domain: *const libc::c_char) -> bool;
//     fn pg_mbcliplen(
//         mbstr: *const libc::c_char,
//         len: libc::c_int,
//         limit: libc::c_int,
//     ) -> libc::c_int;
//     fn pg_database_encoding_max_length() -> libc::c_int;
// }
use super::*;
pub type __u32_t = libc::c_uint;
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
// pub type isize = __darwin_size_t;
// pub type bool = libc::c_uchar;
// pub type usize = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __u32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: libc::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [libc::c_char; 14],
    pub __mask: __u32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [libc::c_char; 8],
    pub __encoding: [libc::c_char; 32],
    pub __sgetrune: Option<
        unsafe extern "C" fn(
            *const libc::c_char,
            __darwin_size_t,
            *mut *const libc::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut libc::c_char,
            __darwin_size_t,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__u32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut libc::c_void,
    pub __variable_len: libc::c_int,
    pub __ncharclasses: libc::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
#[inline]
unsafe extern "C" fn isascii(mut _c: libc::c_int) -> libc::c_int {
    return (_c & !(0x7f as libc::c_int) == 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __istype(mut _c: __darwin_ct_rune_t, mut _f: libc::c_ulong) -> libc::c_int {
    return if isascii(_c) != 0 {
        (_DefaultRuneLocale.__runetype[_c as usize] as libc::c_ulong & _f != 0) as libc::c_int
    } else {
        (__maskrune(_c, _f) != 0) as libc::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isupper(mut _c: libc::c_int) -> libc::c_int {
    return __istype(_c, 0x8000 as libc::c_long as libc::c_ulong);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut _c: libc::c_int) -> libc::c_int {
    return __tolower(_c);
}
#[no_mangle]
pub unsafe extern "C" fn downcase_truncate_identifier(
    mut ident: *const libc::c_char,
    mut len: libc::c_int,
    mut warn: bool,
) -> *mut libc::c_char {
    return downcase_identifier(ident, len, warn, true);
}
#[no_mangle]
pub unsafe extern "C" fn downcase_identifier(
    mut ident: *const libc::c_char,
    mut len: libc::c_int,
    mut warn: bool,
    mut truncate: bool,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut enc_is_single_byte: bool = false;
    result = palloc((len + 1 as libc::c_int) as usize) as *mut libc::c_char;
    enc_is_single_byte =
        (pg_database_encoding_max_length() == 1 as libc::c_int) as libc::c_int as bool;
    i = 0 as libc::c_int;
    while i < len {
        let mut ch: libc::c_uchar = *ident.offset(i as isize) as libc::c_uchar;
        if ch as libc::c_int >= 'A' as i32 && ch as libc::c_int <= 'Z' as i32 {
            ch = (ch as libc::c_int + ('a' as i32 - 'A' as i32)) as libc::c_uchar;
        } else if enc_is_single_byte as libc::c_int != 0
            && ch as libc::c_int & 0x80 as libc::c_int != 0
            && isupper(ch as libc::c_int) != 0
        {
            ch = tolower(ch as libc::c_int) as libc::c_uchar;
        }
        *result.offset(i as isize) = ch as libc::c_char;
        i += 1;
        i;
    }
    *result.offset(i as isize) = '\0' as i32 as libc::c_char;
    if i >= 64 as libc::c_int && truncate as libc::c_int != 0 {
        truncate_identifier(result, i, warn);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn truncate_identifier(
    mut ident: *mut libc::c_char,
    mut len: libc::c_int,
    mut warn: bool,
) {
    if len >= 64 as libc::c_int {
        len = pg_mbcliplen(ident, len, 64 as libc::c_int - 1 as libc::c_int);
        if warn != 0 {
            let elevel_: libc::c_int = 18 as libc::c_int;
            let mut __error: libc::c_int = 0;
            if errstart(elevel_, 0 as *const libc::c_char) != 0 {
                errcode(
                    ('4' as i32 - '0' as i32 & 0x3f as libc::c_int)
                        + (('2' as i32 - '0' as i32 & 0x3f as libc::c_int) << 6 as libc::c_int)
                        + (('6' as i32 - '0' as i32 & 0x3f as libc::c_int) << 12 as libc::c_int)
                        + (('2' as i32 - '0' as i32 & 0x3f as libc::c_int) << 18 as libc::c_int)
                        + (('2' as i32 - '0' as i32 & 0x3f as libc::c_int) << 24 as libc::c_int),
                );
                errmsg(
                    b"identifier \"%s\" will be truncated to \"%.*s\"\0" as *const u8
                        as *const libc::c_char,
                    ident,
                    len,
                    ident,
                );
                errfinish(
                    b"/Users/conrad/Documents/code/pg-parse-rs/postgres/parser/scansup.c\0"
                        as *const u8 as *const libc::c_char,
                    102 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
            if elevel_ >= 21 as libc::c_int {
                abort();
            }
        }
        *ident.offset(len as isize) = '\0' as i32 as libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn scanner_isspace(mut ch: libc::c_char) -> bool {
    if ch as libc::c_int == ' ' as i32
        || ch as libc::c_int == '\t' as i32
        || ch as libc::c_int == '\n' as i32
        || ch as libc::c_int == '\r' as i32
        || ch as libc::c_int == '\u{c}' as i32
    {
        return true;
    }
    return false;
}
