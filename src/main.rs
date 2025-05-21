use std::io;
mod tab;
mod guessing_game;

fn main() {

    println!("Veuillez choisir la fonctionnalite de l'application");
    println!("1 : ToDo ");
    println!("2 : Guessing game");

    let mut choix = String::new();

    io::stdin()
            .read_line(&mut choix)
            .expect("Failed to read ligne");

    let choix = choix.trim();

    if choix == "1" {
        tab::tab();
    }else if choix == "2" {
        guessing_game::guessing_game();
    }else {
        println!("Fermeture de l'application");
    }

}
