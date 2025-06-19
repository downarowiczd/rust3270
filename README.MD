<div align="center">

# rust3270

[![Build status](https://github.com/downarowiczd/rust3270/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/downarowiczd/rust3270/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/rust3270.svg)](https://crates.io/crates/rust3270)
[![dependency status](https://deps.rs/repo/github/downarowiczd/rust3270/status.svg)](https://deps.rs/repo/github/downarowiczd/rust3270)

rust3270 is a terminal server implementation for the IBM 3270 terminal protocol, written in Rust.
</div>

## Usage

See the example in the `examples` directory for how to use the library.

## To Do
- [ ] Implement different code pages
- [ ] Implement SSL/TLS support
- [ ] Implement different screen sizes depending on the terminal type
- [ ] High level API for sending and receiving data and designing screens

## Useful Links

I learned a lot from the following resources:
- [Go3270]: A Go implementation of the 3270 protocol, which is well documented and was the base for me learning the protocol.
- [TommySprinkle]: A great resource for understanding the 3270 data stream.
- [RFC1576]: The RFC that defines the current practices for the 3270 protocol.
- [RFC1041]: The RFC that defines the Telnet regime options.
- [RFC854]: The RFC that defines the Telnet protocol.

[Go3270]: https://github.com/racingmars/go3270
[TommySprinkle]: http://www.tommysprinkle.com/mvs/P3270/
[RFC1576]: https://tools.ietf.org/html/rfc1576
[RFC1041]: https://tools.ietf.org/html/rfc1041
[RFC854]: https://tools.ietf.org/html/rfc854