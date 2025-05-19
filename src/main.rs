use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    println!("please input your guess");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        let mut guess = String::new();

        // Permet de lire l'input de l'utilisateur
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read ligne");


        let guess: u32 = guess.trim().parse().expect("Please try a number");
        // {} permet de dire la ou je dois mettre la variable
        println!("You guessed: {} ", guess);
        

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
            Ordering::Greater => println!("Too Big"),
        }
    }
}
