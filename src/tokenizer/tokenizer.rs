use core::{iter::Iterator, option::Option::Some};

use crate::{
    diagnostics::diagnostics::{Diagnostic, Severity},
    source_map::{source_map::SourceMap, span::Span},
    tokenizer::tokens::{CharType, Token, TokenTypes},
};

#[derive(Debug)]
pub struct TokenList {
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

impl TokenList {
    pub fn new() -> TokenList {
        TokenList {
            tokens: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
    fn append_tokens(&mut self, token: Token) {
        self.tokens.push(token);
    }
    fn push_warning(&mut self, message: &str, span: Span, label: Option<&str>) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Warning,
            message: message.to_string(),
            span,
            label: label.map(|s| s.to_string()),
        });
    }

    fn push_error(&mut self, message: &str, span: Span, label: Option<&str>) {
        self.diagnostics.push(Diagnostic {
            severity: Severity::Error,
            message: message.to_string(),
            span,
            label: label.map(|s| s.to_string()),
        });
    }
    pub fn tokenize(&mut self, source_map: &SourceMap) {
        let mut buffer = String::new();

        let source = &source_map.source;
        let mut chars = source.char_indices().peekable();
        while let Some(&(start, ch)) = chars.peek() {
            match CharType::classify_char(ch) {
                CharType::WhiteSpace => {
                    chars.next();
                    let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                    let span = Span::new(start, end);
                    self.append_tokens(Token::new(
                        TokenTypes::WhiteSpace,
                        Some(" ".to_string()),
                        span,
                    ));
                }
                CharType::Letter => {
                    buffer.clear();
                    while let Some(&(_, c)) = chars.peek() {
                        match CharType::classify_char(c) {
                            CharType::Letter => {
                                buffer.push(c);
                                chars.next();
                            }
                            _ => break,
                        }
                    }
                    let value = std::mem::take(&mut buffer);
                    let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                    let span = Span::new(start, end);
                    self.append_tokens(Token::new(TokenTypes::Text, Some(value), span));
                }
                CharType::Special => {
                    buffer.clear();
                    while let Some(&(_, c)) = chars.peek() {
                        if CharType::classify_char(c) != CharType::Special {
                            break;
                        }
                        match c {
                            '#' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::Header,
                                    Some("#".to_string()),
                                    span,
                                ));
                            }
                            '[' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);

                                self.append_tokens(Token::new(
                                    TokenTypes::AnchorValueStart,
                                    Some("[".to_string()),
                                    span,
                                ));
                            }
                            ']' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);

                                self.append_tokens(Token::new(
                                    TokenTypes::AnchorValueEnd,
                                    Some("]".to_string()),
                                    span,
                                ));
                            }
                            '(' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::AnchorURLStart,
                                    Some("(".to_string()),
                                    span,
                                ));
                            }
                            ')' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::AnchorURLEnd,
                                    Some(")".to_string()),
                                    span,
                                ));
                            }
                            '<' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::UniqueIDBegin,
                                    Some("<".to_string()),
                                    span,
                                ));
                            }
                            '>' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::UniqueIDEnd,
                                    Some(">".to_string()),
                                    span,
                                ));
                            }
                            '{' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::ClassBegin,
                                    Some("{".to_string()),
                                    span,
                                ));
                            }
                            '}' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::ClassEnd,
                                    Some("}".to_string()),
                                    span,
                                ));
                            }
                            _ => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                let value = source[start..end].to_string();
                                self.append_tokens(Token::new(TokenTypes::Text, Some(value), span));
                            }
                        }
                    }
                }

                CharType::Digit => {
                    let mut is_ordered_list = false;
                    let mut num_str = String::new();

                    while let Some(&(_, c)) = chars.peek() {
                        match c {
                            '0'..='9' => {
                                num_str.push(c);
                                chars.next(); // consume digit
                            }
                            '.' => {
                                let mut check_iter = chars.clone();
                                check_iter.next(); // '.'
                                if check_iter.next().map(|(_, c)| c) == Some(' ')
                                    && !num_str.is_empty()
                                {
                                    // Confirm ordered list
                                    is_ordered_list = true;
                                    chars.next(); // consume '.'
                                } else {
                                    break; // not an ordered list
                                }
                            }
                            _ => break,
                        }
                    }

                    let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                    let span = Span::new(start, end);
                    if is_ordered_list {
                        self.append_tokens(Token::new(
                            TokenTypes::OrderedList,
                            Some(num_str),
                            span,
                        ));
                    } else {
                        self.append_tokens(Token::new(TokenTypes::Text, Some(num_str), span));
                    }
                }

                CharType::NewLine => {
                    chars.next();

                    let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                    let span = Span::new(start, end);
                    self.append_tokens(Token::new(
                        TokenTypes::NewLine,
                        Some("\n".to_string()),
                        span,
                    ));
                }
                CharType::Symbol => {
                    buffer.clear();
                    while let Some(&(_, c)) = chars.peek() {
                        if CharType::classify_char(c) != CharType::Symbol {
                            break;
                        }
                        match c {
                            '!' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);

                                self.append_tokens(Token::new(
                                    TokenTypes::Image,
                                    Some("!".to_string()),
                                    span,
                                ));
                            }

                            '*' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);

                                self.append_tokens(Token::new(
                                    TokenTypes::Emphasis,
                                    Some("*".to_string()),
                                    span,
                                ));
                            }
                            '`' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::BackTick,
                                    Some("`".to_string()),
                                    span,
                                ));
                            }
                            '-' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::Dash,
                                    Some("-".to_string()),
                                    span,
                                ));
                            }
                            '_' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::UnderScore,
                                    Some("_".to_string()),
                                    span,
                                ));
                            }
                            ':' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::Colon,
                                    Some(":".to_string()),
                                    span,
                                ));
                            }
                            '@' => {
                                chars.next();
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(
                                    TokenTypes::At,
                                    Some("@".to_string()),
                                    span,
                                ));
                            }
                            _ => {
                                chars.next();
                                buffer.push(c);
                                let value = std::mem::take(&mut buffer);
                                let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                                let span = Span::new(start, end);
                                self.append_tokens(Token::new(TokenTypes::Text, Some(value), span));
                                break;
                            }
                        }
                    }
                }
                CharType::Escape => {
                    chars.next();
                    if let Some((_, ch)) = chars.next() {
                        buffer.push(ch);
                        let value = std::mem::take(&mut buffer);
                        let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                        let span = Span::new(start, end);
                        self.append_tokens(Token::new(TokenTypes::Escape, Some(value), span));
                    } else {
                        let span = Span::new(start, source.len());

                        self.push_error(
                            "unclosed escape sequence",
                            span,
                            Some("expected a character after backslash"),
                        );
                    }
                }
                CharType::Unknown => {
                    buffer.clear();
                    chars.next();
                    buffer.push(ch);
                    let value = std::mem::take(&mut buffer);
                    let end = chars.peek().map(|&(pos, _)| pos).unwrap_or(source.len());
                    let span = Span::new(start, end);
                    self.append_tokens(Token::new(TokenTypes::Text, Some(value), span));
                }
            }
        }
    }
}
