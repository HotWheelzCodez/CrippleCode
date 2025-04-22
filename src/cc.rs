#[derive(Debug, Clone, PartialEq)]
enum Token {
    Null,
    StrLit(String),
    IntLit(i32),
    UIntLit(u32),
    BigIntLit(i64),
    UBigIntLit(u64),
    FltLit(f32),
    BigFltLit(f64),
    Semicolen,
    OpenScope,
    CloseScope,
    OpenArea,
    CloseArea,
    Comma,
    Main,
    Print,
    Func,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Null => write!(f, ""),
            Token::StrLit(s) => write!(f, "\"{}\"", s),
            Token::IntLit(n) => write!(f, "{}", n),
            Token::UIntLit(n) => write!(f, "{}", n),
            Token::BigIntLit(n) => write!(f, "{}", n),
            Token::UBigIntLit(n) => write!(f, "{}", n),
            Token::FltLit(n) => write!(f, "{}", n),
            Token::BigFltLit(n) => write!(f, "{}", n),
            Token::Semicolen => write!(f, ";"),
            Token::OpenScope => write!(f, "{{"),
            Token::CloseScope => write!(f, "}}"),
            Token::OpenArea => write!(f, "("),
            Token::CloseArea => write!(f, ")"),
            Token::Comma => write!(f, ","),
            Token::Main => write!(f, "main"),
            Token::Print => write!(f, "print"),
            Token::Func => write!(f, "func"),
        }
    }
}

struct ASTNode {
    token: Token, 
    children: Vec<ASTNode>
}

pub fn log_error(msg: &str) {
    println!("CRIPPLE CODE: ERROR: {msg}");
}

pub fn check_extension(file_path: &str) -> bool {
    let index: usize = match file_path.find('.') {
        Some(n) => n,
        None => return false 
    };

    if &file_path[index..file_path.len()] == ".cc" {
        return true;   
    }

    return false;
}

fn process_stmt(stmt: &str) -> Token {
    match stmt.trim() {
        "print" => return Token::Print,
        "func" => return Token::Func,
        "main" => return Token::Main,
        _ => return Token::Null,
    }
}

fn tokenize(file_contents: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut parse_str = false;

    let mut temp = String::new();
    for c in file_contents.chars() {
        match c {
            ' ' => {
                if parse_str {
                    temp.push(c);
                    continue;
                }

                let token = process_stmt(&temp);
                if let Token::Null = token {
                    continue;
                }
                tokens.push(token);
                temp.clear();
            }
            ';' => {
                let token = process_stmt(&temp);
                if let Token::Null = token {
                    tokens.push(Token::Semicolen);
                    continue;
                }
                tokens.push(token);
                tokens.push(Token::Semicolen);
                temp.clear();
            }
            '{' => tokens.push(Token::OpenScope),
            '}' => tokens.push(Token::CloseScope),
            '(' => tokens.push(Token::OpenArea),
            ')' => tokens.push(Token::CloseArea),
            ',' => tokens.push(Token::Comma),
            '"' => {
                if parse_str {
                    tokens.push(Token::StrLit(temp.clone()));
                    temp.clear();
                }
                parse_str = !parse_str;
            } 
            _ => {
                if parse_str {
                    temp.push(c);
                } else if !c.is_whitespace() {
                    temp.push(c);
                }
            }
        }
    }

    return tokens;
}

fn parse_main(tokens: &[Token], i: &mut usize) -> ASTNode {
    *i += 1;

    if tokens.get(*i) == Some(&Token::OpenScope) {
        *i += 1;

        let children = parse_ast(tokens, i);

        if tokens.get(*i) == Some(&Token::CloseScope) {
            *i += 1;
        }

        return ASTNode {
            token: Token::Main,
            children,
        }
    } else {
        panic!("Expected open scope after Main");
    }
}

fn parse_print(tokens: &[Token], i: &mut usize) -> ASTNode {
    *i += 1;
    let mut children = Vec::new();

    while *i < tokens.len() {
        match &tokens[*i] {
            Token::Semicolen => {
                *i += 1;
                break;
            }
            token => {
                children.push(ASTNode {
                    token: token.clone(),
                    children: Vec::new(),
                });
                *i += 1;
            }
        }
    }

    return ASTNode {
        token: Token::Print,
        children,
    }
}

fn parse_ast(tokens: &[Token], i: &mut usize) -> Vec<ASTNode> {
    let mut ast = Vec::new();

    while *i < tokens.len() {
        match &tokens[*i] {
            Token::Main => {
                ast.push(parse_main(tokens, i));
            }
            Token::Print => {
                ast.push(parse_print(tokens, i));
            }
            Token::CloseScope => break,
            _ => *i += 1,
        }
    }

    return ast;
}

fn create_ast(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut i = 0;
    return parse_ast(&tokens, &mut i);
}

fn print_ast(ast: &[ASTNode], depth: usize) {
    for node in ast {
        for _ in 0..depth {
            print!("  ");
        }
        println!("{}", node.token);
        print_ast(&node.children, depth+1);
    }
}

pub fn compile(contents: &str) {
    let tokens: Vec<Token> = tokenize(contents);
    let ast: Vec<ASTNode> = create_ast(tokens);

    print_ast(&ast, 0);
}
