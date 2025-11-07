# Hello Solana (Native)

# Build

Generates `.so` file under `target/deploy`
```shell
cargo build-sbf
```

# Test with LiteSVM
```shell
cargo test -- --nocapture
```

# Test with script

Run local validator
```shell
solana config set -ul
solana-test-validator
```

Check program id
```shell
solana address -k ./target/deploy/hello-keypair.json
```

Deploy program
```shell
solana program deploy ./target/deploy/hello.so
```

Execute demo script
```shell
PROGRAM_ID=your program ID
RPC=https://api.devnet.solana.com
KEYPAIR=path to key pair

cargo run --example demo $KEYPAIR $RPC $PROGRAM_ID
```

# Deploy to Devnet

```shell
solana config set -ud

solana balance
solana airdrop 1

cargo build-sbf

solana program deploy ./target/deploy/hello.so

PROGRAM_ID=your program ID
RPC=https://api.devnet.solana.com
KEYPAIR=path to key pair

cargo run --example demo $KEYPAIR $RPC $PROGRAM_ID
```

Check transaction signature at [Solana explorer](https://explorer.solana.com/?cluster=devnet)
