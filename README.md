# dertect

Utility to extract ASN.1 data structures from arbitrary files.

Note: Currently, it only supports DER `SEQUENCE`s with a long form length of exactly two bytes (`[0x30, 0x82, len1, len2, <data>]`).

## Usage

Note: [`der2ascii`](https://github.com/google/der-ascii) must be in your `$PATH`.

```bash
cargo run --release -- <file>
```
