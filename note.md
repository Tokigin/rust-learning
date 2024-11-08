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

## Borrowing and References (Structure)

Similar to object from c#.

    fn main() {
        let mut account: BankAccount = BankAccount {
            owner: "Mya Mya".to_string(),
            balance: 2000.25,
        };
        account.check_balance();
        account.withdraw(500.55);
        account.check_balance();
    }

    struct BankAccount {
        owner: String,
        balance: f64,
    }
    impl BankAccount {
        fn withdraw(&mut self, amount: f64) {
            println!("Withdrawing {} from {} account. ", amount, self.owner);
            self.balance -= amount;
        }
        fn check_balance(&self) {
            println!("Account owned by {} has {} ", self.owner, self.balance);
        }
    }

## Constants

    fn main() {
        let x: i32 = 4 + 5;
        const Y: i32 = 10;
        println!("x - {}", x);
        println!("Y - {}", Y);
        println!("Pi - {}", PI);
        println!("M - {}", M);
    }

    const PI: f32 = 3.14;
    const M: i32 = 60 * 60 * 15;

## Shadowing

    let x = 5;
    let x = x + 1; // 6
    {
        let x = x * 2; // 12
        println!("{x}"); //12
    }
        println!("{x}"); // 6

Can change variable type.

    let spaces: &str = " ";
    let spaces: usize = spaces.len();

## Enum

Somehow varient of something.

    enum IpAddress {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home: IpAddress = IpAddress::V4(127, 0, 0, 1);
    let loopback: IpAddress = IpAddress::V6(String::from("::1"));

## Error Handling

There are 2 types. Option and Result.

    enum Option<T> {
        Some(T), // value
        None,    // no value
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

Use like this.

    match read_file(path) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }

## Vectors

In c3 -> Collections

    let mut _v: Vec<i32> = Vec::new();
    _v.push(5);
    let _the_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let third: &i32 = &_the_vec[2];
    println!("{third}");

    let third = _the_vec.get(5);
    match third {
        Some(third) => println!("{third}"),
        None => println!("None"),
    }

## HashMap

Like Object from javascript.

let mut scores = HashMap::new();
// Insert key-value pairs
scores.insert("Alice", 50);
scores.insert("Bob", 30);
scores.insert("Charlie", 40);

    // Access a value
    if let Some(&score) = scores.get("Alice") {
        println!("Alice's score before: {}", score);
    }
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
    // Update a value
    scores.insert("Alice", 60); // Update Alice's score

    // Remove an entry
    scores.remove("Bob");

    // Iterate over the HashMap
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
