#[derive(Debug, Default, Clone)]
pub struct Attributes {
    pub(crate) id: Option<String>,
    pub(crate) classes: Vec<String>,
}
#[derive(Debug, Default, Clone)]
pub(crate) struct ListItem {
    pub(crate) attrs: Attributes,
    pub(crate) children: Vec<Inline>,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct Document {
    pub(crate) children: Vec<Block>,
}

#[derive(Debug, Clone)]
pub(crate) enum Block {
    Paragraph {
        attrs: Attributes,
        children: Vec<Inline>,
    },
    Header {
        attrs: Attributes,
        level: u8,
        children: Vec<Inline>,
    },
    OrderedList {
        attrs: Attributes,
        start: usize,
        children: Vec<ListItem>,
    },
    UnorderedList {
        attrs: Attributes,
        children: Vec<ListItem>,
    },
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    Note {
        children: Vec<Block>,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum Inline {
    Image {
        alt: String,
        src: String,
        attrs: Attributes,
    },
    Link {
        url: String,
        children: Vec<Inline>,
        attrs: Attributes,
    },
    Bold {
        children: Vec<Inline>,
    },
    Italic {
        children: Vec<Inline>,
    },
    InlineCode {
        code: String,
    },

    Text(String),
}
