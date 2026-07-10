// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
#![cfg(windows)]

#[cfg(test)]
mod tests {
    use windy::*;
    use windy_macros::*;

    const CP_SJIS: u32 = 932;

    // Makes a string.
    macro_rules! ms {
        (@a $cp:expr, $x:expr) => {
            AString::<$cp>::from_utf8_lossy($x)
        };
        (@ab $cp:expr, $x:expr) => {
            AString::<$cp>::from_utf8_lossy($x).as_bytes_with_nul()
        };
        (@ac $x:expr) => {
            ACPString::from_utf8_lossy($x)
        };
        (@acb $x:expr) => {
            ACPString::from_utf8_lossy($x).as_bytes_with_nul()
        };
        (@w $x:expr) => {
            WString::from_utf8_lossy($x)
        };
        (@wb $x:expr) => {
            WString::from_utf8_lossy($x).as_bytes_with_nul()
        };
        (@s $x:expr) => {
            $x.to_string()
        };
    }

    #[test]
    fn test_wstring() {
        let x: WString = wstring!("test");
        assert_eq!(ms!(@w "test"), x);
        assert_ne!(ms!(@w "test2"), x);
        assert_eq!(ms!(@w "testテスト🍣"), wstring!("testテスト🍣"));
        assert_eq!(ms!(@w "4649"), wstring!(4649));
        assert_eq!(ms!(@w "あ"), wstring!('あ'));
        assert_eq!(ms!(@w "3.14"), wstring!(3.14));
        assert_eq!(ms!(@w "true"), wstring!(true));
    }

    #[test]
    fn test_wstring_lossy() {
        let x: WString = wstring_lossy!("test");
        assert_eq!(ms!(@w "test"), x);
        assert_ne!(ms!(@w "test2"), x);
        assert_eq!(ms!(@w "testテスト🍣"), wstring_lossy!("testテスト🍣"));
        assert_eq!(ms!(@w "4649"), wstring_lossy!(4649));
        assert_eq!(ms!(@w "あ"), wstring_lossy!('あ'));
        assert_eq!(ms!(@w "3.14"), wstring_lossy!(3.14));
        assert_eq!(ms!(@w "true"), wstring_lossy!(true));
    }

