# Kiba: An in-memory, multithreaded key-value store

## About
Kiba is an in-memory database that's designed to be extremely performant and simple to use.
Kiba is fundamentally a key-value store, but supports complex value types such as lists, sets, and hashes.
It exposes a similar API to [Redis](https://github.com/redis/redis), such as `GET`, `SET`,
`INCR`, `DECR`, `LPUSH`, `RPUSH`, `SADD`, `SREM`, `HSET`, `HGET` and more.

*Disclaimer*: Kiba is a side-project that's still very early in its development. Needless to say, it
shouldn't be trusted in any remotely serious setting. I plan to continue developing its feature set and
improving reliability so that it'll someday be production-ready.

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
% cd target/release
% ./kiba
```
Alternatively, you can specify custom settings in the config file `kiba.conf` and pass it in as a command-line argument:
```
% ./kiba /path/to/kiba.conf
```
You can interact with the server instance through a CLI by opening another terminal and running:
```
% cd target/release
% ./kiba-cli
```
Alternatively, you can specify the URL of the server you're connecting to (if you've changed it from the default) by passing it in as a command-line argument:
```
% ./kiba-cli <hostname>:<port>
```

## Docker
You can build and run a Kiba server instance inside a Docker container.  

To pull the image locally, run:
```
% docker pull shoyo64/kiba:0.1
```

To run a Kiba server instance in a Docker container, run:
```
% docker run -p 6464:6464 --name kiba shoyo64/kiba:0.1
```
Start and stop the server instance (respectively) with:
```
% docker start kiba
% docker stop kiba
```

You can then connect to the container running on the host machine through the CLI as before:
```
% ./kiba-cli
```

(Optional) Instead of pulling from Docker Hub, you can also clone the `Dockerfile` in this repository and build it yourself:
```
% docker build -t <tagname> .
% docker run -p <host_port>:6464 --name <name> <tagname>
```

## Examples
The following shows some basic examples of interacting with an instance through the CLI.

Strings:
```
kiba> SET name "FOO BAR"
OK

kiba> GET name
"FOO BAR"

kiba> GET bar
(nil)

kiva> SET counter 9999
OK

kiba> INCR counter
(integer) 10000

kiba> DECRBY counter 3000
(integer) 7000
```

Lists:
```
kiba> LPUSH letters b
(integer) 1

kiba> LPUSH letters a
(integer) 2

kiba> RPUSH letters c
(integer) 3

kiba> LPOP letters
"a"

kiba> LPOP letters
"b"

kiba> LPOP letters
"c"
```

Sets:
```
kiba> SADD colors red
(integer) 1

kiba> SADD colors blue
(integer) 2

kiba> SADD colors green
(integer) 3

kiba> SMEMBERS colors
1) blue
2) green
3) red
```

Hashes:
```
kiba> HSET user:321 name "John Smith"
(integer) 1

kiba> HSET user:321 date_joined 2020-01-01
(integer) 1

kiba> HGET user:321 username
"John Smith"

kiba> HGET user:321 date_joined
"2020-01-01"
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
                                 operator and vector of string argument tokens.

           Lexer                 The lexer performs very basic validation,
   (bytestream -> tokens)        such as flagging unknown commands as
                                 unrecognized and empty inputs as no-ops.

                                 Passes along a `LexerResult` struct to the
                                 validator for more rigorous validation.
----------------------------
                               PRIMARY TASK: Validate the semantics of the user
                                 query and construct a corresponding request to
                                 be executed.

                                 Checks that for a given operator, its arguments
                                 have the correct length and data types.
           Parser
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

