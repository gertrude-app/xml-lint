# xml-lint

Lint and reformat XML files. A command line tool for CI pipelines and local dev wrapping
the [`xmlem`](https://github.com/xmlem/xmlem) crate.

## Usage

```sh
# lint XML files
xml-lint path/to/file.xml
xml-lint file1.xml file2.xml file3.xml

# fix formatting
xml-lint --fix path/to/file.xml

# see config options, flags, etc.
xml-lint --help
```

## Installation

```sh
# requires rust/cargo toolchain
cargo install xml-lint
```

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or
<a href="LICENSE-MIT">MIT license</a> at your option. Any contribution intentionally
submitted for inclusion in Asciidork by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
