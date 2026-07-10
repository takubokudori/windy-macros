use crate::raw::*;

/// Gets the required buffer size and gets a wide string.
#[inline(always)]
fn get_locale_cp(locale_name: *const u16, lc_type: u32) -> OsResult<u32> {
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
    s.encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>()
}

pub(crate) fn get_system_default_acp() -> OsResult<u32> {
    let locale = to_utf16("!x-sys-default-locale");

    get_locale_cp(
        locale.as_ptr(),
        LOCALE_IDEFAULTANSICODEPAGE | LOCALE_RETURN_NUMBER,
    )
}
