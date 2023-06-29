use std::collections::HashMap;

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
    fn new(
        element: String,
        attributes: HashMap<String, String>,
        children: Vec<TreeNode>,
    ) -> TreeNode {
        TreeNode {
            element,
            attributes,
            children,
        }
    }
}

fn parse_attributes(tree: &mut TreeNode, tokens: &[Token]) -> usize {
    let mut attribute_name = None;
    let mut has_seen_equals = false;

    let mut iter = tokens.iter().enumerate();

    while let Some((i, token)) = iter.next() {
        match token {
            Token::Identifier(identifier) => {
                attribute_name = Some(identifier.to_string());
            }
            Token::Equals => match has_seen_equals {
                true => panic!("Only one equals permitted"),
                false => has_seen_equals = true,
            },
            Token::StringLiteral(value) => {
                let attribute = attribute_name.expect("Attribute name should be specified");

                if !has_seen_equals {
                    panic!("Equal sign needed between attribute name and value")
                }

                tree.attributes.insert(attribute, value.to_string());

                has_seen_equals = false;
                attribute_name = None;
            }
            Token::CloseParen => {
                return i;
            }
            _ => panic!("Token in invalid position"),
        }
    }

    return tokens.len();
}

fn parse_element(tokens: &[Token]) -> (TreeNode, usize) {
    let mut tree = TreeNode::new(String::from(""), HashMap::new(), vec![]);

    let mut iter = tokens.iter().enumerate();

    while let Some((i, token)) = iter.next() {
        match token {
            Token::Identifier(identifier) => tree.element = identifier.to_string(),
            Token::OpenParen => {
                if tree.element == "" {
                    panic!("Cannot have unnamed elements");
                }

                let start_at = i + 1;

                let end = parse_attributes(&mut tree, &tokens[start_at..]);

                // skip loop to iteration "end"
                iter.nth(end);
            }
            Token::CloseParen => panic!(") in invalid position"),
            Token::OpenBracket => {
                if tree.element == "" {
                    panic!("Cannot have unnamed elements");
                }

                let start_at = i + 1;

                let (parsed, end) = parse_element(&tokens[start_at..]);

                tree.children.push(parsed);

                // skip loop to iteration "end"
                iter.nth(end);
            }
            Token::CloseBracket => return (tree, i),
            _ => panic!("Token in invalid position"),
        };
    }

    return (tree, tokens.len());
}

fn parse(tokens: &[Token]) -> TreeNode {
    parse_element(tokens).0
}

pub fn markup_to_html(tree: &TreeNode, ident_level: usize) -> String {
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

    for node in &tree.children {
        html.push_str(&markup_to_html(node, ident_level + 1));
    }
    html.push_str(&format!("\n{ident}<{}/>", tree.element));

    html
}

pub fn markup_text_to_html(text: &str) -> String {
    let tree = parse(&tokenize(text));

    markup_to_html(&tree, 0)
}
