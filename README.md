Snapshot tool to get all addresses with their amount and outputs from a snapshot file and writes it to a .json file.

Download a new full snapshot from https://chrysalis-dbfiles.iota.org/snapshots/hornet/latest-full_snapshot.bin or another source and then run it with `cargo run`

## Prerequisites

- [Rust](https://www.rust-lang.org/) (>= 1.56.1)
- [Cargo](https://doc.rust-lang.org/cargo/) (>= 1.56.0)

Output looks like
```JSON
{
  "addresses": {
    "iota1qp000s7l8e4lkpgevaz6ev37c8xtewhpckg583rhrq68tvf53ld5c9xek3f": {
      "balance": 3000000,
      "output_ids": [
        "a748f0524e8d7c29d106a9498cf471a8a53eb3a2d0fbd6fe686a418d4eff92a41500",
        "ab2198249f9be68e16c269da9e39d2438f470ab29b83e5197c850eb7e296131d1b00",
        "c931fe2f11e44549c031fee81bc7389feca647bdaaa9838c02cf5afe90e31b941a00"
      ]
    },
    "iota1qp004pff7fyvvkkyru2tq8ew8qykvf2m58ps77rp9dnvxhzkqvply8wcqw0": {
      "balance": 325174000,
      "output_ids": [
        "343616fa45b281f9a6e0183895fb78faadd9cd3b1d02237cb39dd34b15b873040000"
      ]
    },

    ...

    "iota1qzzzwhwn6w9kqvmf565m5hc8qfe695htsnm2c27s65kullwulxpfkvl65lz": {
      "balance": 13000000000,
      "output_ids": [
        "10581a75b56df9f52064e480a1c4702a1d84807c38310c1d55249c958401005b0100"
      ]
    }
  },
  "ledgerIndex": 1851913,
  "treasuryOutputAmount": 478017793194081
}
```
