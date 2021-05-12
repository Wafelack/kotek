kotek
=====

kotek (polish): kitten, cat.

Kotek is a simple stack based concatenative programming language.

Installation
------------

```bash
$ git clone https://github.com/wafelack/kotek.git
$ cargo install --path kotek/
```

Tutorial
--------

### Declaring a variable.

Syntax: `let <name> ( <expr>* )`.

Examples:
```
let foo ( 99 )
let square ( dup * )
```

### Types

| Name | Description | Example |
|------|-------------|---------|
| Integer | A 32 bits relative number (Z set) | `44` |
| Real | A 32 bits single precision floating point number (R set) | `3.1415` |
| String | A null terminated UTF8 string | `"Foo"` |
| Symbol | A symbol identifiying something, like booleans for example. | `#t` |
| Quote | An internal stack containing instructions | `[4 dup *]` |

### Functions

| Name | Args count | Args type | Description |
|------|------------|-----------|-------------|
| `+`  |      2     | Integer/Real | Adds two numbers. |
| `-`  |      2     | Integer/Real | Substract one number to another. |
| `*`  |      2     | Integer/Real | Multiply two numbers. |
| `/`  |      2     | Integer/Real | Divide one nunber by another. |
| `%`  |      2     | Integer/Real | Get the remainder of the division of one number by another. |
| `dup` | 1    | Any | Duplicate the top of the stack. |
| `app` | 1    | Quote | Unquote the top of the stack.  |
| `cat` |   2  | String | Concatenate two strings.    |
| `pop` |   1  | Any | Remove the top of the stack. |
| `swap` | 2 | Any | Swap the top of the stack with the value below it. |
| `print_stack` | 0 | N/A | Print the stack. |
| `eq`  | 2 | !Quote | Test equality between two values. |
| `not` | 1 | Symbol | Push `#t` if top of the stack is `#f` and vice-versa. |
| `gt`  | 2 | Integer/Real/String | Test if a value if greater than another value. |
| `lt`  | 2 | Integer/Real/String | Test if a value if less than another value. |
