# `system_shutdown`

[![CI/CD][ci-cd-badge]][ci-cd-url]
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![License][license-badge]][license-url]

`system_shutdown` provides a cross platform way to shut down, reboot or log out operations.

Supported platforms: Linux, Windows and macOS.

## Usage

Add this to your `Cargo.toml`:

```ini
[dependencies]
system_shutdown = "*"
```

and then:

```rust
use system_shutdown::shutdown;

fn main() {
    match shutdown() {
        Ok(_) => println!("Shutting down, bye!"),
        Err(error) => eprintln!("Failed to shut down: {}", error),
    }
}
```

In most of the systems it does not requires the user to be root/admin.

## Contributions

Pull Requests are welcome! =)

## License

`system_shutdown` is licensed under either of the following, at your option:

- [Apache License 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

[ci-cd-badge]: https://github.com/risoflora/system_shutdown/actions/workflows/CI.yml/badge.svg
[ci-cd-url]: https://github.com/risoflora/system_shutdown/actions/workflows/CI.yml
[crates-badge]: https://img.shields.io/crates/v/system_shutdown.svg
[crates-url]: https://crates.io/crates/system_shutdown
[docs-badge]: https://docs.rs/system_shutdown/badge.svg
[docs-url]: https://docs.rs/system_shutdown
[license-badge]: https://img.shields.io/crates/l/system_shutdown.svg
[license-url]: https://github.com/risoflora/system_shutdown#license
