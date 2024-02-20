fn find_median(nums: &[i32]) -> f64 {
    let len = nums.len();
    
    if len % 2 == 0 {
         let mid1 = len / 2 - 1;
        let mid2 = len / 2;
        return (nums[mid1] as f64 + nums[mid2] as f64) / 2.0;
    } else {
         let mid = len / 2;
        return nums[mid] as f64;
    }
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 5];
    let median = find_median(&sorted_array);
    println!("Median: {}", median);

    let another_array = vec![1, 2, 3, 4, 5, 6];
    let another_median = find_median(&another_array);
    println!("Median: {}", another_median);
}
