pub enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
}

impl Operator {

  pub fn from(string: &str) -> Result<Self,&'static str> {
    match string {
      "+" => Ok(Operator::Plus),
      "-" => Ok(Operator::Minus),
      "x" => Ok(Operator::Multiply),
      "/" => Ok(Operator::Divide),
      _ => Err("Not a valid string"),
    }
  }


  pub fn operate(&self, l: i32, r: i32) -> i32{
    match self {
      Operator::Plus => l + r,
      Operator::Minus => l - r,
      Operator::Multiply => l * r,
      Operator::Divide => l / r,
    }
  }
}

pub enum Token {
  Operator(Operator),
  Operand(i32),
}

impl Token {

  pub fn from(string: String) -> Self {
    match  Operator::from(&string) {
      Ok(op) => Token::Operator(op),
      Err(_) => match string.parse::<i32>() {
          Ok(i) => Token::Operand(i),
          _ => panic!("Invalid argument: {}", string),
        }
    }
  }
}

pub struct Expression {
  pub tokens: Vec<Token>,
}

impl Expression {
  pub fn new(args: std::env::Args) -> Result<Expression, &'static str> {
    let tokens = args
      .skip(1)
      .map( Token::from )
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
              stk.push(Token::Operand(op.operate(l,r)))
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
