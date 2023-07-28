# Stake Pool Snapshotter

![CI Status](https://github.com/wowica/snapshotter/actions/workflows/ci.yml/badge.svg)

This CLI program takes a snapshot of a Cardano Stake Pool. Given a Stake Pool ID, it generates a list of Public Key Hashes currently delegated to the pool.

## Usage

Create a _blockfrost.toml_ with your https://blockfrost.io/ Project ID as follows:

```toml
project_id = "value-here"
```

Install the snapshotter CLI with the following command:

`cargo install --path .`

Run the CLI passing a Cardano Stake Pool Bech32 id as argument. For example, in order to take a snapshot from [JUNGLE Stake Pool](https://preview.cexplorer.io/pool/pool1j3x329u0uxh9s9vjvsad9kx37tzal8gndz6ttxumcz4nw947djw) in the Preview Network, run the folllowing:

`./target/release/snapshotter pool1j3x329u0uxh9s9vjvsad9kx37tzal8gndz6ttxumcz4nw947djw`