# Security Policy 

Security issues affecting the NXD Language Server or its integration with NXD language tooling are taken seriously. 


## Reporting a Vulnerability

Please do not publicly disclose a suspected security vulnerability through a GitHub issue, discussion, pull request, or other public channel.

Security vulnerabilities may be reported privately through:

- GitHub Private Vulnerability Reporting, when available
- Email: lsp.security@nxdlang.org

**When reporting a vulnerability, please include:**

- a description of the vulnerability
- steps required to reproduce it
- the affected component and version
- the affected operating system or editor, when relevant
- the potential security impact
- any known workaround or mitigation

--- 

## Scope 

This repository contains the editor-neutral Language Server Protocol implementation for the NXD programming language. 
Security issues that may belong in this repository include vulnerabilities involving: 

- LSP request or notification handling 
- JSON-RPC communication 
- document synchronization 
- malformed or hostile editor input 
- diagnostic conversion or publication 
- workspace or document URI handling 
- language-server process behavior 
- interactions between an LSP client and `nxdlang_lsp` 
- denial-of-service conditions caused specifically by the language-server layer

 NXD language behavior remains owned by `nxd_language`.
 
Issues involving the NXD compiler, parser, semantic analyzer, type system, runtime behavior, or canonical language diagnostics should generally be reported to the `nxd_language` repository unless disclosure would expose an active security vulnerability.

Editor-specific vulnerabilities should be reported to the repository responsible for that integration when the issue does not originate in `nxdlang_lsp`. 

For example, vulnerabilities specific to the VS Code extension packaging or VS Code integration may belong in `nxdlang-extension`. 

---

## Dependencies 

`nxdlang_lsp` depends on external Rust libraries and directly consumes `nxd_language`. 

Security reports involving a dependency should identify the affected dependency and version when known. 

Dependency vulnerabilities that affect the operation or distribution of the NXD Language Server are considered relevant to this repository even when the underlying vulnerability originates upstream. 

---

## Public Disclosure 

Please allow reasonable time for a reported vulnerability to be investigated and addressed before publicly disclosing technical details. 

## Supported Versions 

`nxdlang_lsp` is currently under active development. Security fixes apply to the current supported version of the language server. Historical development snapshots are not separately maintained unless explicitly stated in a release. 

As formal NXD Language Server releases are introduced, supported release versions and security-maintenance expectations may be documented here.