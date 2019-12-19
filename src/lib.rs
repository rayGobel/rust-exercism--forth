use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    commands: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            commands: HashMap::new()
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        for token in input.split_whitespace() {
            match token.to_lowercase().as_str() {
                ":" => {
                    // User defined word
                },
                ";" => { // ignored
                },
                "+" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }

                    self.sum_stack();
                },
                "-" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }

                    self.sub_stack();
                },
                "*" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }

                    self.mul_stack();
                },
                "/" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }

                    if self.stack.last().unwrap() == &0 {
                        return Err(Error::DivisionByZero);
                    }

                    self.div_stack();
                },
                "dup" => {
                    if self.stack.len() < 1 {
                        return Err(Error::StackUnderflow);
                    }

                    self.dup_stack();
                },
                "drop" => {
                    if self.stack.len() < 1 {
                        return Err(Error::StackUnderflow);
                    }

                    self.drop_stack();
                },
                "swap" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }

                    self.swap_stack();
                },
                "over" => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }

                    self.over_stack();
                },
                _ => {
                    let n = token.parse::<Value>().unwrap();
                    self.stack.push(n);
                }
            }
        }
        Ok(())
    }

    fn over_stack(&mut self) -> () {
        let one_before_last = self.stack[self.stack.len() - 2];
        self.stack.push(one_before_last);
    }

    fn swap_stack(&mut self) -> () {
        let (a, b) = (self.stack.len() - 2, self.stack.len() - 1);
        self.stack.swap(a,b);
    }

    fn drop_stack(&mut self) -> () {
        self.stack.pop();
    }

    fn dup_stack(&mut self) -> () {
        let dupl = self.stack[self.stack.len() - 1];
        self.stack.push(dupl);
    }

    fn sum_stack(&mut self) -> () {
        let sum = self.stack.iter().sum();
        self.stack = vec![sum];
    }

    fn sub_stack(&mut self) -> () {
        let mut sub: i32 = 0;
        for (i, n) in self.stack.iter().enumerate() {
            if i == 0 {
                sub = *n;
            } else {
                sub = sub - *n;
            }
        }
        self.stack = vec![sub];
    }

    fn mul_stack(&mut self) -> () {
        let mul = self.stack.iter().fold(1, |sum, x| sum * x);
        self.stack = vec![mul];
    }

    fn div_stack(&mut self) -> () {
        let mut div: i32 = 0;
        for (i, n) in self.stack.iter().enumerate() {
            if i == 0 {
                div = *n;
            } else {
                div = div / *n;
            }
        }
        self.stack = vec![div];
    }
}
