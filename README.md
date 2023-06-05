# Rustlings Solutions

This repository contains my solutions to the [Rustlings](https://github.com/rust-lang/rustlings) exercises. Basically, we will be going through core Rust concepts in the following doc as we solve each exercise in this problem set. At the end, you should be able to ...

* Understand the basics of Rust
* Be familiar with Rust developer tooling
* Be able to write your own programs in Rust

## Section 1 - Intro

The first concept to cover is that Rust is a functional programming language. The entry point to every Rust program, hence, is a `main` function.

```
fn main() {
    println!("Hello, world!");
}
```

This section covers the simplest tool for debugging - `println!` . We have already seen the simplest implementation of this command. However, to print something such that it retains formatting (eg. extra spaces), try ...

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

Finally, like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you aren’t allowed to use `mut` with constants. Constants aren’t just immutable by default—they’re always immutable. You declare constants using the `const` keyword instead of the `let` keyword, and the type of the value must be annotated.

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about. The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime. Eg.

```
const MAX_POINTS: u32 = 100_000;
```

## Section 3 - Functions

[Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) are declared using the `fn` keyword. Its arguments are declared with their types, and, if the function returns a value, the return type must be specified after an arrow ( `->` ). The final expression in the function will be used as return value. For eg.

```
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```

As a syntactic feature, Rust functions which end with a statement that is not terminated with a `;` act as a default return values. Moreover, it is best practice to use snakeCase for function names in the Rust language.

## Section 4 - Control Flow (If)

[Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions) is the order in which the statements of a program execute. In Rust, the `if` expression is similar to that of other languages. However, it is important to note that the condition must be a `bool` . For eg.

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

Sometimes it is more efficient to express simple binary control flow using the `ternary operator` . For eg.

```
let price = if num_apples_bought > 40 { 1 } else { 2 };
```

## Section 5 - Primitive Types

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data. Rust has a few basic [data types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html) that are directly implemented into the compiler.

It is also best to remember that Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it. However, programming cnventions dictate that we should add type annotations in most cases.

### Scalar Types

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

Characters are expressed using `''` quotes in Rust. A string is just an array of characters. To express a string in Rust, do something like this ...

```
let s = "Hello, world!";
```

### Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

The difference between a tuple and an array is that tuples can store data of different types, whereas arrays can only store data of the same type.

```
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
```

Moreover, tuples have a unique indexing system which goes something like ...

```
let first_value: i32 = tup.0;
```

Whereas for an array, we would have to do something like this ...

```
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
```

We can also declare sequential arrays simply like ...

```
let a = [0; 100];
```

### Slice Type

The [slice type](https://doc.rust-lang.org/stable/book/ch04-03-slices.html) lets you reference a contiguous sequence of elements in a collection rather than the whole collection. Slices are similar to arrays, but their size is not known at compile time.

Instead, a slice is a two-word object, the first word is a pointer to the data, and the second word is the length of the slice. You can make a slice out of a string as follows ...

```
let s = String::from("hello world");
let hello = &s[0..5];
```

## Section 6 - Vectors

Vectors are one of the most-used Rust data structures. In other programming languages, they'd simply be called Arrays, but since Rust operates on a bit of a lower level, an array in Rust is stored on the stack (meaning it can't grow or shrink, and the size needs to be known at compile time),  and a Vector is stored in the heap (where these restrictions do not apply).

Vectors are declared using either of the following methods ...

```
let v: Vec<i32> = Vec::new(); // Using the new method
let v = vec![1, 2, 3]; // Using the vec! macro
```

Vectors can be mutated as follows ...

```
let mut v = Vec::new();
v.push(5);
```

Further, there are 2 ways to access and modify the elements of a vector in Rust. You can either loop through it or create a mapping. For the loop ...

```
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        *element = *element * 2;
    }   
    v
}
```

For the mapping ...

```
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        *element * 2
    }).collect()
}
```

In both cases, we have to deference the element using `*` because the elements of the vector are stored on the heap. More on this [here](https://www.hackertouch.com/rust-loop-over-vector.html).

## Section 7 - Move Semantics

Rust uses a stack based architecture and manages its memory using this concept of "ownership". I recommend watching [this](https://www.youtube.com/watch?v=8M0QfLUDaaA) video to learn about it. Ownership is the reason Rust is able to work without a background garbage collector. 

### Ownership

As per the Rust textbook, there are 3 rules of ownership in this language - 

- Every value has a variable which is called its owner. 
- A value can only have 1 owner at a time. 
- When a variable goes out of scope, the value will be dropped.

First, let's see how simple numbers are stored in this architecture. Numbers (Integers more precisely) are stored in the stack. So, they can be "copied" when being assigned as follows ...
```
let a = 3;
let b = a; // 5 is copied to b, still retained with a
```
Do remember, now `a` and `b` have ownership of different memory slots with the same value. 

Further, let's look at how strings are stored. Since a string may not have a fixed length, Rust stores it in a heap and returns the pointer to this heap to the stack. Unlike the stack, values in the heap cannot be copied around.

This means, we cannot assign 2 variables to the same pointer value - since in this case they will point to the same place in the heap and break rule 2.  So how does Rust process the following ...

```
let x = String:from("hello");
let a = x; // x no longer has ownership of "hello"
```
Simple (kinda) ... it "moves" the ownership of "hello"'s pointer from x to a. Now, x does not own any value in the program. And if you do want x to retain it's original value - you will have to clone it ...

```
let x = String:from("hello");
let a = x.clone();
```

Now both `a` and `x` have ownership of memory locations containing the word "hello".

Moreover, in Cairo, when an argument is passed to a function and it's not explicitly returned, you can't use the original variable anymore. You can, however, use a different one that it returns to. We call this "moving" a variable. 

Moreover, the `&mut` keyword is used to pass a mutable reference to a variable. This is useful when you want to modify a variable in a function and return it to the original variable. The ownership of the value is not transferred in this case. For example ... 

```
fn main() {
    let mut x = 5;
    let y = &mut x;
    *y = 6; // Assigning dereferenced value
    println!("{}", x);
}
```

Here, y has a mutable reference to x. So, when we change the value of y, we are also changing the value of x.

Variables that are moved into a function (or block scope) and aren't explicitly returned get "dropped" at the end of that function. This is also what happens here. There's a few ways to fix this, try them all if you want:
- Make another, separate version of the data that's in `arr0` and pass that to `fill_arr` instead.
- Make `fill_arr` *mutably* borrow a reference to its argument (which will need to be mutable) with the `ref` keyword , modify it directly, then not return anything. Then you can get rid of `arr1` entirely -- note that this will change what gets printed by the first `print`
- Make `fill_arr` borrow an immutable view of its argument instead of taking ownership by using the snapshot operator `@`,and then copy the data within the function in order to return an owned `Array<felt>`. This requires an explicit clone of the array and should generally be avoided in Cairo, as the memory is write-once and cloning can be expensive.

Finally, the third rule is useful when thinking about functions - since functions in Rust have their own [scope](https://en.wikipedia.org/wiki/Scope_%28computer_science%29). Passing values to a function follows the same ownership rules, meaning they can only have one owner at a time, and free up memory once out of scope.

### References and Borrowing

Ok wait a minute ... 

This procedure of moving ownerships can get quite messy in complicated programs - so Rust introduces this new idea of "borrowing". 

References allow coders to use a function that has a reference to an object as a parameter instead of taking ownership of the value. Now, instead of passing objects by value, objects can be passed by their reference. Read more on it [here](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html).

## Section 8 - Structs

Rust has three [struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) types: a classic C struct, a tuple struct, and a unit struct.

### Classic C Struct

A classic C struct is useful when you want to define an object with multiple fields. For example ...

```
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

### Tuple Struct

A tuple struct is useful when you want to define an object with multiple fields but don't want to name them. For example ...

```
struct Color(i32, i32, i32);
```

### Unit Struct

A unit struct is useful when you want to define an object with no fields. For example ...

```
struct UnitStruct;
```

Structs contain data, but can also have logic associated with them. This logic is defined using `impl` blocks. Read on ...
### Methods on Structs

Methods are functions that are defined within the context of a struct. For example ...

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

where `self` is a reference to the struct instance, and `Rectangle` is the struct name.

As an aside for the purposes of debugging, `#[derive(Debug)]` is an annotation that allows us to print the struct using `println!("{:?}", rect1);`. Other useful annotations are 

- `#[derive(PartialEq)]` to compare structs using `==` and `!=`
- `#[derive(Copy, Clone)]` to copy structs using `let rect2 = rect1;` instead of `let rect2 = &rect1;`
- `#[derive(Hash)]` to use structs as keys in a `HashMap`

### Associated Functions

Associated functions are functions that are defined within the context of a struct but don't take `self` as a parameter. For example ...

```
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

where `Rectangle` is the struct name. Associated functions are often used for constructors that will return a new instance of the struct. They are different from methods in that they don't have an instance of the struct to work with.

## Section 9 - Enums

Rust allows you to define types called [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html) which enumerate possible values. For example ...

```
enum IpAddrKind {
    V4,
    V6,
}
```

Enums can also have data associated with them. For example ...

```
enum IpAddr {
    V4(String),
    V6(String),
}
```

Further, you can add logic to enums by defining methods within an `impl` block. For example ...

```
impl IpAddr {
    fn call(&self) {
        // method body would be defined here
    }
}
```

Finally, Rust's "pattern matching" facility works with Enums too. This makes it easy to run different code for different values of an enumeration. For instance ... 

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

The match statement basically says "match the value of `coin` to one of the values in the enum `Coin` and run the code associated with that value".

## Section 10 - Strings

Rust has two [string](https://doc.rust-lang.org/book/ch08-02-strings.html) types, a string slice (`&str`) and an owned string (`String`). They are interchangeable for the most part. To declare, either you may ...

```
let slice = "hello";
let ownder = String::from("hello");
```

There are a few useful methods that can be called on strings. For instance ... 

```
input.trim().to_string()
input.to_string() + " " + world
input.replace("cars", "balloons").to_string()
```

## Section 11 - Modules

Rust organizes code using a [module system](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html). This system allows you to group related code together in a module, and then use that module in other parts of your program. 

Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private). By default, items in Rust are private. 

### Defining Modules

Modules are defined using the `mod` keyword. For example ...

```
mod sound {
    mod instrument {
        fn clarinet() {
            // Function body code goes here
        }
    }

    mod voice {
        fn trill() {
            // Function body code goes here
        }
    }
}
```

### Paths

Paths are used to refer to items in modules. An example of referring to one module in another using its path is ...

```
mod sound {
    mod instrument {
        fn clarinet() {
            // Function body code goes here
        }
    }

    mod voice {
        fn trill() {
            instrument::clarinet();
        }
    }
}
```

The `::` operator is used to separate the path segments. 

### Bringing Paths into Scope with `use`

The `use` keyword can be used to bring a path into scope. 

```
use std::time::{SystemTime, UNIX_EPOCH};
```

### Separating Modules into Different Files

Modules can be separated into different files. For example, the `sound` module can be separated into `sound.rs` and `instrument.rs` files. Multiple modules are compiled into a unit called crate. Rust programs may contain a binary crate or a library crate. A binary crate is an executable project that has a main() method. 

A library crate is a group of components that can be reused in other projects. Unlike a binary crate, a library crate does not have an entry point (main() method). The Cargo tool is used to manage crates in Rust. For instance ...

### The `super` Keyword 

The `super` keyword is used to refer to the parent module. For example ...

```
mod sound {
    mod instrument {
        fn clarinet() {
            // Function body code goes here
        }
    }

    mod voice {
        fn trill() {
            super::instrument::clarinet();
        }
    }
}
```

As opposed to this, the `self` keyword is used to refer to the current module. Further, structs and enums in a module are private by default. To make them public, use the `pub` keyword. For example ...

```
pub use self::fruits::PEAR as fruit; 
```

## Section 12 - HashMaps 

A [hash map](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) allows you to associate a value with a particular key. 

Hashmaps can be simply declared using ... 

```
let mut basket: HashMap<String, u32> = HashMap::new();
basket.insert(String::from("banana"), 2);
basket.insert(String::from("apple"), 3);
basket.insert(String::from("orange"), 4);
```

To access a value from the hashmap, pass a reference to the key ... 

```
let count = basket.get(&String::from("banana")); 
```

To iterate over the hashmap, use a for loop ... 

```
for (key, value) in &basket {
    println!("{}: {}", key, value);
}
```

## Section 13 - Options 

Type [Option](https://doc.rust-lang.org/std/option/enum.Option.html) represents an optional value: every Option is either Some and contains a value, or None, and does not. Option types are very common in Rust code, as they have a number of uses:

- Initial values
- Return values for functions that are not defined over their entire input range (partial functions)
- Return value for otherwise reporting simple errors, where None is returned on error

Options are often also used as return values for functions that could fail for some reason. For example, the `parse` method on strings parses a string into some kind of number. If successful, it returns the number wrapped in `Some`. If unsuccessful, it returns `None`. 

```
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

You can also convert values to options and use them for checks as ... 

```
let target = "rustlings";
let optional_target = Some(target);

if let Some(word) = optional_target {
    assert_eq!(word, target);
}
```

Finally, it is possible to pass a pointer to an option to a function. For example ...

```
fn print_optional(optional: &Option<String>) {
    match optional {
        Some(value) => println!("The value is {}", value),
        None => println!("The value is empty"),
    }
}
```

## Section 14 - Error Handling

Most [errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.


### Result Type

The `Result` type is an enum defined as ...

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `T` and `E` are generic type parameters. The `T` represents the type of the value that will be returned in a success case within the `Ok` variant, and the `E` represents the type of the error that will be returned in a failure case within the `Err` variant. For example ... 

Usually, a function which returns generic type `T` will return `Result<T, E>` and will have the following signature ...

```
fn do_something() -> Result<T, E> {
    // Function body code goes here
}
```

Where `E` can take different error types for instance it can be `ParseIntError` or even `ParseFloatError` and so on. Any type that implements the `std::error::Error` trait can be used as an error type. Further, to force a function to return an `OK` value, you can simply terminate it with `Ok(())`.

 `Ok(())` is the same as `Ok(Ok(()))` which is the same as `Ok(Result<(), ParseIntError>)`.

### The `?` Operator

The `?` operator can be used to propagate errors. For example ...

```
fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
```

The above code can be simplified as ...

```
fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = number_str.parse::<i32>()?;
    println!("{}", number);
    Ok(())
}
```

The `?` operator can only be used in functions that return `Result` or `Option`. 

### The `?` Operator and `main()`

The `?` operator can be used in the `main()` function. For example ...

```
fn main() -> Result<(), Box<dyn Error>> {
    let number_str = "10";
    let number = number_str.parse::<i32>()?;
    println!("{}", number);
    Ok(())
}
```

The `main()` function returns `Result<(), Box<dyn Error>>` which means that it returns `Ok(())` or `Err(Box<dyn Error>)`. The `?` operator can be used to propagate errors.

### The `map_err()` Method

The `map_err()` method can be used to convert an error type to another error type. For example ...

```
fn main() -> Result<(), Box<dyn Error>> {
    let number_str = "10";
    let number = number_str.parse::<i32>().map_err(|e| ParseIntError::from(e))?;
    println!("{}", number);
    Ok(())
}
```

The `map_err()` method takes a closure as an argument. The closure takes an error as an argument and returns a new error. The `?` operator can be used to propagate errors.

### The Box Type

The [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html) type is a smart pointer type. It is used to store data on the heap. For example ...

```
let x = 5;
let y = Box::new(x);
```

The `y` variable is a box that points to the value `5` on the heap. The `x` variable is stored on the stack. 

## Section 15 - Generics 

[Generics](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html) is the topic of generalizing types and functionalities to broader cases. This is extremely useful for reducing code duplication in many ways, but can call for rather involving syntax. Namely, being generic requires taking great care to specify over which types a generic type is actually considered valid. 

The simplest and most common use of generics is for type parameters.

```
struct Point<T> {
    x: T,
    y: T,
}
```

The above code defines a struct named `Point` that has a field `x` and a field `y`. The `x` and `y` fields have the same type `T`. The `T` is a generic type parameter. The `T` can be any type. For example ...

```
let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
```

A more concrete example would be something like ...

```
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```

### Displaying Generics 

The `Display` trait can be used to display a generic type. For example ...

```
use std::fmt::Display;

struct Wrapper<T> {
    value: T,
}

impl<T: Display> Wrapper<T> {
    pub fn print(&self) {
        println!("{}", self.value);
    }
}

fn main() {
    let w = Wrapper { value: 5 };
    w.print();
}
```

## Section 16 - Traits

A [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics. The `#derive()` attribute can be used to automatically implement some traits for a data type. For example, the `Debug` trait can be automatically implemented for a data type by writing `#[derive(Debug)]` above the data type definition.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

The difference between self and Self is that self is a reference to the current instance of the struct, whereas Self is the type name of the current struct.

### Privacy and Traits

By default, trait methods are private. When defining a trait, it can be made public or private. For example ...

```
pub trait Summary {
    fn summarize(&self) -> String;
}
```

### Selecting Data Type based on Trait 

A function can take a generic type parameter that implements a trait. For example ...

```
fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

The `impl Summary` syntax means that the `item` parameter must be a data type that implements the `Summary` trait. You can even define a function that takes multiple generic type parameters that implement the same trait. For example ...

```
fn notify<T: Summary>(item1: T, item2: T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}
```

And finally, to define a parameter that implements multiple traits, you can use the `+` syntax. For example ...

```
fn notify(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}
```

## Section 17 - Lifetimes 

[Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html) are a way of ensuring that references are valid as long as they are used. Rust requires that all references be valid. Lifetimes are a way of enforcing this requirement.

Checking references is one of the borrow checker’s main responsibilities. Lifetimes help the borrow checker ensure that you never have invalid references. In many cases, the borrow checker can infer the correct lifetimes and take care of everything on its own. But often it needs your help to figure it out.

### Lifetime Annotations

Lifetime annotations are a way of telling the borrow checker how generic lifetime parameters relate to each other. For example ...

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The `'a` syntax is a lifetime annotation. The `'a` syntax means that the lifetime of the return value is the same as the lifetime of the two parameters. To learn more about lifetime anotations, please refer to these [exercises](https://tfpk.github.io/lifetimekata/).

### Lifetimes in Structs

Lifetimes can be used in structs. For example ...

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Bobby. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

The `ImportantExcerpt` struct has a lifetime parameter `'a`. The `part` field is a reference to a string slice. The lifetime of the string slice must be the same as the lifetime of the struct.

## Section 18 - Testing

Rust has a built-in [testing](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) framework. To write tests, you must create a `tests` directory in the same directory as the file you want to test. Then, you can write tests there. 

For example, if you have a file named `adder.rs`, you can write tests in the `tests` directory in a file named `adder_test.rs`.

```
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

The `#[test]` attribute indicates that the function is a test. The `assert_eq!` macro is used to assert that the first argument is equal to the second argument. If the two arguments are not equal, the test will fail.

You can also add tests to the same file as the function you want to test. For example ...

```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```

The `#[cfg(test)]` attribute indicates that the following code is only compiled when running tests. The `mod tests` syntax creates a module named `tests`. The `use super::*` syntax imports all items from the parent module. This allows the `add_two` function to be used in the `tests` module.

### Asserting Errors

You can also write tests that assert that a function returns an error. There are different types of assert statements out there. For example ...

- `assert!` - asserts that the argument is true
- `assert_eq!` - asserts that the two arguments are equal
- `assert_ne!` - asserts that the two arguments are not equal
- `assert_matches!` - asserts that the argument matches a pattern
- `assert_panic!` - asserts that the argument panics

And so on. 

### Handling Panic

When a test panics, the test fails. However, sometimes you want to test that a function panics. For example, you might want to test that a function panics when given invalid input. To do this, you can use the `should_panic` attribute. For example ...

```
#[test]
#[should_panic(expected = "A meaningful message")]
fn it_panics() {
    // ...
}
```

## Section 19 - Iterators

[Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) are a way of iterating over a collection of items. For example, you can iterate over a vector like this ...

```
let v = vec![1, 2, 3];

for i in &v {
    println!("{}", i);
}
```

The `&` syntax is used to borrow the vector. This is because the `for` loop takes ownership of the vector. If you want to iterate over a vector without borrowing it, you can use the `into_iter` method. For example ...

```
let v = vec![1, 2, 3];

for i in v.into_iter() {
    println!("{}", i);
}
```

The `into_iter` method returns an iterator that takes ownership of the vector. This means that the vector is moved into the iterator. If you want to iterate over a vector without moving it, you can use the `iter` method. For example ...

```
let v = vec![1, 2, 3];

for i in v.iter() {
    println!("{}", i);
}
```

The `iter` method returns an iterator that borrows the vector. This means that the vector is not moved into the iterator. If you want to iterate over a vector and mutate it, you can use the `iter_mut` method. For example ...

```
let mut v = vec![1, 2, 3];

for i in v.iter_mut() {
    *i += 1;
}
```

The `iter_mut` method returns an iterator that mutably borrows the vector. This means that the vector is not moved into the iterator, but it can be mutated.

### Creating Iterators

You can also create your own iterators. For example ...

```
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    for i in counter {
        println!("{}", i);
    }
}
```

The `Counter` struct has a `count` field. The `new` method returns a `Counter` struct with a `count` field set to `0`. The `next` method returns the next item in the iterator. The `next` method returns an `Option` type. 

If the `next` method returns `None`, the iterator is done. If the `next` method returns `Some`, the iterator is not done. The `next` method mutates the `count` field. The `next` method returns the `count` field if the `count` field is less than `6`. Otherwise, the `next` method returns `None`.

The `Iterator` trait has an associated type named `Item`. The `Item` type is the type of the items in the iterator. In this case, the `Item` type is `u32`.

### Using Other Iterator Trait Methods

The `Iterator` trait has many methods. For example ...

- `sum` - returns the sum of the items in the iterator
- `product` - returns the product of the items in the iterator
- `min` - returns the minimum item in the iterator
- `max` - returns the maximum item in the iterator
- `count` - returns the number of items in the iterator
- `nth` - returns the nth item in the iterator
- `last` - returns the last item in the iterator
- `collect` - collects the items in the iterator into a collection
- `map` - maps the items in the iterator to another type
- `filter` - filters the items in the iterator

And so on.

### Closures

Closures are a way of creating functions that can be used as arguments to other functions. For example ...

```
fn add_one<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(1)
}

fn main() {
    let result = add_one(|x| x + 1);
    println!("{}", result);
}
```

The `add_one` function takes a closure as an argument. The closure takes an `i32` as an argument and returns an `i32`. The `add_one` function returns an `i32`. The `add_one` function calls the closure with `1` as an argument. The closure adds `1` to the argument and returns the result.

The `add_one` function can be called with any closure that takes an `i32` as an argument and returns an `i32`. For example ...

```
fn main() {
    let result = add_one(|x| x + 1);
    println!("{}", result);

    let result = add_one(|x| x + 2);
    println!("{}", result);

    let result = add_one(|x| x + 3);
    println!("{}", result);
}
```
### Iteration Example 

A simple example of iterating over range of numbers can be found as ...

```
let mut result = 1;
for i in 1..=num {
    result *= i;
}
result
```

Another way to iterate over a collection and map changes to its items is ... 

```
let v1 = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```

This will create a new vector `v2` with the values `[2, 3, 4]`. The `collect()` method is used to collect the items in the iterator into a collection. 

The `map()` method is used to map the items in the iterator to another type. The `|x| x + 1` syntax is used to create a closure. The closure is used to map the items in the iterator to another type.

