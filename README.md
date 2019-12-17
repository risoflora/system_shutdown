# `system_shutdown`

`system_shutdown` provides a cross platform way to shut down or reboot the machine.

Supported platforms: Linux, Windows and MacOS.

## Example

The example below shows how to shut down the machine:

```rust
extern crate system_shutdown;

use system_shutdown::shutdown;

fn main() {
    if shutdown(true) {
        println!("Shutting down, bye!");
    } else {
        println!("Failed to shut down.");
    }
}
```

In most of the systems it does not require the user to be root/admin.

## Usage

Add this to your `Cargo.toml`:

```ini
[dependencies]
system_shutdown = "1.0.0"
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
