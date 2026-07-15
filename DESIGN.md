
# T.I.M Design Rationale

> *Why T.I.M exists and the principles that guide its design.*

---

# Introduction

T.I.M (This Isn't Markdown) is a markup language inspired by Markdown, but it is **not** a Markdown implementation.

The goal of T.I.M is not to replace Markdown or to be fully compatible with CommonMark. Instead, it explores a different set of design priorities.

Markdown optimized for human writing.

T.I.M optimizes for both **human writing** and **deterministic compilation**.

---

# Why Another Markup Language?

Markdown has become the standard lightweight markup language because it is simple to read and write.

However, over time Markdown has accumulated numerous extensions, dialects, and implementation-specific behaviors. The result is that two Markdown parsers can interpret the same document differently.

This flexibility is valuable for authors, but it also makes parser implementations significantly more complex.

T.I.M takes a different approach.

Rather than preserving decades of historical behavior, T.I.M defines a single, explicit grammar.

The objective is not compatibility.

The objective is consistency.

---

# Deterministic Parsing

One of the primary goals of T.I.M is deterministic parsing.

Every construct should have one valid interpretation.

Whenever possible, syntax should be designed so that the lexer and parser can recognize structures without relying on ambiguous heuristics or implementation-specific edge cases.

This makes compiler implementations simpler, easier to reason about, and easier to maintain.

---

# Explicit Structure

Document structure should be represented directly by the language.

Instead of embedding raw HTML into a document simply to attach styling information, T.I.M introduces dedicated syntax for describing document structure.

For example:

```tim
{hero, landing}

# Welcome
```

Rather than:

```html
<h1 id="hero" class="landing">
    Welcome
</h1>
```

The source remains clean while preserving the same semantic information.

---

# Attributes

Attributes are intentionally minimal.

The current language supports:

* One optional ID
* Zero or more CSS classes

The first value represents the ID.

Subsequent values represent classes.

```tim
{hero, dark, rounded}
```

When no ID is required, `_` may be used.

```tim
{_, card, shadow}
```

This positional syntax keeps parsing straightforward while reflecting HTML's underlying data model.

Future versions may introduce key-value attributes if they can be added without significantly increasing language complexity.

---

# Note Blocks

Traditional Markdown relies on blockquotes for many purposes beyond quoting text.

T.I.M introduces explicit note blocks instead.

```tim
::note

Important information.

::
```

A note is a distinct document node rather than a special interpretation of a quote.

This makes its meaning explicit both to readers and compiler implementations.

---

# Minimalism

Every language feature increases complexity.

New syntax is added only when it provides meaningful expressive power without making the language significantly harder to understand or implement.

Whenever multiple syntaxes solve the same problem, preference is given to the simpler alternative.

---

# AST-First Design

The T.I.M compiler is built around an Abstract Syntax Tree.

The language is designed so that source documents naturally map into structured nodes.

The parser is not merely transforming text into HTML.

Instead, it constructs an intermediate representation that can be analyzed, transformed, or rendered.

```
Source
   │
Lexer
   │
Parser
   │
AST
   │
Renderer
   │
HTML
```

This separation keeps the language independent from any particular output format.

---

# Deliberate Divergence

T.I.M intentionally differs from Markdown in several places.

These differences are not accidental.

They exist because they simplify parsing, improve readability, or provide clearer semantics.

Compatibility is considered where it aligns with the language goals, but compatibility is not itself a goal.

---

# Language Evolution

T.I.M is still under active development.

Language features are introduced conservatively.

A feature should solve a real problem encountered during authoring or implementation before becoming part of the language.

This helps prevent unnecessary complexity and keeps the language focused.

---

# Guiding Principles

Every change to the language should support one or more of these principles.

* Deterministic parsing.
* Explicit document structure.
* Readable source.
* Minimal syntax.
* Compiler-friendly grammar.
* Incremental evolution.

These principles are intended to remain stable even as the language grows.

---

# Closing

T.I.M is an exploration of what a modern lightweight markup language can look like when compiler design is considered from the beginning rather than added later.

The project values clarity over cleverness, explicit structure over ambiguity, and simplicity over feature count.
