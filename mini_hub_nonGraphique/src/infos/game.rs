use crate::infos::score::add_score;
use crate::infos::users::User;
use rusqlite::Connection;
use jeu1::play_game1;
use jeu2::play_game2;
use jeu3::play_game3;

struct Game {
    id: i64,
    name: String,
}


pub fn add_game(conn: &Connection, game_name: &str) {
    if get_game_by_name(conn, game_name).is_none() {
        conn.execute("INSERT INTO games (name) VALUES (?1)", &[&game_name])
            .expect("Erreur lors de l'ajout du jeu");
    }
}

// Définir une fonction pour chaque jeu
pub fn play_game(conn: &Connection, user: &User, game_name_to_search: &str, ) -> Result<i64, &'static str> {
    // recuperer jeu dans BDD
    if let Some(game) = get_game_by_name(&conn, game_name_to_search) {
        println!("Lancement du jeu pour l'utilisateur {} !", user.name);
        // Logique du jeu à lancer
        let score: i64;
        match game.name.as_str() {
            // score temporaire, il faudra plus tard faire appel à la librairie externe !
            "jeu1" => {
                score = play_game1();
            }
            // score temporaire, il faudra plus tard faire appel à la librairie externe !
            "jeu2" => {
                score = play_game2();
            }
            // score temporaire, il faudra plus tard faire appel à la librairie externe !
            "jeu3" => {
                score = play_game3();
            }
            _ => return Err("Game not found!"),
        }

        // Enregistrement du score dans la base de données
        add_score(&conn, user.id, game.id, score);
        Ok(score)
    } else {
        return Err("Game not found!");
    }
}

fn get_game_by_name(conn: &Connection, name: &str) -> Option<Game> {
    conn.query_row(
        "SELECT id, name FROM games WHERE name = ?1",
        &[&name],
        |row| {
            Ok(Game {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        },
    )
        .ok()
}

pub const GAMES: [&str; 3] = ["jeu1", "jeu2", "jeu3"];