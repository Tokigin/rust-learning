fn merge_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    if len < 2 {
        return; // Base case: if the array has less than 2 elements, it's already sorted.
    }

    let mid = len / 2;
    let mut left = arr[0..mid].to_vec(); // Create a left subarray
    let mut right = arr[mid..len].to_vec(); // Create a right subarray

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

fn main() {
    let mut arr: Vec<i32> = vec![64, 34, 25, 12, 22, 11, 90];
    merge_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}
