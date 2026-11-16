# Contributing to NXD Language Server

Thank you for your interest in contributing to `nxdlang_lsp`.

`nxdlang_lsp` provides the editor-neutral Language Server Protocol implementation for the NXD programming language.

The project is designed so that NXD language behavior remains centralized in `nxd_language`, while this repository exposes those capabilities to editors through the Language Server Protocol.

## Project Boundary

Before contributing, please keep the following architectural boundary in mind.

### `nxd_language` owns

- lexical analysis
- parsing
- AST structures
- semantic analysis
- type behavior
- diagnostics and warnings
- compiler behavior
- language rules

### `nxdlang_lsp` owns

- Language Server Protocol communication
- JSON-RPC transport
- document synchronization
- conversion of NXD source locations to LSP ranges
- publication of NXD diagnostics
- LSP capabilities such as completion, hover, navigation, symbols, and related editor-neutral language services

Language rules should not be independently reimplemented in `nxdlang_lsp`.

If a change requires modifying how NXD parses, validates, or interprets source code, that change likely belongs in `nxd_language`.

## Development Setup

Clone `nxd_language` and `nxdlang_lsp` as sibling repositories:

```text
Projects/
├── nxd_language/
└── nxdlang_lsp/
```