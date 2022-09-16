pub fn is_valid(s: String) -> bool {
    let mut characters: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => characters.push(c),
            ')' | '}' | ']' => match characters.pop() {
                Some(poped) => {
                    if !(((poped == '(') && (c == ')'))
                        || ((poped == '{') && (c == '}'))
                        || ((poped == '[') && (c == ']')))
                    {
                        return false;
                    }
                }
                None => return false,
            },
            _ => {}
        }
    }
    characters.is_empty()
}
