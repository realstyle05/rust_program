use rusqlite::Connection;
use crate::infos::game::{add_game, GAMES};
pub fn initiate_db(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            )",
        (),
    )
        .expect("Erreur lors de la création de la table users");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        (),
    )
        .expect("Erreur lors de la création de la table games");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS scores (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            game_id INTEGER NOT NULL,
            score INTEGER NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (game_id) REFERENCES games(id)
        )",
        (),
    )
        .expect("Erreur lors de la création de la table scores");
    for game in GAMES {
        add_game(&conn, game);
    }
}

