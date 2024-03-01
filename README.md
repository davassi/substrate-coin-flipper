
# substrate-coin-flipper

This substrate node exposes Extrinsics to create, flip and toss a coin.

Why a coin flipper? It's a common smart contract example, and my aim is to make it easier to understand how to start developing a Substrate FRAME palette.

There is some random logic to make it fancier and lots of tests.

Enjoy!

### Build

Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Connect with Polkadot-JS Apps Front-End

After you start the node template locally, you can interact with it using the hosted version of the [Polkadot/Substrate Portal](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) front-end by connecting to the local node endpoint.
