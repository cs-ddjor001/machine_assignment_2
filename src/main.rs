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

/// Reads fractional numbers in base 10 and the target base for conversion
/// from the command-line arguments and parses them into a vector of `f64` values.
///
/// # Returns
///
/// A tuple contaiting a u32 target base for conversion, and a f64 vector of floating point numbers.
///
/// # Panics
///
/// This function assumes the first command line arguement is a valid
/// u32 number to be used as target base for conversion. If no arguement is provided,
/// or the arguement is a non-integer, the target base defaults to 2.
/// All arguments after the target base are valid
/// floating-point numbers. If invalid arguments are provided, they will
/// be skipped.
///
/// # Example
/// ```
/// // Assuming the program is run as follows:
/// // cargo run -- 2 0.1 0.25 0.5
/// let (base, parsed) = parse_input();
/// assert_eq!(base, 2);
/// assert_eq!(parsed, vec![0.1, 0.25, 0.5]);
/// ```
fn parse_input() -> (u32, Vec<f64>) {
    let args: Vec<String> = env::args().collect();

    let target_base: u32 = args
        .get(1)
        .and_then(|arg| arg.parse::<u32>().ok())
        .unwrap_or(2);

    let skip_count = if args
        .get(1)
        .and_then(|arg| arg.parse::<u32>().ok())
        .is_some()
    {
        2
    } else {
        1
    };

    let f64_numbers: Vec<f64> = args
        .iter()
        .skip(skip_count)
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
/// A `String` containing the target base representation of the input `decimal`
/// Each digit is seperated by a ; for easier readability
///
/// # Example
///
/// ```
/// let binary = convert_from_decimal_to_binary(0.5, 2);
/// assert_eq!(binary, "0.1;");
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

/// Outputs the decimal numbers and their target base fractional representations in a table format.
///
/// # Arguments
///
/// * `target_base` - An integer indicating the base of converted numbers.
/// * `f64_numbers` - A vector of decimal numbers in base 10.
/// * `target_base_numbers` - A vector of target base fractional strings corresponding to the decimal numbers.
///
/// # Example
/// ```
/// display(vec![0.5, 0.25], vec!["0.1;".to_string(), "0.0;1;".to_string()]);
/// ```
/// Output:
/// |   Base 10   |   Base 2   |
/// |:------------|:-----------|
/// | 0.5         | 0.1;       |
/// | 0.25        | 0.0;1;     |
fn display(target_base: u32, f64_numbers: Vec<f64>, target_base_numbers: Vec<String>) {
    println!(
        "| {:^10} | {:^22} |",
        "Base 10",
        format!("Base {}", target_base)
    );

    println!("|{:-<12}|{:-<24}|", ":", ":");

    for i in 0..target_base_numbers.len() {
        println!(
            "| {:<7} | {:<22} |",
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
    fn test_conversion_base_2() {
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

    #[test]
    fn test_conversion_base_8() {
        assert_that!(convert_from_decimal_to_binary(0.5, 8), equal_to("0.4;"));
        assert_that!(convert_from_decimal_to_binary(0.25, 8), equal_to("0.2;"));
        assert_that!(convert_from_decimal_to_binary(0.75, 8), equal_to("0.6;"));
        assert_that!(
            convert_from_decimal_to_binary(0.8, 8),
            equal_to("0.6;3;1;4;6;3;1;4;")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.16666, 8),
            equal_to("0.1;2;5;2;5;0;7;2;")
        );
    }

    #[test]
    fn test_conversion_base_16() {
        assert_that!(convert_from_decimal_to_binary(0.5, 16), equal_to("0.8;"));
        assert_that!(convert_from_decimal_to_binary(0.25, 16), equal_to("0.4;"));
        assert_that!(convert_from_decimal_to_binary(0.75, 16), equal_to("0.12;"));
        assert_that!(
            convert_from_decimal_to_binary(0.8, 16),
            equal_to("0.12;12;12;12;12;12;12;12;")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.16666, 16),
            equal_to("0.2;10;10;10;3;10;13;1;")
        );
    }

    #[test]
    fn test_conversion_base_60() {
        assert_that!(convert_from_decimal_to_binary(0.5, 60), equal_to("0.30;"));
        assert_that!(convert_from_decimal_to_binary(0.25, 60), equal_to("0.15;"));
        assert_that!(convert_from_decimal_to_binary(0.75, 60), equal_to("0.45;"));
        assert_that!(convert_from_decimal_to_binary(0.8, 60), equal_to("0.48;"));
        assert_that!(
            convert_from_decimal_to_binary(0.16666, 60),
            equal_to("0.9;59;58;33;36;0;0;0;")
        );
    }
}
