fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;

        if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    if low < arr.len() && arr[low] == target {
        Some(low)
    } else {
        None
    }
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 9];
    let target_number = 4;

    match find_first_occurrence(&sorted_array, target_number) {
        Some(index) => println!("The first occurrence of {} is at index {}", target_number, index),
        None => println!("{} is not present in the array", target_number),
    }
}
