#![allow(
    unused,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    non_snake_case
)]

use std::mem::MaybeUninit;

pub(crate) type c_char = i8;
pub(crate) type c_ushort = u16;
pub(crate) type c_int = i32;
pub(crate) type c_uint = u32;
pub(crate) type c_ulong = u32;
pub(crate) type wchar_t = u16;

pub(crate) type USHORT = c_ushort;
pub(crate) type UINT = c_uint;
pub(crate) type DWORD = c_ulong;
pub(crate) type LPBOOL = *mut c_int;
pub(crate) type LPSTR = *mut c_char;
pub(crate) type LPCSTR = *const c_char;
pub(crate) type PSTR = LPSTR;
pub(crate) type PCSTR = LPCSTR;
pub(crate) type LPWSTR = *mut wchar_t;
pub(crate) type LPCWSTR = *const wchar_t;
pub(crate) type PWSTR = LPWSTR;
pub(crate) type PCWSTR = LPCWSTR;

pub(crate) type OsResult<T> = Result<T, u32>;

pub(crate) const CP_ACP: UINT = 0;
pub(crate) const CP_UTF8: UINT = 65001;
pub(crate) const MB_ERR_INVALID_CHARS: DWORD = 0x8;
pub(crate) const WC_ERR_INVALID_CHARS: DWORD = 0x80;
pub(crate) const WC_NO_BEST_FIT_CHARS: DWORD = 0x400;
pub(crate) const ERROR_INSUFFICIENT_BUFFER: DWORD = 0x7a;
pub(crate) const ERROR_NO_UNICODE_TRANSLATION: DWORD = 0x459;

pub(crate) const LOCALE_IDEFAULTANSICODEPAGE: u32 = 0x00001004;
pub(crate) const LOCALE_RETURN_NUMBER: u32 = 0x20000000;

pub(crate) const MAX_DEFAULTCHAR: usize = 2;
pub(crate) const MAX_LEADBYTES: usize = 12;
pub(crate) const MAX_PATH: usize = 260;

#[repr(C)]
#[derive(Debug)]
struct _cpinfoexw {
    MaxCharSize: u32,
    DefaultChar: [u8; MAX_DEFAULTCHAR],
    LeadByte: [u8; MAX_LEADBYTES],
    UnicodeDefaultChar: u16,
    CodePage: u32,
    CodePageName: [u16; MAX_PATH],
}

unsafe extern "system" {
    pub(crate) fn MultiByteToWideChar(
        CodePage: u32,
        dwFlags: u32,
        lpMultiByteStr: *const i8,
        cbMultiByte: i32,
        lpWideCharStr: *mut u16,
        cchWideChar: i32,
    ) -> i32;

    pub(crate) fn WideCharToMultiByte(
        CodePage: u32,
        dwFlags: u32,
        lpWideCharStr: *const u16,
        cchWideChar: i32,
        lpMultiByteStr: *mut i8,
        cbMultiByte: i32,
        lpDefaultChar: *const i8,
        lpUsedDefaultChar: *mut i32,
    ) -> i32;

    pub(crate) fn GetLocaleInfoEx(
        lpLocaleName: *const u16,
        LCType: u32,
        lpLCData: *mut u16,
        cchData: i32,
    ) -> i32;

    pub(crate) fn GetLastError() -> u32;
}
