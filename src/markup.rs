use std::{collections::HashMap, iter::Peekable, slice::Iter};

#[derive(Debug)]
enum Token {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    StringLiteral(String),
    Identifier(String),
    Equals,
}

enum IncompleteToken {
    None,
    StringLiteral(String),
    Identifier(String),
}

fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut state = IncompleteToken::None;

    for char in text.chars() {
        match state {
            IncompleteToken::StringLiteral(ref mut string) => {
                if char == '"' {
                    tokens.push(Token::StringLiteral(string.to_string()));
                    state = IncompleteToken::None;
                } else {
                    string.push(char);
                }

                continue;
            }
            _ => {}
        }

        match char {
            char if char.is_alphabetic() => match state {
                IncompleteToken::None => {
                    state = IncompleteToken::Identifier(char.to_string());
                }
                IncompleteToken::Identifier(ref mut string) => {
                    string.push(char);
                }
                _ => panic!(),
            },
            ' ' | '\n' | '{' | '}' | '(' | ')' | '=' => {
                match state {
                    IncompleteToken::Identifier(identifier) => {
                        tokens.push(Token::Identifier(identifier));
                        state = IncompleteToken::None;
                    }
                    IncompleteToken::None => {}
                    _ => panic!(),
                }

                match char {
                    '{' => tokens.push(Token::OpenBracket),
                    '}' => tokens.push(Token::CloseBracket),
                    '(' => tokens.push(Token::OpenParen),
                    ')' => tokens.push(Token::CloseParen),
                    '=' => tokens.push(Token::Equals),
                    _ => {}
                }
            }
            '"' => state = IncompleteToken::StringLiteral(String::new()),
            _ => panic!("Unknown character '{char}'"),
        }
    }

    return tokens;
}

#[derive(Debug)]
pub struct TreeNode {
    element: String,
    attributes: HashMap<String, String>,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(element: String) -> TreeNode {
        TreeNode {
            element,
            attributes: HashMap::new(),
            children: vec![],
        }
    }
}

fn parse_attributes(tree: &mut TreeNode, iter: &mut Peekable<Iter<Token>>) {
    enum State {
        None,
        Attribute(String),
        Equals(String),
    }

    let mut state = State::None;

    while let Some(token) = iter.next() {
        match (state, token) {
            (State::None, Token::Identifier(identifier)) => {
                state = State::Attribute(identifier.clone())
            }
            (State::Attribute(attribute), Token::Equals) => {
                state = State::Equals(attribute.clone())
            }
            (State::Equals(attribute), Token::StringLiteral(value)) => {
                tree.attributes.insert(attribute, value.to_string());

                state = State::None;
            }
            (State::None, Token::CloseParen) => {
                return;
            }
            _ => panic!(),
        }
    }
}

fn parse_elements(iter: &mut Peekable<Iter<Token>>) -> Vec<TreeNode> {
    let mut elements = vec![];

    while let Some(_) = iter.peek() {
        elements.push(parse_element(iter));
    }

    return elements;
}

fn parse_element(mut iter: &mut Peekable<Iter<Token>>) -> TreeNode {
    let element = match iter.next().unwrap() {
        Token::Identifier(identifier) => identifier.to_string(),
        _ => panic!("Cannot have unnamed elements"),
    };

    let mut tree = TreeNode::new(element);

    while let Some(token) = iter.peek() {
        match token {
            Token::OpenParen => {
                iter.next();
                parse_attributes(&mut tree, iter);
            }
            Token::OpenBracket => {
                iter.next();
                tree.children = parse_elements(&mut iter);
            }
            Token::CloseBracket => {
                iter.next();
                iter.next(); // skip to token after bracket
                return tree;
            }
            Token::Identifier(_) => {
                return tree;
            }
            _ => panic!("Token in invalid position"),
        };
    }

    return tree;
}

fn parse(tokens: &[Token]) -> Vec<TreeNode> {
    parse_elements(&mut tokens.iter().peekable())
}

fn treenodes_to_html(tree_nodes: Peekable<Iter<TreeNode>>, ident_level: usize) -> String {
    let html = tree_nodes
        .map(|node| treenode_to_html(node, ident_level))
        .collect::<Vec<_>>()
        .join("\n");

    return html;
}

pub fn treenode_to_html(tree: &TreeNode, ident_level: usize) -> String {
    let mut html = String::new();

    let mut attributes_html = String::new();

    for (attribute, value) in &tree.attributes {
        attributes_html.push_str(&format!("{attribute}=\"{value}\""));
    }

    let ident = vec!["\t"; ident_level].join("");

    if tree.attributes.is_empty() {
        html.push_str(&format!("{ident}<{}>\n", tree.element))
    } else {
        html.push_str(&format!("{ident}<{} {attributes_html}>\n", tree.element))
    }

    html.push_str(&treenodes_to_html(
        tree.children.iter().peekable(),
        ident_level + 1,
    ));

    html.push_str(&format!("\n{ident}</{}>", tree.element));

    return html;
}

pub fn markup_text_to_html(text: &str) -> String {
    let tree = parse(&tokenize(text));

    treenodes_to_html(tree.iter().peekable(), 0)
}
