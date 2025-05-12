use crate::utils::ui;

fn _try_parse_float(value: &str) -> f64 {
    match value.parse::<f64>() {
        Ok(v) => v,
        Err(_) => 0.0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Number(f64),
    Op(char),
    Eof,
}

#[derive(Debug)]
struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut tokens: Vec<Token> = Vec::new();
        let mut number: String = String::new();

        for c in input.chars().filter(|it| !it.is_ascii_whitespace()) {
            if c.is_digit(10) || c == '.' {
                number.push(c);
            } else {
                if !number.is_empty() {
                    tokens.push(Token::Number(_try_parse_float(&number)));
                    number.clear();
                }
                tokens.push(Token::Op(c));
            }
        }
        if !number.is_empty() {
            tokens.push(Token::Number(_try_parse_float(&number)));
        }

        tokens.reverse();
        Lexer { tokens }
    }
    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    fn peek(&mut self) -> Token {
        self.tokens.last().copied().unwrap_or(Token::Eof)
    }
}

#[derive(Clone)]
pub enum Expression {
    Number(f64),
    Operation(char, Vec<Expression>),
}
impl Expression {
    pub fn from_str(input: &str) -> Expression {
        let mut lexer = Lexer::new(input);
        parse_expression(&mut lexer, 0.0)
    }

    #[allow(unused)]
    pub fn eval(&self) -> f64 {
        match self {
            Expression::Number(c) => *c,
            Expression::Operation(operator, operands) => {
                let lhs = operands.first().unwrap().eval();
                let rhs = operands.last().unwrap().eval();
                match operator {
                    '+' => return lhs + rhs,
                    '-' => return lhs - rhs,
                    '*' => return lhs * rhs,
                    '/' => return lhs / rhs,
                    '^' => return lhs.powf(rhs),
                    '√' => return lhs.powf(1.0 / (rhs)),
                    op => panic!("Bad operator: {}", op),
                }
            }
        }
    }
}

fn parse_expression(lexer: &mut Lexer, min_bp: f32) -> Expression {
    let mut lhs = match lexer.next() {
        Token::Number(it) => Expression::Number(it),
        Token::Op('(') => {
            let lhs = parse_expression(lexer, 0.0);
            assert_eq!(lexer.next(), Token::Op(')'));
            lhs
        }
        t => panic!("bad token: {:?}", t),
    };
    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(')') => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };
        let (l_bp, r_bp) = infix_binding_power(op);

        if l_bp < min_bp {
            break;
        }
        lexer.next();
        let rhs = parse_expression(lexer, r_bp);
        lhs = Expression::Operation(op, vec![lhs, rhs]);
    }
    lhs
}

fn infix_binding_power(op: char) -> (f32, f32) {
    match op {
        '+' | '-' => (1.0, 1.1),
        '*' | '/' => (2.0, 2.1),
        '^' | '√' => (3.1, 3.0),
        '.' => (4.0, 4.1),
        _ => panic!("bad op: {:?}", op),
    }
}

pub fn calculator() {
    println!("Calculator v2");
    println!("enter 'exit' to quit");
    println!("############");
    loop {
        let input: String = ui::get_input("");
        if input == "exit" {
            break;
        }
        let expr: Expression = Expression::from_str(&input);
        let value = expr.eval();
        println!("{}", value);
    }
}
