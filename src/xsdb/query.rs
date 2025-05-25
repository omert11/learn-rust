use std::{iter::Peekable, slice::Iter};

use super::tokenizer::{Token, TokenKind};

#[derive(Debug)]
#[allow(dead_code)]
pub enum BodyKind {
    CreateTable,
    Field,
}

#[derive(Debug)]
pub enum WhenKind {
    None,
    IfExists,
    IfNotExists,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Query {
    body: BodyKind,
    when: WhenKind,
    value: Option<String>,
    subqueries: Vec<Box<Query>>,
}

impl Query {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut token_iter = tokens.iter().peekable();
        let base_token = token_iter.next().unwrap();

        match base_token.token() {
            TokenKind::CREATE => {
                let created_keyword = token_iter.next().unwrap();

                match created_keyword.token() {
                    TokenKind::TABLE => {
                        let when_kind = peek_when_kind(&mut token_iter);
                        let value = token_iter.next().unwrap().text();
                        let remaining_tokens = token_iter.cloned().collect::<Vec<_>>();
                        let subquery = Query::new(remaining_tokens);
                        return Self {
                            body: BodyKind::CreateTable,
                            when: when_kind,
                            value: Some(value.to_string()),
                            subqueries: vec![Box::new(subquery)],
                        };
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        return Self {
            body: BodyKind::CreateTable,
            when: WhenKind::None,
            value: None,
            subqueries: vec![],
        };
    }
}

fn peek_when_kind(token_iter: &mut Peekable<Iter<Token>>) -> WhenKind {
    let mut is_when = false;
    let mut is_not = false;
    while let Some(token) = token_iter.peek() {
        match token.token() {
            TokenKind::IF => {
                token_iter.next();
                is_when = true;
            }
            TokenKind::NOT => {
                token_iter.next();
                is_not = true;
            }
            TokenKind::EXISTS => {
                token_iter.next();
                break;
            }
            _ => {
                if is_when {
                    break;
                }
                token_iter.next();
            }
        }
    }
    if !is_when {
        return WhenKind::None;
    }
    if is_not {
        return WhenKind::IfNotExists;
    }
    return WhenKind::IfExists;
}
