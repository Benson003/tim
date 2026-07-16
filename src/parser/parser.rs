use std::collections::{HashMap, HashSet};

use crate::{
    parser::ast::{Attributes, Block, Document, Inline, ListItem},
    source_map::span::Span,
    tokenizer::tokens::{Token, TokenTypes},
};

#[derive(Clone, Debug)]
enum Frame {
    Document {
        children: Vec<Block>,
    },
    Paragraph {
        attrs: Attributes,
        children: Vec<Inline>,
    },
    Header {
        level: u8,
        attrs: Attributes,
        children: Vec<Inline>,
    },
    CodeBlock {
        language: Option<String>,
        content: String,
    },
    OrderedList {
        start: usize,
        children: Vec<ListItem>,
        attrs: Attributes,
    },
    UnorderedList {
        attrs: Attributes,
        children: Vec<ListItem>,
    },
    ListItem {
        children: Vec<Inline>,
    },
    Note {
        children: Vec<Block>,
    },
    Bold {
        children: Vec<Inline>,
    },
    Italic {
        children: Vec<Inline>,
    },
    Link {
        url: String,
        children: Vec<Inline>,
        attrs: Attributes,
        //state: LinkState,
    },
}

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub(crate) frame: Frame,
    pub(crate) start_span: Span,
}
//#[derive(Clone)]
//enum LinkState {
//    ReadingUrl,
//   ParsingChildren,
//}

#[derive(Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    stack: Vec<StackFrame>,
    pending_attrs: Attributes,
    global_ids: HashSet<String>,
}

impl Frame {
    pub(crate) fn into_inline(self, start_span: Span, end_span: Span) -> Option<Inline> {
        let total_span = start_span.to(end_span);
        match self {
            Frame::Bold { children } => Some(Inline::Bold {
                children,
                span: total_span,
            }),
            Frame::Italic { children } => Some(Inline::Italic {
                children,
                span: total_span,
            }),
            Frame::Link {
                url,
                children,
                attrs,
                ..
            } => Some(Inline::Link {
                url,
                children,
                attrs,
                span: total_span,
            }),
            _ => None,
        }
    }

    pub(crate) fn add_inline(&mut self, inline: Inline) {
        match self {
            Frame::Paragraph { children, .. }
            | Frame::Header { children, .. }
            | Frame::ListItem { children, .. }
            | Frame::Bold { children, .. }
            | Frame::Italic { children, .. }
            | Frame::Link { children, .. } => {
                if let Inline::Text {
                    value: new_text, ..
                } = &inline
                {
                    if let Some(Inline::Text {
                        value: existing_text,
                        ..
                    }) = children.last_mut()
                    {
                        existing_text.push_str(new_text);
                        return;
                    }
                }
                children.push(inline)
            }

            Frame::Document { .. }
            | Frame::OrderedList { .. }
            | Frame::UnorderedList { .. }
            | Frame::CodeBlock { .. }
            | Frame::Note { .. } => {
                panic!("failed to handle invalid placment of inline")
            }
        }
    }

    pub(crate) fn into_block(self, start_span: Span, end_span: Span) -> Option<Block> {
        let total_span = start_span.to(end_span);
        match self {
            Frame::Paragraph { attrs, children } => Some(Block::Paragraph {
                attrs,
                children,
                span: total_span,
            }),
            Frame::Header {
                level,
                attrs,
                children,
            } => Some(Block::Header {
                level,
                children,
                attrs,
                span: total_span,
            }),
            Frame::Note { children } => Some(Block::Note {
                children,
                span: total_span,
            }),
            Frame::OrderedList {
                start,
                children,
                attrs,
            } => Some(Block::OrderedList {
                start,
                children,
                attrs,
                span: total_span,
            }),
            Frame::UnorderedList { attrs, children } => Some(Block::UnorderedList {
                children,
                attrs,
                span: total_span,
            }),
            Frame::CodeBlock { language, content } => Some(Block::CodeBlock {
                language,
                code: content,
                span: total_span,
            }),

            Frame::Document { .. }
            | Frame::ListItem { .. }
            | Frame::Bold { .. }
            | Frame::Italic { .. }
            | Frame::Link { .. } => None,
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            cursor: 0,
            stack: Vec::new(),
            pending_attrs: Attributes::default(),
            global_ids: HashSet::new(),
        }
    }

