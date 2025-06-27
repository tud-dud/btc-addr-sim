# btc-addr-sim

![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

Simulate occupying a node's outgoing connection slots at a given rate of
disconnections with a given number of IP prefixes.
It parses the output of `bitcoin-cli getrawaddrman`, runs the simulation and
writes the results to a CSV file.

## Requirements

1. [rustup](https://rustup.rs/)
    - assumes a C linker is already installed, e.g., `sudo apt install
      build-essential` on Ubuntu or `xcode-select --install` on MacOS.

## Usage

1. Compile
      ```bash
        cargo build --release
      ```
2. Execute

    ```bash
         btc-addr-sim [OPTIONS] [VERBOSE]

         Arguments:
           [VERBOSE]

         Options:
           -l, --log <LOG_LEVEL>            [default: info]
           -a, --addrman <PEERS>            Path to JSON file containing the
                                            addrman dump obtained using bitcoin-cli getrawaddrman. The expected
                                            format is that of Bitcoin Core 28.0 [default: ./rawaddrman.json]
           -n, --num <NUM_ADDRS>            Number of /16 attacker addresses to generate [default: 10]
           -s, --seed <SEED>                Seed for the RNG [default: 999]
           -u, --until <STOP_AFTER>         Stop after this many steps [default: 0]
           -c, --concurrency <CONCURRENCY>  How many regular connections to terminate per round [default: 1]
           -o, --out <OUTPUT_DIR>           Path to directory where the results will be stored
           -h, --help                       Print help
           -V, --version                    Print version
    ```
