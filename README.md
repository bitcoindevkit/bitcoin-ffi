# Readme

This crate does the light wrapping on [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin/) required for use in [uniffi](https://github.com/mozilla/uniffi-rs)-based libraries.

## Usage

To leverage these types in your uniffi library, simply:
1. Add a dependency on this crate:
```toml
bitcoin-ffi = { git = "https://github.com/bitcoindevkit/bitcoin-ffi.git", tag = "v0.1.2" }
```

2. Add the following declarations to your UDL file for the types you wish to import. To read more about external type definitions, see [this page on the Uniffi documentation](https://mozilla.github.io/uniffi-rs/latest/udl/ext_types_external.html).
```idl
[ExternalInterface="bitcoin_ffi"]
typedef extern Script;

[External="bitcoin_ffi"]
typedef extern Network;
```

3. Add the following to your uniffi.toml config:
```toml
[bindings.kotlin.external_packages]
# Map the crate names from [External={name}] into Kotlin package names
bitcoin_ffi = "org.bitcoindevkit.bitcoin"
```
