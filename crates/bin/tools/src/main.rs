use lib_data::create_database_pool;
use tools::MIGRATOR;

#[derive(serde::Deserialize)]
struct Configuration {
    pub database_url: String,
}

#[derive(Debug)]
enum Error {
    Sqlx(sqlx::migrate::MigrateError),
    Json(serde_json::Error),
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Sqlx(e) => write!(f, "Sqlx error: {}", e),
            Error::Json(e) => write!(f, "Json error: {}", e),
            Error::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}
impl std::error::Error for Error {}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let configuration = std::fs::read_to_string("config.json").map_err(Error::Io)?;
    let configuration: Configuration = serde_json::from_str(&configuration).map_err(Error::Json)?;

    let pool = create_database_pool(&configuration.database_url)
        .await
        .expect("Could not initialize database pool instance");

    println!("Initialized database pool");

    MIGRATOR.run(&pool).await.map_err(Error::Sqlx)?;

    println!("Applied migrations successfully");

    Ok(())
}
