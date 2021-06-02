# evtx-cli
An EVTX parser CLI written in rust. Wraps the [`evtx`](https://github.com/omerbenamram/evtx) crate.

## Features

* Accepts a glob string matching for matching one or multiple files files (ie. './HOSTNAME_*/*.evtx').
* Simple JSON output, easy for doing additional transformations into 

## Usage and Installation

Until the tool is in a state where I can be on crates.io please clone the project and build (`cargo build --release`) the tool locally.