    #[test]
    fn test_astring() {
        let x: AString<CP_SJIS> = astring!(932, "test");
        assert_eq!(ms!(@a CP_SJIS, "test"), x);
        assert_ne!(ms!(@a CP_SJIS, "test2"), x);
        assert_eq!(ms!(@a CP_SJIS, "a"), astring!(932, 'a'));
        assert_eq!(ms!(@a CP_SJIS, "4649"), astring!(932, 4649));
        assert_eq!(ms!(@a CP_SJIS, "3.14"), astring!(932, 3.14));
        assert_eq!(ms!(@a CP_SJIS, "true"), astring!(932, true));
        let x: AString<CP_SJIS> = astring!(932, "テスト");
        assert_eq!(
            x.as_bytes_with_nul(),
            &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0]
        );
    }

    #[test]
    fn test_astring_lossy() {
        let x: AString<CP_SJIS> = astring_lossy!(932, "test");
        assert_eq!(ms!(@a CP_SJIS, "test"), x);
        assert_ne!(ms!(@a CP_SJIS, "test2"), x);
        assert_eq!(ms!(@a CP_SJIS, "a"), astring_lossy!(932, 'a'));
        assert_eq!(ms!(@a CP_SJIS, "4649"), astring_lossy!(932, 4649));
        assert_eq!(ms!(@a CP_SJIS, "3.14"), astring_lossy!(932, 3.14));
        assert_eq!(ms!(@a CP_SJIS, "true"), astring_lossy!(932, true));
        let x: AString<CP_SJIS> = astring_lossy!(932, "テスト€");
        assert_eq!(
            x.as_bytes_with_nul(),
            &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, b'?', 0]
        );
    }

    #[test]
    fn test_acpstring() {
        let x: ACPString = acpstring!("test");
        assert_eq!(ms!(@ac "test"), x);
        assert_ne!(ms!(@ac "test2"), x);
        assert_eq!(ms!(@ac "a"), acpstring!('a'));
        assert_eq!(ms!(@ac "4649"), acpstring!(4649));
        assert_eq!(ms!(@ac "3.14"), acpstring!(3.14));
        assert_eq!(ms!(@ac "true"), acpstring!(true));
    }

    #[test]
    fn test_acpstring_lossy() {
        let x: ACPString = acpstring_lossy!("test");
        assert_eq!(ms!(@ac "test"), x);
        assert_ne!(ms!(@ac "test2"), x);
        assert_eq!(ms!(@ac "testテスト🍣"), acpstring_lossy!("testテスト🍣"));
        assert_eq!(ms!(@ac "あ"), acpstring_lossy!('あ'));
        assert_eq!(ms!(@ac "4649"), acpstring_lossy!(4649));
        assert_eq!(ms!(@ac "3.14"), acpstring_lossy!(3.14));
        assert_eq!(ms!(@ac "true"), acpstring_lossy!(true));
    }

    #[test]
    fn test_wstr() {
        fn to(x: &WStr) -> WString {
            unsafe {
                WString::from_vec_with_nul_unchecked(
                    x.as_bytes_with_nul().to_vec(),
                )
            }
        }
        let x = to(wstr!("test"));
        assert_eq!(ms!(@w "test"), x);
        assert_ne!(ms!(@w "test2"), x);
        assert_eq!(ms!(@w "testテスト🍣"), to(wstr!("testテスト🍣")));
        assert_eq!(ms!(@w "あ"), to(wstr!('あ')));
        assert_eq!(ms!(@w "4649"), to(wstr!(4649)));
        assert_eq!(ms!(@w "3.14"), to(wstr!(3.14)));
        assert_eq!(ms!(@w "true"), to(wstr!(true)));
    }

    #[test]
    fn test_wstr_lossy() {
        fn to(x: &WStr) -> WString {
            unsafe {
                WString::from_vec_with_nul_unchecked(
                    x.as_bytes_with_nul().to_vec(),
                )
            }
        }
        let x = to(wstr_lossy!("test"));
        assert_eq!(ms!(@w "test"), x);
        assert_ne!(ms!(@w "test2"), x);
        assert_eq!(ms!(@w "testテスト🍣"), to(wstr_lossy!("testテスト🍣")));
        assert_eq!(ms!(@w "あ"), to(wstr_lossy!('あ')));
        assert_eq!(ms!(@w "4649"), to(wstr_lossy!(4649)));
        assert_eq!(ms!(@w "3.14"), to(wstr_lossy!(3.14)));
        assert_eq!(ms!(@w "true"), to(wstr_lossy!(true)));
    }

    #[test]
    fn test_astr() {
        fn to<const CP: u32>(x: &AStr<CP>) -> AString<CP> {
            unsafe {
                AString::from_vec_with_nul_unchecked(
                    x.as_bytes_with_nul().to_vec(),
                )
            }
        }
        let x = astr!(932, "test");
        assert_eq!(ms!(@a CP_SJIS, "test"), x);
        assert_ne!(ms!(@a CP_SJIS, "test2"), x);
        assert_eq!(ms!(@a CP_SJIS, "a"), to(astr!(932, 'a')));
        assert_eq!(ms!(@a CP_SJIS, "4649"), to(astr!(932, 4649)));
        assert_eq!(ms!(@a CP_SJIS, "3.14"), to(astr!(932, 3.14)));
        assert_eq!(ms!(@a CP_SJIS, "true"), to(astr!(932, true)));
        let x = astr!(932, "テスト");
        assert_eq!(
            x.as_bytes_with_nul(),
            &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0]
        );
    }

    #[test]
    fn test_astr_lossy() {
        let x = astr_lossy!(932, "test");
        assert_eq!(ms!(@a CP_SJIS, "test"), x);
        assert_ne!(ms!(@a CP_SJIS, "test2"), x);
        assert_eq!(ms!(@a CP_SJIS, "a"), astr_lossy!(932, 'a'));
        assert_eq!(ms!(@a CP_SJIS, "4649"), astr_lossy!(932, 4649));
        assert_eq!(ms!(@a CP_SJIS, "3.14"), astr_lossy!(932, 3.14));
        assert_eq!(ms!(@a CP_SJIS, "true"), astr_lossy!(932, true));
        let x = astr_lossy!(932, "テスト€");
        assert_eq!(
            x.as_bytes_with_nul(),
            &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, b'?', 0]
        );
    }

    #[test]
    fn test_acpstr() {
        let x = acpstr!("test");
        assert_eq!(ms!(@ac "test"), x);
        assert_ne!(ms!(@ac "test2"), x);
        assert_eq!(ms!(@ac "a"), acpstr!('a'));
        assert_eq!(ms!(@ac "4649"), acpstr!(4649));
        assert_eq!(ms!(@ac "3.14"), acpstr!(3.14));
        assert_eq!(ms!(@ac "true"), acpstr!(true));
    }

    #[test]
    fn test_acpstr_lossy() {
        let x = acpstr_lossy!("test");
        assert_eq!(ms!(@ac "test"), x);
        assert_ne!(ms!(@ac "test2"), x);
        assert_eq!(ms!(@ac "testテスト🍣"), acpstr_lossy!("testテスト🍣"));
        assert_eq!(ms!(@ac "あ"), acpstr_lossy!('あ'));
        assert_eq!(ms!(@ac "4649"), acpstr_lossy!(4649));
        assert_eq!(ms!(@ac "3.14"), acpstr_lossy!(3.14));
        assert_eq!(ms!(@ac "true"), acpstr_lossy!(true));
    }

    #[test]
    fn test_warr() {
        let x: &[u16] = &warr!("test");
        assert_eq!(ms!(@wb "test"), x);
        assert_ne!(ms!(@wb "test2"), x);
        assert_eq!(ms!(@wb "testテスト🍣"), &warr!("testテスト🍣"));
        assert_eq!(ms!(@wb "あ"), &warr!('あ'));
        assert_eq!(ms!(@wb "4649"), &warr!(4649));
        assert_eq!(ms!(@wb "3.14"), &warr!(3.14));
        assert_eq!(ms!(@wb "true"), &warr!(true));
    }

    #[test]
    fn test_warr_lossy() {
        let x: &[u16] = &warr_lossy!("test");
        assert_eq!(ms!(@wb "test"), x);
        assert_ne!(ms!(@wb "test2"), x);
        assert_eq!(ms!(@wb "testテスト🍣"), &warr_lossy!("testテスト🍣"));
        assert_eq!(ms!(@wb "あ"), &warr_lossy!('あ'));
        assert_eq!(ms!(@wb "4649"), &warr_lossy!(4649));
        assert_eq!(ms!(@wb "3.14"), &warr_lossy!(3.14));
        assert_eq!(ms!(@wb "true"), &warr_lossy!(true));
    }

    #[test]
    fn test_aarr() {
        let x: &[u8] = &aarr!(932, "test");
        assert_eq!(ms!(@ab CP_SJIS, "test"), x);
        assert_ne!(ms!(@ab CP_SJIS, "test2"), x);
        assert_eq!(ms!(@ab CP_SJIS, "a"), &aarr!(932, 'a'));
        assert_eq!(ms!(@ab CP_SJIS, "4649"), &aarr!(932, 4649));
        assert_eq!(ms!(@ab CP_SJIS, "3.14"), &aarr!(932, 3.14));
        assert_eq!(ms!(@ab CP_SJIS, "true"), &aarr!(932, true));
        let x: &[u8] = &aarr!(932, "テスト");
        assert_eq!(x, &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0]);
    }

    #[test]
    fn test_aarr_lossy() {
        let x: &[u8] = &aarr!(932, "test");
        assert_eq!(ms!(@ab CP_SJIS, "test"), x);
        assert_ne!(ms!(@ab CP_SJIS, "test2"), x);
        assert_eq!(ms!(@ab CP_SJIS, "a"), &aarr_lossy!(932, 'a'));
        assert_eq!(ms!(@ab CP_SJIS, "4649"), &aarr_lossy!(932, 4649));
        assert_eq!(ms!(@ab CP_SJIS, "3.14"), &aarr_lossy!(932, 3.14));
        assert_eq!(ms!(@ab CP_SJIS, "true"), &aarr_lossy!(932, true));
        let x: &[u8] = &aarr_lossy!(932, "テスト€");
        assert_eq!(x, &[0x83, 0x65, 0x83, 0x58, 0x83, 0x67, b'?', 0]);
    }

    #[test]
    fn test_acparr() {
        let x: &[u8] = &acparr!("test");
        assert_eq!(ms!(@acb "test"), x);
        assert_ne!(ms!(@acb "test2"), x);
        assert_eq!(ms!(@acb "a"), &acparr!('a'));
        assert_eq!(ms!(@acb "4649"), &acparr!(4649));
        assert_eq!(ms!(@acb "3.14"), &acparr!(3.14));
        assert_eq!(ms!(@acb "true"), &acparr!(true));
    }

    #[test]
    fn test_acparr_lossy() {
        let x: &[u8] = &acparr_lossy!("test");
        assert_eq!(ms!(@acb "test"), x);
        assert_ne!(ms!(@acb "test2"), x);
        assert_eq!(ms!(@acb "testテスト🍣"), &acparr_lossy!("testテスト🍣"));
        assert_eq!(ms!(@acb "あ"), &acparr_lossy!('あ'));
        assert_eq!(ms!(@acb "4649"), &acparr_lossy!(4649));
        assert_eq!(ms!(@acb "3.14"), &acparr_lossy!(3.14));
        assert_eq!(ms!(@acb "true"), &acparr_lossy!(true));
    }
}
