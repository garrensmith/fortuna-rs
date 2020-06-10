# Fortuna-rs

A javascript view engine for CouchDB 4.x written in Rust using Google V8.

It follows the [Ateles](https://github.com/cloudant-labs/ateles) protocol. 

## Usage

Setup Apache CouchDB on FDB (branch: prototype/fdb-layer).
Add Ateles to CouchDB and set it as the javascript server in `default.ini`.

Then in this folder:
```
$ cargo run --release --bin fortuna
```

## Benchmarking

`client.rs` can be used to run some basic benchmarks against Fortuna-rs.
You can configure the number of total requests, simultaneous requests and the
number of docs to map.

To run:

```
$ cargo run --release --bin client
```