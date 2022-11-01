
# /aɪ pʰiː eɪ/

[aɪ pʰiː eɪ] is the IPA transcription of the IPA itself.
In this case, however, it is an esoteric programming language,
that uses IPA symbols as its syntax. It also doesn't allow 
impossible combinations of phonemes, such as [ɪ ɪ], which is
not physically possible to produce.

## Usage

This interpreter depends on... well, nothing. It's a single-file
executable, created in rust. To run it, simply run the executable
For example, on unix-based systems, you can run it with 
`./ai-phi-ei`. On windows, you can simply run the `ai-phi-ei.exe` 
file.

## Syntax
Commands are written in the form of a phonemic command, followed
by any number of arguments. For example, running `θ#123;` would 
push (`θ`) a number (`#`) with the value `123` onto the stack, 
and then end the argument list (`;`).

### Commands
Commands are written in the form of a phoneme, followed by an
optional argument list, and a semicolon (`;`). 

List of commands:

| Command       | Arguments     | Description                                                                                     |
|---------------|---------------|-------------------------------------------------------------------------------------------------|
| `θ`           | 1             | Pushes a number or character (as an ASCII value) onto the stack.                                |
| `θ̠`          | 1             | Push a string onto the stack, one character at a time (as an ASCII value)                       |
| `ð`           | 0             | Pops a value off the stack                                                                      |
| `ð̠`          | 0             | Pops a string off the stack, one character at a time                                            |
| `ð̼`          | 0             | Pops a value off the current stack, and pushes it onto the previous stack                       |
| `θ̼`          | 0             | Pops a value off the current stack, and pushes it onto the next stack                           |
| `ʃ`           | 0             | Prints the top value of the stack, as a number                                                  |
| `ʒ`           | 0             | Prints the top value of the stack, as an ASCII character                                        |
| `ʃ̠`          | 1             | Prints the top `arg1` values of the stack, as ASCII characters                                  |
| `i`           | 0             | Duplicates the top value of the stack                                                           |
| `ɪ`           | 0             | Swaps the top two values of the stack                                                           |
| `ɛ`           | 0             | Pops the top two values of the stack, and pushes their sum                                      |
| `e`           | 0             | Pops the top two values of the stack, and pushes their difference                               |
| `æ`           | 0             | Pops the top two values of the stack, and pushes their product                                  |
| `ɑ`           | 0             | Pops the top two values of the stack, and pushes their quotient                                 |
| `ɔ`           | 0             | Pops the top two values of the stack, and pushes their modulo                                   |
| `ɒ`           | 0             | Pops the top two values of the stack, and pushes their exponentiation                           |
| `x`           | 0             | Pops the top value of the stack, and trashs it                                                  |
| `χ`           | 1             | Pops the top `arg1` values of the stack, and trashs them                                        |
| `k`           | 0             | Switches to the previous stack                                                                  |
| `g`           | 0             | Switches to the next stack                                                                      |
| `ʝ`           | 0             | Takes user input and pushes it to the stack. If it is a string, it pushes a 1-terminated string |
 | `ʔ`           | 0             | If the top value of the stack is 1, goes down. If it's 0, goes up. Otherwise, does nothing.     |
| `ʕ`           | 0             | If the top value of the stack is 1, goes up. If it's 0, goes down. Otherwise, does nothing.     |
| `ʡ`           | 0             | If the top value of the stack is 1, goes right. If it's 0, goes left. Otherwise, does nothing.  |
| `ʔ̞`          | 0             | If the top value of the stack is 1, goes left. If it's 0, goes right. Otherwise, does nothing.  |
| More to come! | More to come! | More to come!                                                                                   |


## Building
To build, you must have Rust and Cargo installed. Then, simply run
```shell
# Clone the repository
git clone https://github.com/CATboardBETA/ai-phi-ei.git
cd ai-phi-ei
# Build the project
cargo build --release
# The executable will be in target/release
```

## Contributing
If you want to contribute, you can open a pull request. If you
want to add a feature, please open an issue first, so we can
discuss it.

## License
This project is licensed under the LGPL-3.0 License. See the
LICENSE.md file for more information.
