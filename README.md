# text_to_brainfuck

This is a little rust script that reads text in from stdin, and outputs a brainfuck program that will output that text.

A tiny bit of optimization has been done, using loops where possible, but nothing very complex.

If I continue to work on this I might look into more complex pattern detection and optimization stuff but eh.

## Usage

Compile using Rust. If it has been installed on your system, `cargo run` should be enough.

Accepts an optional argument for line length, which is the number of characters on each line before a new line is inserted. The default value is 20, and if an argument is provided it will use this length instead. A value of 0 means that no new lines will be printed, and an invalid integer will result in an error being printed.

## Example:

`echo "Hello World\!" | cargo run --release` produces:

```Brainfuck
>++++++++[<+++++++++>-]<
.>+++++[<++++++>-]<-.>++
+[<++>-]<+..>++[<++>-]<-
.>+++++++++[<--------->-
]<++.>+++++++[<++++++++>
-]<-.>+++++[<+++++>-]<-.
>++[<++>-]<-.>++[<--->-]
<.>+++[<--->-]<+.>++++++
++[<-------->-]<---.>+++
++[<----->-]<++.
```

When run in a brainfuck compiler such as https://copy.sh/brainfuck/, this outputs `Hello World!`.
