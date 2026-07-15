# T.I.M

> **This Isn't Markdown**

T.I.M is a Markdown-inspired markup language designed around deterministic parsing, explicit document structure, and clean authoring.

Unlike traditional Markdown implementations, T.I.M defines a concrete grammar that produces a predictable Abstract Syntax Tree (AST), making it suitable for compilers, static site generators, documentation systems, and other tooling.


## Getting Started

### Prerequisites

- Rust (latest stable)
- Cargo

### Build

```bash
cargo build
```

### Run

```bash
cargo run -- <input.tim>
```

Or, if compiling in release mode:

```bash
cargo build --release
./target/release/tim <input.tim>
```

The generated executable can be found in:

```
target/debug/
```

or

```
target/release/
```

depending on the build profile.

---


## Why T.I.M?

Markdown is excellent for writing, but modern documents often require features that traditionally force authors to embed HTML directly into their source.

T.I.M extends familiar Markdown-style syntax with native language constructs for structure and styling while keeping documents readable and easy to parse.

```tim
{hero, landing}

# Welcome

::note

This note is a first-class document block.

::
```

Produces HTML similar to:

```html
<h1 id="hero" class="landing">
    Welcome
</h1>

<div class="tim-note">
    <p>This note is a first-class document block.</p>
</div>
```

---

## Features

- Markdown-inspired syntax
- Deterministic parsing
- Strongly structured AST
- Native note blocks
- Block-level IDs and classes
- Inline formatting
- Links and images
- Fenced code blocks
- Clean HTML generation

---

## Attribute Syntax

Attributes are declared immediately before the block they apply to.

```tim
{hero, dark, centered}
# Welcome
```

The first element represents the block's ID.

Every remaining element is interpreted as a CSS class.

Use `_` when no ID is required.

```tim
{_, card, shadow}
This paragraph has no ID.
```

---

## Note Blocks

Notes are explicit document containers.

```tim
::note

Important information.

::
```

Unlike block quotes, note blocks are represented directly in the document tree.

---

## Compiler Architecture

```
Source (.tim)
      │
      ▼
    Lexer
      │
      ▼
    Parser
      │
      ▼
      AST
      │
      ▼
 Tree Walker
      │
      ▼
 HTML Renderer
```


---

## Project Status

T.I.M is currently under active development.

Implemented:

- Lexer
- Parser
- Abstract Syntax Tree
- Tree Walker
- HTML Renderer

Planned:

- Language specification
- Better diagnostics
- Extended attribute syntax
- Components

---

## Philosophy

T.I.M intentionally diverges from Markdown where doing so improves consistency, readability, or parser simplicity.

The project prioritizes:

- Deterministic parsing
- Explicit syntax
- Clean document structure
- Simple language rules

---

## License

MIT
