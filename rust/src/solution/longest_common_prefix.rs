use std::str::Chars;
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    let mut characters: Vec<Chars> = strs.iter().map(|c| c.chars()).collect();
    let mut current_char: Option<char> = None;
    if strs.len() < 1 {
        return prefix;
    }
    loop {
        //to push empty char to string
        current_char.take().map(|ch| {
            prefix.push(ch);
        });
        for character in characters.iter_mut() {
            let mut ch = character.next();
            if ch.is_none() {
                return prefix;
            }

            match current_char {
                Some(chare) => {
                    if chare != ch.unwrap() {
                        return prefix;
                    }
                }
                None => current_char = ch.take(),
            }
        }
    }
}
