use crate::expression_parser::Calculate;

use super::{
    check_operator_char_order, is_minus_unary_operator, parse_number, validate_infix_notation,
    Operator, ParsingError, Token,
};

static INVALID_RPN_PANIC_ERROR: &str = "Invalid RPN";

fn push_number_to_rpn_container(
    rpn_container: &mut Vec<Token>,
    parsing_number: &mut String,
) -> Result<(), ParsingError> {
    if parsing_number.is_empty() {
        return Ok(());
    }

    rpn_container.push(Token::from(parse_number(parsing_number.as_str())?));

    parsing_number.clear();
    Ok(())
}

#[derive(Default, Debug)]
pub struct ReversePolishNotation {
    tokens: Vec<Token>,
}

/**
 * Implementation of shunting yard algorithm:
 * https://mathcenter.oxford.emory.edu/site/cs171/shuntingYardAlgorithm/
 */
impl TryFrom<String> for ReversePolishNotation {
    type Error = ParsingError;

    fn try_from(input: String) -> Result<Self, ParsingError> {
        let input: String = input.split_whitespace().collect();
        validate_infix_notation(&input)?;
        let mut parsing_number = String::new();
        let mut previous_char = None;
        let mut operator_stack: Vec<Operator> = Vec::new();
        let mut rpn_container: Vec<Token> = Vec::new();
        let mut is_next_expression_negative = false;

        for current_char in input.chars() {
            if is_minus_unary_operator(current_char, &previous_char) {
                is_next_expression_negative = true;
                continue;
            };

            if current_char.is_numeric() {
                if is_next_expression_negative {
                    parsing_number.push('-');
                    is_next_expression_negative = false;
                }

                parsing_number.push(current_char);
                previous_char = Some(current_char);
                continue;
            }

            push_number_to_rpn_container(&mut rpn_container, &mut parsing_number)?;
            check_operator_char_order(current_char, previous_char)?;
            previous_char = Some(current_char);

            let current_operator = Operator::try_from(current_char)?;
            let last_stack_operator = operator_stack.last();

            if current_operator == Operator::LeftParenthesis {
                operator_stack.push(current_operator);
                continue;
            }

            let last_stack_operator = if let Some(last_stack_operator) = last_stack_operator {
                last_stack_operator
            } else {
                operator_stack.push(current_operator);
                continue;
            };

            if current_operator == Operator::RightParenthesis {
                while let Some(last_stack_operator) = operator_stack.pop() {
                    if last_stack_operator == Operator::LeftParenthesis {
                        break;
                    }

                    rpn_container.push(last_stack_operator.into());
                }
                continue;
            }

            if last_stack_operator == &Operator::LeftParenthesis {
                operator_stack.push(current_operator);
                continue;
            }

            if current_operator.precedence() > last_stack_operator.precedence() {
                operator_stack.push(current_operator);
                continue;
            } else {
                while let Some(previous_operator_on_top) = operator_stack.pop() {
                    if current_operator.precedence() <= previous_operator_on_top.precedence() {
                        rpn_container.push(previous_operator_on_top.into());
                        continue;
                    }
                    operator_stack.push(previous_operator_on_top);
                    break;
                }
                operator_stack.push(current_operator);
            }
        }

        push_number_to_rpn_container(&mut rpn_container, &mut parsing_number)?;

        while let Some(last_stack_operator) = operator_stack.pop() {
            rpn_container.push(last_stack_operator.into());
        }

        Ok(Self {
            tokens: rpn_container,
        })
    }
}

