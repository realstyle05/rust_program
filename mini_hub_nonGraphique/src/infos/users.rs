use rusqlite::Connection;

// Structure pour représenter un utilisateur
pub struct User {
    pub id: i64,
    pub name: String,
}

// Fonction pour récupérer l'ID de l'utilisateur s'il existe
pub fn get_user(conn: &Connection, user_name: &str) -> User {
    let option: Option<i64> = conn
        .query_row(
            "SELECT id FROM users WHERE name = ?1",
            &[&user_name],
            |row| row.get(0),
        )
        .ok();

    match option {
        Some(id) => {
            return User {
                id,
                name: user_name.to_string(),
            }
        }
        None => {
            return User {
                id: create_user(&conn, &user_name),
                name: user_name.to_string(),
            }
        }
    }
}

// Fonction pour créer un nouvel utilisateur et récupérer son ID
fn create_user(conn: &Connection, user_name: &str) -> i64 {
    conn.execute("INSERT INTO users (name) VALUES (?1)", &[&user_name])
        .expect("Erreur lors de la création de l'utilisateur");
    conn.last_insert_rowid()
}