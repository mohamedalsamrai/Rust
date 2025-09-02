use std::io;

fn main() {
    println!("Guess the Number!");
    println!("Pleace input your guess ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("not good :)");
    println!("You gussed : {}",guess);
}
