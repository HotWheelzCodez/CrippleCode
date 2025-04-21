mod cc {
    pub enum TokenType {
        Main,
        OpenScope,
        CloseScope,
        Return,
        Print,
        For,
        Through,
        Var,
        VarName,
        Assign,
        IntLit,
        StrLit,
    }

    pub struct Token {
        pub contents: String,
        pub token_type: TokenType,
    }

    pub fn log_error(msg: &str) {
        println!("CRIPPLE CODE: ERROR: {}", String::from(msg));
    }

    pub fn str_find(text: &String, to_find: char) -> usize {
        let mut count: usize = 0;
        for c in text.chars() {
            if c == to_find {
                return count;
            }
            count += 1;
        }
        return usize::MAX;
    }

    pub fn get_extension(file: String) -> String {
        let index = str_find(&file, '.'); 
        if index == usize::MAX {
            log_error("Invalid file format!");
            std::process::exit(1);
        }

        let end = &file[index..file.len()];
        return String::from(end);
    }

    pub fn show_tokens(tokens: &Vec<Token>) {
        for token in tokens {
            println!("{}", token.contents);
        }
    }

    pub fn tokenize(content: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut temp = String::new();

        // Parse modes
        let mut parse_str = false;
        let mut parse_var = false;

        for c in content.chars() { 
            match c {
                ' ' => {
                    if parse_str {
                        temp.push(c);
                        continue;
                    }

                    if temp == "main".to_string() {
                        let token = Token { contents: temp.clone(), token_type: TokenType::Main };
                        tokens.push(token);
                        temp.clear();
                    } else if temp == "return".to_string() {
                        let token = Token { contents: temp.clone(), token_type: TokenType::Return };
                        tokens.push(token);
                        temp.clear();
                    } else if temp == "print" {
                        let token = Token { contents: temp.clone(), token_type: TokenType::Print };
                        tokens.push(token);
                        temp.clear();
                    } else if temp == "for" {
                        let token = Token { contents: temp.clone(), token_type: TokenType::For };
                        tokens.push(token);
                        temp.clear();
                    } else if temp == "through" {
                        let token = Token { contents: temp.clone(), token_type: TokenType::Through };
                        tokens.push(token);
                        temp.clear();
                    } else if temp == "var" {
                        let token = Token { contents: temp.clone(), token_type: TokenType::Var };
                        tokens.push(token);
                        temp.clear();
                        parse_var = true;
                    } else {
                        if parse_var {
                            let token = Token { contents: temp.clone(), token_type: TokenType::VarName };
                            tokens.push(token);
                            temp.clear();
                            parse_var = false;
                        } else if parse_str {
                            temp.push(c);
                        } else {
                            match temp.parse::<i32>() {
                                Ok(_) => {
                                    let token = Token { contents: temp.clone(), token_type: TokenType::IntLit };
                                    tokens.push(token);
                                    temp.clear();
                                }
                                Err(_) => {
                                    let token = Token { contents: temp.clone(), token_type: TokenType::VarName };
                                    tokens.push(token);
                                    temp.clear();
                                }
                            }
                        }
                    }
                }
                '{' => {
                    let token = Token { contents: "{".to_string(), token_type: TokenType::OpenScope };
                    tokens.push(token);
                    temp.clear();
                }
                '}' => {
                    let token = Token { contents: "}".to_string(), token_type: TokenType::CloseScope };
                    tokens.push(token);
                    temp.clear();
                }
                ';' => {
                    match temp.parse::<usize>() {
                        Ok(_) => {
                            let token = Token { contents: temp.clone(), token_type: TokenType::IntLit };
                            tokens.push(token);
                            temp.clear();
                        }
                        Err(_) => {
                            let token = Token { contents: temp.clone(), token_type: TokenType::VarName };
                            tokens.push(token);
                            temp.clear();
                        }
                    }
                    parse_str = false;
                }
                '"' => {
                    if parse_str {
                        let token = Token { contents: temp.clone(), token_type: TokenType::StrLit };
                        tokens.push(token);
                        temp.clear();
                        parse_str = false;
                    } else {
                        parse_str = true;
                    }
                }
                '=' => {
                    let token = Token { contents: "=".to_string(), token_type: TokenType::Assign };
                    tokens.push(token);
                    temp.clear();
                    parse_var = false;
                }
                _ => temp.push(c)
            }
        }

        return tokens;
    }

    pub fn compile(tokens: Vec<Token>) {
        struct StackVar {
            pub name: String,
            pub offset: usize,
        }

        let mut label_count: usize = 0;

        let mut asm = String::new();
        asm.push_str("format ELF64 executable\n");

        let mut code = String::new();
        let mut data = String::new();

        let mut loop_control: usize = 0;

        let mut stack_vars: Vec<StackVar> = Vec::new();

        for (index, token) in tokens.iter().enumerate() {
            match token.token_type {
                TokenType::Main => code.push_str("entry _start\n_start:\npush ebp\nmov ebp,esp\n"), 
                TokenType::Return => {
                    if index == tokens.len()-1 {
                        log_error("No value after return!");
                        std::process::exit(1);
                    }

                    let next_token = &tokens[index+1];
                    match next_token.token_type {
                        TokenType::IntLit => {
                            code.push_str(&"mov rax, 60\nmov rdi, ".to_string());
                            code.push_str(&next_token.contents);
                            code.push_str(&"\nsyscall\n".to_string());
                        }
                        _ => { }
                    }
                }
                TokenType::Print => {
                    if index == tokens.len()-1 {
                        log_error("No value after print! Expected \"message\"!");
                        std::process::exit(1);
                    }

                    let next_token = &tokens[index+1];
                    match next_token.token_type {
                        TokenType::StrLit => {
                            data.push_str(&"print_".to_string());
                            data.push_str(&label_count.to_string());
                            data.push_str(" db \"");
                            data.push_str(&next_token.contents);
                            data.push_str("\", 0, 10\n");

                            data.push_str("print_end_");
                            data.push_str(&label_count.to_string());
                            data.push_str(":\n");

                            data.push_str("print_len_");
                            data.push_str(&label_count.to_string());
                            data.push_str(" equ print_end_");
                            data.push_str(&label_count.to_string());
                            data.push_str("-");
                            data.push_str("print_");
                            data.push_str(&label_count.to_string());
                            data.push_str("\n");

                            code.push_str("mov rax, 1\nmov rdi, 1\nmov rsi, print_");
                            code.push_str(&label_count.to_string());
                            code.push_str("\nmov rdx, print_len_");
                            code.push_str(&label_count.to_string());
                            code.push_str("\nsyscall\n");

                            label_count += 1;
                        }
                        TokenType::VarName => {
                            let name = next_token.contents.clone();
                            code.push_str("mov rax, 1\nmov rdi, 1\nmov rsi, ");
                            code.push_str(&name.to_string());
                            code.push_str("\nmov rdx, ");
                            code.push_str(&name.to_string());
                            code.push_str("_len\n");
                            code.push_str("syscall\n");
                        }
                        _ => { }
                    }
                }
                TokenType::For => {
                    code.push_str("repeat ");

                    let next_token = &tokens[index+1];
                    match next_token.token_type {
                        TokenType::IntLit => {
                            loop_control = next_token.contents.parse::<usize>().unwrap();
                        }
                        _ => { }
                    }
                }
                TokenType::Through => {
                    if index == tokens.len()-1 {
                        log_error("No value after through! Expected a end value!");
                        std::process::exit(1);
                    }

                    let next_token = &tokens[index+1];
                    match next_token.token_type {
                        TokenType::IntLit => {
                            loop_control = next_token.contents.parse::<usize>().unwrap() - loop_control;
                            code.push_str(&loop_control.to_string());
                            code.push_str("\n");
                        }
                        _ => { }
                    }
                }
                TokenType::CloseScope => {
                    if loop_control > 0 {
                        code.push_str("end repeat\n");
                        loop_control = 0;
                    }
                }
                TokenType::Var => {
                    let name_token = &tokens[index+1];
                    match name_token.token_type {
                        TokenType::VarName => {
                            let name = name_token.contents.clone();
                            // Skip assign, assume it is there lol
                            let value_token = &tokens[index+4];
                            match value_token.token_type {
                                TokenType::StrLit => {
                                    data.push_str(&name);
                                    data.push_str(" db \"");
                                    data.push_str(&value_token.contents);
                                    data.push_str("\", 0, 10\n");
                                    
                                    data.push_str(&name);
                                    data.push_str("_end:\n");

                                    data.push_str(&name);
                                    data.push_str("_len equ ");
                                    data.push_str(&name);
                                    data.push_str("_end-");
                                    data.push_str(&name);
                                    data.push_str("\n");
                                }
                                TokenType::IntLit => {
                                    code.push_str("push ");
                                    code.push_str(&value_token.contents);
                                    code.push_str("\n");

                                    let var = StackVar { name: name_token.contents.clone(), offset: 0 };
                                    stack_vars.push(var);
                                    for stack_var in &mut stack_vars {
                                        stack_var.offset += 4;
                                    }
                                }
                                _ => { }
                            }
                        }
                        _ => { } 
                    }
                }
                TokenType::VarName => {
                    let mut offset = 0;
                    for stack_var in &stack_vars {
                        if stack_var.name == token.contents {
                            offset = stack_var.offset;
                            break;
                        }
                    }
                    code.push_str("mov ecx,[ebp-");
                    code.push_str(&offset.to_string());
                    code.push_str("]\n");
                }
                _ => { }
            }
        }

        asm.push_str(&data);
        asm.push_str(&code);

        let _ = std::fs::write("main.asm", asm);
    }

    pub fn build() {
        std::process::Command::new("fasm").args(["main.asm"]).output().expect("CRIPPLE CODE: ERROR: Could not generate asm!");
        //std::process::Command::new("rm").args(["main.asm"]).output().expect("CRIPPLE CODE: ERROR: Could not cleanup properly!");
        std::process::Command::new("./main").status().expect("CRIPPLE CODE: ERROR: Could not execute program!");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        cc::log_error("No file given!");
        std::process::exit(1);
    }

    let file = &args[1];
    let extension = cc::get_extension(file.to_string());
    if extension != ".cc" {
        cc::log_error("Unknown file type! Expecting '.cc' files!");
        std::process::exit(1);
    }

    let contents = std::fs::read_to_string(file).expect("CRIPPLE CODE: ERROR: Could not read file!"); 

    let tokens: Vec<cc::Token> = cc::tokenize(contents);    
    //cc::show_tokens(&tokens);
    cc::compile(tokens);
    cc::build();
}
