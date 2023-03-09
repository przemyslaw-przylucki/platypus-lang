use super::*;

#[test]
fn handles_one_char_tokens() {
    let source = "{(( ))}";
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens().unwrap();

    assert_eq!(scanner.tokens.len(), 7);
    println!("{:?}", scanner.tokens);
    assert_eq!(scanner.tokens[0].token_type, TokenType::LeftBrace);
    assert_eq!(scanner.tokens[1].token_type, TokenType::LeftParen);
    assert_eq!(scanner.tokens[2].token_type, TokenType::LeftParen);
    assert_eq!(scanner.tokens[3].token_type, TokenType::RightParen);
    assert_eq!(scanner.tokens[4].token_type, TokenType::RightParen);
    assert_eq!(scanner.tokens[5].token_type, TokenType::RightBrace);
    assert_eq!(scanner.tokens[6].token_type, TokenType::Eof);
}

#[test]
fn handles_operators() {
    let source = "! != == >=";
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens().unwrap();

    assert_eq!(scanner.tokens.len(), 5);
    assert_eq!(scanner.tokens[0].token_type, TokenType::Bang);
    assert_eq!(scanner.tokens[1].token_type, TokenType::BangEqual);
    assert_eq!(scanner.tokens[2].token_type, TokenType::EqualEqual);
    assert_eq!(scanner.tokens[3].token_type, TokenType::GreaterEqual);
    assert_eq!(scanner.tokens[4].token_type, TokenType::Eof);
}

#[test]
fn handles_number_literals() {
    let source = "420 69 420.69";
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens().unwrap();

    assert_eq!(scanner.tokens.len(), 4);


    assert_eq!(scanner.tokens[0].token_type, TokenType::Number);
    // assert_eq!(scanner.tokens[0].literal, LiteralValue::FloatValue);
    assert_eq!(scanner.tokens[1].token_type, TokenType::Number);
    // assert_eq!(scanner.tokens[1].literal, LiteralValue::FloatValue);
    assert_eq!(scanner.tokens[2].token_type, TokenType::Number);
    // assert_eq!(scanner.tokens[2].literal, LiteralValue::FloatValue);

    assert_eq!(scanner.tokens[3].token_type, TokenType::Eof);
}

#[test]
fn handles_string_literals() {
    let source = r#""platypus""#;
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens().unwrap();

    assert_eq!(scanner.tokens.len(), 2);

    assert_eq!(scanner.tokens[0].token_type, TokenType::String);

    match scanner.tokens[0].literal.as_ref().unwrap() {
        StringValue(val) => assert_eq!(val, "platypus"),
        _ => panic!("Incorrect literal"),
    }

    assert_eq!(scanner.tokens[1].token_type, TokenType::Eof);
}
#[test]
fn handles_string_multiline() {
    let source = "\"platypus\n is \n awesome\"";
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens().unwrap();

    assert_eq!(scanner.tokens.len(), 2);

    assert_eq!(scanner.tokens[0].token_type, TokenType::String);

    match scanner.tokens[0].literal.as_ref().unwrap() {
        StringValue(val) => assert_eq!(val, "platypus\n is \n awesome"),
        _ => panic!("Incorrect literal"),
    }

    assert_eq!(scanner.tokens[1].token_type, TokenType::Eof);
}

#[test]
fn handles_string_literals_unterminated() {
    let source = r#""platypus"#;
    let mut scanner = Scanner::new(source);
    let result = scanner.scan_tokens();

    match result {
        Err(_) => (),
        _ => panic!("Test didn't fail but it should"),
    }
}

#[test]
fn handles_identifiers() {
    let source = "foo_asd = \"bar\";";
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens().unwrap();

    assert_eq!(scanner.tokens[0].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[1].token_type, TokenType::Equal);
    assert_eq!(scanner.tokens[2].token_type, TokenType::String);
    assert_eq!(scanner.tokens[3].token_type, TokenType::Semicolon);
    assert_eq!(scanner.tokens[4].token_type, TokenType::Eof);

    assert_eq!(scanner.tokens.len(), 5);
}

#[test]
fn handles_keywords() {
    let source = "\
        let foo = 5;\
        while foo == 5 {\
            print \"haha\";\
        }\
        ";

    let mut scanner = Scanner::new(source);
    scanner.scan_tokens().unwrap();

    assert_eq!(scanner.tokens[0].token_type, TokenType::Let);
    assert_eq!(scanner.tokens[1].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[2].token_type, TokenType::Equal);
    assert_eq!(scanner.tokens[3].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[4].token_type, TokenType::Semicolon);
    assert_eq!(scanner.tokens[5].token_type, TokenType::While);
    assert_eq!(scanner.tokens[6].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[7].token_type, TokenType::EqualEqual);
    assert_eq!(scanner.tokens[8].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[9].token_type, TokenType::LeftBrace);
    assert_eq!(scanner.tokens[10].token_type, TokenType::Print);
    assert_eq!(scanner.tokens[11].token_type, TokenType::String);
    assert_eq!(scanner.tokens[12].token_type, TokenType::Semicolon);
    assert_eq!(scanner.tokens[13].token_type, TokenType::RightBrace);
    assert_eq!(scanner.tokens[14].token_type, TokenType::Eof);
}

#[test]
fn handles_single_line_comments() {
    let source = "\n
        let foo = 5;\n
        // This is a comment \n
        let bar = 6;\n
        ";

    let mut scanner = Scanner::new(source);
    scanner.scan_tokens().unwrap();

    scanner.debug();

    assert_eq!(scanner.tokens[0].token_type, TokenType::Let);
    assert_eq!(scanner.tokens[1].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[2].token_type, TokenType::Equal);
    assert_eq!(scanner.tokens[3].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[4].token_type, TokenType::Semicolon);
    assert_eq!(scanner.tokens[5].token_type, TokenType::Let);
    assert_eq!(scanner.tokens[6].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[7].token_type, TokenType::Equal);
    assert_eq!(scanner.tokens[8].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[9].token_type, TokenType::Semicolon);
    assert_eq!(scanner.tokens[10].token_type, TokenType::Eof);
}

#[test]
fn handles_multi_line_comments() {
    let source = "\
        let foo = 5;
        /*

        This is a multi line comment\n

        let xd = \"asdasd\";\n

        this is a random comment

        */
        let bar = 6;
        ";

    let mut scanner = Scanner::new(source);
    scanner.scan_tokens().unwrap();

    assert_eq!(scanner.tokens[0].token_type, TokenType::Let);
    assert_eq!(scanner.tokens[1].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[2].token_type, TokenType::Equal);
    assert_eq!(scanner.tokens[3].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[4].token_type, TokenType::Semicolon);
    assert_eq!(scanner.tokens[5].token_type, TokenType::Let);
    assert_eq!(scanner.tokens[6].token_type, TokenType::Identifier);
    assert_eq!(scanner.tokens[7].token_type, TokenType::Equal);
    assert_eq!(scanner.tokens[8].token_type, TokenType::Number);
    assert_eq!(scanner.tokens[9].token_type, TokenType::Semicolon);
    assert_eq!(scanner.tokens[10].token_type, TokenType::Eof);
}