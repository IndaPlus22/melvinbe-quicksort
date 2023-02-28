use std::io;
use std::io::prelude::*;

fn main() {
    // Get input text as string
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    // Turn input string into int array
    let mut arr: Vec<i32> = input
        .split_whitespace()
        .skip(1) 
        .map(|l| l.parse().unwrap())
        .collect();

    // Sort array
    quicksort(&mut arr);

    // Print array as output
    println!("{}", arr
        .iter()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .join(" ")
    );
}

fn quicksort(arr: &mut [i32]) {
    // Handle bad argument, to pass my own test...
    if arr.is_empty() {
        return;
    }
    // Begin recursion
    quicksort_hoare(arr, 0, arr.len() - 1);
}

fn quicksort_hoare(arr: &mut [i32], left: usize, right: usize) {
    // Make sure there is at least one element in subarray
    if left < right {
        // Choose quicksort for small subarrays
        if right - left <= 20 {
            insertion_sort(&mut arr[left..=right]);
        } else {
            // Otherwise use Hoare's scheme to find good pivot...
            let pivot_index = partition_hoare(arr, left, right);
            // ... and use it to partition the subarray and sort it recursively
            quicksort_hoare(arr, left, pivot_index);
            quicksort_hoare(arr, pivot_index + 1, right);
        }
    }
}

// Finds good pivots
fn partition_hoare(arr: &mut [i32], left: usize, right: usize) -> usize {
    // Choose the middle element as pivot
    let pivot_index = (left + right) / 2;
    let pivot = arr[pivot_index];
    // Two pointers starting at either side of subarray
    let mut i = left as i32 - 1;
    let mut j = right as i32 + 1;
    
    // Loop until pointers meet
    loop {
        // Move left pointer right until it reaches element greater than or equal to pivot
        i += 1;
        while arr[i as usize] < pivot {
            i += 1;
        }
        // Move right pointer left until it reaches element less than or equal to pivot
        j -= 1;
        while arr[j as usize] > pivot {
            j -= 1;
        }
        // Return index of right pointer when pointers cross
        if i >= j {
            return j as usize;
        }

        // Swap elements pointed to by pointers
        arr.swap(i as usize, j as usize);
    }
}

// Regular insertion sort
fn insertion_sort(arr: &mut [i32]) {
    // Go through array starting from second element
    for i in 1..arr.len() {
        let mut j = i;
        // Go backwards from current element, swapping adjacent elements if they are out of order
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut arr: [i32; 0] = [];
        quicksort(&mut arr);
        assert_eq!(&arr, &[]);
    }

    #[test]
    fn one_element() {
        let mut arr = [1];
        quicksort(&mut arr);
        assert_eq!(&arr, &[1]);
    }

    #[test]
    fn repeated() {
        let mut arr = [5, 2, 1, 3, 5, 4, 5];
        quicksort(&mut arr);
        assert_eq!(&arr, &[1, 2, 3, 4, 5, 5, 5]);
    }

    #[test]
    fn sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn reverse_sorted() {
        let mut arr = [5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn negatives() {
        let mut arr = [-5, 2, 1, -3, 0, 4, -2];
        quicksort(&mut arr);
        assert_eq!(&arr, &[-5, -3, -2, 0, 1, 2, 4]);
    }
}