use std::arch::aarch64::vmin_u8;
use std::fs;
use std::cmp::PartialEq;

pub fn run_file(path: &str) {
    let values = fs::read(path).expect("Invalid Filepath");
    let content = std::str::from_utf8(&values).unwrap();


    let mut arr:Vec<&str> = content.split("").collect();
    arr.pop();






    let potato:Vec<Token> = Vec::new();

    let mut scanner:Scanner = Scanner{tokens: potato, source: content.parse().unwrap() };
    let mut result:Vec<String> = Vec::new();

    let tokens:&Vec<Token> = scanner.tokenize();


    for x in tokens{
        match x.token_type {
            TokenType::LINE_START => result.push("START".parse().unwrap()),
            TokenType::NUMBER => result.push("NUMBER".parse().unwrap()),
            TokenType::IDENTIFIER => result.push("IDENTIFIER".parse().unwrap()),
            TokenType::VARIABLE_ASSIGNMENT => result.push("ASSIGNMENT".parse().unwrap()),
            TokenType::EQUAL=> result.push("EQUAL".parse().unwrap()),
            TokenType::SEMICOLON => result.push("END_OF_LINE".parse().unwrap()),
            TokenType::IMMUTABLE => result.push("IMMUTABLE".parse().unwrap()),
            TokenType::STRING => result.push("STRING".parse().unwrap()),
            TokenType::PLUS => result.push("ADD".parse().unwrap()),
            TokenType::MINUS => result.push("SUBTRACT".parse().unwrap()),
            TokenType::SLASH => result.push("DIVIDE".parse().unwrap()),
            TokenType::STAR => result.push("MULTIPLY".parse().unwrap()),
            _ => {}

        }
    }

    println!("{:?}",result);




}

enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    // One or two character tokens.
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,
    // Literals.
    IDENTIFIER, STRING, NUMBER,
    LINE_START, VARIABLE_ASSIGNMENT, IMMUTABLE,

    EOF
}

pub struct Token {



    token_type: TokenType,
    literal:String,
    line:i32
}

impl Token{
    pub fn new_token(token_type: TokenType, literal:String,line:i32 )->Self{
        Self{
            token_type,
            literal,
            line,
        }
    }


}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}



pub struct Scanner{

    tokens:Vec<Token>,
    source:String

}



impl Scanner{

    pub fn tokens(&self)-> &Vec<Token>{
       &self.tokens
    }





    pub fn tokenize(&mut self) -> &Vec<Token> {
        let mut chars = self.source.chars().peekable();
        let mut token_list: Vec<Token> = Vec::new();
        let mut current_string = String::new();
        let mut in_string = false;

        while let Some(&c) = chars.peek() {
            if in_string {
                match c {
                    '"' => {
                        // End of string
                        in_string = false;
                        chars.next();
                        token_list.push(Token::new_token(TokenType::STRING, current_string.clone(), 1));
                        current_string.clear();
                    }
                    _ => {
                        // Continue the string
                        current_string.push(c);
                        chars.next();
                    }
                }
            } else {
                match c {
                    ' ' | '\r' | '\t' | '\n' => {
                        chars.next(); // Ignore whitespace
                    }
                    '"' => {
                        // Start of string
                        in_string = true;
                        chars.next();
                    }
                    ';' => {
                        token_list.push(Token::new_token(TokenType::SEMICOLON, c.to_string(), 1));
                        chars.next();
                    }
                    _ => {
                        // Handle other tokens
                        let mut literal = String::new();
                        while let Some(&c) = chars.peek() {
                            if c.is_whitespace() || c == ';' || c == '"' {
                                break;
                            }
                            literal.push(c);
                            chars.next();
                        }

                        if literal == "bro" {
                            token_list.push(Token::new_token(TokenType::LINE_START, literal, 1));
                        } else if literal == "be" {
                            token_list.push(Token::new_token(TokenType::VARIABLE_ASSIGNMENT, literal, 1));
                        } else if literal == "noCap" {
                            token_list.push(Token::new_token(TokenType::IMMUTABLE, literal, 1));
                        } else if literal == "*" {
                            token_list.push(Token::new_token(TokenType::STAR, literal, 1));
                        }
                         else if literal == "+" {
                        token_list.push(Token::new_token(TokenType::PLUS, literal, 1));
                        }
                         else if literal == "-" {
                             token_list.push(Token::new_token(TokenType::MINUS, literal, 1));
                         }
                         else if literal == "/" {
                             token_list.push(Token::new_token(TokenType::SLASH, literal, 1));
                         }
                         else if literal == "(" {
                             token_list.push(Token::new_token(TokenType::LEFT_PAREN, literal, 1));
                         }
                         else if literal == ")" {
                             token_list.push(Token::new_token(TokenType::RIGHT_PAREN, literal, 1));
                         }



                         else if literal.parse::<i32>().is_ok() {
                            token_list.push(Token::new_token(TokenType::NUMBER, literal, 1));
                        } else {
                            token_list.push(Token::new_token(TokenType::IDENTIFIER, literal, 1));
                        }
                    }
                }
            }
        }

        self.tokens = token_list;
        &self.tokens
    }
    pub fn scanner()->Self{
        Self{ tokens: vec![], source: "".to_string() }
    }
}




// Run testing command: cargo run --bin bussin "/Users/rohan/RustroverProjects/bussinScript/src/test_script"