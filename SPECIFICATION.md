
# T.I.M Language Specification

> **Status:** Draft (Version 0.1)

This document describes the syntax and semantics of the T.I.M markup language.

T.I.M is a Markdown-inspired markup language designed around deterministic parsing, explicit document structure, and compiler-friendly grammar.

Unlike CommonMark, T.I.M intentionally defines strict parsing rules. Where ambiguity exists, the language specifies a single valid interpretation.

As the language evolves, this specification serves as the authoritative reference for both users and compiler implementations.

---

# 1. Goals

T.I.M has four primary goals.

* Produce deterministic parse trees.
* Remain simple and readable for authors.
* Avoid embedded HTML for common document structures.
* Compile into structured document representations.

The language prioritizes simplicity over feature count.

---

# 2. Source Files

T.I.M source files should use the `.tim` extension.

A document consists of a sequence of block elements.

Whitespace outside block syntax is generally ignored unless it forms part of a paragraph or code block.

---

# 3. Blocks

The current language defines the following block types.

* Paragraph
* Header
* Ordered List
* Unordered List
* Code Block
* Note Block

Future versions may introduce additional block types.

---

# 4. Headers

Headers are introduced using one or more leading `#` characters.

Examples:

```tim
# Heading One

## Heading Two

### Heading Three
```

The number of leading `#` characters determines the heading level.

---

# 5. Paragraphs

Any non-empty line that does not begin another block becomes part of a paragraph.

Consecutive lines belong to the same paragraph until another block begins.

---

# 6. Attributes

Attributes apply to the block immediately following the attribute declaration.

Attribute declarations occupy an entire line.

Example:

```tim
{hero, dark, centered}

# Welcome
```

The first element represents the block ID.

Remaining elements represent CSS classes.

Example:

```tim
{hero, dark, rounded}
```

Produces:

* ID: `hero`
* Classes:

  * `dark`
  * `rounded`

When no ID is required, the placeholder `_` may be used.

Example:

```tim
{_, card, shadow}
```

Produces:

* No ID
* Classes:

  * `card`
  * `shadow`

Attributes apply only to the next block.

They do not affect subsequent blocks.

Future versions may extend the attribute grammar with key-value pairs.

---

# 7. Note Blocks

Notes are explicit container blocks.

A note begins with:

```tim
::note
```

and ends with

```tim
::
```

Everything between these delimiters belongs to the note.

Notes may contain multiple paragraphs and other block elements.

---

# 8. Lists

## Unordered Lists

Unordered list items begin with `-`.

Example:

```tim
- First
- Second
- Third
```

## Ordered Lists

Ordered list items begin with a numeric prefix.

Example:

```tim
1. First
2. Second
3. Third
```



---

# 9. Code Blocks

Code blocks begin with a fenced opening delimiter.

Example:

```text
``rust
fn main() {
    println!("Hello");
}
``
```

The language identifier is optional.

The contents of a code block are treated as literal text.

No parsing is performed within a code block.

---

# 10. Inline Elements

The language currently supports inline elements including:

* Inline code
* Links
* Images

Additional inline constructs may be introduced in future revisions.

---

# 11. Document Model

A T.I.M document is parsed into an Abstract Syntax Tree (AST).

Each block becomes a node in the tree.

Blocks containing nested content own their children.

Renderers operate on the AST rather than the original source text.

---

# 12. HTML Rendering

The reference implementation currently targets HTML.

Block IDs become HTML `id` attributes.

Remaining attribute values become CSS classes.

The HTML renderer preserves document structure while translating T.I.M constructs into appropriate HTML elements.

---

# 13. Error Handling

Invalid syntax should produce clear compiler diagnostics.

Where possible, parsing should continue after recoverable errors to allow multiple diagnostics to be reported during a single compilation.

---

# 14. Language Philosophy

T.I.M intentionally diverges from Markdown where doing so improves consistency, readability, or parser simplicity.

The language is guided by the following principles.

* Deterministic parsing.
* Explicit syntax.
* Readable source.
* Minimal language rules.
* Compiler-friendly design.

Every feature should justify its existence.

---

# 15. Future Work

The following features are being explored.

* Extended attribute syntax
* Components
* Additional block types
* Better diagnostics
* Language specification revisions

Inclusion of these features is not guaranteed.

They remain subject to language design and implementation experience.
