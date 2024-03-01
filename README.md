
# substrate-coin-flipper

Project to learn how to create a Substrate Node by developing a custom pallet that to create, flip and toss a Coin.

Why a coin flipper? It's a [common smart contract example](https://github.com/paritytech/ink-playgroung-flipper/blob/main/lib.rs), and the aim is to show how to start developing a simple Substrate FRAME pallet. There is also logic to toss a coin randomly and lots of tests.

This project belongs to Part 1 of a series of Substrate Tutorials.

Enjoy!

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

Start with:

```sh
./target/release/node-template --dev
```

### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint, in Development mode using Local Node.
