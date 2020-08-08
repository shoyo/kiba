# Kiba: A fast, multithreaded, in-memory database

## About
Kiba is an in-memory database that's designed to be extremely performant and simple to use.
Kiba is fundamentally a key-value store, but supports data types such as lists, sets, and hashes.
It exposes a similar API to [Redis](https://github.com/redis/redis).

## Benchmarks
TODO

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
% ./kiba
```
You can interact with the instance through a CLI by opening another terminal and running:
```
% ./kiba-cli
```

## Implementation
Kiba serves requests over a TCP connection for getting and setting values.
Channels are used to achieve memory safety for concurrent requests. A lightweight,
user-space thread is spawned for each client connection, which passes
messages through a channel to a single executor thread that manages the data store.
This pattern prevents threads from sharing state and causing race conditions or
dirty reads.

Kiba is comprised of layers that collectively parse and execute user queries:

```
                          -------------------
                          Layers of Execution
                          -------------------
      bytestream input
        (user query)
             v
----------------------------
                               PRIMARY TASK: Deserialize bytestream into an
                                 operator and vector of string arguments.

          Parser                 The parser performs very basic validation,
   (bytestream -> tokens)        such as flagging unknown commands as
                                 unrecognized and empty inputs as no-ops.

                                 Passes along a `ParserResult` struct to the
                                 validator for more rigorous validation.
----------------------------
                               PRIMARY TASK: Validate the semantics of the user
                                 query and construct a corresponding request to
                                 be executed.

                                 Checks that for a given operator, its arguments
                                 have the correct length and data types.
         Validator
    (tokens -> requests)         Constructs a `Request` struct and passes it
                                 along to the executor.

                                 Note: Never outright rejects a query. For
                                 invalid queries, simply creates an "Invalid"
                                 variant of a request containing an error
                                 message.
----------------------------
                               PRIMARY TASK: Execute requests by making
                                 appropriate calls to the store, and construct
                                 a response to be sent back to the user.

         Executor                If the store returns an error, the executor
  (requests -> responses)        creates a response containing the error
                                 message.

                                 Note: For "NoOp" and "Invalid" requests, the
                                 executor does not interact with the store and
                                 simply creates a corresponding response with an
                                 error message if needed.
----------------------------
                               PRIMARY TASK: Store data and expose an API.
          Store
    (source of truth)            Provides functions that are called by the
                                 executor.  Simply returns a result or an error
                                 according to its defined API.
----------------------------
```
              
## Author
Shoyo Inokuchi (shoyoinokuchi@gmail.com)

