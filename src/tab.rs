use std::io;

pub fn tab() {
    let mut to_do: Vec<String> = Vec::new();

    println!("Ajout dans la Todo");
    println!("Print 'exit' pour ne plus ajouter dans la ToDo");

    loop {
        let mut to_do_add = String::new();

        // Permet de lire l'input de l'utilisateur
        io::stdin()
            .read_line(&mut to_do_add)
            .expect("Failed to read ligne");

        if to_do_add.trim() == "exit" {
            break;
        }
        to_do.push(to_do_add);
    }

    println!("Affichage de la Todo");
    for test in &to_do {
        println!("{}", test);
    }
}
