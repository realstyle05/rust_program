mod database;
mod infos;
use std::io;
use rusqlite::{Connection, Result}; // Importer la librairie rusqlite
use database::sqlite::initiate_db; // Importer la fonction initiate_db du module database::sqlite
use infos::users::get_user;
use infos::score::show_all_scores;
use infos::game::{GAMES, play_game};


fn main() -> Result<()> {

    let conn = Connection::open("scores.sqlite").expect("Erreur lors de l'ouverture de la base de données"); // Ouvrir la connexion à la base de données
    initiate_db(&conn); // Initialiser la base de données

    // Recuperation ou création user dans la fonction main
    println!("Veuillez entrer votre prénom :");
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).expect("Erreur lors de la lecture de l'entrée");
    let user_name = user_name.trim().to_string();
    let user = get_user(&conn, &user_name);

    loop {
        // Afficher le menu principal
        println!("1. Jouer au jeu 1");
        println!("2. Jouer au jeu 2");
        println!("3. Jouer au jeu 3");
        println!("4. Afficher les scores");
        println!("5. Quitter");

        // Lire l'entrée de l'utilisateur
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Erreur lors de la lecture de l'entrée");

        // Convertir le choix en nombre entier
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide.");
                continue; // Recommencer la boucle
            }
        };

        // Exécuter la fonction correspondant au choix de l'utilisateur

        // Dans la fonction main
        match choice {
            1 => match play_game(&conn, &user, GAMES[0]) {
                Ok(score) => println!("Votre score est de : {}", score),
                Err(e) => println!("{}", e),
            },
            2 => match play_game(&conn, &user, GAMES[1]) {
                Ok(score) => println!("Votre score est de : {}", score),
                Err(e) => println!("{}", e),
            },
            3 => match play_game(&conn, &user, GAMES[2]) {
                Ok(score) => println!("Votre score est de : {}", score),
                Err(e) => println!("{}", e),
            },
            4 => show_all_scores(&conn, &user)?,
            5 => {
                println!("Au revoir !");
                break Ok(());
            }
            _ => println!("Choix non valide. Veuillez entrer un nombre entre 1 et 5."),
        }
    }
}
