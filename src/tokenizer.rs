const DIGIT_RADIX: u32 = 10;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenParens,
    CloseParens,
    Number(String),
    Identifier(String),
}

fn tokenize(src: &str) -> Vec<Token> {
    let mut src = src;
    let mut tokens = Vec::new();
    loop {
        if src.is_empty() {
            break;
        }
        let chr = src.chars().nth(0).unwrap();
        match chr {
            ' ' => {
                src = &src[1..];
                continue;
            },
            '(' => {
                src = consume_input(&src, &tokenize_open_parens, &mut tokens);
            },
            ')' => {
                src = consume_input(&src, &tokenize_close_parens, &mut tokens);
            },
            ch if is_ident_first(ch) => {
                src = consume_input(&src, &tokenize_ident, &mut tokens);
            },
            ch if is_digit(ch) => {
                src = consume_input(&src, &tokenize_number, &mut tokens);
            },
            _ => break,
        }
    }
    tokens
}

fn consume_input<'a>(
    src: &'a str,
    tokenize: &dyn Fn(&'a str) -> (Token, &'a str),
    tokens: &mut Vec<Token>) -> &'a str {
    
    let (token, rest_src) = tokenize(&src);
    tokens.push(token);
    rest_src 
}

fn tokenize_open_parens(src: &str) -> (Token, &str) {
    (Token::OpenParens, &src[1..])
}

fn tokenize_close_parens(src: &str) -> (Token, &str) {
    (Token::CloseParens, &src[1..])
}

fn tokenize_ident(src: &str) -> (Token, &str) {
    let mut index = 0;
    let mut ident = String::new();
    for ch in src.chars() {
        if !is_ident(ch) {
            break;
        }
        ident.push(ch);
        index += 1;
    }
    (Token::Identifier(ident), &src[index..])
}

fn is_ident_first(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '+' || c == '-' || c == '*' || c == '/'
}

fn is_ident(c: char) -> bool {
    is_ident_first(c) || is_digit(c)
}

fn tokenize_number(src: &str) -> (Token, &str) {
    let mut index = 0;
    let mut num = String::new();
    for ch in src.chars() {
        if !is_digit(ch) {
            break;
        }
        num.push(ch);
        index += 1;
    }
    (Token::Number(num), &src[index..])
}

fn is_digit(ch: char) -> bool {
    ch.is_digit(DIGIT_RADIX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digit() {
        let input = "0";
        let expected = vec![
            Token::Number("0".to_string()),
        ];
        let actual = tokenize(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn indent() {
        let input = "an-identifier-123";
        let expected = vec![
            Token::Identifier("an-identifier-123".to_string()),
        ];
        let actual = tokenize(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn empty_sexpr() {
        let input = "()";
        let expected = vec![
            Token::OpenParens,
            Token::CloseParens,
        ];
        let actual = tokenize(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn simple_sexpr() {
        let input = "(+ 1 1)";
        let expected = vec![
            Token::OpenParens,
            Token::Identifier("+".to_string()),
            Token::Number("1".to_string()),
            Token::Number("1".to_string()),
            Token::CloseParens,
        ];
        let actual = tokenize(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn generic_sexpr() {
        let input = "(operation operand0 0987654321 operand1)";
        let expected = vec![
            Token::OpenParens,
            Token::Identifier("operation".to_string()),
            Token::Identifier("operand0".to_string()),
            Token::Number("0987654321".to_string()),
            Token::Identifier("operand1".to_string()),
            Token::CloseParens,
        ];
        let actual = tokenize(input);
        assert_eq!(expected, actual);
    }
}
