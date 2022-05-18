# connect-sdk-rust

The 1Password Connect Rust SDK provides your Rust applications access to the 1Password Connect API hosted on your infrastructure and leverage the power of 1Password Secrets Automation

The library can be used by Rust applications, tools, and other automations to access and manage items in 1Password Vaults.

## Early draft, please wait for initial release

This is an early draft which is a work in progress.  I'm post this earlier so as to setup my crate
online.  Please wait for a proper release, out in due course.

## Installation

## Usage

1. Start by copying `.env-sample` to `.env`, making sure to update its values.
2. Follow the [instructions to start
   Connect](https://github.com/1Password/connect) and make sure the Docker
   container starts at `http://localhost:8080`
3. Make sure to update `OP_API_TOKEN` in the `.env` file.

#### Creating an API client

```rust
use connect_1password::connect::Connect;

let connect = Connect::new();
```

#### Retrieving Vaults

```rust
// Get all vaults
let (vaults, _) = connect.vault().get_vaults().await?;
```

Refer to the [docs](https://docs.rs/connect-1password/0.1.0/connect_1password/) for further examples.

### HTTPClient

Under the hood, [Hyper](https://hyper.rs/) is used with [hyper_rustls](https://docs.rs/hyper-rustls/latest/hyper_rustls/) which supports both HTTP and TLS connections out of the box.

### Plans for the future

- The current goal is to cover all existing API end-points, this should ideally not take too long
- Make the HTTP client pluggable
- TBD

## Development

### Running Tests

From repository root:

```shell script
cargo test
```

### Building

```shell script
cargo build --release
```

## This is not an official SDK

This Rust crate has been created solely by me and so far is not "officially" supported by the
awesome folk at 1Password.  This may change, maybe and if they would like to help out and/or adopt
this into the 1Password ecosystem, that would be nice as well.

---

# About 1Password

**[1Password](https://1password.com/)** is the world’s most-loved password manager. By combining industry-leading security and award-winning design, the company provides private, secure, and user-friendly password management to businesses and consumers globally. More than 60,000 business customers trust 1Password as their enterprise password manager.

# Security

1Password requests you practice responsible disclosure if you discover a vulnerability. Please submit discoveries via [BugCrowd](https://bugcrowd.com/agilebits).

For information about security practices, please visit our [Security homepage](https://1password.com/security/).

# MSRV

This project is tested against the most [recent stable rust version](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c).
