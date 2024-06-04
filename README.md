# Readme

This crate does the light wrapping on [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin/) required for use in [uniffi](https://github.com/mozilla/uniffi-rs)-based libraries.

<br>

## Usage

To leverage these types in your uniffi library, simply:
1. Add a dependency on this crate:
```toml
rust-bitcoin-ffi = { git = "https://github.com/thunderbiscuit/rust-bitcoin-ffi.git", branch = "master" }
```

2. Add the following declarations to your UDL file:
```idl
[ExternalInterface="bitcoin_ffi"]
typedef extern Script;
```

3. Add the following to your uniffi.toml config:
```toml
[bindings.kotlin.external_packages]
# Map the crate names from [External={name}] into Kotlin package names
bitcoin_ffi = "org.bitcoindevkit.bitcoin"
```
