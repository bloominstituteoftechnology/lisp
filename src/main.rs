use std::io::{self, Write};

enum LispExpr {
    /// TODO: Add a `LispExpr` type to capture the type
    /// of each expression
}

enum LispErr {
    /// TODO: Add a `LispErr` type to capture errors and 
    /// propogate them back to the user
}

enum LispEnv {
    /// TODO: Add a `LispEnv` type, which is responsible for
    /// storing defined variables, built-in functions, etc.
}

impl LispEnv {
    /// TODO: Add a `new` method on the `LispEnv` to initialize
    /// an instance of it
    fn new() -> Self {
        unimplemented!();
    }

    /// TODO: Add an `eval` method on the `LispEnv` that takes
    /// a `LispExpr` reference and evaluates it, returning a
    /// `LispErr` if the eval process failed
    fn eval(expr: &LispExpr) -> Result<LispExpr, LispErr> {
        unimplemented!();
    }
}

/// Given a String expression, returns a Vector of 
/// all the tokens in the expression
fn tokenize(expr: String) -> Vec<String> {
    unimplemented!();
}

/// Given some tokens, parses them into a `LispExpr` or
/// returns a `LispErr` if parsing failed
fn parse<'a>(tokens: &'a [String]) -> Result<(LispExpr, &'a [String]), LispErr> {
    unimplemented!();
}

/// Reads input from stdin and returns it as a String
fn slurp_expr() -> String {
    let mut expr = String::new();

    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");

    expr
}

/// Main function that initializes the REPL loop
fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let expr = slurp_expr();

        match expr.trim() {
            ":q" | ":quit" => break,
            _ => {
                print!("> {}\n", expr);
                io::stdout().flush().unwrap();
            }
        }
    }
}
