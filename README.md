# Program Description #
The program takes in a real number between 0 and 1 in Base 10 and converts that number to a chosen base equivalent (i.e. 2, 8, 16, 60...)

# How to run the program #
The program is written in Rust and compiled and run by Cargo. 
Use cargo build to compile the program. Use cargo run to run the program.
Use cargo test to run the unit tests.
Use cargo run -- followed by an integer for the target base, followed by a list of real between 0 and 1 seperated by a space (e.g. cargo run -- 2 0.5 0.25 0.75) to get an output similair to the one below.

Example output:
| Base 10 | Base 2   |
| :-------|:---------|
| 0.5     | 0.1;     |
| 0.25    | 0.0;1;   |
| 0.75    | 0.1;1;   |

# Output Description
The program prints the fractional numbers in base 10 and their chosen base equivalents in the table format as above.
The ; in the right columns is used to seperate the digits for easier readability.

# Dependecies #
The program uses hamcrest library for unit testing.
