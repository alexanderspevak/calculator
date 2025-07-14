use crate::expression_parser::{Calculate, ParsingError, ReversePolishNotation};
use std::io::{stdin, stdout, Write};

pub fn get_expression_from_user_input<T>() -> Result<T, ParsingError>
where
    T: Calculate + TryFrom<String, Error = ParsingError>,
{
    println!("Press Ctrl+C to exit");
    print!("Please enter expression and enter to calculate: ");
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .map_err(|_| ParsingError::InvalidInput)?;
    T::try_from(input)
}

pub fn run_interface() {
    loop {
        match get_expression_from_user_input::<ReversePolishNotation>() {
            Ok(expression) => {
                println!("Notation: {:?}", expression);
                println!("Result: {}", expression.calculate())
            }
            Err(parsing_error) => println!("{}", parsing_error),
        }
    }
}
