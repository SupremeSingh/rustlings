# Rustlings Solutions 

This repository contains my solutions to the [Rustlings](https://github.com/rust-lang/rustlings) exercises. Basically, we will be going through core Rust concepts in the following doc as we solve each exercise in this problem set. At the end, you should be able to ... 

- Understand the basics of Rust
- Be familiar with Rust developer tooling 
- Be able to write your own programs in Rust

## Section 1 - Intro 

The first concept to cover is that Rust is a functional programming language. The entry point to every Rust program, hence, is a `main` function. 

```
fn main() {
    println!("Hello, world!");
}
```

This section covers the simplest tool for debugging - `println!`. We have already seen the simplest implementation of this command. However, to print something such that it retains formatting (eg. extra spaces), try ...

```
println!(r#"       Hello, world!                      "#);
```

And finally, you can add variables in the print statement as follows ...

```
let x = 5;
println!("x = {}", x);
```

## Section 2 - Variables

In Rust, [variables](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) are immutable by default. When a variable is immutable, once a value is bound to a name, you can’t change that value. You can make them mutable by adding `mut` in front of the variable name.

The Rust compiler forbids use of uninitialized variables, as this would lead to undefined behavior. Additonally, it is best practice to assign a type to every variable. For eg. 

```
let x: i32 = 5;
```

Moreover, Rust compiler can accept strings shorter than 32 bits as integers. For eg. 

```
let x: i32 = 'Hey There';
```

You can declare a new variable with the same name as a previous variable. The first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.

Finally, Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you aren’t allowed to use `mut` with constants. Constants aren’t just immutable by default—they’re always immutable. You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value must be annotated. 

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about. The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime. Eg. 

```
const MAX_POINTS: u32 = 100_000;
```

## Section 3 - Functions

[Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) are declared using the `fn` keyword. Its arguments are declared with their types, and, if the function returns a value, the return type must be specified after an arrow (`->`). The final expression in the function will be used as return value. For eg. 

```
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

As a syntactic feature, Rust functions which end with a statement that is not terminated with a `;` act as a default return values. Moreover, it is best practice to use snakeCase for function names in the Rust language.

## Section 4 - Control Flow (If)

[Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions) is the order in which the statements of a program execute. In Rust, the `if` expression is similar to that of other languages. However, it is important to note that the condition must be a `bool`. For eg. 

```
pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    } else if fizzish == "literally anything" {
        "baz"
    } else {
        "1"
    }
}
```




