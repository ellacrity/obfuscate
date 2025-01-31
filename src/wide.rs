/// Encodes the input string as a wide string (utf-16) constant.
///
/// The type of the returned constant is `&'static [u16; LEN]`.
///
/// # Examples
///
/// ```
/// let expected = &['W' as u16, 'i' as u16, 'd' as u16, 'e' as u16, 0];
/// assert_eq!(expected, obfuscate::wide!("Wide\0"));
/// ```
#[macro_export]
macro_rules! wide {
    ($s:expr) => {{
        const _WIDE_STRING: &str = $s;
        const _WIDE_LEN: usize = $crate::wide::len(_WIDE_STRING);
        const _WIDE_WORDS: [u16; _WIDE_LEN] =
            $crate::wide::encode::<_WIDE_LEN>(_WIDE_STRING);
        &_WIDE_WORDS
    }};
}

const fn next(bytes: &[u8]) -> Option<(u32, &[u8])> {
    match *bytes {
        [a, ref tail @ ..] if a & 0x80 == 0x00 => Some((a as u32, tail)),
        [a, b, ref tail @ ..] if a & 0xE0 == 0xC0 => {
            Some(((a as u32 & 0x1F) << 6 | (b as u32 & 0x3F), tail))
        }
        [a, b, c, ref tail @ ..] if a & 0xF0 == 0xE0 => Some((
            (a as u32 & 0x0F) << 12 | (b as u32 & 0x3F) << 6 | (c as u32 & 0x3F),
            tail,
        )),
        [a, b, c, d, ref tail @ ..] if a & 0xF8 == 0xF0 => Some((
            (a as u32 & 0x07) << 18
                | (b as u32 & 0x3F) << 12
                | (c as u32 & 0x3F) << 6
                | (d as u32 & 0x3F),
            tail,
        )),
        [..] => None,
    }
}

#[doc(hidden)]
pub const fn len(s: &str) -> usize {
    let mut bytes = s.as_bytes();
    let mut len = 0;
    while let Some((chr, tail)) = next(bytes) {
        bytes = tail;
        len += if chr >= 0x10000 { 2 } else { 1 };
    }
    len
}

#[doc(hidden)]
pub const fn encode<const LEN: usize>(s: &str) -> [u16; LEN] {
    let mut bytes = s.as_bytes();
    let mut data = [0u16; LEN];
    let mut i = 0usize;
    while let Some((chr, tail)) = next(bytes) {
        bytes = tail;
        if chr >= 0x10000 {
            data[i] = (0xD800 + (chr - 0x10000) / 0x400) as u16;
            data[i + 1] = (0xDC00 + (chr - 0x10000) % 0x400) as u16;
            i += 2;
        } else {
            data[i] = chr as u16;
            i += 1;
        }
    }
    data
}

#[test]
fn test_example() {
    let text = &[
        'e' as u16, 'x' as u16, 'a' as u16, 'm' as u16, 'p' as u16, 'l' as u16,
        'e' as u16,
    ];
    assert_eq!(text, wide!("example"));
}

#[test]
fn test_escapes() {
    let text = &[
        '\t' as u16,
        '\n' as u16,
        '\r' as u16,
        '\\' as u16,
        '\0' as u16,
        '\'' as u16,
        '\"' as u16,
        '\x52' as u16,
        '\u{00B6}' as u16,
    ];
    assert_eq!(text, wide!("\t\n\r\\\0\'\"\x52\u{00B6}"));
}

#[test]
fn test_raw() {
    let text = &[b'\\' as u16];
    assert_eq!(text, wide!(r"\"));
    assert_eq!(text, wide!(r#"\"#));
    assert_eq!(text, wide!(r##"\"##));
}

#[test]
fn test_const() {
    const STRING: &str = "Wide\0";
    let text = &['W' as u16, 'i' as u16, 'd' as u16, 'e' as u16, 0];
    assert_eq!(text, wide!(STRING));
}
