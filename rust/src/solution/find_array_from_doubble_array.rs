pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
    let mut original_array: Vec<i32> = Vec::new();
    let mut doub = changed.clone();
    let (first, second) = doub.split_at_mut(changed.len() / 2);
    first.sort();
    second.sort();

    if first.len() != second.len() {
        return original_array;
    }
    if (first.len() == 1) && (second.len() == 1) {
        if (first[0] != 0) && (second[0] != 0) {
            if second[0] > first[0] {
                if second[0] / first[0] == 2 {
                    original_array.push(first[0]);
                    return original_array;
                }
            } else {
                if (first[0] / second[0]) == 2 {
                    original_array.push(second[0]);
                    return original_array;
                }
            }
        } else {
            return vec![];
        }
    }

    for i in 0..second.len() {
        if (first[i] != 0) && (second[i] != 0) {
            if second[i] > first[i] {
                if second[i] / first[i] == 2 {
                    original_array.push(first[i])
                }
            } else {
                if first[i] / second[i] == 2 {
                    original_array.push(second[i])
                }
            }
        } else {
            original_array.push(0)
        }
    }

    if original_array.len() != second.len() {
        return vec![];
    }

    original_array
}
