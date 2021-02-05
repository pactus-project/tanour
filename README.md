# Tanour

Tanour is a stateless VM based on [parity-wasm](https://github.com/paritytech/parity-wasm) for blockchains.


## Building

Tanour requires **latest stable Rust version** to build. You can install Rust through [rustup](https://www.rustup.rs/).

In order to use Tanour as a webservice you also need to install [Cap'n Proto](https://capnproto.org/install.html).

To build the Tanour from the source code, you can follow these commands:

```
$ git clone https://github.com/zarb/tanour
$ cd tanour

# build in release mode
$ cargo build --release
```



## License

This package is licensed under the MIT License.
