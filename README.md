# Wavefront

[![crates.io](https://img.shields.io/crates/v/wavefront.svg)](https://crates.io/crates/wavefront)
[![crates.io](https://docs.rs/wavefront/badge.svg)](https://docs.rs/wavefront)

A [Wavefront OBJ](https://en.wikipedia.org/wiki/Wavefront_.obj_file) parser and utility crate.

```toml
[dependencies]
wavefront = "x.y.z"
```

## Example

```rust
let model = wavefront::Obj::from_file("tests/ship.obj").unwrap();
```

<img src="https://raw.githubusercontent.com/zesterer/wavefront/master/misc/screenshot.png" alt="A parsec isn't a unit of time, Han" width="50%"/>

# Features

- Ergonomic API for parsing OBJs from files and readers.

- Wrapper types that automatically perform indexing and hide the annoyances of the OBJ format if you just want to
  grab some triangles.

- Correct handling of complex polygons.

## Roadmap

- Support for materials and the MTL format.

- Support for arbitrary geometry.

## License

`wavefront` is distributed under either of:

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at the disgression of the user.
