// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
#![cfg(windows)]

#[cfg(test)]
mod tests {
    use windy::*;
    use windy_macros::*;

    // Makes a string.
    macro_rules! ms {
        (@a $x:expr) => {
            AString::from_str_lossy($x)
        };
        (@ab $x:expr) => {
            AString::from_str_lossy($x).to_bytes_with_nul()
        };
        (@w $x:expr) => {
            WString::from_str_lossy($x)
        };
        (@wb $x:expr) => {
            WString::from_str_lossy($x).to_bytes_with_nul()
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
        let x: AString = astring!("test");
        assert_eq!(ms!(@a "test"), x);
        assert_ne!(ms!(@a "test2"), x);
        assert_eq!(ms!(@a "a"), astring!('a'));
        assert_eq!(ms!(@a "4649"), astring!(4649));
        assert_eq!(ms!(@a "3.14"), astring!(3.14));
        assert_eq!(ms!(@a "true"), astring!(true));
    }

    #[test]
    fn test_astring_lossy() {
        let x: AString = astring_lossy!("test");
        assert_eq!(ms!(@a "test"), x);
        assert_ne!(ms!(@a "test2"), x);
        assert_eq!(ms!(@a "testテスト🍣"), astring_lossy!("testテスト🍣"));
        assert_eq!(ms!(@a "あ"), astring_lossy!('あ'));
        assert_eq!(ms!(@a "4649"), astring_lossy!(4649));
        assert_eq!(ms!(@a "3.14"), astring_lossy!(3.14));
        assert_eq!(ms!(@a "true"), astring_lossy!(true));
    }

    #[test]
    fn test_wstr() {
        fn to(x: &WStr) -> WString {
            unsafe {
                WString::new_nul_unchecked(x.to_bytes_with_nul().to_vec())
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
                WString::new_nul_unchecked(x.to_bytes_with_nul().to_vec())
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
        fn to(x: &AStr) -> AString {
            unsafe {
                AString::new_nul_unchecked(x.to_bytes_with_nul().to_vec())
            }
        }
        let x = to(astr!("test"));
        assert_eq!(ms!(@a "test"), x);
        assert_ne!(ms!(@a "test2"), x);
        assert_eq!(ms!(@a "a"), to(astr!('a')));
        assert_eq!(ms!(@a "4649"), to(astr!(4649)));
        assert_eq!(ms!(@a "3.14"), to(astr!(3.14)));
        assert_eq!(ms!(@a "true"), to(astr!(true)));
    }

    #[test]
    fn test_astr_lossy() {
        fn to(x: &AStr) -> AString {
            unsafe {
                AString::new_nul_unchecked(x.to_bytes_with_nul().to_vec())
            }
        }
        let x = to(astr_lossy!("test"));
        assert_eq!(ms!(@a "test"), x);
        assert_ne!(ms!(@a "test2"), x);
        assert_eq!(ms!(@a "testテスト🍣"), to(astr_lossy!("testテスト🍣")));
        assert_eq!(ms!(@a "あ"), to(astr_lossy!('あ')));
        assert_eq!(ms!(@a "4649"), to(astr_lossy!(4649)));
        assert_eq!(ms!(@a "3.14"), to(astr_lossy!(3.14)));
        assert_eq!(ms!(@a "true"), to(astr_lossy!(true)));
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
        let x: &[u8] = &aarr!("test");
        assert_eq!(ms!(@ab "test"), x);
        assert_ne!(ms!(@ab "test2"), x);
        assert_eq!(ms!(@ab "a"), &aarr!('a'));
        assert_eq!(ms!(@ab "4649"), &aarr!(4649));
        assert_eq!(ms!(@ab "3.14"), &aarr!(3.14));
        assert_eq!(ms!(@ab "true"), &aarr!(true));
    }

    #[test]
    fn test_aarr_lossy() {
        let x: &[u8] = &aarr_lossy!("test");
        assert_eq!(ms!(@ab "test"), x);
        assert_ne!(ms!(@ab "test2"), x);
        assert_eq!(ms!(@ab "testテスト🍣"), &aarr_lossy!("testテスト🍣"));
        assert_eq!(ms!(@ab "あ"), &aarr_lossy!('あ'));
        assert_eq!(ms!(@ab "4649"), &aarr_lossy!(4649));
        assert_eq!(ms!(@ab "3.14"), &aarr_lossy!(3.14));
        assert_eq!(ms!(@ab "true"), &aarr_lossy!(true));
    }
}
