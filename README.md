# `system_shutdown`

[![Build Status][travis-badge]][travis-url]
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![License][license-badge]][license-url]

[travis-badge]: https://travis-ci.org/risoflora/system_shutdown.svg
[travis-url]: https://travis-ci.org/risoflora/system_shutdown
[crates-badge]: https://img.shields.io/crates/v/system_shutdown.svg
[crates-url]: https://crates.io/crates/system_shutdown
[docs-badge]: https://docs.rs/system_shutdown/badge.svg
[docs-url]: https://docs.rs/system_shutdown
[license-badge]: https://img.shields.io/crates/l/system_shutdown.svg
[license-url]: https://github.com/risoflora/system_shutdown#license

`system_shutdown` provides a cross platform way to shut down or reboot the machine.

Supported platforms: Linux, Windows and MacOS.

## Example

The example below shows how to shut down the machine:

```rust
extern crate system_shutdown;

use system_shutdown::shutdown;

fn main() {
    match shutdown(true) {
        None => println!("Shutting down, bye!"),
        Some(code) => println!("Failed to shut down. (Os code: {})", code),
    }
}
```

In most of the systems it does not require the user to be root/admin.

## Usage

Add this to your `Cargo.toml`:

```ini
[dependencies]
system_shutdown = "2.0.1"
```

and this to your crate root:

```rust
extern crate system_shutdown;
```

## Contributions

Pull Requests and Issues welcome!

## License

`system_shutdown` is licensed under either of the following, at your option:

- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
