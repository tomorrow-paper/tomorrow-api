# tomorrow-api

[![](https://api.travis-ci.org/tomorrow-paper/tomorrow-api.svg?branch=master)](https://travis-ci.org/tomorrow-paper/tomorrow-api)
[![](http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-2.png)](http://www.wtfpl.net/)

## Quickstart 

`tomorrow-api` requires the latest nightly version of the Rust toolchain.
To override your toolchain configuration for this project, use the following command in the project's directory:

```sh
$ rustup override set nightly
```

## Build

In order to use `tomorrow-api` at its full potential, do not forget to build it in *release* mode:

```sh
$ cargo build --release
$ ./target/release/tomorrow-api
```