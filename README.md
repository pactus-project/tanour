# Tanour

Tanour is a Contract actor executor for [Pactus](https://pactus.org/) blockchain.

## Features

### Standalone and stateless

Tanour can be launched as an independent and standalone process and it has no internal state.
Interacting with the Pactus blockchain happens through the set of Provider APIs.

![tanour-stateless](https://user-images.githubusercontent.com/8073510/133919171-0f5aea21-3f71-4b4b-99cd-465818b467d8.png)

### Actor model

Contract in Pactus are like Actors in the [Actor model](https://en.wikipedia.org/wiki/Actor_model).

Each contract actor:
 - Can be instantiated through the `instantiate` method.
 - Can process the message it receives through `process` method.
 - Can concurrently send a message to another contract actor through the `send_msg` method.

These are the only methods that each contract can expose to the outside world.

### Storage as file

Pactus storage is not a typical key-value pair system, but rather a separate file that each contract actor has read and write access to. Pactus storage offers a unique way of managing data in a distributed environment.

**Storage as key-value pairs**

![storage_map](https://user-images.githubusercontent.com/8073510/133919511-4924578b-d9bb-40a3-976d-9e3305872b55.png)

**Storage as file**

![storage_file](https://user-images.githubusercontent.com/8073510/133919510-b3c2b63f-f5bc-49f8-b90b-b93aa8ee5285.png)


### WebAssembly

Contract actor are written in WebAssembly and currently Tanour is using [Wasmer](https://wasmer.io/) to execute the contracts.

### Gas metering

TODO


## Building

Tanour requires **latest stable Rust version** to build. You can install Rust through [rustup](https://www.rustup.rs/).

In order to use Tanour as a webservice you also need to install [Cap'n Proto](https://capnproto.org/install.html).

To build the Tanour from the source code, you can follow these commands:

```
$ git clone https://github.com/pactus-project/tanour
$ cd tanour

# build in release mode
$ cargo build --release
```



## License

This package is licensed under the MIT License.
