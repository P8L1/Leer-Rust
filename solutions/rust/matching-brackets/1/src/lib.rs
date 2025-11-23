pub fn brackets_are_balanced(input: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in input.chars() {
        match ch {
            '(' | '[' | '{' => {
                stack.push(ch);
            }
            ')' | ']' | '}' => {
                let open = match stack.pop() {
                    Some(c) => c,
                    None => return false,
                };

                if !is_matching_pair(open, ch) {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

fn is_matching_pair(open: char, close: char) -> bool {
    matches!(
        (open, close),
        ('(', ')') | ('[', ']') | ('{', '}')
    )
}
