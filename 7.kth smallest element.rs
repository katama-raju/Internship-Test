fn kth_smallest_element(nums: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= nums.len() {
        let mut sorted_nums = nums.to_vec();
        sorted_nums.sort();
        Some(sorted_nums[k - 1])
    } else {
        None
    }
}

fn main() {
    let array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let k = 3;
    match kth_smallest_element(&array, k) {
        Some(result) => println!("The {}th smallest element is: {}", k, result),
        None => println!("Invalid value of k"),
    }
}
