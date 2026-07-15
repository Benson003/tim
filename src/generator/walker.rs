use crate::parser::ast::{Attributes, Block, Document, Inline, ListItem};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, doc: Document) -> String {
        let mut result = String::new();
        for block in doc.children {
            result.push_str(&self.walk_block(block));
            result.push('\n');
        }
        result
    }

    fn format_attrs(&self, attrs: &Attributes) -> String {
        let mut html_attrs = String::new();

        if let Some(ref id) = attrs.id {
            html_attrs.push_str(&format!(" id=\"{}\"", id));
        }

        if !attrs.classes.is_empty() {
            let class_string = attrs.classes.join(" ");
            html_attrs.push_str(&format!(" class=\"{}\"", class_string));
        }

        html_attrs
    }

    fn walk_inlines(&self, children: &[Inline]) -> String {
        children
            .iter()
            .map(|inline| self.walk_inline(inline))
            .collect()
    }

    fn walk_inline(&self, inline: &Inline) -> String {
        match inline {
            Inline::Text(text) => text.clone(),
            Inline::Bold { children } => {
                format!("<strong>{}</strong>", self.walk_inlines(children))
            }
            Inline::Italic { children } => {
                format!("<em>{}</em>", self.walk_inlines(children))
            }
            Inline::InlineCode { code } => {
                format!("<code>{}</code>", code)
            }
            Inline::Link {
                url,
                children,
                attrs,
            } => {
                let attr_str = self.format_attrs(attrs);
                format!(
                    "<a href=\"{}\"{}>{}</a>",
                    url,
                    attr_str,
                    self.walk_inlines(children)
                )
            }
            Inline::Image { alt, src, attrs } => {
                let attr_str = self.format_attrs(attrs);
                format!("<img src=\"{}\" alt=\"{}\" {}>", src, alt, attr_str)
            }
        }
    }

    fn walk_block(&self, block: Block) -> String {
        match block {
            Block::Paragraph { attrs, children } => {
                let attr_string = self.format_attrs(&attrs);
                let content = self.walk_inlines(&children);
                format!("<p{}>{}</p>", attr_string, content)
            }
            Block::Header {
                attrs,
                level,
                children,
            } => {
                let attr_string = self.format_attrs(&attrs);
                let content = self.walk_inlines(&children);
                format!("<h{}{}>{}</h{}>", level, attr_string, content, level)
            }
            Block::CodeBlock { language, code } => {
                let class_attr = match language {
                    Some(lang) => format!(" class=\"language-{}\"", lang),
                    None => String::new(),
                };

                format!("<pre><code{}>{}</code></pre>", class_attr, code)
            }
            Block::Note { children } => {
                let mut note_content = String::new();
                for inner_block in children {
                    note_content.push_str(&self.walk_block(inner_block));
                    note_content.push('\n');
                }
                format!("<div class=\"tim-note\">\n{}</div>", note_content)
            }
            Block::UnorderedList { attrs, children } => {
                let attr_str = self.format_attrs(&attrs);
                let mut list_items_html = String::new();
                for item in children {
                    list_items_html.push_str(&self.walk_list_item(item));
                    list_items_html.push('\n');
                }
                format!("<ul{}>\n{}</ul>", attr_str, list_items_html)
            }
            Block::OrderedList {
                attrs,
                start,
                children,
            } => {
                let mut attr_string = self.format_attrs(&attrs);
                if start != 1 {
                    attr_string.push_str(&format!(" start=\"{}\"", start));
                }
                let mut list_items_html = String::new();
                for item in children {
                    list_items_html.push_str(&self.walk_list_item(item));
                    list_items_html.push('\n');
                }
                format!("<ol{}>\n{}</ol>", attr_string, list_items_html)
            }
            
        }
    }

    fn walk_list_item(&self, item: ListItem) -> String {
        let content = self.walk_inlines(&item.children);
        format!("<li>{}</li>", content)
    }
}
