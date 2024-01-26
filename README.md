# String Obfuscation

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/obfuscate.svg)](https://crates.io/crates/obfuscate)
[![docs.rs](https://docs.rs/obfuscate/badge.svg)](https://docs.rs/obfuscate)
[![Build status](https://github.com/CasualX/obfuscate/workflows/CI/badge.svg)](https://github.com/CasualX/obfuscate/actions)

Compiletime string constant obfuscation for Rust.

The string constant itself is embedded in obfuscated form and deobfuscated locally.
This reference to a temporary value must be used in the same statement it was generated.
See the documentation for more advanced use cases.

Looking for obfuscating formatting strings? See `fmtools` ([github](https://github.com/CasualX/fmtools), [crates.io](https://crates.io/crates/fmtools), [docs.rs](https://docs.rs/fmtools/0.1.2/fmtools/)) with the optional dependency `obfuscate` enabled to automatically apply string obfuscation to formatting strings.

## Examples

The `obfstr!` macro returns the deobfuscated string as a temporary value:

```rust
use obfuscate::obfuscate as s;
assert_eq!(s!("Hello 🌍"), "Hello 🌍");
```

The `wide!` macro provides compiletime utf16 string constants:

```rust
let expected = &['W' as u16, 'i' as u16, 'd' as u16, 'e' as u16, 0];
assert_eq!(obfuscate::wide!("Wide\0"), expected);
```

The `random!` macro provides compiletime random values:

```rust
const RND: i32 = obfuscate::random!(u8) as i32;
assert!(RND >= 0 && RND <= 255);
```

Compiletime random values are based on `file!()`, `line!()`, `column!()` and a fixed seed to ensure reproducibility.
This fixed seed is stored as text in the environment variable `OBFSTR_SEED` and can be changed as desired.

## Hashing Function

The djb2 hashing algorithm (`k=33`) was first reported by Dan Bernstein many years ago in comp.lang.c. Another version of this algorithm (now favored by Bernstein) uses xor:

```C
hash(i) = hash(i - 1) * 33 ^ str[i];
```

The magic of number 33 (why it works better than many other constants, prime or not) has never been adequately explained.

## License

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
