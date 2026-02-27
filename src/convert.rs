use crate::raw::*;
use std::ptr::{null, null_mut};

pub(crate) const CP_UTF8: UINT = 65001;
pub(crate) const MB_ERR_INVALID_CHARS: DWORD = 0x8;
pub(crate) const WC_NO_BEST_FIT_CHARS: DWORD = 0x400;
pub(crate) const ERROR_INVALID_PARAMETER: DWORD = 0x57;
pub(crate) const ERROR_INSUFFICIENT_BUFFER: DWORD = 0x7a;
pub(crate) const ERROR_NO_UNICODE_TRANSLATION: DWORD = 0x459;

pub(crate) type OsResult<T> = Result<T, u32>;

pub(crate) fn utf8_to_wide(x: &str) -> OsResult<Vec<u16>> {
    // UTF-8 to Unicode is loss less
    multi_byte_to_wide_char_wrap(CP_UTF8, MB_ERR_INVALID_CHARS, x.as_bytes())
}

pub(crate) fn wide_to_mb(code_page: u32, x: &[u16]) -> OsResult<Vec<u8>> {
    wide_char_to_multi_byte_wrap(code_page, WC_NO_BEST_FIT_CHARS, x, true)
}

pub(crate) fn wide_to_mb_lossy(code_page: u32, x: &[u16]) -> OsResult<Vec<u8>> {
    wide_char_to_multi_byte_wrap(code_page, WC_NO_BEST_FIT_CHARS, x, false)
}

/// Safe wrapper function of MultiByteToWideChar.
#[inline(always)]
fn multi_byte_to_wide_char(
    code_page: UINT,
    mb_flags: DWORD,
    mb_bytes: &[u8],
    wc_bytes: &mut [u16],
) -> OsResult<usize> {
    unsafe {
        match MultiByteToWideChar(
            code_page,
            mb_flags,
            mb_bytes.as_ptr() as *const i8,
            mb_bytes
                .len()
                .try_into()
                .map_err(|_| ERROR_INVALID_PARAMETER)?,
            wc_bytes.as_mut_ptr(),
            wc_bytes
                .len()
                .try_into()
                .map_err(|_| ERROR_INVALID_PARAMETER)?,
        ) {
            0 => Err(GetLastError()),
            x => Ok(x as usize),
        }
    }
}

/// Safe wrapper function of WideCharToMultiByte.
#[inline(always)]
fn wide_char_to_multi_byte<'a>(
    code_page: UINT,
    wc_flags: DWORD,
    wc_bytes: &[u16],
    mb_bytes: &mut [u8],
    default_char: impl Into<Option<u8>>,
    used_default_char: impl Into<Option<&'a mut i32>>,
) -> OsResult<usize> {
    let dc = default_char.into().map_or(null(), |x| &x);
    unsafe {
        match WideCharToMultiByte(
            code_page,
            wc_flags,
            wc_bytes.as_ptr(),
            wc_bytes
                .len()
                .try_into()
                .map_err(|_| ERROR_INVALID_PARAMETER)?,
            mb_bytes.as_mut_ptr() as *mut i8,
            mb_bytes
                .len()
                .try_into()
                .map_err(|_| ERROR_INVALID_PARAMETER)?,
            dc as *const i8,
            used_default_char.into().map_or(null_mut(), |x| x),
        ) {
            0 => Err(GetLastError()),
            x => Ok(x as usize),
        }
    }
}

#[allow(clippy::uninit_vec)]
pub(crate) fn wide_char_to_multi_byte_wrap(
    code_page: UINT,
    wc_flags: DWORD,
    x: &[u16],
    used_default_char: bool,
) -> OsResult<Vec<u8>> {
    let x = if x.is_empty() { &[0x00] } else { x };
    let l = x.len() * 4;
    let mut ret: Vec<u8> = Vec::with_capacity(l);
    unsafe {
        ret.set_len(l);
    }
    let mut udc_flag = 0;
    let udc = if used_default_char {
        Some(&mut udc_flag)
    } else {
        None
    };

    match wide_char_to_multi_byte(
        code_page,
        wc_flags,
        x,
        ret.as_mut_slice(),
        None,
        udc,
    ) {
        Ok(l2) => {
            if udc_flag != 0 {
                return Err(ERROR_NO_UNICODE_TRANSLATION);
            }
            unsafe {
                ret.set_len(l2);
            }
            Ok(ret)
        }
        Err(ERROR_INSUFFICIENT_BUFFER) => {
            wide_char_to_multi_byte2(code_page, wc_flags, x, used_default_char)
        }
        Err(x) => Err(x),
    }
}

