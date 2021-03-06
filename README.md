# WebSockets (websockets)

This is a library which implements [RFC
6455](https://tools.ietf.org/html/rfc6455), "The WebSocket Protocol".

[![Crates.io](https://img.shields.io/crates/v/websockets.svg)](https://crates.io/crates/websockets)
[![Documentation](https://docs.rs/websockets/badge.svg)][dox]

More information about the Rust implementation of this library can be found in
the [crate documentation][dox].

[dox]: https://docs.rs/websockets

The `WebSocket` type implements the WebSocket protocol, in either client or
server role.

This is a multi-language library containing independent implementations
for the following programming languages:

* C++
* Rust

## Testing the Rust Implementation with the Autobahn Testsuite

Two example programs are provided which were designed to facilitate testing the
WebSockets implementation against the [Autobahn Testsuite]:

* `autobahn-server` &ndash; operates a WebSocket server meant to be tested with
  the `fuzzingclient`.
* `autobahn-client` &ndash; runs the tests provided by the `fuzzingserver`
  meant to test a WebSocket client.

See the [Autobahn Testsuite] documentation for how to install the testsuite
Docker image and operate the `fuzzingserver`.  Part of the process involves
setting up a configuration file and running Docker.

Note that because network connections must be established between servers and
clients operating inside and outside the Docker container, IP addresses are
different, even if running Docker and the example programs on the same host.
The examples below assume the same host is running both the Docker container
and the example programs, so that `autobahn-server` is accessible from
`fuzzingclient` (connecting from inside the Docker container to the outside)
through the address `172.17.0.1`, and that `fuzzingserver` is accessible from
`autobahn-client` (connecting to the inside of the Docker container from
outside) through the address `192.168.0.221`.  In other words, `172.17.0.1` is
the address of the host from within the Docker container, and `192.168.0.221`
is the address of the host itself from outside the Docker container.  You may
need to change either or both of these IP addresses to make the tests work
properly for your environment.

Also note that certain paths in the example below may be different depending on
where in the Docker host filesystem the container is located.  For example, you
may need to change `/home/rwalters/autobahn` to match where you have installed
the Docker container.

[Autobahn Testsuite]: https://github.com/crossbario/autobahn-testsuite

### Using `fuzzingserver` to test WebSocket clients

The following is an example configuration file for operating the
`fuzzingserver`:

```json
{
    "url": "ws://127.0.0.1:9001",
    "outdir": "./reports/clients",
    "cases": ["*"],
    "exclude-cases": [],
    "exclude-agent-cases": {}
}
```

The following is an example command for operating `fuzzingserver`:

```sh
docker run -it --rm -v /home/rwalters/autobahn/config:/config -v /home/rwalters/autobahn/reports:/reports -p 9001:9001 --name fuzzingserver crossbario/autobahn-testsuite
```

Start the testsuite server first and then launch the client to run the tests:

```sh
cargo run --example autobahn_client -- ws://192.168.0.221:9001
```

Note that the `fuzzingserver` will continue running/serving after the tests are
complete.  The test results are generated by `fuzzingserver` as HTML files; to
view them for example:

```sh
firefox /home/rwalters/autobahn/reports/servers/index.html
```

### Using `fuzzingclient` to test WebSocket server

The following is an example configuration file for operating the
`fuzzingclient`:

```json
{
    "servers": [
        {
            "agent": "rhymu-websocket",
            "url": "ws://172.17.0.1:9002"
        }
    ],
    "outdir": "./reports/servers",
    "cases": [
        "*"
    ],
    "exclude-cases": [],
    "exclude-agent-cases": {}
}
```

Note that the order of running the testsuite and the example are swapped.
Start the example server first:

```sh
cargo run --example autobahn_server
```

After the example server is started, run the `fuzzingclient` to perform the
tests.  The following is an example command for operating `fuzzingclient`:

```sh
docker run -it --rm -v /home/rwalters/autobahn/config:/config -v /home/rwalters/autobahn/reports:/reports --name fuzzingclient crossbario/autobahn-testsuite wstest -m fuzzingclient -s /config/fuzzingclient.json
```

Note that the example server will continue running/serving after the tests are
complete.  The test results are generated by `fuzzingclient` as HTML files; to
view them for example:

```sh
firefox /home/rwalters/autobahn/reports/clients/index.html
```

## Building the C++ Implementation

A portable library is built which depends on the C++11 compiler, the C++
standard library, and non-standard dependencies listed below.  It should be
supported on almost any platform.  The following are recommended toolchains for
popular platforms.

* Windows -- [Visual Studio](https://www.visualstudio.com/) (Microsoft Visual
  C++)
* Linux -- clang or gcc
* MacOS -- Xcode (clang)

This library is not intended to stand alone.  It is intended to be included in
a larger solution which uses [CMake](https://cmake.org/) to generate the build
system and build applications which will link with the library.

There are two distinct steps in the build process:

1. Generation of the build system, using CMake
2. Compiling, linking, etc., using CMake-compatible toolchain

### Prerequisites

* [CMake](https://cmake.org/) version 3.8 or newer
* C++11 toolchain compatible with CMake for your development platform (e.g.
  [Visual Studio](https://www.visualstudio.com/) on Windows)
* [Base64](https://github.com/rhymu8354/Base64.git) - a library which
  implements encoding and decoding data using the Base64 algorithm, which
  is defined in [RFC 4648](https://tools.ietf.org/html/rfc4648).
* [Hash](https://github.com/rhymu8354/Hash.git) - a library which implements
  various cryptographic hash and message digest functions.
* [Http](https://github.com/rhymu8354/Http.git) - a library which implements
  [RFC 7230](https://tools.ietf.org/html/rfc7230), "Hypertext Transfer Protocol
  (HTTP/1.1): Message Syntax and Routing".
* [StringExtensions](https://github.com/rhymu8354/StringExtensions.git) - a
  library containing C++ string-oriented libraries, many of which ought to be
  in the standard library, but aren't.
* [SystemAbstractions](https://github.com/rhymu8354/SystemAbstractions.git) - a
  cross-platform adapter library for system services whose APIs vary from one
  operating system to another
* [Utf8](https://github.com/rhymu8354/Utf8.git) - a library which implements
  [RFC 3629](https://tools.ietf.org/html/rfc3629), "UTF-8 (Unicode
  Transformation Format)".

### Build system generation

Generate the build system using [CMake](https://cmake.org/) from the solution
root.  For example:

```bash
mkdir build
cd build
cmake -G "Visual Studio 15 2017" -A "x64" ..
```

### Compiling, linking, et cetera

Either use [CMake](https://cmake.org/) or your toolchain's IDE to build.
For [CMake](https://cmake.org/):

```bash
cd build
cmake --build . --config Release
```
