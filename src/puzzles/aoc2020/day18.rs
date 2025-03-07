/*
** src/puzzles/aoc2020/day18.rs
*/

use super::Solution;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Number(u64),
    OpAdd,
    OpMul,
    LParen,
    RParen,
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '+' => Self::OpAdd,
            '*' => Self::OpMul,
            '(' => Self::LParen,
            ')' => Self::RParen,
            _ => Self::Number(c.to_digit(10).unwrap() as u64),
        }
    }
}

struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    fn parse_token_stream(s: &str) -> Vec<Token> {
        s.chars().filter(|&c| c != ' ').map(Token::from).collect()
    }

    fn into_rpn(tokens: Vec<Token>, add_prec: u8, mul_prec: u8) -> Vec<Token> {
        // an implementation of the shunting-yard algorithm
        // converts the token stream into reverse-Polish notation
        let mut output = Vec::with_capacity(tokens.len());
        let mut op_stack = Vec::with_capacity(tokens.len());

        let op_prec = |op| match op {
            Token::OpAdd => add_prec,
            Token::OpMul => mul_prec,
            _ => panic!("invalid operator {:?}", op),
        };

        for token in tokens.into_iter() {
            match token {
                // push the number to the output queue
                Token::Number(_) => output.push(token),
                // while the top of the operator stack is not a left parenthesis
                // and has a greater precedence than the current operator, pop
                // from the operator stack onto the output queue; then push the
                // operator to the operator stack
                Token::OpAdd | Token::OpMul => {
                    while op_stack.last().is_some()
                        && *op_stack.last().unwrap() != Token::LParen
                        && op_prec(*op_stack.last().unwrap()) >= op_prec(token)
                    {
                        output.push(op_stack.pop().unwrap());
                    }
                    op_stack.push(token);
                }
                // push the lef parenthesis onto the operator stack
                Token::LParen => op_stack.push(token),
                Token::RParen => {
                    // pop operators onto the output queue while the top of the
                    // operator stack is not a left parenthesis
                    while *op_stack.last().unwrap() != Token::LParen {
                        output.push(op_stack.pop().unwrap());
                    }
                    // pop the left parenthesis
                    op_stack.pop();
                }
            }
        }

        // pop remaining operators onto the output queue
        while let Some(operator) = op_stack.pop() {
            output.push(operator);
        }

        output
    }

    fn parse(s: &str, add_prec: u8, mul_prec: u8) -> Self {
        let tokens = Self::parse_token_stream(s);
        Self {
            tokens: Self::into_rpn(tokens, add_prec, mul_prec),
        }
    }

    fn evaluate(&self) -> u64 {
        let mut operand_stack = Vec::with_capacity(self.tokens.len());

        for token in self.tokens.iter() {
            match token {
                // add operands to the operand stack
                Token::Number(x) => operand_stack.push(*x),
                // pop operands and evaluate
                Token::OpAdd => {
                    let op_a = operand_stack.pop().unwrap();
                    let op_b = operand_stack.pop().unwrap();
                    operand_stack.push(op_a + op_b);
                }
                // pop operands and evaluate
                Token::OpMul => {
                    let op_a = operand_stack.pop().unwrap();
                    let op_b = operand_stack.pop().unwrap();
                    operand_stack.push(op_a * op_b);
                }
                _ => panic!("invalid token {:?}", token),
            }
        }

        operand_stack.pop().unwrap()
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    let lines = input.split('\n').collect::<Vec<_>>();

    // Part A: Evaluate the expression on each line of the homework; what is the sum of the
    // resulting values?
    let sum = lines
        .iter()
        .map(|line| Expression::parse(line, 1, 1))
        .map(|expr| expr.evaluate())
        .sum::<u64>();
    solution.set_part_a(sum);

    // Part B: What do you get if you add up the results of evaluating the homework problems when
    // addition has higher precedence than multiplication?
    let sum = lines
        .iter()
        .map(|line| Expression::parse(line, 2, 1))
        .map(|expr| expr.evaluate())
        .sum::<u64>();
    solution.set_part_b(sum);

    solution
}
