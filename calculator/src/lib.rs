pub fn calculate_plus_three(number: i32) -> String {
    format!("{} + 3 = {}", number, add_three(number))
}

fn add_three(number: i32) -> i32 {
    number + 3
}
