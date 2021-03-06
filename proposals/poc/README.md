# wasi-crypto-preview

A proposal for a WASI cryptography API.

* Interface definition:
  * common types and functions ([witx](witx/proposal_common.witx), [doc](witx/proposal_common.md))
  * symmetric operations ([witx](witx/proposal_siymmetric.witx), [doc](witx/proposal_symmetric.md))
  * common types and functions for asymmetric operations ([witx](witx/proposal_asymmetric_common.witx), [doc](witx/proposal_asymmetric_common.md))
  * signatures ([witx](witx/proposal_signatures.witx), [doc](witx/proposal_signatures.md))
  * key exchange ([witx](witx/proposal_kx.witx), [doc](witx/proposal_kx.md))
* [Short API overview](witx/wasi_ephemeral_crypto.txt)
* [Toy implementation](https://github.com/jedisct1/wasi-crypto-preview/tree/master/implementation)
* [Wasmtime integration](https://github.com/jedisct1/wasmtime-crypto)
* [Example AssemblyScript bindings](https://github.com/jedisct1/as-crypto)
* [Example Rust bindings](https://github.com/jedisct1/rust-wasi-crypto-guest-api)

## Testing the API

The example implementation exports:

* A Rust interface `CryptoCtx` modeled after the `witx` file, but that can be directly used without a WebAssembly runtime.
* A thin `WasiCryptoCtx` layer that directly maps that API to the WASI calling conventions, using `wiggle`.

`CryptoCtx` can be used to quickly experiment with the API in Rust.

Other languages can use the [`wasmtime` fork](https://github.com/jedisct1/wasmtime-crypto) above as a WebAssembly runtime in order to access the crypto API.

In that configuration, the API can be accessed via the exported `wasi_ephemeral_crypto` module.

See the AssemblyScript and Rust bindings as an example.

Currently supported algorithms as a proof of concept:

* `ECDSA_P256_SHA256`
* `ECDSA_P384_SHA384`
* `Ed25519`
* `RSA_PKCS1_2048_8192_SHA256`
* `RSA_PKCS1_2048_8192_SHA384`
* `RSA_PKCS1_2048_8192_SHA512`
* `RSA_PKCS1_3072_8192_SHA384`
* `HKDF-EXTRACT/SHA-256`
* `HKDF-EXTRACT/SHA-512`
* `HKDF-EXPAND/SHA-256`
* `HKDF-EXPAND/SHA-512`
* `HMAC/SHA-256`
* `HMAC/SHA-512`
* `SHA-256`
* `SHA-512`
* `SHA-512/256`
* `AES-128-GCM`
* `AES-256-GCM`
* `XOODYAK-128`
* `XOODYAK-256`
* `X25519`
* `KYBER768`
