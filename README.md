# Building a Lisp Interpreter

For our second Rust project, we'll be building a simple interpreter for the [lisp](https://en.wikipedia.org/wiki/Lisp) programming language!

We'll start off by implementing a REPL, which stands for *R*ead *E*val *P*rint *L*oop. Our REPL will be able to accept Lisp
expressions and statements, interpret them, and print a result if appropriate. 

## Steps

1. The first step is to implement a REPL that loops forever, waiting on user input. When the user types something into the REPL,
have it echo the user's input. Additionally, allow the user to shut down the REPL by typing in a special "quit" command.

- [ ] Implement a REPL that loops forever and responds to user input by echoing the input back.
- [ ] Add a special "quit" command so that users can exit the REPL.

2. Now let's add the ability for our REPL to perform arithmetic calculations. In the Lisp language, arithmetic expressions 
follow [reverse Polish notation](https://en.wikipedia.org/wiki/Polish_notation). For example, to add `3` and `2`, the expression would be `(+ 3 2)`. In other words, the operator comes first, followed by the operands, all surrounded by parentheses. This notationmight seem weird, but it has some benefits, namely that it's easier for to parse. It's also nice because we're not limited toonly two operands; we could have an expression like `(* 3 8 4 7 0)` to calculate the product all of the operands. 

We'll proceed by first implementing the ability to "tokenize" an arithmetic expression, which means to split the inputs up into individual tokens, where each token is part of the Lisp syntax. For example, the expression `(+ 3 2)` would be tokenized into individual characters that we might store in a vector: `['(', '+', '3', '2', ')']`. Tokenizing is an important step when it comes to parsing some syntax. 

Once the tokenization logic is in place, we'll implement the ability to parse the input. The output of the parsing phase will
be an expression that out interpreter actually understands and can evaluate. 

- [ ] Implement logic to tokenize an arithmetic expression of the format specified above. 
- [ ] Implement logic to parse a sequence of tokens into an expression.
- [ ] Implement the ability to evaluate an expression and output the answer to the REPL.