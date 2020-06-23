# Fortuna-rs

A javascript view engine for CouchDB 4.x written in Rust using Google V8.

It follows the [Ateles](https://github.com/cloudant-labs/ateles) protocol. 

## Usage With CouchDB

Install [FoundationDB](https://apple.github.io/foundationdb/downloads.html)
Install [CouchDB dependancies](https://github.com/apache/couchdb/blob/prototype/fdn/INSTALL.Unix.md)
Setup CouchDB:
```
    git clone https://github.com/apache/couchdb.git
    git checkout -t origin/prototype/fdb-layer
    
```
Add:
```
{ateles, {url, "https://github.com/cloudant-labs/ateles"}, {branch, "master"}}
```
to the third party deps in `rebar.config.script`. 
Then in `[couch_eval.languages]` in `rel/overlay/etc/default.ini` set `javascript = ateles`. 
In `rel/reltool.config` add `ateles` to the list.

run `./configure --dev`
Finally in the CouchDB repo run `make && dev/run -n 1 -a adm:pass` to start CouchDB. 


In the Fortuna-rs repo run: 
```
cargo run --release --bin fortuna.
```

Create documents and design docs and watch Fortuna help index them.

## Benchmarking

`client.rs` can be used to run benchmarks against Fortuna-rs.
You can configure the number of total requests, simultaneous requests and the
number of docs to map.

To run:

```
$ cargo run --release --bin client
```