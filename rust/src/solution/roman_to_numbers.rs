pub fn roman_to_number(s: String) -> i32 {
    println!("Hello, world!");
    let mut sum = 0;
    let roman_num = s.chars().collect::<Vec<_>>();
    let mut counter = 0;
    while counter < roman_num.len() {
        let first_element = get_value(roman_num[counter]);
        if counter + 1 < roman_num.len() {
            let second_element = get_value(roman_num[counter + 1]);
            if first_element >= second_element {
                sum = sum + first_element;
            } else {
                sum = sum + second_element - first_element;
                counter += 1;
            }
        } else {
            sum = sum + first_element;
        }
        counter += 1;
    }
    sum
}

fn get_value(num: char) -> i32 {
    match num {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}
