pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let mut total = 0;
    let t_l = num_str.len();
    if t_l == 1 {
        return true;
    }
    for char in num_str.chars() {
        let c_num = char.to_digit(10).expect("Failed to parse");
        let x = c_num.pow(t_l as u32);
        total += x;
    }
    total == num.into()
}
