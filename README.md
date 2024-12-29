# Typst-Analyzer

---

A Language Server for [Typst](https://typst.app/).

## Overview

Typst-Analyzer is a Language Server Protocol (LSP) implementation designed for the Typst typesetting system. It aims to provide insights and tools for developers working with Typst files.

> [!WARNING]
> This project is a work in progress and lacks most essential features. It currently serves more as a study resource for those learning about LSP development.

## Features

- Currently, there are no implemented features.
- Future plans include:
  - Autocompletion [Implemented but lack completion items]
  - Go To Definition [WIP]
  - Hover Actions [WIP - Implemented but lack hover text items]
  - Code Actions [WIP - Implemented but only for tables code-action can add additional params to tables]
  - Syntax highlighting [ Not in priority as it can be done using treesitter]
  - Linting for Typst documents [not in priority]
  - Formatting tools [not in priority]

## Usage

### Prerequisites

- Rust (latest stable version recommended)

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/abhi-xyz/typst-analyzer.git
   cd typst-analyzer
   ```
2. Build the project:
   ```sh
   cargo build --release
   ```

## Running the Language Server

put the compiled binary in $PATH and then you can configure your editor to connect to this server using an LSP client.

## Contributing

While the project is not feature-complete, contributions are welcome. Feel free to fork the repository, implement a new feature, and submit a pull request.

## Learning Resources

If you're using this project as a learning resource for LSP development, here are a few helpful links:

- [Language Server Protocol Specification](https://microsoft.github.io/language-server-protocol/)
- [Tower LSP GitHub Repository](https://github.com/ebkalderon/tower-lsp)
- [Rust Programming Language](https://www.rust-lang.org/)
