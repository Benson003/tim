use std::collections::HashMap;

use crate::source_map::span::{HasSpan, Span};

#[derive(Debug, Default, Clone)]
pub struct Attributes {
    pub(crate) span: Span,
    pub(crate) id: Option<String>,
    pub(crate) classes: Vec<String>,
    pub(crate) properties: HashMap<String, String>,
}

impl HasSpan for Attributes {
    fn span(&self) -> Span {
        self.span
    }
}
#[derive(Debug, Default, Clone)]
pub(crate) struct ListItem {
    pub(crate) span: Span,
    pub(crate) children: Vec<Inline>,
}

impl HasSpan for ListItem {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Document {
    pub(crate) span: Span,
    pub(crate) children: Vec<Block>,
}

impl HasSpan for Document {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Block {
    Paragraph {
        span: Span,
        attrs: Attributes,
        children: Vec<Inline>,
    },
    Header {
        span: Span,
        attrs: Attributes,
        level: u8,
        children: Vec<Inline>,
    },
    OrderedList {
        span: Span,
        attrs: Attributes,
        start: usize,
        children: Vec<ListItem>,
    },
    UnorderedList {
        span: Span,
        attrs: Attributes,
        children: Vec<ListItem>,
    },
    CodeBlock {
        span: Span,
        language: Option<String>,
        code: String,
    },
    Note {
        span: Span,
        children: Vec<Block>,
    },
}

impl HasSpan for Block {
    fn span(&self) -> Span {
        match self {
            Block::Paragraph { span, .. } => *span,
            Block::Header { span, .. } => *span,
            Block::OrderedList { span, .. } => *span,
            Block::UnorderedList { span, .. } => *span,
            Block::CodeBlock { span, .. } => *span,
            Block::Note { span, .. } => *span,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Inline {
    Image {
        span: Span,
        alt: String,
        src: String,
        attrs: Attributes,
    },
    Link {
        span: Span,
        url: String,
        children: Vec<Inline>,
        attrs: Attributes,
    },
    Bold {
        span: Span,
        children: Vec<Inline>,
    },
    Italic {
        span: Span,
        children: Vec<Inline>,
    },
    InlineCode {
        span: Span,
        code: String,
    },

    Text {
        span: Span,
        value: String,
    },
}

impl HasSpan for Inline {
    fn span(&self) -> Span {
        match self {
            Inline::Image { span, .. } => *span,
            Inline::Link { span, .. } => *span,
            Inline::Bold { span, .. } => *span,
            Inline::Italic { span, .. } => *span,
            Inline::InlineCode { span, .. } => *span,
            Inline::Text { span, .. } => *span,
        }
    }
}
