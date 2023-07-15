fn main() {
    while let Some(number) = read_number() {
        println!("{}", calculator::calculate_plus_three(number));
    }
    println!("Bye!");
}

fn read_number() -> Option<i32> {
    let mut line = String::new();
    println!("Type your number, or \"exit\" to exit.");
    while let Ok(_) = std::io::stdin().read_line(&mut line) {
        line = line.trim().into();
        if let Ok(number) = line.parse::<i32>() {
            return Some(number);
        } else {
            if line == "exit" || line == "Exit" {
                return None;
            }
            println!(
                "\"{}\" is not a number. Try again, or type \"exit\" to exit.",
                line
            );
            line = String::new();
        }
    }
    None
}
