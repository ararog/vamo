---
layout: default
title: Vamo - A Rest API Client for deboa
nav_order: 1
description: "A Rest API Client for deboa"
permalink: /
---
<div align="center">
<h1><b>Vamo</b></h1>
</div>

[![crates.io](https://img.shields.io/crates/v/vamo?style=flat-square)](https://crates.io/crates/vamo)
[![Build Status](https://github.com/ararog/vamo/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/vamo/actions/workflows/rust.yml)
[![Documentation](https://docs.rs/vamo/badge.svg)](https://docs.rs/vamo/latest/vamo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**vamo** ("fine" portuguese slang) is a straightforward, non opinionated, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

Built on top of [deboa](https://github.com/ararog/deboa).

## Features

Please refer to the individual crate documentation for details:

- [vamo](./vamo#features)
- [vamo-macros](./vamo-macros#features)

## Quick Start

Please refer to the individual crate documentation for quick start guides:

- [vamo](./vamo#quick-start)
- [vamo-macros](./vamo-macros#quick-start)

## Crates

| Crate | Description | Documentation |
|-------|-------------|---------------|
| [vamo](./vamo) | DRY REST client wrapper | [![docs.rs](https://img.shields.io/docsrs/vamo/latest)](https://docs.rs/vamo) |
| [vamo-macros](./vamo-macros) | Macros for Vamo | [![docs.rs](https://img.shields.io/docsrs/vamo-macros/latest)](https://docs.rs/vamo-macros) |

## Create project from template

You can create a new project from the template using `cargo generate`:

`cargo generate ararog/deboa-templates`

## Documentation

- [API Reference](https://docs.rs/deboa)
- [Migration Guide](./MIGRATION_GUIDE.md)
- [Contributing Guide](./CONTRIBUTING.md)

## Other Projects

- [deboa](https://crates.io/crates/deboa) - HTTP client library
- [deboa-contrib](https://crates.io/crates/deboa-contrib) - HTTP client library extensions
- [caramelo](https://crates.io/crates/caramelo) - Assertion based test framrwork
- [easyhttpmock](https://crates.io/crates/easyhttpmock) - HTTP mock server
- [sofie](https://crates.io/crates/sofie) - Fullstack web framework
- [uget](https://crates.io/crates/uget) - CLI HTTP client
- [vetis](https://crates.io/crates/vetis) - Very Tiny Http server

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
