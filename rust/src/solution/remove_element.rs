pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }
    let mut size = 0;
    for i in 0..len {
        if nums[i] != val {
            nums[size] = nums[i];
            size += 1;
        }
    }

    size as i32
}
