#![feature(bool_to_option)]

use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;
pub type StackOp = dyn Fn(&mut Vec<Value>) -> ForthResult + Sync;
pub type BinaryOp = dyn Fn(Value, Value) -> Result<Value, Error> + Sync;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

enum Operation {
    Stack(Box<StackOp>),
    Binary(Box<BinaryOp>),
}

impl From<Box<StackOp>> for Operation {
    fn from(f: Box<StackOp>) -> Self {
        Self::Stack(f)
    }
}

impl From<Box<BinaryOp>> for Operation {
    fn from(f: Box<BinaryOp>) -> Self {
        Self::Binary(f)
    }
}

lazy_static! {
    static ref BASE_OPERATIONS: HashMap<&'static str, Operation> = hashmap! {
        "+"    => (Box::new(|a, b| Ok(a + b)) as Box<BinaryOp>).into(),
        "-"    => (Box::new(|a, b| Ok(a - b)) as Box<BinaryOp>).into(),
        "*"    => (Box::new(|a, b| Ok(a * b)) as Box<BinaryOp>).into(),
        "/"    => (Box::new(|a, b| <i32>::checked_div(a, b).ok_or(Error::DivisionByZero)) as Box<BinaryOp>).into(),
        "dup"  => (Box::new(|stack: &mut Vec<Value>| {
            if let Some(a) = stack.last().copied() {
                stack.push(a);
                Ok(())
            } else {
                Err(Error::StackUnderflow)
            }
        }) as Box<StackOp>).into(),
        "drop" => (Box::new(|stack: &mut Vec<Value>| {
            stack.pop().map(|_| ()).ok_or(Error::StackUnderflow)
        }) as Box<StackOp>).into(),
        "swap" => (Box::new(|stack: &mut Vec<Value>| {
            if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                stack.push(b);
                stack.push(a);
                Ok(())
            } else {
                Err(Error::StackUnderflow)
            }
        }) as Box<StackOp>).into(),
        "over" => (Box::new(|stack: &mut Vec<Value>| {
            if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                stack.push(a);
                stack.push(b);
                stack.push(a);
                Ok(())
            } else {
                Err(Error::StackUnderflow)
            }
        }) as Box<StackOp>).into(),
    };
}

#[derive(Default)]
pub struct Forth {
    stack: Option<Vec<Value>>,
    words: HashMap<String, Vec<String>>,
}

impl Forth {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn stack(&mut self) -> Vec<Value> {
        let Self { stack, .. } = self;
        stack.take().unwrap_or_default()
    }

    fn eval_token(
        &mut self,
        token: String,
        tokens: &mut Vec<String>,
        stack: &mut Vec<Value>,
    ) -> ForthResult {
        if let Ok(n) = token.parse() {
            // Parse number
            stack.push(n);
        } else if let Some(s) = self.words.get(token.as_str()) {
            // Evaluate defined word
            tokens.append(&mut s.clone());
        } else if let Some(op) = BASE_OPERATIONS.get(token.as_str()) {
            // Evaluate base operation
            match op {
                Operation::Stack(f) => {
                    (f)(stack)?;
                }
                Operation::Binary(f) => {
                    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                        stack.push((f)(a, b)?);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
            }
        } else if token == ":" {
            // Add defined word
            if let Some(i) = tokens
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, t)| (t == ";").then_some(i))
            {
                let mut word_tokens = tokens.split_off(i + 1);
                tokens.pop();

                if let Some(word) = word_tokens.pop() {
                    if word.parse::<i32>().is_err() {
                        let mut new_word_tokens = vec![];

                        for t in word_tokens.drain(0..) {
                            if let Some(wt) = self.words.get(&t) {
                                new_word_tokens.append(&mut wt.clone());
                            } else {
                                new_word_tokens.push(t);
                            }
                        }

                        self.words.insert(word, new_word_tokens);
                        return Ok(());
                    }
                }
            }
            return Err(Error::InvalidWord);
        } else {
            return Err(Error::UnknownWord);
        }

        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut stack = Vec::new();
        let mut tokens: Vec<String> = input
            .split_whitespace()
            .rev()
            .map(<str>::to_lowercase)
            .collect();

        while let Some(token) = tokens.pop() {
            self.eval_token(token, &mut tokens, &mut stack)?;
        }

        self.stack = Some(stack);

        Ok(())
    }
}
