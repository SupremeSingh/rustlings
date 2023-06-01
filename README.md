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

Modules can be separated into different files. For example, the `sound` module can be separated into `sound.rs` and `instrument.rs` files. The `instrument.rs` file would look like ...

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








