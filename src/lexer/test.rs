#[cfg(test)]
mod lexer_tests {

    use std::assert_eq;

    use crate::lexer::{
        reserved::{Keyword, Reserved},
        token::Token,
        Lexer,
    };

    const INPUT: &'static str = "=+(){},;";
    const ZPP_FILES_DIR: &'static str = "data/";
    const CODE: &'static str = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y
        }

        let result = add(five, ten);"#;

    #[test]
    fn test_next_token_in_sample() {
        let mut tokens: Vec<(&str, &str)> = Vec::new();
        tokens.push((Reserved::ASSIGN.as_str(), "="));
        tokens.push((Reserved::ARITHMETIC.as_str(), "+"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::LBRACE.as_str(), "{"));
        tokens.push((Reserved::RBRACE.as_str(), "}"));
        tokens.push((Reserved::COMMA.as_str(), ","));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));

        let mut lexer = Lexer::new(INPUT.chars().collect());

        for (key, value) in tokens.into_iter() {
            let tok: Token = lexer.next_token();

            assert_eq!(tok.t, key);
            assert_eq!(tok.value, value);
        }
    }

    #[test]
    fn test_next_token_in_code() {
        let mut tokens: Vec<(&str, &str)> = Vec::new();

        tokens.push((Reserved::KEYWORD(Keyword::LET).as_str(), "let"));
        tokens.push((Reserved::IDENT.as_str(), "five"));
        tokens.push((Reserved::ASSIGN.as_str(), "="));
        tokens.push((Reserved::INT.as_str(), "5"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));

        tokens.push((Reserved::KEYWORD(Keyword::LET).as_str(), "let"));
        tokens.push((Reserved::IDENT.as_str(), "ten"));
        tokens.push((Reserved::ASSIGN.as_str(), "="));
        tokens.push((Reserved::INT.as_str(), "10"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));

        tokens.push((Reserved::KEYWORD(Keyword::LET).as_str(), "let"));
        tokens.push((Reserved::IDENT.as_str(), "add"));
        tokens.push((Reserved::ASSIGN.as_str(), "="));
        tokens.push((Reserved::KEYWORD(Keyword::FUNCTION).as_str(), "fn"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::IDENT.as_str(), "x"));
        tokens.push((Reserved::COMMA.as_str(), ","));
        tokens.push((Reserved::IDENT.as_str(), "y"));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::LBRACE.as_str(), "{"));
        tokens.push((Reserved::IDENT.as_str(), "x"));
        tokens.push((Reserved::ARITHMETIC.as_str(), "+"));
        tokens.push((Reserved::IDENT.as_str(), "y"));
        tokens.push((Reserved::RBRACE.as_str(), "}"));

        tokens.push((Reserved::KEYWORD(Keyword::LET).as_str(), "let"));
        tokens.push((Reserved::IDENT.as_str(), "result"));
        tokens.push((Reserved::ASSIGN.as_str(), "="));
        tokens.push((Reserved::IDENT.as_str(), "add"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::IDENT.as_str(), "five"));
        tokens.push((Reserved::COMMA.as_str(), ","));
        tokens.push((Reserved::IDENT.as_str(), "ten"));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));
        tokens.push((Reserved::EOF.as_str(), "\0"));

        let mut lexer = Lexer::new(CODE.chars().collect());

        for (key, value) in tokens.into_iter() {
            let tok: Token = lexer.next_token();

            // println!("type: {}, value: {}", tok.t, tok.value);

            assert_eq!(tok.t, key);
            assert_eq!(tok.value, value);
        }
    }

    #[test]
    fn test_token_number() {
        let code: &'static str = r#"
            const i = 0;
            let j = 0;

            let result = i + j;
        "#;

        let mut lexer = Lexer::new(code.chars().collect());

        let mut tokens: Vec<(&str, &str, usize)> = Vec::new();

        tokens.push((Reserved::KEYWORD(Keyword::CONST).as_str(), "const", 2));
        tokens.push((Reserved::IDENT.as_str(), "i", 2));
        tokens.push((Reserved::ASSIGN.as_str(), "=", 2));
        tokens.push((Reserved::INT.as_str(), "0", 2));
        tokens.push((Reserved::SEMICOLON.as_str(), ";", 2));

        tokens.push((Reserved::KEYWORD(Keyword::LET).as_str(), "let", 3));
        tokens.push((Reserved::IDENT.as_str(), "j", 3));
        tokens.push((Reserved::ASSIGN.as_str(), "=", 3));
        tokens.push((Reserved::INT.as_str(), "0", 3));
        tokens.push((Reserved::SEMICOLON.as_str(), ";", 3));

        tokens.push((Reserved::KEYWORD(Keyword::LET).as_str(), "let", 5));
        tokens.push((Reserved::IDENT.as_str(), "result", 5));
        tokens.push((Reserved::ASSIGN.as_str(), "=", 5));
        tokens.push((Reserved::IDENT.as_str(), "i", 5));
        tokens.push((Reserved::ARITHMETIC.as_str(), "+", 5));
        tokens.push((Reserved::IDENT.as_str(), "j", 5));
        tokens.push((Reserved::SEMICOLON.as_str(), ";", 5));

        for (key, value, line) in tokens.into_iter() {
            let tok: Token = lexer.next_token();

            // println!("type: {}, value: {}", tok.t, tok.value);

            assert_eq!(tok.t, key);
            assert_eq!(tok.value, value);
            assert_eq!(tok.line.unwrap(), line);
        }
    }

    #[test]
    fn test_control_tokens() {
        let code: &'static str = r#"
            if true {
                print(true);
            } else {
                return false;
            }

            if a > b {
                print("hot");
            } else {
                print("cold");
            }

            if true do: print(true), else: print(false)

            if true do
                print(true);
            end
        "#;
        let mut tokens: Vec<(&str, &str)> = Vec::new();

        tokens.push((Reserved::KEYWORD(Keyword::IF).as_str(), "if"));
        tokens.push((Reserved::KEYWORD(Keyword::TRUE).as_str(), "true"));
        tokens.push((Reserved::LBRACE.as_str(), "{"));
        tokens.push((Reserved::IDENT.as_str(), "print"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::KEYWORD(Keyword::TRUE).as_str(), "true"));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));
        tokens.push((Reserved::RBRACE.as_str(), "}"));
        tokens.push((Reserved::KEYWORD(Keyword::ELSE).as_str(), "else"));
        tokens.push((Reserved::LBRACE.as_str(), "{"));
        tokens.push((Reserved::KEYWORD(Keyword::RETURN).as_str(), "return"));
        tokens.push((Reserved::KEYWORD(Keyword::FALSE).as_str(), "false"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));
        tokens.push((Reserved::RBRACE.as_str(), "}"));

        tokens.push((Reserved::KEYWORD(Keyword::IF).as_str(), "if"));
        tokens.push((Reserved::IDENT.as_str(), "a"));
        tokens.push((Reserved::GREATER.as_str(), ">"));
        tokens.push((Reserved::IDENT.as_str(), "b"));
        tokens.push((Reserved::LBRACE.as_str(), "{"));
        tokens.push((Reserved::IDENT.as_str(), "print"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::DQUOTE.as_str(), "\""));
        tokens.push((Reserved::IDENT.as_str(), "hot"));
        tokens.push((Reserved::DQUOTE.as_str(), "\""));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));
        tokens.push((Reserved::RBRACE.as_str(), "}"));
        tokens.push((Reserved::KEYWORD(Keyword::ELSE).as_str(), "else"));
        tokens.push((Reserved::LBRACE.as_str(), "{"));
        tokens.push((Reserved::IDENT.as_str(), "print"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::DQUOTE.as_str(), "\""));
        tokens.push((Reserved::IDENT.as_str(), "cold"));
        tokens.push((Reserved::DQUOTE.as_str(), "\""));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));
        tokens.push((Reserved::RBRACE.as_str(), "}"));

        tokens.push((Reserved::KEYWORD(Keyword::IF).as_str(), "if"));
        tokens.push((Reserved::KEYWORD(Keyword::TRUE).as_str(), "true"));
        tokens.push((Reserved::KEYWORD(Keyword::DO).as_str(), "do"));
        tokens.push((Reserved::COLON.as_str(), ":"));
        tokens.push((Reserved::IDENT.as_str(), "print"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::KEYWORD(Keyword::TRUE).as_str(), "true"));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::COMMA.as_str(), ","));
        tokens.push((Reserved::KEYWORD(Keyword::ELSE).as_str(), "else"));
        tokens.push((Reserved::COLON.as_str(), ":"));
        tokens.push((Reserved::IDENT.as_str(), "print"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::KEYWORD(Keyword::FALSE).as_str(), "false"));
        tokens.push((Reserved::RPAREN.as_str(), ")"));

        tokens.push((Reserved::KEYWORD(Keyword::IF).as_str(), "if"));
        tokens.push((Reserved::KEYWORD(Keyword::TRUE).as_str(), "true"));
        tokens.push((Reserved::KEYWORD(Keyword::DO).as_str(), "do"));
        tokens.push((Reserved::IDENT.as_str(), "print"));
        tokens.push((Reserved::LPAREN.as_str(), "("));
        tokens.push((Reserved::KEYWORD(Keyword::TRUE).as_str(), "true"));
        tokens.push((Reserved::RPAREN.as_str(), ")"));
        tokens.push((Reserved::SEMICOLON.as_str(), ";"));
        tokens.push((Reserved::KEYWORD(Keyword::END).as_str(), "end"));

        let mut lexer = Lexer::new(code.chars().collect());

        for (key, value) in tokens {
            let token = lexer.next_token();

            assert_eq!(token.t, key);
            assert_eq!(token.value, value);
        }
    }

    #[test]
    fn test_peek_next_token_in_code() {
        let code: &'static str = r#"
            ++--**//&&||..;
        "#;
        let mut tokens: Vec<(&str, &str)> = Vec::new();

        tokens.push((Reserved::ARITHMETIC.as_str(), "++"));
        tokens.push((Reserved::ARITHMETIC.as_str(), "--"));
        tokens.push((Reserved::ARITHMETIC.as_str(), "**"));
        tokens.push((Reserved::ARITHMETIC.as_str(), "//"));

        let mut lexer = Lexer::new(CODE.chars().collect());
    }
}
