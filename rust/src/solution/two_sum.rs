use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for (first_index, &num) in nums.iter().enumerate() {
        if let Some(&second_index) = hm.get(&(target - num)) {
            return vec![second_index, first_index as i32];
        } else {
            hm.insert(num, first_index as i32);
        }
    }
    vec![]
}
