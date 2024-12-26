#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use sqlx::MssqlPool;
use std::collections::HashMap;  // Import HashMap from std::collections
use serde_json::Value;  // Import Value from serde_json

mod gridcase;

use gridcase::get_tbl_type_dynamic;  // Import the updated function

// Route to fetch tblType data
#[get("/tblType?<query>&<col>")]
async fn fetch_tbl_type(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
) -> Result<Json<Vec<HashMap<String, Value>>>, String> {  // Use HashMap here
    match get_tbl_type_dynamic(db_pool.inner(), query, col).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(format!("Failed to fetch data: {}", err)),
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Database connection string
    let database_url = "mssql://sa:pass,123@172.16.6.31:1433/Portal_HelpDesk_CS";

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
