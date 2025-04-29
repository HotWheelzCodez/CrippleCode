#[derive(Debug, Clone, PartialEq)]
enum Token {
    StrLit(String),
    IntLit(i32),
    UIntLit(u32),
    BigIntLit(i64),
    UBigIntLit(u64),
    FloatLit(f32),
    BigFloatLit(f64),
    Ref(String),
    Semicolen,
    OpenScope,
    CloseScope,
    OpenExpr,
    CloseExpr,
    Comma,
    Assign,
    Comp,
    Main,
    Print,
    Func,
    Var,
    If,
    Res,
    And,
    Or,
    For,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::StrLit(s) => write!(f, "\"{}\"", s),
            Token::IntLit(n) => write!(f, "{}", n),
            Token::UIntLit(n) => write!(f, "{}", n),
            Token::BigIntLit(n) => write!(f, "{}", n),
            Token::UBigIntLit(n) => write!(f, "{}", n),
            Token::FloatLit(n) => write!(f, "{}", n),
            Token::BigFloatLit(n) => write!(f, "{}", n),
            Token::Ref(s) => write!(f, "{}", s),
            Token::Semicolen => write!(f, ";"),
            Token::OpenScope => write!(f, "{{"),
            Token::CloseScope => write!(f, "}}"),
            Token::OpenExpr => write!(f, "open"),
            Token::CloseExpr => write!(f, "close"),
            Token::Assign => write!(f, "="),
            Token::Comp => write!(f, "=="),
            Token::Comma => write!(f, ","),
            Token::Main => write!(f, "main"),
            Token::Print => write!(f, "print"),
            Token::Func => write!(f, "func"),
            Token::Var => write!(f, "var"),
            Token::If => write!(f, "if"),
            Token::Res => write!(f, "res"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::For => write!(f, "for"),
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

    if &file_path[index..file_path.len()] == ".crip" {
        return true;   
    }

    return false;
}

fn process_stmt(stmt: &str) -> Token {
    match stmt.trim() {
        "print" => return Token::Print,
        "func" => return Token::Func,
        "main" => return Token::Main,
        "var" => return Token::Var,
        "if" => return Token::If,
        "equals" => return Token::Comp,
        "and" => return Token::And,
        "or" => return Token::Or,
        "for" => return Token::For,
        _ => return Token::Ref(stmt.trim().to_string()),
    }
}

fn tokenize(file_contents: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut parse_str = false;

    let mut temp = String::new();
    let mut chars = file_contents.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ' ' => {
                if parse_str {
                    temp.push(c);
                } else if !temp.is_empty() {
                    tokens.push(process_stmt(&temp));
                    temp.clear();
                }
            }
            ';' => {
                if !temp.is_empty() {
                    tokens.push(process_stmt(&temp));
                    temp.clear();
                }
                tokens.push(Token::Semicolen);
            }
            '{' => tokens.push(Token::OpenScope),
            '}' => tokens.push(Token::CloseScope),
            '(' => {
                if !temp.is_empty() {
                    tokens.push(process_stmt(&temp));
                    temp.clear();
                }
                tokens.push(Token::OpenExpr);
            }
            ')' => {
                if !temp.is_empty() {
                    tokens.push(process_stmt(&temp));
                    temp.clear();
                }
                tokens.push(Token::CloseExpr);
            }
            ',' => tokens.push(Token::Comma),
            '=' => {
                if let Some('=') = chars.peek() {
                    chars.next(); // consume the '='
                    tokens.push(Token::Comp);
                } else {
                    tokens.push(Token::Assign);
                }
            }
            '"' => {
                if parse_str {
                    tokens.push(Token::StrLit(temp.clone()));
                    temp.clear();
                }
                parse_str = !parse_str;
            }
            _ => {
                if parse_str || !c.is_whitespace() {
                    temp.push(c);
                }
            }
        }
    }

    if !temp.is_empty() {
        tokens.push(process_stmt(&temp));
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

fn parse_var(tokens: &[Token], i: &mut usize) -> ASTNode {
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
        token: Token::Var,
        children,
    }
}

fn parse_if(tokens: &[Token], i: &mut usize) -> ASTNode {
    *i += 1;
    let mut children = Vec::new();

    while *i < tokens.len() {
        match &tokens[*i] {
            Token::Semicolen => {
                *i += 1;
                break;
            }
            Token::CloseScope => {
                *i += 1;
                children.push(ASTNode {
                    token: Token::CloseExpr,
                    children: Vec::new(),
                });
                break;
            }
            Token::OpenExpr => {
                *i += 1;
                let if_expr = parse_ast(tokens, i);
                children.push(ASTNode {
                    token: Token::OpenExpr,
                    children: if_expr,
                });
            }
            Token::OpenScope => {
                *i += 1;
                let if_scope = parse_ast(tokens, i);
                children.push(ASTNode {
                    token: Token::Res,
                    children: if_scope,
                });
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
        token: Token::If,
        children,
    }
}

fn parse_func(tokens: &[Token], i: &mut usize) -> ASTNode {
    *i += 1;
    let mut children = Vec::new();

    while *i < tokens.len() {
        match &tokens[*i] {
            Token::Semicolen => {
                *i += 1;
                break;
            }
            Token::CloseScope => {
                *i += 1;
                break;
            }
            Token::OpenExpr => {
                *i += 1;
                let if_expr = parse_ast(tokens, i);
                children.push(ASTNode {
                    token: Token::OpenExpr,
                    children: if_expr,
                });
            }
            Token::OpenScope => {
                *i += 1;
                let if_scope = parse_ast(tokens, i);
                children.push(ASTNode {
                    token: Token::Res,
                    children: if_scope,
                });
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
        token: Token::Func,
        children,
    }

}

fn parse_ast(tokens: &[Token], i: &mut usize) -> Vec<ASTNode> {
    let mut ast = Vec::new();

    while *i < tokens.len() {
        match &tokens[*i] {
            Token::Main => ast.push(parse_main(tokens, i)),
            Token::Print => ast.push(parse_print(tokens, i)),
            Token::Var => ast.push(parse_var(tokens, i)),
            Token::If => ast.push(parse_if(tokens, i)),
            Token::Func => ast.push(parse_func(tokens, i)),
            Token::For => ast.push(parse_main(tokens, i)),
            Token::OpenExpr => {
                *i += 1;
                ast.push(ASTNode {
                    token: Token::OpenExpr,
                    children: parse_ast(tokens, i),
                });
                ast.push(ASTNode {
                    token: Token::CloseExpr,
                    children: Vec::new(),
                });
            }
            Token::CloseScope => break,
            Token::CloseExpr => {
                *i += 1;
                break;
            }
            _ => {
                ast.push(ASTNode {
                    token: tokens[*i].clone(),
                    children: Vec::new(),
                });
                *i += 1;
            }
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
    // for token in &tokens {
    //     println!("{}", token); 
    // }
    let ast: Vec<ASTNode> = create_ast(tokens);
    print_ast(&ast, 0);
}
