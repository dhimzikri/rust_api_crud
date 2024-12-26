#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use sqlx::MssqlPool;

mod gridcase;

use gridcase::{get_tbl_type, QueryParams, TblType};

// Route to fetch tblType data
#[get("/tblType?<query>&<col>")]
async fn fetch_tbl_type(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
) -> Result<Json<Vec<TblType>>, String> {
    match get_tbl_type(db_pool.inner(), query, col).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(format!("Failed to fetch data: {}", err)),
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = rocket::build()
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(), // Bind to all interfaces
            port: 8585,                         // Use port 8080
            ..Default::default()
        });
    // Database connection string
    let database_url = "mssql://sa:pass,123@172.16.6.31/Portal_HelpDesk_CS";

    // Create a database connection pool
    let db_pool = MssqlPool::connect_lazy(database_url).expect("Failed to create database pool");

    // Launch the Rocket application
    rocket::build()
        .manage(db_pool)
        .mount("/", routes![fetch_tbl_type])
        .launch()
        .await?;

    Ok(())
}
