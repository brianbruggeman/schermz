# Testing

The tests are run using `cargo`:

```bash
cargo test
```

## Insta

The tests use [insta](https://docs.rs/insta/0.15.0/insta/) for snapshot testing.


### Installation

To install, run:

```bash
cargo install cargo-insta
```

### Usage

If you want to update the snapshots, run:

```bash
cargo insta review
```

