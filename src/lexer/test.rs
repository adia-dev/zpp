#[cfg(test)]
mod lexer_tests {

    use std::assert_eq;

    use crate::{
        enums::{arithmetic::Arithmetic, cmp::Cmp, keyword::Keyword, token_type::TokenType, logicop::LogicOp},
        lexer::Lexer,
        token::Token,
    };

    const INPUT: &'static str = "=+(){},;";
    const CODE: &'static str = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y
        }

        let result = add(five, ten);"#;

    #[test]
    fn test_next_token_in_sample() {
        let mut tokens: Vec<(TokenType, &str)> = Vec::new();
        tokens.push((TokenType::ASSIGN, "="));
        tokens.push((TokenType::ARITHMETIC(Arithmetic::PLUS), "+"));
        tokens.push((TokenType::LPAREN, "("));
        tokens.push((TokenType::RPAREN, ")"));
        tokens.push((TokenType::LBRACE, "{"));
        tokens.push((TokenType::RBRACE, "}"));
        tokens.push((TokenType::COMMA, ","));
        tokens.push((TokenType::SEMICOLON, ";"));

        let mut lexer = Lexer::new(INPUT.chars().collect());

        for (key, value) in tokens.into_iter() {
            let tok: Token = lexer.next_token();

            assert_eq!(tok.t, key);
            assert_eq!(tok.value, value);
        }
    }

    #[test]
    fn test_next_token_in_code() {
        let mut tokens: Vec<(TokenType, &str)> = Vec::new();

        tokens.push((TokenType::KEYWORD(Keyword::LET), "let"));
        tokens.push((TokenType::IDENT, "five"));
        tokens.push((TokenType::ASSIGN, "="));
        tokens.push((TokenType::INT, "5"));
        tokens.push((TokenType::SEMICOLON, ";"));

        tokens.push((TokenType::KEYWORD(Keyword::LET), "let"));
        tokens.push((TokenType::IDENT, "ten"));
        tokens.push((TokenType::ASSIGN, "="));
        tokens.push((TokenType::INT, "10"));
        tokens.push((TokenType::SEMICOLON, ";"));

        tokens.push((TokenType::KEYWORD(Keyword::LET), "let"));
        tokens.push((TokenType::IDENT, "add"));
        tokens.push((TokenType::ASSIGN, "="));
        tokens.push((TokenType::KEYWORD(Keyword::FUNCTION), "fn"));
        tokens.push((TokenType::LPAREN, "("));
        tokens.push((TokenType::IDENT, "x"));
        tokens.push((TokenType::COMMA, ","));
        tokens.push((TokenType::IDENT, "y"));
        tokens.push((TokenType::RPAREN, ")"));
        tokens.push((TokenType::LBRACE, "{"));
        tokens.push((TokenType::IDENT, "x"));
        tokens.push((TokenType::ARITHMETIC(Arithmetic::PLUS), "+"));
        tokens.push((TokenType::IDENT, "y"));
        tokens.push((TokenType::RBRACE, "}"));

        tokens.push((TokenType::KEYWORD(Keyword::LET), "let"));
        tokens.push((TokenType::IDENT, "result"));
        tokens.push((TokenType::ASSIGN, "="));
        tokens.push((TokenType::IDENT, "add"));
        tokens.push((TokenType::LPAREN, "("));
        tokens.push((TokenType::IDENT, "five"));
        tokens.push((TokenType::COMMA, ","));
        tokens.push((TokenType::IDENT, "ten"));
        tokens.push((TokenType::RPAREN, ")"));
        tokens.push((TokenType::SEMICOLON, ";"));
        tokens.push((TokenType::EOF, "\0"));

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

        let mut tokens: Vec<(TokenType, &str, usize)> = Vec::new();

        tokens.push((TokenType::KEYWORD(Keyword::CONST), "const", 2));
        tokens.push((TokenType::IDENT, "i", 2));
        tokens.push((TokenType::ASSIGN, "=", 2));
        tokens.push((TokenType::INT, "0", 2));
        tokens.push((TokenType::SEMICOLON, ";", 2));

        tokens.push((TokenType::KEYWORD(Keyword::LET), "let", 3));
        tokens.push((TokenType::IDENT, "j", 3));
        tokens.push((TokenType::ASSIGN, "=", 3));
        tokens.push((TokenType::INT, "0", 3));
        tokens.push((TokenType::SEMICOLON, ";", 3));

        tokens.push((TokenType::KEYWORD(Keyword::LET), "let", 5));
        tokens.push((TokenType::IDENT, "result", 5));
        tokens.push((TokenType::ASSIGN, "=", 5));
        tokens.push((TokenType::IDENT, "i", 5));
        tokens.push((TokenType::ARITHMETIC(Arithmetic::PLUS), "+", 5));
        tokens.push((TokenType::IDENT, "j", 5));
        tokens.push((TokenType::SEMICOLON, ";", 5));

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
        let mut tokens: Vec<(TokenType, &str)> = Vec::new();

        tokens.push((TokenType::KEYWORD(Keyword::IF), "if"));
        tokens.push((TokenType::KEYWORD(Keyword::TRUE), "true"));
        tokens.push((TokenType::LBRACE, "{"));
        tokens.push((TokenType::IDENT, "print"));
        tokens.push((TokenType::LPAREN, "("));
        tokens.push((TokenType::KEYWORD(Keyword::TRUE), "true"));
        tokens.push((TokenType::RPAREN, ")"));
        tokens.push((TokenType::SEMICOLON, ";"));
        tokens.push((TokenType::RBRACE, "}"));
        tokens.push((TokenType::KEYWORD(Keyword::ELSE), "else"));
        tokens.push((TokenType::LBRACE, "{"));
        tokens.push((TokenType::KEYWORD(Keyword::RETURN), "return"));
        tokens.push((TokenType::KEYWORD(Keyword::FALSE), "false"));
        tokens.push((TokenType::SEMICOLON, ";"));
        tokens.push((TokenType::RBRACE, "}"));

        tokens.push((TokenType::KEYWORD(Keyword::IF), "if"));
        tokens.push((TokenType::IDENT, "a"));
        tokens.push((TokenType::CMP(Cmp::GT), ">"));
        tokens.push((TokenType::IDENT, "b"));
        tokens.push((TokenType::LBRACE, "{"));
        tokens.push((TokenType::IDENT, "print"));
        tokens.push((TokenType::LPAREN, "("));
        tokens.push((TokenType::DQUOTE, "\""));
        tokens.push((TokenType::IDENT, "hot"));
        tokens.push((TokenType::DQUOTE, "\""));
        tokens.push((TokenType::RPAREN, ")"));
        tokens.push((TokenType::SEMICOLON, ";"));
        tokens.push((TokenType::RBRACE, "}"));
        tokens.push((TokenType::KEYWORD(Keyword::ELSE), "else"));
        tokens.push((TokenType::LBRACE, "{"));
        tokens.push((TokenType::IDENT, "print"));
        tokens.push((TokenType::LPAREN, "("));
        tokens.push((TokenType::DQUOTE, "\""));
        tokens.push((TokenType::IDENT, "cold"));
        tokens.push((TokenType::DQUOTE, "\""));
        tokens.push((TokenType::RPAREN, ")"));
        tokens.push((TokenType::SEMICOLON, ";"));
        tokens.push((TokenType::RBRACE, "}"));

        tokens.push((TokenType::KEYWORD(Keyword::IF), "if"));
        tokens.push((TokenType::KEYWORD(Keyword::TRUE), "true"));
        tokens.push((TokenType::KEYWORD(Keyword::DO), "do"));
        tokens.push((TokenType::COLON, ":"));
        tokens.push((TokenType::IDENT, "print"));
        tokens.push((TokenType::LPAREN, "("));
        tokens.push((TokenType::KEYWORD(Keyword::TRUE), "true"));
        tokens.push((TokenType::RPAREN, ")"));
        tokens.push((TokenType::COMMA, ","));
        tokens.push((TokenType::KEYWORD(Keyword::ELSE), "else"));
        tokens.push((TokenType::COLON, ":"));
        tokens.push((TokenType::IDENT, "print"));
        tokens.push((TokenType::LPAREN, "("));
        tokens.push((TokenType::KEYWORD(Keyword::FALSE), "false"));
        tokens.push((TokenType::RPAREN, ")"));

        tokens.push((TokenType::KEYWORD(Keyword::IF), "if"));
        tokens.push((TokenType::KEYWORD(Keyword::TRUE), "true"));
        tokens.push((TokenType::KEYWORD(Keyword::DO), "do"));
        tokens.push((TokenType::IDENT, "print"));
        tokens.push((TokenType::LPAREN, "("));
        tokens.push((TokenType::KEYWORD(Keyword::TRUE), "true"));
        tokens.push((TokenType::RPAREN, ")"));
        tokens.push((TokenType::SEMICOLON, ";"));
        tokens.push((TokenType::KEYWORD(Keyword::END), "end"));

        let mut lexer = Lexer::new(code.chars().collect());

        for (key, value) in tokens {
            let token = lexer.next_token();

            // println!("type: {:#?}, value: {:#?}", token.t, token.value);

            assert_eq!(token.t, key);
            assert_eq!(token.value, value);
        }
    }

    #[test]
    fn test_peek_next_token_in_code() {
        let code: &'static str = r#"
            ++--**//&&||....=::
        "#;
        let mut tokens: Vec<(TokenType, &str)> = Vec::new();

        tokens.push((TokenType::ARITHMETIC(Arithmetic::INC), "++"));
        tokens.push((TokenType::ARITHMETIC(Arithmetic::DEC), "--"));
        tokens.push((TokenType::ARITHMETIC(Arithmetic::POW), "**"));
        tokens.push((TokenType::ARITHMETIC(Arithmetic::FDIV), "//"));

        tokens.push((TokenType::LOGICOP(LogicOp::AND), "&&"));
        tokens.push((TokenType::LOGICOP(LogicOp::OR), "||"));

        tokens.push((TokenType::RANGE, ".."));
        tokens.push((TokenType::IRANGE, "..="));
        tokens.push((TokenType::SCOPE, "::"));

        let mut lexer = Lexer::new(code.chars().collect());

        for (key, value) in tokens {
            let token = lexer.next_token();

            println!("key: {:#?}, value: {:#?}", token.t, token.value);

            assert_eq!(token.t, key);
            assert_eq!(token.value, value);
        }
    }
}
