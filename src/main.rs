use std::env;

const MAX_DIGITS: u32 = 8;

/// The entry point of the program that reads command-line arguments,
/// Converts the arguments from decimal to target base, and prints the results.
///
/// This function expects that the first command-line argument is the target base for conversion
/// followed by a list of floating-point numbers, which will be converted to target base. The program
/// will print a table displaying the original decimal numbers and their
/// target base equivalents.
fn main() {
    let (target_base, f64_numbers) = parse_input();

    let target_base_numbers: Vec<String> = f64_numbers
        .iter()
        .map(|&num| convert_from_decimal_to_binary(num, target_base))
        .collect();

    display(target_base, f64_numbers, target_base_numbers);
}

fn parse_input() -> (u32, Vec<f64>) {
    let args: Vec<String> = env::args().collect();

    let target_base: u32 = args[1].parse::<u32>().unwrap();

    let f64_numbers: Vec<f64> = args
        .iter()
        .skip(2)
        .flat_map(|arg| arg.parse::<f64>())
        .collect();
    (target_base, f64_numbers)
}

/// Converts a decimal number (f64) to its target base representation as a string.
///
/// # Arguments
///
/// * `decimal` - A floating-point number to convert.
/// * `target_base` - The base to convert to.
///
/// # Returns
///
/// A `String` containing the target base representation of the input `decimal`.
///
/// # Example
///
/// ```
/// let binary = convert_from_decimal_to_binary(0.5, 2);
/// assert_eq!(binary, "0.1");
/// ```
fn convert_from_decimal_to_binary(decimal: f64, target_base: u32) -> String {
    let mut result = String::from("0.");
    let mut fraction = decimal;

    for _ in 0..MAX_DIGITS {
        fraction *= target_base as f64;
        let digit = fraction.floor() as u32;
        result += &format!("{};", digit);
        fraction -= digit as f64;

        if fraction == 0.0 {
            break;
        }
    }

    result
}

fn display(target_base: u32, f64_numbers: Vec<f64>, target_base_numbers: Vec<String>) {
    println!(
        "| {:^10} | {:^10} |",
        "Base 10",
        format!("Base {}", target_base)
    );

    println!("|{:-<12}|{:-<12}|", ":", ":");

    for i in 0..target_base_numbers.len() {
        println!(
            "| {:<7} | {:<10} |",
            format!("{:.1$}", f64_numbers[i], MAX_DIGITS as usize),
            target_base_numbers[i]
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_conversion() {
        assert_that!(convert_from_decimal_to_binary(0.5, 2), equal_to("0.1;"));
        assert_that!(convert_from_decimal_to_binary(0.25, 2), equal_to("0.0;1;"));
        assert_that!(convert_from_decimal_to_binary(0.75, 2), equal_to("0.1;1;"));
        assert_that!(
            convert_from_decimal_to_binary(0.125, 2),
            equal_to("0.0;0;1;")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.6875, 2),
            equal_to("0.1;0;1;1;")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.7, 2),
            equal_to("0.1;0;1;1;0;0;1;1;")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.8, 2),
            equal_to("0.1;1;0;0;1;1;0;0;")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.9, 2),
            equal_to("0.1;1;1;0;0;1;1;0;")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.6, 2),
            equal_to("0.1;0;0;1;1;0;0;1;")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.3, 2),
            equal_to("0.0;1;0;0;1;1;0;0;")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.1, 2),
            equal_to("0.0;0;0;1;1;0;0;1;")
        );
    }
}
