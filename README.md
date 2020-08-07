# Kiba: A fast, multithreaded, in-memory database

## About
Kiba is an extremely fast in-memory database influenced heavily by Redis.
It is designed to be extremely performant and simple to use. Kiba is fundamentally
a key-value store, but supports complex data types such as lists, sets, and hashes.
It exposes a similar API to Redis, ...


## Benchmarks

## Implementation
Kiba serves network requests over a TCP connection for setting and retrieving values.
Multiple hosts can make requests to the same data store instance concurrently.
Under the hood, it utilizes user-space threads and channels to handle concurrency and
scale to heavy load.

## Building
Install the Rust toolchain if you haven't already (https://www.rust-lang.org/tools/install).

To build Kiba, run:
```
% cargo build --release
```
To test the build, run:
```
% cargo test
```

## Running
To spin up Kiba with default settings, run:
```
% ./kiba-server
```
You can interact with the instance through a CLI by opening another terminal and running:
```
% ./kiba-cli
```

## Author
Shoyo Inokuchi (shoyoinokuchi@gmail.com)

