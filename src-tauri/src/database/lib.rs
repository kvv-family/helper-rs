use diesel::prelude::*;
use directories::UserDirs;

pub fn establish_connection() -> SqliteConnection {
    let database_url = UserDirs::new()
        .unwrap()
        .home_dir()
        .join("image-helper")
        .join("database.db");
    SqliteConnection::establish(&database_url.to_string_lossy())
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url.to_string_lossy()))
}
