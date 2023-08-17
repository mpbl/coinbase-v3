# Rust bindings for Coinbase Advanced Trade API (v3) -- OAuth2 based

Coinbase's Advanced Trade API description can be found
[there](https://docs.cloud.coinbase.com/advanced-trade-api/docs/welcome).
Most of what this library provides are data bindings for the json responses and 
the GET/POST functions defined in the 
[API Reference](https://docs.cloud.coinbase.com/advanced-trade-api/reference).

All API calls need to be authenticated, either with the help of a Token, 
or using an OAuth2 based authentication. 
**This library implements the OAuth2 alternative**.

Developed and tested using ```API Version: 2022-10-16```

[![Rust](https://github.com/mpbl/coinbase-v3/actions/workflows/rust.yml/badge.svg)](https://github.com/mpbl/coinbase-v3/actions/workflows/rust.yml)

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, 
shall be dual licensed as above, without any additional terms or conditions.


## Thanks

Inspiration and some code was taken from the following projects:
- [coinbase-pro-rs](https://github.com/inv2004/coinbase-pro-rs)
- [coinbase-rs](https://https://github.com/j16r/coinbase-rs)
