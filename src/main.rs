/// arithmetic parser
/// not check wrong grammar, assert grammar is right

#[derive(Debug)]
pub enum Value {
    Literal(String),
    Expression(Box<Expression>)
}

#[derive(Debug)]
pub enum Expression {
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
}

impl Value {
    pub fn value(&self) -> f32 {
        match self {
            Value::Literal(num) => num.parse().unwrap(),
            Value::Expression(ex) => ex.value()
        }
    }
}

impl Expression {
    pub fn value(&self) -> f32 {
        match self {
            Expression::Add(l, r) => l.value() + r.value(),
            Expression::Sub(l, r) => l.value() - r.value(),
            Expression::Mul(l, r) => l.value() * r.value(),
            Expression::Div(l, r) => l.value() / r.value(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token{
    Number(String),
    LeftParen,
    RightParen,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
}

fn parse(input: &str) -> Vec<Token>{
    let mut tokens: Vec<Token> = vec![];
    let bytes = input.as_bytes();
    let mut pos = 0usize;
    
    let mut last_token : Option<Token> = None;
    loop {
        if pos >= bytes.len() {
            break;
        }

        let token = match bytes[pos] {
            b' ' => { pos += 1; continue;},
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'+' => {
                Token::OpAdd
            },
            b'-' => {
                match last_token {
                    Some(Token::Number(_)) => {
                        Token::OpSub
                    },
                    _ => {
                        pos += 1; 
                        Token::Number(String::from("-") + &parse_num(&bytes, &mut pos))
                    }
                }
            },
            b'*' => Token::OpMul,
            b'/' => Token::OpDiv,
            b'0'..=b'9' => {
                Token::Number(parse_num(&bytes, &mut pos))
            },
            _ => panic!("unkown character {}", bytes[pos] as char)
        };

        last_token = Some(token.clone());

        tokens.push(token);

        pos += 1;
    }

    tokens
}

fn parse_num(bytes: &[u8], pos: &mut usize) -> String {
    let mut num = String::new();
    loop {
        if *pos >= bytes.len() {
            break;
        }
        match bytes[*pos] {
            n @ b'0'..=b'9' | n @ b'.' => num.push(n as char),
            _ => break
        };
        *pos += 1;
    }

    *pos -= 1;
    num
}

fn reduce(stack: &mut Vec<Value>, opstack: &mut Vec<Token>) {
    while let Some(op) = opstack.pop() {
        let rv = stack.pop().unwrap();
        let lv = stack.pop().unwrap();
        match op {
            Token::OpAdd => stack.push(Value::Expression(Box::new(Expression::Add(lv, rv)))),
            Token::OpSub => stack.push(Value::Expression(Box::new(Expression::Sub(lv, rv)))),
            Token::OpMul => stack.push(Value::Expression(Box::new(Expression::Mul(lv, rv)))),
            Token::OpDiv => stack.push(Value::Expression(Box::new(Expression::Div(lv, rv)))),
            _ => unreachable!()
        }
    }
}

pub fn eval_expression(tokens: &[Token], pos: &mut usize) -> Value {
    let mut stack : Vec<Value> = Vec::new();
    let mut opstack: Vec<Token> = Vec::new();
    
    loop {
        if *pos >= tokens.len() {
            break;
        }
        match tokens[*pos] {
            Token::Number(ref num) => stack.push(Value::Literal(num.clone())),
            ref op @ Token::OpAdd | ref op @ Token::OpSub => {
                reduce(&mut stack, &mut opstack);

                opstack.push(op.clone());
            },
            ref op @ Token::OpMul | ref op @ Token::OpDiv  => {
                match opstack.last() {
                    Some(Token::OpMul) | Some(Token::OpDiv) => {
                        reduce(&mut stack, &mut opstack);
                    }
                    _ => ()
                }
                opstack.push(op.clone())
            },
            Token::LeftParen => {
                *pos += 1;
                stack.push(eval_expression(tokens, pos));
                //println!("{:?}", stack);
            },
            Token::RightParen => {
                //*pos += 1;
                break;
            }
        };
        
        *pos += 1;
    }

    reduce(&mut stack, &mut opstack);

    stack.pop().unwrap()
}

pub fn eval_value(input: &str) -> f32 {
    let tokens = parse(input);
    // println!("{:?}", tokens);

    let mut pos = 0usize;
    let value = eval_expression(&tokens, &mut pos);

    //println!("{:?}", value);
    value.value()
}

fn main() {
    println!("{}", 5 * -3);
}

#[cfg(test)]
mod test{
    use super::eval_value;
    #[test]
    fn test_number(){
        assert_eq!(eval_value("3"), 3f32);
        assert_eq!(eval_value("5"), 5f32);
        assert_eq!(eval_value("-5"), -5f32);
    }

    #[test]
    fn test_add()
    {        
        assert_eq!(eval_value("3 + 4"), 7f32);
        assert_eq!(eval_value("1 + 0"), 1f32);
        assert_eq!(eval_value("-1 + 0"), -1f32);
        assert_eq!(eval_value("1 + 3 + 4"), 8f32);
        assert_eq!(eval_value("333 + 222"), 555f32);
    }

    #[test]
    fn test_sub()
    {        
        assert_eq!(eval_value("3 - 2"), 1f32);
        assert_eq!(eval_value("13 - 21 - 12"), -20f32);
        assert_eq!(eval_value("333 - 21"), 312f32);
    }

    #[test]
    fn test_mul()
    {        
        assert_eq!(eval_value("3 * 5"), 15f32);
        assert_eq!(eval_value("3 * 5 * 4"), 60f32);
    }

    #[test]
    fn test_div()
    {        
        assert_eq!(eval_value("3 / 2"), 1.5f32);
    }

    #[test]
    fn test_together()
    {        
        assert_eq!(eval_value("3 * 4 + 5 - 2"), 15f32);
    }

    #[test]
    fn test_paren()
    {        
        assert_eq!(eval_value("(1 + 2) * 4 / 2"), 6f32);
        assert_eq!(eval_value("12 / 6 * 3 + 2 * (111 - 11)"), 206f32);
    }

    #[test]
    fn test_negative()
    {
        assert_eq!(eval_value("(1 + -2) * 4 / 2"), -2f32);
        assert_eq!(eval_value("(1 + -2) * 4 / -2"), 2f32);
    }

    #[test]
    fn test_float()
    {
        assert_eq!(eval_value("1 - 2.05"), -1.05f32);
        assert_eq!(eval_value("0.86 * 2"), 1.72f32);
    }
}
