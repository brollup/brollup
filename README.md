![Brollup](https://i.ibb.co/tc7S2JL/brollup-github.png)
Brollup is a Bitcoin layer two to enable bridgeless smart contract execution natively on Bitcoin. Providing a fully trustless execution environment with unilateral exit, Brollup ensures users retain complete control over their funds. By combining virtual UTXOs with a symmetric state channel architecture and leveraging Bitcoin DA, Brollup integrates Bitcoin payments directly into a virtual machine with global-state.
> [!NOTE]
> Brollup is currently in the early development phase.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Clone the repository and navigate into the project directory:

```sh
git clone https://github.com/brollup/brollup
cd brollup
```

## Usage

Run the program with the following command:

```sh
cargo run <chain> <mode> <bitcoin-rpc-url> <bitcoin-rpc-user> <bitcoin-rpc-password>
```

### Parameters:

- `<chain>`: The Bitcoin network to use. Supported values:
  - `signet`
  - `mainnet`
- `<mode>`: The mode in which the program runs. Supported values:
  - `node`: For running a Brollup node.
  - `operator`: For liquidity providers.
- `<bitcoin-rpc-url>`: The RPC URL of the Bitcoin node.
- `<bitcoin-rpc-user>`: The RPC username.
- `<bitcoin-rpc-password>`: The RPC password.

### Example:

```sh
cargo run signet node http://127.0.0.1:38332 user password
```

```sh
cargo run mainnet operator http://127.0.0.1:8332 user password
```

## Contributing

We welcome contributions! Please [check the areas where we need help](https://github.com/brollup/brollup/blob/main/CONTRIB.md) for more details on how you can contribute.

## License

This project is licensed under the CC0 1.0 Universal License. See the `LICENSE` file for details.

