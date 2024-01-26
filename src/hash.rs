use core::ffi::CStr;

/// Compiletime string constant hash.
///
/// # Remarks
///
/// This function **must** be evaluated within a `const` context to benefit from
/// compile time function evaluation. If this function is called from a non-`const`
/// context, it will be evaluated at runtime.
///
/// Implemented using a modified version of the [DJB2 XOR] hash function.
///
/// For the bytestring hashing variant, see [`hash_bytes`].
///
/// [DJB2 XOR]: http://www.cse.yorku.ca/~oz/hash.html#djb2
#[inline(always)]
pub const fn hash_utf8(source: &str) -> u32 {
    let mut result: u32 = 3581u32;
    let mut index: usize = 0;
    let buf = source.as_bytes();
    while index < buf.len() {
        let val = buf[index];
        if val == b'\0' {
            index += 1;
            continue;
        }

        let value = (if val <= b'Z' { val + 0x20 } else { val }) as u32;
        result = result.wrapping_mul(33) ^ value;
        index += 1;
    }

    result
}

/// Returns the hash of a byte sequence, evaluated within a `const` context.
///
/// This variant of the hashing function skips NULL bytes, allowing it to process
/// data in a slightly different way. This function also **does not** include any
/// trailing NUL bytes. They are cleaved off the end, and the hash is calculated from
/// the byte string **excluding** any trailing NUL bytes.
///
/// # Remarks
///
/// This function **must** be evaluated within a `const` context to benefit from
/// compile time function evaluation. If this function is called from a non-`const`
/// context, it will be evaluated at runtime.
///
/// For the string-hashing version, see [`hash`].
#[inline(always)]
pub const fn hash_bytes(buf: &[u8]) -> u32 {
    let mut result: u32 = 3581u32;
    let mut index: usize = 0;
    while index < buf.len() {
        let val = buf[index];
        if val == b'\0' {
            index += 1;
            continue;
        }

        let value = (if val <= b'Z' { val + 0x20 } else { val }) as u32;
        result = result.wrapping_mul(33) ^ value;
        index += 1;
    }

    result
}

#[inline(always)]
pub const fn hash_cstr(buf: &CStr) -> u32 {
    let mut result: u32 = 3581u32;
    let mut index: usize = 0;
    let buf = buf.to_bytes_with_nul();
    while index < buf.len() {
        let val = buf[index];
        if val == b'\0' {
            break;
        }

        let value = (if val <= b'Z' { val + 0x20 } else { val }) as u32;
        result = result.wrapping_mul(33) ^ value;
        index += 1;
    }

    result
}

/// Compiletime string constant hash.
///
/// Helper macro guarantees compiletime evaluation of the string constant hash.
///
/// ```
/// const STRING: &str = "Hello World";
/// assert_eq!(obfuscate::hash_str!(STRING), 0x6520f29d);
/// ```
#[macro_export]
macro_rules! hash_str {
    ($src:expr) => {{
        const _HASH_VALUE: u32 = $crate::hash_utf8($src);
        _HASH_VALUE
    }};
}

/// Compiletime C string constant hash.
///
/// This helper macro guarantees compiletime evaluation of the C string constant
/// hash.
///
/// ```
/// const STRING: &str = "Hello World";
/// assert_eq!(obfuscate::hash_str!(STRING), 0x6520f29d);
/// ```
#[macro_export]
macro_rules! hash_cstr {
    ($src:expr) => {{
        const _HASH_VALUE: u32 = $crate::hash_cstr($src);
        _HASH_VALUE
    }};
}

/// Compiletime byte string constant hash.
///
/// This helper macro guarantees compiletime evaluation of the byte string constant
/// hash.
///
/// ```
/// const STRING: &str = "Hello World";
/// assert_eq!(obfuscate::hash_str!(STRING), 0x6520f29d);
/// ```
#[macro_export]
macro_rules! hash_bytes {
    ($src:expr) => {{
        const _HASH_VALUE: u32 = $crate::hash_bytes($src);
        _HASH_VALUE
    }};
}
