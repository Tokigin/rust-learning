use rand::Rng;
fn merge_sort(arr: &mut Vec<i32>) {
    let len: usize = arr.len();
    if len < 2 {
        return; // Base case: if the array has less than 2 elements, it's already sorted.
    }

    let mid: usize = len / 2;
    let mut left: Vec<i32> = arr[0..mid].to_vec(); // Create a left subarray
    let mut right: Vec<i32> = arr[mid..len].to_vec(); // Create a right subarray

    merge_sort(&mut left); // Sort the left subarray
    merge_sort(&mut right); // Sort the right subarray

    merge(arr, &left, &right); // Merge the sorted subarrays back into arr
}

fn merge(arr: &mut Vec<i32>, left: &[i32], right: &[i32]) {
    let mut i: usize = 0; // Index for left subarray
    let mut j: usize = 0; // Index for right subarray
    let mut k: usize = 0; // Index for main array

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}
fn search(arr: &mut Vec<i32>) {
    let low_bound: i32 = 5;
    let up_bound: i32 = 8;

    // Find lower bound index using binary search
    let low_index: usize = arr.binary_search(&low_bound).unwrap_or_else(|x: usize| x);

    // Find upper bound index using binary search
    let up_index: usize = arr
        .binary_search(&(up_bound + 1))
        .unwrap_or_else(|x: usize| x);

    // Count how many numbers fall between 1000 and 2000
    let count: usize = up_index - low_index;

    // Print the count of numbers in the specified range
    println!("Count of numbers between {low_bound} and {up_bound}: {count}");
}
fn main() {
    let mut rng = rand::thread_rng();
    let mut numbers: Vec<i32> = (0..10000).map(|_| rng.gen_range(1..=10000)).collect();
    println!("Random Numbers: {:?}", numbers);
    merge_sort(&mut numbers);
    println!("Sorted array: {:?}", numbers);
    search(&mut numbers);
    let numbers: Vec<i32> = vec![3, 4, 4, 4, 5];
    let num: i32 = 3;
    let result: Result<usize, usize> = numbers.binary_search(&num);
    match result {
        Ok(index) => println!("{}", index), // Print only the index if found
        Err(_) => println!("Not found"),    // Handle not found case
    }
    let nums: [(i32, char); 3] = [(1, 'a'), (2, 'b'), (3, 'c')];
    let result: Result<usize, usize> = nums.binary_search_by_key(&3, |&(num, _)| num);
    match result {
        Ok(index) => println!("Binary search by key - {:?}", index), // Print only the index if found
        Err(_) => println!("Not found"),                             // Handle not found case
    }
    let _people: &[&String] = &[
        &"Mg Mg".to_string(),
        &"Mg Ba".to_string(),
        &"Hla Hla".to_string(),
        &"Mya Mya".to_string(),
    ];
    // Ok(1)
    let mut hello: String = String::from("Hello");
    hello.push_str("My Fren");
    let _slice: &str = &hello[0..5]; // result => hello
    let _x: () = {
        let price: i32 = 500;
        let qty: i32 = 10;
        let _ = price * qty;
    };
    //
    let a1: String = String::from("This is a1 text");
    let _a2: String = a1;
}
fn add(a: i32, b: i32) -> i32 {
    let c: i32 = a + b;
    c + 2
}
