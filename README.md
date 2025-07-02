# flutter_web_hasher

[![Crate](https://img.shields.io/crates/v/flutter_web_hasher.svg)](https://crates.io/crates/flutter_web_hasher)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/flutter_web_hasher)](https://github.com/xuxiaocheng0201/flutter_web_hasher/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/flutter_web_hasher)](https://github.com/xuxiaocheng0201/flutter_web_hasher/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/flutter_web_hasher)](https://github.com/xuxiaocheng0201/flutter_web_hasher/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/flutter_web_hasher)](https://github.com/xuxiaocheng0201/flutter_web_hasher/blob/master/LICENSE-MIT)

# Description

A tool that solves the browser caching problem of flutter web building products.

# Install

Install this tool by cargo:
`cargo install flutter_web_hasher`

# Usage

Run the command in the root directory of your flutter project:
```shell
flutter build web --release
flutter_web_hasher --skip index.html
```

The files in `build/web` have been processed.

```text
Add hash suffixes to flutter web products to resolve browser caching problem.

Usage: flutter_web_hasher [OPTIONS]

Options:
  -d, --directory <DIRECTORY>  Target directory [default: ./build/web]
  -s, --skip <SKIP>            Skip hash files. Stripped from `directory`. Not start with '/'
  -h, --help                   Print help
  -V, --version                Print version
```

# License

This project is licensed under either of

Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
