# Vamo

[![crates.io](https://img.shields.io/crates/v/vamo?style=flat-square)](https://crates.io/crates/vamo) [![Build Status](https://github.com/ararog/vamo/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/vamo/actions/workflows/rust.yml) [![codecov](https://codecov.io/gh/ararog/vamo/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/vamo) [![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/vamo/latest/vamo)

## Description

**vamo** ("fine" portuguese slang) is a straightforward, non opinionated, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

## Attention

This release has a major api change. Please check the [migration guide](https://github.com/ararog/deboa/blob/main/MIGRATION_GUIDE.md) for more information. Keep in mind API for prior to 0.1.0 is subject to change. Proper deprecation will be added in the next stable release.

## Install

```toml
vamo = { version = "0.0.9" }
```

## Usage

```rust
use deboa::{
    request::{DeboaRequest, FetchWith, get},
    Result,
};
use deboa_tokio::Client;
use deboa_extras::serde::json::JsonBody;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a new Client instance, set timeouts, catches and protocol.
  let client = Client::new();

  let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
    .header(header::CONTENT_TYPE, "application/json")
    .send_with(&client)
    .await?
    .body_as(JsonBody)
    .await?;

  println!("posts: {:#?}", posts);

  Ok(())
}
```

## Subprojects

### [vamo](https://github.com/ararog/deboa/tree/develop/vamo)

Nice wrapper on top of deboa for dry rest client. Set base url once
and use it for all requests.

### [vamo-macros](https://github.com/ararog/deboa/tree/develop/vamo-macros)

Vamo macros is a collection of macros to make possible use structs as resources to be sent over vamo as client.
It is also the new home of bora macro.

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