impl Calculate for ReversePolishNotation {
    fn calculate(&self) -> f32 {
        let mut value_stack: Vec<f32> = Vec::new();
        for token in self.tokens.iter() {
            match token {
                Token::Operand(value) => value_stack.push(*value as f32),
                Token::Operator(operator) => {
                    let value_1 = value_stack.pop().expect(INVALID_RPN_PANIC_ERROR);
                    let value_2 = value_stack.pop().expect(INVALID_RPN_PANIC_ERROR);

                    let result = match operator {
                        Operator::Add => value_1 + value_2,
                        Operator::Substract => value_2 - value_1,
                        Operator::Multiply => value_1 * value_2,
                        Operator::Divide => value_2 / value_1,
                        _ => panic!("{}", INVALID_RPN_PANIC_ERROR),
                    };
                    value_stack.push(result);
                }
            }
        }

        value_stack.pop().expect(INVALID_RPN_PANIC_ERROR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_expression_with_invalid_symbol() {
        let invalid_expression = String::from("{2+3}");
        assert!(ReversePolishNotation::try_from(invalid_expression)
            .is_err_and(|e| e == ParsingError::InvalidInput,),);
    }

    #[test]
    fn test_invalid_expression_with_invalid_token_order() {
        let invalid_expression = String::from("+(-3+2)");
        assert!(ReversePolishNotation::try_from(invalid_expression)
            .is_err_and(|e| e == ParsingError::InvalidInput,),);
    }

    #[test]
    fn test_invalid_expression_with_just_operator() {
        let invalid_expression = String::from("-");
        assert!(ReversePolishNotation::try_from(invalid_expression)
            .is_err_and(|e| e == ParsingError::InvalidInput,),);
    }

    #[test]
    fn test_invalid_expression_with_operator_at_end() {
        let invalid_expression = String::from("1+1+(");
        assert!(ReversePolishNotation::try_from(invalid_expression)
            .is_err_and(|e| e == ParsingError::InvalidInput,),);
    }

    #[test]
    fn test_invalid_expression_with_operator_at_the_beginning() {
        let invalid_expression = String::from("+3+7");
        assert!(ReversePolishNotation::try_from(invalid_expression)
            .is_err_and(|e| e == ParsingError::InvalidInput,),);
    }

    #[test]
    fn test_invalid_expression_with_consecutive_operators() {
        let invalid_expression = String::from("1++4");
        assert!(ReversePolishNotation::try_from(invalid_expression)
            .is_err_and(|e| e == ParsingError::InvalidInput,),);
    }

    #[test]
    fn test_invalid_expression_with_consecutive_operators_with_opening_parenthesis() {
        let invalid_expression = String::from("1+(+1+1)");
        assert!(ReversePolishNotation::try_from(invalid_expression)
            .is_err_and(|e| e == ParsingError::InvalidInput,),);
    }

    #[test]
    fn test_valid_expression() -> Result<(), ParsingError> {
        let valid_expression = String::from("-3+5/5*(10-3/3)-6");
        assert_eq!(
            ReversePolishNotation::try_from(valid_expression)?.calculate(),
            0 as f32
        );

        Ok(())
    }

    #[test]
    fn test_valid_expression_with_multiple_parentheses() -> Result<(), ParsingError> {
        let valid_expression = String::from("((-3+5/5*(((10-3/3)))-6))");
        assert_eq!(
            ReversePolishNotation::try_from(valid_expression)?.calculate(),
            0 as f32
        );

        Ok(())
    }

    #[test]
    fn test_valid_expression_with_multiple_white_space() -> Result<(), ParsingError> {
        let valid_expression = String::from("-3     +    501/501*(    ((10-3/3)))   -6");
        assert_eq!(
            ReversePolishNotation::try_from(valid_expression)?.calculate(),
            0 as f32
        );

        Ok(())
    }

    #[test]
    fn test_valid_expression_with_value_in_parentheses() -> Result<(), ParsingError> {
        let valid_expression = String::from("-3     +    5/(5)*(    ((10-3/3)))   -(6)");
        assert_eq!(
            ReversePolishNotation::try_from(valid_expression)?.calculate(),
            0 as f32
        );

        Ok(())
    }

    #[test]
    fn test_valid_expression_with_just_a_number() -> Result<(), ParsingError> {
        let valid_expression = String::from("(((-1000)))");
        assert_eq!(
            ReversePolishNotation::try_from(valid_expression)?.calculate(),
            -1000.0
        );

        Ok(())
    }
}
