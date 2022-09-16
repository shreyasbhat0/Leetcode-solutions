pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let len = nums.len();
    let mut last = nums[0];
    let mut size = 1;
    for i in 1..len {
        if nums[i] != last {
            last = nums[i];
            nums[size] = nums[i];
            size += 1;
        }
    }

    size as i32
}
