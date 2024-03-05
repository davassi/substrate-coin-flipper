
# substrate-coin-flipper

*Substrate-coin-flipper* is a project to learn how to create a substrate node by developing a custom Pallet that creates, flips and tosses a coin.

Why a coin flipper? It's a [common smart contract example](https://github.com/paritytech/ink-playgroung-flipper/blob/main/lib.rs) rewritten for Substrate, with logic to flip a coin at random and lots of testing.

The goal is to show how to develop and test a simple Substrate FRAME palette,
This project is part 1 of a series of Substrate tutorials.

Enjoy!

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Test

Use the following command to run the regression tests:

```sh
cargo test
```

### Start

To start the node (in development mode), just type:

```sh
cargo run --dev
```

### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint, in Development mode using Local Node.

## Contribution

This tutorial is a continuous work in progress. If you have suggestions for features, or if you find any issues in the code, design, interface, tutorial etc, please feel free to share them on [GitHub](https://github.com/davassi/substrate-coin-flipper/issues).

I appreciate very much your feedback!
