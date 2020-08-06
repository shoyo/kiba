# Kiva: A fast, concurrent, in-memory database

## About
Kiva is an extremely fast in-memory database influenced heavily by Redis.
Kiva is designed to be extremely performant and simple to use. Kiva is fundamentally
a key-value store, but supports complex data types such as lists, sets, and hashes.
It exposes a similar API to Redis, and 


## Benchmarks

## Implementation
Kiva serves network requests over a TCP connection for setting and retrieving values.
Multiple hosts can make requests to the same data store instance concurrently.
Under the hood, it utilizes user-space threads and channels to handle concurrency and
scale to heavy load.

## Building
To build Kiva, run:
```
% cargo build --release
```
To test the build, run:
```
% cargo test
```

## Running
To spin up Kiva with default settings, run:
```
% ./kiva-server
```
You can interact with the instance through a CLI by opening another terminal and running:
```
% ./kiva-cli
```

## Author
Shoyo Inokuchi (shoyoinokuchi@gmail.com)

