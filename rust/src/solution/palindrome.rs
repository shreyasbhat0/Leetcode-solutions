pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut digits: Vec<i32> = Vec::new();
    let mut input = x;
    while input != 0 {
        digits.push(input % 10);
        input = input / 10;
    }

    let length_of_digits = digits.len();

    if length_of_digits < 2 {
        return true;
    }

    if digits[0] == 0 {
        return false;
    }

    let mut i = 0;
    while i < length_of_digits / 2 {
        if digits[i] != digits[length_of_digits - 1 - i] {
            return false;
        }
        i += 1;
    }

    true
}
