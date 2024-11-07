# Note

## Array

    let arr:[i32;5] = [1,2,3,4,5];

    let arr:[&str;2] = ["LOL","Haha"];

## Tuples

    let human: (&str, i32, bool) = ("Mg Mg",12,false);

    let human:(String,i32,bool) = ("Mg Mg".to_string(),12,false);

    let mix: (&str, i32, bool, [i32; 5]) = ("Gg", 23, true, [1, 2, 3, 4, 5]);

## Slices

    let number: &[i32] = &[1, 2, 3, 4, 5];

    let people: &[&str] = &["Mg Mg", "Mg Ba", "Hla Hla", "Mya Mya"];

    let people: &[&String] = &[
        &"Mg Mg".to_string(),
        &"Mg Ba".to_string(),
        &"Hla Hla".to_string(),
        &"Mya Mya".to_string(),
    ];

## String vd String slices (&str)

String => growable , mutable, owned by string type, sometime slow

    let mut hello: String = String::from("Hello");
    hello.push_str("My Ma Ma");

&str => good for memory, react quickly

    let mut hello: String = String::from("Hello");
    hello.push_str("My Fren");
    let slice: &str = &hello; // result => Hello My Fren

    let slice: &str = &hello[0..5]; // result => Hello

## Function

This is also a function.

    let x: () = {
        let price: i32 = 500;
        let qty: i32 = 10;
        price * qty;
    };

Arrow function.

    fn add(a: i32, b: i32) -> i32 {
        let c: i32 = a + b;
        c + 2 // no need ";" here
    }

## Owner

What?

- Stopping/Resuming the program
- solve memory issues and high performance

Rules?

- Each value has only a 1 owner
- Only 1 owner at a time
- when owner goes out of scope, value is dropped

Example

    let a1: String = String::from("This is a1 text");
    let a2: String = a1;// take text from a1, a1 don't have value anymore
