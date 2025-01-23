# Program Description #
The program takes in a real number between 0 and 1 in Base 10 and converts that number to it's Base 2 equivalent.

# How to run the program #
The program is written in Rust and compiled and run by Cargo. 
Use cargo build to compile the program. Use cargo run to run the program.
Use cargo test to run the unit tests.
Use cargo run -- followed by a list of real between 0 and 1 seperated by a space (e.g. cargo run -- 0.5 0.25 0.75) to get an output similair to the one below.

Example output:
| Base 10 | Base 2 |
| :-------|:-------|
| 0.5     | 0.1    |
| 0.25    | 0.01   |
| 0.75    | 0.11   |

# Dependecies #
The program uses hamcrest library for unit testing.
