# NXD Language Server

`nxdlang_lsp` is the editor-neutral Language Server Protocol (LSP) implementation for the NXD programming language.

It provides a standard interface between NXD language tooling and editors or IDEs that support LSP.

## Architecture

NXD language behavior remains owned by `nxd_language`.

`nxdlang_lsp` does not implement a second parser, semantic analyzer, or diagnostic system. Instead, it consumes `nxd_language` as a Rust library and exposes its capabilities through the Language Server Protocol.

```text
                  nxd_language
                       │
                       │ Rust library
                       ▼
                  nxdlang_lsp
                       │
                       │ Language Server Protocol
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
       VS Code      Sublime      Other LSP
                                compatible editors
```