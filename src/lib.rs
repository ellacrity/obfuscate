//! Compiletime string constant obfuscation.
#![feature(strict_provenance)]
#![deny(fuzzy_provenance_casts)]
#![deny(lossy_provenance_casts)]
#![deny(clippy::missing_const_for_fn)]
#![allow(clippy::ptr_offset_with_cast)]
#![cfg_attr(not(test), no_std)]

use core::str;

#[doc(hidden)]
pub mod wide;

#[doc(hidden)]
pub mod cfo;

pub mod hash;

mod murmur3;
pub use hash::{hash, hash_bytes, hash_cstr, hash_words};

pub use self::murmur3::murmur3;

mod pos;
pub use self::pos::position;

#[doc(hidden)]
pub mod xref;

//----------------------------------------------------------------

/// Compiletime random number generator.
///
/// Supported types are `u8`, `u16`, `u32`, `u64`, `usize`, `i8`, `i16`, `i32`,
/// `i64`, `isize`, `bool`, `f32` and `f64`.
///
/// The integer types generate a random value in their respective range.  
/// The float types generate a random value in range of `[1.0, 2.0)`.
///
/// ```
/// const RND: i32 = obfuscate::random!(u8) as i32;
/// assert!(RND >= 0 && RND <= 255);
/// # const _: f32 = obfuscate::random!(f32);
/// # const _: f64 = obfuscate::random!(f64);
/// ```
///
/// The behavior of the macro inside other macros can be surprising:
///
/// ```
/// // When used as top-level input to macros, random works as expected
/// assert_ne!(obfuscate::random!(u64), obfuscate::random!(u64));
///
/// // When used inside the definition of a macro, random does not work as expected
/// macro_rules! inside {
///     () => {
///         assert_eq!(obfuscate::random!(u64), obfuscate::random!(u64));
///     };
/// }
/// inside!();
///
/// // When provided a unique seed, random works as expected
/// // Note that the seeds must evaluate to a literal!
/// macro_rules! seeded {
///     () => {
///         assert_ne!(obfuscate::random!(u64, "lhs"), obfuscate::random!(u64, "rhs"));
///     };
/// }
/// seeded!();
///
/// // Repeated usage in macros, random does not work as expected
/// macro_rules! repeated {
///     ($($name:ident),*) => {
///         $(let $name = obfuscate::random!(u64, "seed");)*
///     };
/// }
/// repeated!(a, b);
/// assert_eq!(a, b);
///
/// // Provide additional unique seeds, random works as expected
/// macro_rules! repeated_seeded {
///     ($($name:ident),*) => {
///         $(let $name = obfuscate::random!(u64, "seed", stringify!($name));)*
///     };
/// }
/// repeated_seeded!(c, d);
/// assert_ne!(c, d);
/// ```
#[macro_export]
macro_rules! random {
	($ty:ident $(, $seeds:expr)* $(,)?) => {{
		const _RANDOM: $ty = $crate::__random_cast!($ty,
			$crate::entropy(concat!(file!(), ":", line!(), ":", column!() $(, ":", $seeds)*)));
		_RANDOM
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __random_cast {
    (u8, $seed:expr) => {
        $seed as u8
    };
    (u16, $seed:expr) => {
        $seed as u16
    };
    (u32, $seed:expr) => {
        $seed as u32
    };
    (u64, $seed:expr) => {
        $seed
    };
    (usize, $seed:expr) => {
        $seed as usize
    };
    (i8, $seed:expr) => {
        $seed as i8
    };
    (i16, $seed:expr) => {
        $seed as i16
    };
    (i32, $seed:expr) => {
        $seed as i32
    };
    (i64, $seed:expr) => {
        $seed as i64
    };
    (isize, $seed:expr) => {
        $seed as isize
    };
    (bool, $seed:expr) => {
        $seed as i64 >= 0
    };

    // {f32, f64}::from_bits is unstable as const fn due to issues with NaN
    (f32, $seed:expr) => {
        unsafe {
            ::core::mem::transmute::<u32, f32>(
                0b0_01111111 << (f32::MANTISSA_DIGITS - 1) | ($seed as u32 >> 9),
            )
        }
    };
    (f64, $seed:expr) => {
        unsafe {
            ::core::mem::transmute::<u64, f64>(
                0b0_01111111111 << (f64::MANTISSA_DIGITS - 1) | ($seed >> 12),
            )
        }
    };

    ($ty:ident, $seed:expr) => {
        compile_error!(concat!("unsupported type: ", stringify!($ty)))
    };
}

#[test]
fn test_random_f32() {
    #[track_caller]
    fn t(v: f32) {
        assert!((1.0..2.0).contains(&v), "{}", v);
    }
    use random as r;
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
    t(r!(f32));
}

#[test]
fn test_random_f64() {
    #[track_caller]
    fn t(v: f64) {
        assert!((1.0..2.0).contains(&v), "{}", v);
    }
    use random as r;
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
    t(r!(f64));
}

/// Compiletime bitmixing.
///
/// Takes an intermediate hash that may not be thoroughly mixed and increase its
/// entropy to obtain both better distribution. See [Better Bit Mixing](https://zimbry.blogspot.com/2011/09/better-bit-mixing-improving-on.html) for reference.
#[inline(always)]
pub const fn splitmix(seed: u64) -> u64 {
    let next = seed.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = next;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

/// Produces pseudorandom entropy from the given string.
#[doc(hidden)]
#[inline(always)]
pub const fn entropy(string: &str) -> u64 {
    splitmix(SEED ^ splitmix(hash(string) as u64))
}

/// Compiletime RNG seed.
///
/// This value is derived from the environment variable `ENTROPY_SEED` and has a
/// fixed value if absent. If it changes all downstream dependents are recompiled
/// automatically.
pub const SEED: u64 = splitmix(hash(match option_env!("ENTROPY_SEED") {
    Some(seed) => seed,
    None => "FIXED",
}) as u64);

//----------------------------------------------------------------

#[doc(hidden)]
pub mod bytes;

#[doc(hidden)]
pub mod words;

#[doc(hidden)]
#[inline(always)]
pub fn unsafe_as_str(bytes: &[u8]) -> &str {
    // When used correctly by this crate's macros this should be safe
    #[cfg(debug_assertions)]
    {
        core::str::from_utf8(bytes).unwrap()
    }
    #[cfg(not(debug_assertions))]
    {
        unsafe { core::str::from_utf8_unchecked(bytes) }
    }
}
