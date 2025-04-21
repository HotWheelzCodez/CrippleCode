enum Token {
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
    Print,
    Func,
}

struct ASTNode {
    token: Token, 
    children: Vec<Token>
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

pub fn tokenize(file_contents: &str) -> Vec<Token> {
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

                match temp.as_str() {
                    "print" => {
                        tokens.push(Token::Print);
                        temp.clear();
                    }
                    "func" => {
                        tokens.push(Token::Func);
                        temp.clear();
                    }
                    _ => temp.push(c),
                }
            }
            ';' => tokens.push(Token::Semicolen),
            '{' => tokens.push(Token::OpenScope),
            '}' => tokens.push(Token::CloseScope),
            '(' => tokens.push(Token::OpenArea),
            ')' => tokens.push(Token::CloseArea),
            ',' => tokens.push(Token::Comma),
            '"' => {
                if parse_str {
                    tokens.push(Token::StrLit(temp.clone()));
                    parse_str = false;
                    temp.clear();
                }
                parse_str = !parse_str;
            } 
            _ => temp.push(c)
        }
    }

    return tokens;
}

pub fn create_ast(tokens: Vec<Token>) -> Vec<ASTNode> {
    let mut ast_tree: Vec<ASTNode> = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::Print => {
                let mut ast_node = ASTNode {
                    token: Token::Print,
                    children: {
                        let mut sub_tokens: Vec<Token> = Vec::new();
                        for i in [(index+1)..tokens.len()] {
                            let sub_token = tokens[i];
                            match sub_token {
                                Token::Semicolen => return sub_tokens,
                                _ => return sub_tokens,
                            }
                        }
                        sub_tokens
                    }
                };
            }
            _ => { }
        } 
    }

    return ast_tree;
}
