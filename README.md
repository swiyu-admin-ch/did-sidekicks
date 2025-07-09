![Public Beta banner](https://github.com/e-id-admin/eidch-public-beta/blob/main/assets/github-banner-publicbeta.jpg)

# DID sidekicks

An official Swiss Government project made by
the [Federal Office of Information Technology, Systems and Telecommunication FOITT](https://www.bit.admin.ch/)
as part of the electronic identity (e-ID) project.

This repo features a set of various Rust modules required while implementing either of 
[did:tdw (v0.3)](https://identity.foundation/didwebvh/v0.3/) and [did:webvh (v1.0)](https://identity.foundation/didwebvh/v1.0) specifications.
However, the modules are neither closely related nor specific to any of these specs and may be freely used elsewhere, for other purposes.

More specifically, each of the Rust modules has its own purpose and implements rather partially one of the [referenced specifications](https://identity.foundation/didwebvh/v1.0/#references),
such as:
- [Data Integrity EdDSA Cryptosuites v1.0](https://www.w3.org/TR/vc-di-eddsa)
- [Decentralized Identifiers (DIDs) v1.0](https://www.w3.org/TR/did-core/)
- [Multiformats](https://datatracker.ietf.org/doc/draft-multiformats-multibase/08/)
- [JSON Canonicalization Scheme (JCS)](https://www.rfc-editor.org/rfc/rfc8785)
- etc.

## Using the library

The library can be used either directly in Rust as is.

### Rust

The library can be used directly in rust by adding the following dependency to your `Cargo.toml`:

````toml
[dependencies]
# Alternatively, feel free to so use tag=<ANY_EXISTING_VERSION> instead of branch="main"
did_sidekicks = { git = "https://github.com/swiyu-admin-ch/did-sidekicks.git", branch = "main" }
````

## License

This project is licensed under the terms of the MIT license. See the [LICENSE](LICENSE.md) file for details.