    fn take_pending_attrs(&mut self) -> Attributes {
        let attrs = self.pending_attrs.clone();
        self.pending_attrs = Attributes::default();
        attrs
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }
    fn advance(&mut self) -> Option<&Token> {
        if self.is_eof() {
            None
        } else {
            self.cursor += 1;
            self.tokens.get(self.cursor - 1)
        }
    }
    fn is_eof(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    fn current_span(&self) -> Span {
        self.peek().map(|t| t.span).unwrap_or_else(|| {
            self.tokens
                .last()
                .map(|t| t.span)
                .unwrap_or(Span { lo: 0, hi: 0 })
        })
    }

    fn last_span(&self) -> Span {
        if self.cursor > 0 {
            self.tokens[self.cursor - 1].span
        } else {
            self.current_span()
        }
    }

    fn push_frame(&mut self, frame: Frame, start_span: Span) {
        self.stack.push(StackFrame { frame, start_span });
    }
    fn pop_frame(&mut self) -> Option<StackFrame> {
        self.stack.pop()
    }

    fn current_mut(&mut self) -> Option<&mut Frame> {
        self.stack.last_mut().map(|sf| &mut sf.frame)
    }

    fn add_inline_to_current_frame(&mut self, inline: Inline) {
        if let Some(frame) = self.current_mut() {
            frame.add_inline(inline);
        }
    }
    fn is_note_end(&self) -> bool {
        let first = self.tokens.get(self.cursor);
        let second = self.tokens.get(self.cursor + 1);
        let third = self.tokens.get(self.cursor + 2);

        match (first, second) {
            (Some(t1), Some(t2)) => {
                let is_double_colon = matches!(t1.token_type, TokenTypes::Colon)
                    && matches!(t2.token_type, TokenTypes::Colon);

                if is_double_colon {
                    // Ensure it is on its own line (not part of text like ::something)
                    match third {
                        None => true,
                        Some(t3) => {
                            matches!(t3.token_type, TokenTypes::NewLine | TokenTypes::WhiteSpace)
                        }
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }
    fn is_note_start(&self) -> bool {
        let first = self.tokens.get(self.cursor);
        let second = self.tokens.get(self.cursor + 1);
        let third = self.tokens.get(self.cursor + 2);

        match (first, second, third) {
            (Some(t1), Some(t2), Some(t3)) => {
                matches!(t1.token_type, TokenTypes::Colon)
                    && matches!(t2.token_type, TokenTypes::Colon)
                    && t3.value.as_deref() == Some("note")
            }
            _ => false,
        }
    }

    pub fn parse(&mut self) -> Document {
        let start_span = self.current_span();
        self.push_frame(
            Frame::Document {
                children: Vec::new(),
            },
            start_span,
        );

        while !self.is_eof() {
            self.parse_block();
        }

        let doc_stack_frame = self
            .pop_frame()
            .expect("Critical Error: Document frame was popped prematurely!");

        let total_span = doc_stack_frame.start_span.to(self.last_span());
        match doc_stack_frame.frame {
            Frame::Document { children } => Document {
                children,
                span: total_span,
            },
            _ => unreachable!("The root frame must always be a Document"),
        }
    }

    fn parse_attributes(&mut self) {
        let start_span = self.current_span();
        self.advance(); // consume ClassBegin {
        let mut raw_content = String::new();
        let mut closed = false;

        while !self.is_eof() {
            let token = self.peek().expect("Expected token inside attribute");
            if matches!(token.token_type, TokenTypes::ClassEnd) {
                self.advance(); // consume ClassEnd }
                closed = true;
                break;
            }

            // Handle special tokens that have no value
            match token.token_type {
                TokenTypes::UnderScore => {
                    raw_content.push('_');
                    self.advance();
                }
                TokenTypes::Emphasis => {
                    raw_content.push('*');
                    self.advance();
                }
                TokenTypes::Dash => {
                    raw_content.push('-');
                    self.advance();
                }
                TokenTypes::Colon => {
                    raw_content.push(':');
                    self.advance();
                }
                TokenTypes::WhiteSpace => {
                    raw_content.push(' ');
                    self.advance();
                }
                _ => {
                    if let Some(tok) = self.advance() {
                        if let Some(val) = &tok.value {
                            raw_content.push_str(val);
                        }
                    }
                }
            }
        }

        if !closed {
            panic!("Syntax Error: Unterminated attribute block. expected a closing '}}'");
        }

        let end_span = self.last_span();
        let total_span = start_span.to(end_span);

        while let Some(token) = self.peek() {
            match token.token_type {
                TokenTypes::WhiteSpace => {
                    self.advance();
                }
                TokenTypes::NewLine => {
                    self.advance();
                    break;
                }
                _ => {
                    panic!(
                        "Syntax Error: Attributes must sit on thier own line directly above the block"
                    )
                }
            }
        }
        let mut parts = raw_content.split(',');

        let id_part = parts.next().map(|s| s.trim());
        let id = match id_part {
            Some("_") | None => None,
            Some(actual_id) if actual_id.is_empty() => None,
            Some(actual_id) => {
                let id_str = actual_id.to_string();
                if !self.global_ids.insert(id_str.clone()) {
                    panic!(
                        "Syntax Error: Duplicate ID '{}' found.IDs must be unique accros the document",
                        id_str
                    )
                }
                Some(id_str)
            }
        };

        let mut classes = Vec::new();
        let mut properties = HashMap::new();
        for item in parts {
            let trimmed = item.trim();

            if !trimmed.is_empty() {
                if item.contains(':') {
                    if let Some(colon_pos) = item.find(':') {
                        let key = item[..colon_pos].trim().to_string();

                        if key.is_empty() {
                            panic!("Syntax Error: empty key")
                        }

                        let value_part = item[colon_pos + 1..].trim();

                        let value = if value_part.starts_with('"') {
                            if !value_part.ends_with('"') {
                                panic!("Syntax Error: unclosed quote in value")
                            }
                            value_part[1..value_part.len() - 1].to_string()
                        } else if value_part.contains('"') {
                            panic!("Syntax Error: invalid quote placment")
                        } else {
                            value_part.to_string()
                        };
                        properties.insert(key, value);
                    }
                } else {
                    classes.push(trimmed.to_string());
                }
            }
        }
        self.pending_attrs = Attributes {
            id,
            classes,
            properties,
            span: total_span,
        };
    }

    fn parse_note(&mut self) {
        let start_span = self.current_span();
        // 1. Consume ":", ":", and "note"
        self.advance(); // consume first ':'
        self.advance(); // consume second ':'
        self.advance(); // consume word 'note'

        // Consume any remaining spaces/newlines after the opening directive
        while let Some(tok) = self.peek() {
            if matches!(tok.token_type, TokenTypes::WhiteSpace | TokenTypes::NewLine) {
                self.advance();
            } else {
                break;
            }
        }

        // 2. Push the Note frame onto the stack
        self.push_frame(
            Frame::Note {
                children: Vec::new(),
            },
            start_span,
        );

        let mut closed = false;

        // 3. Keep parsing blocks inside the note
        while !self.is_eof() {
            // Trim whitespace/newlines between blocks
            while let Some(token) = self.peek() {
                if matches!(
                    token.token_type,
                    TokenTypes::NewLine | TokenTypes::WhiteSpace
                ) {
                    self.advance();
                } else {
                    break;
                }
            }

            if self.is_eof() {
                break;
            }

            // Check if we hit the closing '::'
            if self.is_note_end() {
                self.advance(); // consume first ':'
                self.advance(); // consume second ':'
                closed = true;
                break;
            }

            // Parse blocks recursively (e.g., lists, code blocks, headers, paragraphs)
            self.parse_block();
        }

        if !closed {
            panic!("Syntax Error: Unterminated note block. Expected closing '::'");
        }

        let end_span = self.last_span();

        // 4. Pop the Note frame, convert to block, and add to parent container
        if let Some(stack_frame) = self.pop_frame() {
            if let Some(block) = stack_frame
                .frame
                .into_block(stack_frame.start_span, end_span)
            {
                self.add_block_to_current_frame(block);
            }
        }
    }

    fn parse_block(&mut self) {
        while let Some(token) = self.peek() {
            match token.token_type {
                TokenTypes::NewLine | TokenTypes::WhiteSpace => {
                    self.advance();
                }
                _ => break,
            }
        }
        if self.is_eof() {
            return;
        }

        let token = self.peek().expect("Expected a token");

        match token.token_type {
            TokenTypes::ClassBegin => {
                self.parse_attributes();
                self.parse_block();
            }

            TokenTypes::Header => self.parse_header(),
            TokenTypes::BackTick => {
                let is_double_backtick = if let Some(next_tok) = self.tokens.get(self.cursor + 1) {
                    matches!(next_tok.token_type, TokenTypes::BackTick)
                } else {
                    false
                };

                if is_double_backtick {
                    self.parse_code_block();
                } else {
                    self.parse_paragraph();
                }
            }
            TokenTypes::OrderedList => {
                self.parse_ordered_list();
            }

            TokenTypes::Dash => {
                self.parse_unordered_list();
            }

            TokenTypes::Colon => {
                if self.is_note_start() {
                    self.parse_note();
                } else if self.is_note_end() {
                    panic!("Syntax Error: Found closing '::' without an open note block.");
                } else {
                    // It's just a regular paragraph starting with a colon
                    self.parse_paragraph();
                }
            }
            _ => self.parse_paragraph(),
        }
    }
    fn add_block_to_current_frame(&mut self, block: Block) {
        if let Some(parent_frame) = self.current_mut() {
            match parent_frame {
                Frame::Document { children } => {
                    children.push(block);
                }
                Frame::Note { children } => {
                    children.push(block);
                }
                // If the top of the stack is a ListItem or active List, we might
                // need to delegate, but usually those consume inlines.
                // To prevent panics, we can search down the stack for the nearest block container:
                _ => {
                    // Fallback: look deeper in the stack for Document or Note
                    let mut found = false;
                    for sf in self.stack.iter_mut().rev() {
                        match &mut sf.frame {
                            Frame::Document { children } | Frame::Note { children } => {
                                children.push(block);
                                found = true;
                                break;
                            }
                            _ => {}
                        }
                    }
                    if !found {
                        panic!(
                            "Syntax Error: Tried to append a block to an invalid container frame!"
                        );
                    }
                }
            }
        }
    }
    fn parse_unordered_list(&mut self) {
        let start_span = self.current_span();
        // 1. Push the container UnorderedList frame
        let attrs = self.take_pending_attrs();
        self.push_frame(
            Frame::UnorderedList {
                attrs,
                children: Vec::new(),
            },
            start_span,
        );

        // 2. Loop and eat all sequential items starting with a Dash
        while !self.is_eof() {
            while let Some(token) = self.peek() {
                if matches!(token.token_type, TokenTypes::WhiteSpace) {
                    self.advance();
                } else {
                    break;
                }
            }

            if let Some(token) = self.peek() {
                if matches!(token.token_type, TokenTypes::Dash) {
                    self.parse_list_item();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        let end_span = self.last_span();
        // 3. Pop the UnorderedList block and commit it to the Document
        if let Some(stack_frame) = self.pop_frame() {
            if let Some(block) = stack_frame
                .frame
                .into_block(stack_frame.start_span, end_span)
            {
                self.add_block_to_current_frame(block);
            }
        }
    }

    fn parse_list_item(&mut self) {
        let start_span = self.current_span();
        // 1. Consume the list marker token (the number or the dash '-')
        self.advance();

        // 2. ONLY consume a period '.' if we are inside an Ordered List!
        let is_ordered = matches!(self.current_mut(), Some(Frame::OrderedList { .. }));
        if is_ordered {
            if let Some(token) = self.peek() {
                if token.value.as_deref() == Some(".") {
                    self.advance();
                }
            }
        }

        // 3. Consume trailing whitespace after the marker/dot
        if let Some(token) = self.peek() {
            if matches!(token.token_type, TokenTypes::WhiteSpace) {
                self.advance();
            }
        }

        // 4. Push the list item frame and parse contents
        self.push_frame(
            Frame::ListItem {
                children: Vec::new(),
            },
            start_span,
        );

        self.parse_inline();

        let end_span = self.last_span();

        // 5. Pop and append directly to the parent context
        if let Some(stack_frame) = self.pop_frame() {
            if let Frame::ListItem { children } = stack_frame.frame {
                let list_item = ListItem {
                    children,
                    span: stack_frame.start_span.to(end_span),
                };

                match self.current_mut() {
                    Some(Frame::OrderedList {
                        children: list_children,
                        ..
                    }) => {
                        list_children.push(list_item);
                    }
                    Some(Frame::UnorderedList {
                        children: list_children,
                        ..
                    }) => {
                        list_children.push(list_item);
                    }
                    _ => panic!("Syntax Error: ListItem parsed outside of a list frame!"),
                }
            }
        }
    }

    fn parse_ordered_list(&mut self) {
        let start_span = self.current_span();
        let first_token = self.peek().expect("Expectered a orderded list token");
        let start_val: usize = first_token
            .value
            .as_ref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let attrs = self.take_pending_attrs();
        self.push_frame(
            Frame::OrderedList {
                start: start_val,
                children: Vec::new(),
                attrs,
            },
            start_span,
        );

        while !self.is_eof() {
            while let Some(token) = self.peek() {
                if matches!(token.token_type, TokenTypes::WhiteSpace) {
                    self.advance();
                } else {
                    break;
                }
            }
            if matches!(
                self.peek().expect("expected token").token_type,
                TokenTypes::OrderedList
            ) {
                self.parse_list_item();
            } else {
                break;
            }
        }
        let end_span = self.last_span();
        if let Some(stack_frame) = self.pop_frame() {
            if let Some(block) = stack_frame
                .frame
                .into_block(stack_frame.start_span, end_span)
            {
                self.add_block_to_current_frame(block);
            }
        }
    }

    fn parse_paragraph(&mut self) {
        let start_span = self.current_span();
        let attrs = self.take_pending_attrs();
        self.push_frame(
            Frame::Paragraph {
                attrs,
                children: Vec::new(),
            },
            start_span,
        );

        self.parse_inline();

        let end_span = self.last_span();

        if let Some(stack_frame) = self.pop_frame() {
            if let Some(block) = stack_frame
                .frame
                .into_block(stack_frame.start_span, end_span)
            {
                self.add_block_to_current_frame(block);
            }
        }
    }

    fn parse_inline(&mut self) {
        while !self.is_eof() {
            let token = self.peek().expect("failed to get next token");
            let start_span = token.span;
            match token.token_type {
                TokenTypes::NewLine => {
                    self.advance(); // Consume the newline to finish the block
                    break; // Exit inline parsing
                }
                TokenTypes::AnchorValueEnd => {
                    break;
                }
                TokenTypes::AnchorURLStart => {
                    self.parse_link();
                }
                TokenTypes::WhiteSpace => {
                    self.advance();
                    self.add_inline_to_current_frame(Inline::Text {
                        value: " ".to_string(),
                        span: start_span,
                    });
                }
                TokenTypes::Emphasis => {
                    self.advance();
                    if let Some(Frame::Bold { .. }) = self.current_mut() {
                        let end_span = self.last_span();
                        if let Some(stack_frame) = self.pop_frame() {
                            if let Some(inline_bold) = stack_frame
                                .frame
                                .into_inline(stack_frame.start_span, end_span)
                            {
                                self.add_inline_to_current_frame(inline_bold);
                            }
                        }
                    } else {
                        self.push_frame(
                            Frame::Bold {
                                children: Vec::new(),
                            },
                            start_span,
                        );
                    }
                }

                TokenTypes::UnderScore => {
                    self.advance();
                    if let Some(Frame::Italic { .. }) = self.current_mut() {
                        if let Some(stack_frame) = self.pop_frame() {
                            let end_span = self.last_span();
                            if let Some(inline_italic) = stack_frame
                                .frame
                                .into_inline(stack_frame.start_span, end_span)
                            {
                                self.add_inline_to_current_frame(inline_italic);
                            }
                        }
                    } else {
                        self.push_frame(
                            Frame::Italic {
                                children: Vec::new(),
                            },
                            start_span,
                        );
                    }
                }
                TokenTypes::BackTick => {
                    self.advance();
                    self.parse_inline_code();
                }
                TokenTypes::Image => {
                    if let Some(next_tok) = self.tokens.get(self.cursor + 1) {
                        if matches!(next_tok.token_type, TokenTypes::AnchorValueStart) {
                            self.parse_image();
                            continue;
                        }
                    }
                    if let Some(tok) = self.advance() {
                        let val_string = tok.value.clone().unwrap_or_default();
                        let span = tok.span;
                        self.add_inline_to_current_frame(Inline::Text {
                            value: val_string,
                            span,
                        });
                    }
                }
                TokenTypes::AnchorValueStart => {
                    self.parse_link();
                }
                _ => {
                    // For now, treat unknown tokens as text
                    if let Some(tok) = self.advance() {
                        let text = tok.clone().value.unwrap_or("".to_string());
                        let span = tok.span;
                        self.add_inline_to_current_frame(Inline::Text { value: text, span });
                    }
                }
            }
        }
    }

    fn parse_inline_code(&mut self) {
        let mut content = String::new();
        let mut closed = false;

        while !self.is_eof() {
            let token = self.peek().expect("Expected token in inline code");

            match token.token_type {
                // If we hit another backtick, we successfully closed the inline code!
                TokenTypes::BackTick => {
                    self.advance(); // Consume the closing backtick
                    closed = true;
                    break;
                }
                TokenTypes::WhiteSpace => {
                    self.advance();
                    content.push_str(" ");
                }
                // If we hit a newline, they forgot to close it on this line!
                TokenTypes::NewLine => {
                    // We don't advance because the parent block needs to handle the NewLine
                    break;
                }
                _ => {
                    // Collect everything else as raw code text
                    if let Some(tok) = self.advance() {
                        content.push_str(&tok.value.clone().unwrap_or_default());
                    }
                }
            }
        }

        if !closed {
            // Your custom compiler error handling!
            panic!(
                "Syntax Error: Unterminated inline code segment. Expected a closing backtick '`'."
            );
        }
        let end_span = self.last_span();
        self.add_inline_to_current_frame(Inline::InlineCode {
            code: content,
            span: end_span,
        });
    }

    fn parse_header(&mut self) {
        let start_span = self.current_span();
        let mut level = 0;
        while let Some(token) = self.peek() {
            if matches!(token.token_type, TokenTypes::Header) {
                level += 1;
                self.advance();
            } else {
                break;
            }
        }

        if level == 0 {
            return;
        }

        if let Some(token) = self.peek() {
            if matches!(token.token_type, TokenTypes::WhiteSpace) {
                self.advance();
            }
        }
        let attrs = self.take_pending_attrs();
        self.push_frame(
            Frame::Header {
                level,
                attrs,
                children: Vec::new(),
            },
            start_span,
        );

        self.parse_inline();

        let end_span = self.last_span();

        if let Some(stack_frame) = self.pop_frame() {
            if let Some(block) = stack_frame
                .frame
                .into_block(stack_frame.start_span, end_span)
            {
                if let Some(Frame::Document { children }) = self.current_mut() {
                    children.push(block);
                }
            }
        }
    }

    fn parse_image(&mut self) {
        let start_span = self.last_span();
        self.advance();
        if !matches!(
            self.peek().map(|t| &t.token_type),
            Some(TokenTypes::AnchorValueStart)
        ) {
            panic!("Syntax Error: Expected '[' after '!' to begin an image.");
        }
        self.advance();

        let mut alt_text = String::new();
        let mut closed_braket = false;

        while !self.is_eof() {
            let token = self.peek().expect("Expected token in image alt");
            if matches!(token.token_type, TokenTypes::AnchorValueStart) {
                panic!("Syntax Error: Nested brackets are not allowed inside image alt text!");
            }
            if matches!(token.token_type, TokenTypes::Image) {
                if let Some(next_tok) = self.tokens.get(self.cursor + 1) {
                    if matches!(next_tok.token_type, TokenTypes::AnchorValueStart) {
                        panic!(
                            "Syntax Error: Cannot embed an image inside another image's alt text!"
                        );
                    }
                }
            }

            if matches!(token.token_type, TokenTypes::AnchorValueEnd) {
                self.advance();
                closed_braket = true;
                break;
            } else {
                if let Some(tok) = self.advance() {
                    alt_text.push_str(&tok.value.clone().unwrap_or_default());
                }
            }
        }

        if !closed_braket {
            panic!("Syntax Error: Unterminated image alt tag. Expected ']'");
        }

        if !matches!(
            self.peek().map(|t| &t.token_type),
            Some(TokenTypes::AnchorURLStart)
        ) {
            panic!("Syntax Error: Expected '(' contaning image source after alt tag ']'")
        }
        self.advance();

        let mut src_url = String::new();
        let mut closed_paren = false;
        while !self.is_eof() {
            let token = self.peek().expect("Expected token in image src");
            if matches!(token.token_type, TokenTypes::AnchorURLEnd) {
                self.advance();
                closed_paren = true;
                break;
            } else {
                if let Some(tok) = self.advance() {
                    src_url.push_str(&tok.value.clone().unwrap_or_default());
                }
            }
        }
        if !closed_paren {
            panic!("Synta Error: Unterminated image source tag. Expected ')'");
        }
        let end_span = self.last_span();
        let attrs = self.take_pending_attrs();

        let image = Inline::Image {
            alt: alt_text,
            src: src_url,
            attrs,
            span: start_span.to(end_span),
        };

        self.add_inline_to_current_frame(image);
    }

    fn parse_link(&mut self) {
        let start_span = self.current_span();
        self.advance();
        let attrs = self.take_pending_attrs();
        self.push_frame(
            Frame::Link {
                url: String::new(),
                children: Vec::new(),
                attrs,
                //state: LinkState::ParsingChildren,
            },
            start_span,
        );
        self.parse_inline();
        if !matches!(
            self.peek().map(|t| &t.token_type),
            Some(TokenTypes::AnchorValueEnd)
        ) {
            panic!("Syntax Error: Expected ']' to close link text");
        }
        self.advance();
        let mut stack_frame = self.pop_frame().expect("Expected Link Frome on stack");
        if !matches!(
            self.peek().map(|t| &t.token_type),
            Some(TokenTypes::AnchorURLStart)
        ) {
            panic!("Syntax Error: Expected '(' contaning link URL after ']'")
        }
        self.advance();
        let mut url_str = String::new();
        let mut closed_paren = false;
        while !self.is_eof() {
            let token = self.peek().expect("Expected token in Link URL");
            if matches!(token.token_type, TokenTypes::AnchorURLEnd) {
                self.advance();
                closed_paren = true;
                break;
            } else {
                if let Some(tok) = self.advance() {
                    url_str.push_str(&tok.value.clone().unwrap_or_default());
                }
            }
        }

        if !closed_paren {
            panic!("Syntax Error: Unterminated link URL.Expected ')'");
        }

        let end_span = self.last_span();

        if let Frame::Link { ref mut url, .. } = stack_frame.frame {
            *url = url_str;
        }
        if let Some(inline_link) = stack_frame
            .frame
            .into_inline(stack_frame.start_span, end_span)
        {
            self.add_inline_to_current_frame(inline_link);
        }
    }

    fn parse_code_block(&mut self) {
        let start_span = self.current_span();
        let mut open_count = 0;

        while let Some(token) = self.peek() {
            if matches!(token.token_type, TokenTypes::BackTick) {
                open_count += 1;
                self.advance();
            } else {
                break;
            }
        }

        if open_count != 2 {
            panic!("Syntax Error: Block code must start with exactly two backticks '``'");
        }

        let mut language = String::new();
        while let Some(token) = self.peek() {
            if matches!(token.token_type, TokenTypes::NewLine) {
                self.advance(); // consume the newline
                break;
            }
            if let Some(tok) = self.advance() {
                language.push_str(&tok.value.clone().unwrap_or_default());
            }
        }

        let language_opt = if language.trim().is_empty() {
            None
        } else {
            Some(language.trim().to_string())
        };

        self.push_frame(
            Frame::CodeBlock {
                language: language_opt,
                content: String::new(),
            },
            start_span,
        );

        // 5. Gather the raw content until we see the closing "``"
        let mut code_content = String::new();
        let mut closed = false;

        while !self.is_eof() {
            if let Some(token) = self.peek() {
                if matches!(token.token_type, TokenTypes::BackTick) {
                    let next_tok = self.tokens.get(self.cursor + 1);
                    if let Some(nxt) = next_tok {
                        if matches!(nxt.token_type, TokenTypes::BackTick) {
                            self.advance(); // consume first backtick
                            self.advance(); // consume second backtick
                            closed = true;
                            break;
                        }
                    }
                }
            }

            if let Some(tok) = self.advance() {
                match tok.token_type {
                    TokenTypes::NewLine => {
                        code_content.push('\n');
                    }
                    TokenTypes::WhiteSpace => {
                        code_content.push(' ');
                    }
                    _ => {
                        code_content.push_str(&tok.value.clone().unwrap_or_default());
                    }
                }
            }
        }

        if !closed {
            panic!("Syntax Error: Unterminated code block. Expected '``'");
        }

        let end_span = self.last_span();

        // 6. Pop the Frame, convert it to Block::CodeBlock, and add to the Document
        if let Some(stack_frame) = self.pop_frame() {
            if let Frame::CodeBlock { language, .. } = stack_frame.frame {
                let block = Block::CodeBlock {
                    span: stack_frame.start_span.to(end_span),
                    language,
                    code: code_content,
                };
                self.add_block_to_current_frame(block);
            }
        }
    }
}
