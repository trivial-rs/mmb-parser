# mmb-parser

[![Crates.io](https://img.shields.io/crates/v/mmb-parser.svg)](https://crates.io/crates/mmb-parser)
[![Documentation](https://docs.rs/mmb-parser/badge.svg)](https://docs.rs/mmb-parser)
![License](https://img.shields.io/crates/l/mmb-parser.svg)

A library for parsing binary [Metamath Zero](https://arxiv.org/abs/1910.10703) proof files.

The proof files for Metamath Zero are designed to be consumed by the verifier in situ, such that parsing the file into complicated data structures is not necessary.
This library exposes the internals of the file format for inspecting and debugging purposes.

## License

This library is distributed under the terms of either the MIT license (see [LICENSE-MIT](LICENSE-MIT)) or the Apache License, Version 2.0 (see [LICENSE-APACHE](LICENSE-APACHE)).