/// Gets the required buffer size and gets a multi-byte string.
#[inline]
#[allow(clippy::uninit_vec)]
pub(crate) fn wide_char_to_multi_byte2(
    code_page: UINT,
    wc_flags: DWORD,
    x: &[u16],
    used_default_char: bool,
) -> OsResult<Vec<u8>> {
    // get the required buffer size.
    let l =
        wide_char_to_multi_byte(code_page, wc_flags, x, &mut [], None, None)?;
    let mut ret: Vec<u8> = Vec::with_capacity(l);
    unsafe {
        ret.set_len(l);
    }
    let mut udc_flag = 0;
    let udc = if used_default_char {
        Some(&mut udc_flag)
    } else {
        None
    };

    let l2 = wide_char_to_multi_byte(
        code_page,
        wc_flags,
        x,
        ret.as_mut_slice(),
        None,
        udc,
    )?;
    if udc_flag != 0 {
        return Err(ERROR_NO_UNICODE_TRANSLATION);
    }
    assert_eq!(l, l2);
    Ok(ret)
}

#[allow(clippy::uninit_vec)]
fn multi_byte_to_wide_char_wrap(
    code_page: UINT,
    mb_flags: DWORD,
    x: &[u8],
) -> OsResult<Vec<u16>> {
    let x = if x.is_empty() { &[0x00] } else { x };
    let l = x.len();
    let mut ret: Vec<u16> = Vec::with_capacity(l);
    unsafe {
        ret.set_len(l);
    }

    match multi_byte_to_wide_char(code_page, mb_flags, x, ret.as_mut_slice()) {
        Ok(l2) => {
            unsafe {
                ret.set_len(l2);
            }
            Ok(ret)
        }
        Err(ERROR_INSUFFICIENT_BUFFER) => {
            multi_byte_to_wide_char2(code_page, mb_flags, x)
        }
        Err(x) => Err(x),
    }
}

/// Gets the required buffer size and gets a wide string.
#[inline]
#[allow(clippy::uninit_vec)]
fn multi_byte_to_wide_char2(
    code_page: UINT,
    mb_flags: DWORD,
    x: &[u8],
) -> OsResult<Vec<u16>> {
    // get the required buffer size.
    let l = multi_byte_to_wide_char(code_page, mb_flags, x, &mut [])?;
    let mut ret: Vec<u16> = Vec::with_capacity(l);
    unsafe {
        ret.set_len(l);

        let l2 = multi_byte_to_wide_char(
            code_page,
            mb_flags,
            x,
            ret.as_mut_slice(),
        )?;
        assert_eq!(l, l2);
    }
    Ok(ret)
}

#[inline(always)]
fn get_locale_cp(
    locale_name: *const u16,
    lc_type: u32,
) -> crate::raw::OsResult<u32> {
    let mut cp = 0u32;
    unsafe {
        match GetLocaleInfoEx(
            locale_name,
            lc_type,
            &mut cp as *mut _ as *mut _,
            2,
        ) {
            0 => Err(GetLastError()),
            _ => Ok(cp),
        }
    }
}

pub(crate) fn to_utf16(s: &str) -> Vec<u16> {
    let mut v = s.encode_utf16().collect::<Vec<_>>();
    v.push(0);
    v
}

pub(crate) fn get_system_default_acp() -> crate::raw::OsResult<u32> {
    let locale = to_utf16("!x-sys-default-locale");

    get_locale_cp(
        locale.as_ptr(),
        LOCALE_IDEFAULTANSICODEPAGE
            | LOCALE_RETURN_NUMBER,
    )
}
