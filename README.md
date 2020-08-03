# Kiva: A small, fast, concurrent key-value store

## About
Kiva is a fast key-value store.
It's intended to be extremely performant and simple to use.
handle X of requests per second across multiple threads.
View [benchmarks](#benchmarks) for more details


## Implementation
Kiva serves network requests over a TCP connection for setting and retrieving values.
Multiple hosts can make requests to the same data store instance concurrently 
Under the hood, it utilizes user-space threads and channels to handle concurrency and
heavy load.


## Benchmarks


## Author
Shoyo Inokuchi (shoyoinokuchi@gmail.com)

