pub enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
}

pub enum Token {
  Operator(Operator),
  Operand(i32),
}

pub struct Expression {
  pub tokens: Vec<Token>,
}

impl Expression {
  pub fn new(args: std::env::Args) -> Result<Expression, &'static str> {
    let tokens = args
      .skip(1)
      .map(|it| match &*it {
        "+" => Token::Operator(Operator::Plus),
        "-" => Token::Operator(Operator::Minus),
        "x" => Token::Operator(Operator::Multiply),
        "/" => Token::Operator(Operator::Divide),
        s => match s.parse::<i32>() {
          Ok(i) => Token::Operand(i),
          _ => panic!("Invalid argument: {}", s),
        },
      })
      .collect();

    Ok(Expression { tokens: tokens })
  }

  pub fn evaluate(&self) -> Result<i32, &'static str> {
    let mut stk: Vec<Token> = Vec::new();
    for t in self.tokens.iter() {
      match t {
        Token::Operator(op) => {
          if let Some(Token::Operand(r)) = stk.pop() {
            if let Some(Token::Operand(l)) = stk.pop() {
              match op {
                Operator::Plus => stk.push(Token::Operand(l + r)),
                Operator::Minus => stk.push(Token::Operand(l - r)),
                Operator::Multiply => stk.push(Token::Operand(l * r)),
                Operator::Divide => stk.push(Token::Operand(l / r)),
              }
            }
          }
        }
        Token::Operand(i) => stk.push(Token::Operand(*i)),
      }
    }
    if stk.len() > 1 {
      Err("missing operators")
    } else if let Some(Token::Operand(sum)) = stk.pop() {
      Ok(sum)
    } else {
      Err("too many operators")
    }
  }
}
