# Wavefront

[![crates.io](https://img.shields.io/crates/v/wavefront.svg)](https://crates.io/crates/wavefront)
[![crates.io](https://docs.rs/wavefront/badge.svg)](https://docs.rs/wavefront)

A [Wavefront OBJ](https://en.wikipedia.org/wiki/Wavefront_.obj_file) parser and
utility crate.

```toml
[dependencies]
wavefront = "x.y.z"
```

## Example

```rust
let model = wavefront::Obj::from_file("tests/ship.obj").unwrap();

for [a, b, c] in model.triangles() {
    // No index lookup required: wavefront handles this for you!
    println!("{:?} {:?} {:?}", a.position(), b.position(), c.position());
}
```

<p align="center">
	<img src="https://raw.githubusercontent.com/zesterer/wavefront/master/misc/screenshot.png" alt="A parsec isn't a unit of time, Han" width="50%"/>
</p>

# Features

- Ergonomic API for parsing OBJs from files and readers.

- Wrapper types that automatically perform indexing and hide the annoyances of
  the OBJ format if you just want to grab some triangles...

- ...but allows you to dip into the nitty-gritty details of OBJ if you want to
  do that too.

- Correct handling of complex polygons.

- No dependencies

## Roadmap

- Materials and the MTL support.

- Object, group, polygon, vertex and vertex attribute insertion

- Saving

- Arbitrary geometry support.

## Why not [alternative]?

`wavefront` was born of a general feeling that the API of existing OBJ parsers
were either unnecessarily verbose or didn't properly handle the heirarchical
structure of the OBJ format. `wavefront` aims to couple correct handling of the
format's features with a clean, terse API that allows you to jump straight to
the thing you want to do: rendering your model.

## License

`wavefront` is distributed under either of:

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at the disgression of the user.
