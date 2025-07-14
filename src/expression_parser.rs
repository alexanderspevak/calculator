pub mod reverse_polish_notation;
use std::fmt;

pub use reverse_polish_notation::ReversePolishNotation;

#[derive(PartialEq, Debug)]
pub enum ParsingError {
    ParenthesesNotMatching,
    InvalidInput,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParsingError::ParenthesesNotMatching => write!(
                f,
                "Parentheses must match. ) can not come before (. Count of ( must equal to )",
            ),
            ParsingError::InvalidInput => {
                write!(f, "Enter valid mathematical infix notation. Valid symbols are: + / - () and integer digits",)
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Operator {
    Add,
    Substract,
    Divide,
    Multiply,
    LeftParenthesis,
    RightParenthesis,
}

impl TryFrom<char> for Operator {
    type Error = ParsingError;
    fn try_from(value: char) -> Result<Self, ParsingError> {
        Ok(match value {
            '(' => Operator::LeftParenthesis,
            ')' => Operator::RightParenthesis,
            '+' => Operator::Add,
            '-' => Operator::Substract,
            '*' => Operator::Multiply,
            '/' => Operator::Divide,
            _ => return Err(ParsingError::InvalidInput),
        })
    }
}

impl Operator {
    fn precedence(&self) -> u8 {
        match self {
            Self::Add => 1,
            Self::Substract => 1,
            Self::Divide => 2,
            Self::Multiply => 2,
            Self::LeftParenthesis => 0,
            Self::RightParenthesis => 0,
        }
    }
}

#[derive(Debug)]
enum Token {
    Operand(i32),
    Operator(Operator),
}

impl From<i32> for Token {
    fn from(value: i32) -> Self {
        Token::Operand(value)
    }
}

impl From<Operator> for Token {
    fn from(value: Operator) -> Self {
        Token::Operator(value)
    }
}

pub trait Calculate {
    fn calculate(&self) -> f32;
}

// Functions bellow are performed on raw input, therefore they could be used with other parsers.

pub fn parse_number(value: &str) -> Result<i32, ParsingError> {
    let mut value = value;
    let mut multiplicator = 1;

    if value.contains('-') {
        value = &value[1..];
        multiplicator = -1;
    }

    if value.is_empty() {
        return Err(ParsingError::InvalidInput);
    }

    Ok(value
        .parse::<i32>()
        .map_err(|_| ParsingError::InvalidInput)?)
    .map(|value| value * multiplicator)
}

fn evaluate_parenthes_match(value: &str) -> Result<(), ParsingError> {
    let mut parentheses_sum = 0;
    for char in value.chars() {
        match char {
            '(' => parentheses_sum += 1,
            ')' => parentheses_sum -= 1,
            _ => continue,
        }
        if parentheses_sum < 0 {
            return Err(ParsingError::ParenthesesNotMatching);
        }
    }

    if parentheses_sum != 0 {
        return Err(ParsingError::ParenthesesNotMatching);
    }

    Ok(())
}

fn check_char_validity(char: char) -> Result<(), ParsingError> {
    let valid_chars_except_for_numbers = [')', '(', '+', '*', '-', '/'];
    if !char.is_ascii_digit() && !valid_chars_except_for_numbers.contains(&char) {
        return Err(ParsingError::InvalidInput);
    }
    Ok(())
}

/*
* TODO: there can be probably more edge cases
**/
fn validate_infix_notation(input: &str) -> Result<(), ParsingError> {
    if input.is_empty() {
        return Err(ParsingError::InvalidInput);
    }

    if input.ends_with('(')
        || input.ends_with('-')
        || input.ends_with('+')
        || input.ends_with('*')
        || input.ends_with('/')
    {
        return Err(ParsingError::InvalidInput);
    }

    if input.starts_with(')')
        || input.starts_with('+')
        || input.starts_with('*')
        || input.starts_with('/')
    {
        return Err(ParsingError::InvalidInput);
    }

    if input.len() == 1 && input.chars().last().unwrap().is_numeric() {
        return Err(ParsingError::InvalidInput);
    }

    evaluate_parenthes_match(input)?;

    let mut previous_char_option: Option<char> = None;
    let chars_which_come_after_digit_or_closing_parenthesis = [')', '+', '*', '-', '/'];

    for current_char in input.chars() {
        check_char_validity(current_char)?;
        let previous_char = if let Some(previous_char) = previous_char_option {
            previous_char
        } else {
            continue;
        };

        if (!previous_char.is_ascii_digit() || previous_char != ')')
            && chars_which_come_after_digit_or_closing_parenthesis.contains(&current_char)
        {
            return Err(ParsingError::InvalidInput);
        };

        if current_char.is_numeric() && previous_char == ')' {
            return Err(ParsingError::InvalidInput);
        };

        if current_char == '('
            && (previous_char == ')'
                || previous_char == '+'
                || previous_char == '-'
                || previous_char == '*'
                || previous_char == '/')
        {
            return Err(ParsingError::InvalidInput);
        }

        if !previous_char.is_numeric() && previous_char != ')' {
            return Err(ParsingError::InvalidInput);
        }

        previous_char_option = Some(current_char);
    }

    Ok(())
}

pub fn check_operator_char_order(
    current_char: char,
    previous_char: Option<char>,
) -> Result<(), ParsingError> {
    if previous_char.is_none() && !current_char.is_numeric() && current_char != '(' {
        println!("returns 1");
        return Err(ParsingError::InvalidInput);
    }

    if current_char == '(' {
        if previous_char
            .is_some_and(|previous_char| previous_char == ')' || previous_char.is_numeric())
        {
            println!("returns 1.1");
            return Err(ParsingError::InvalidInput);
        } else {
            return Ok(());
        }
    }

    if let Some(previous_char) = previous_char {
        if !previous_char.is_numeric() && previous_char != ')' {
            println!("returns 2");
            return Err(ParsingError::InvalidInput);
        }
        Ok(())
    } else {
        println!("returns 3");
        Err(ParsingError::InvalidInput)
    }
}

pub fn is_minus_unary_operator(current_char: char, previous_char: &Option<char>) -> bool {
    if current_char != '-' {
        return false;
    }

    let previous_char = if let Some(previous_char) = previous_char {
        previous_char
    } else {
        return true;
    };

    if previous_char.is_numeric() || previous_char == &')' {
        return false;
    }

    true
}
