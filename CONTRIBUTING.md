# Contributing

Contributions are welcome. Please consider installing recommended extensions
and running any configured linters.

## Prerequisites

You'll need to following software to build and run this project:

* [Node.js](https://nodejs.org) LTS or newer. After installation, run `npm i`
  to install dependencies.
* [Rust](https://www.rust-lang.org). After installation, run `rustup show` to
  to install necessary components and targets.

## Building

To build everything for production, run:

```bash
npm run build
```

This will build the WASM and run Webpack. You can find the static site in the
`dist/` directory.

## Serving

To build and serve a debug version, run:

```bash
npm start
```

This will build the WASM, run Webpack, and serve the content. See console
output for the URL, but this should often be <http://localhost:8080>.
