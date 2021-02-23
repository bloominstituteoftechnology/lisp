use std::io::{self, Write};

fn slurp_expr() -> String {
    let mut expr = String::new();

    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");

    expr
}

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
