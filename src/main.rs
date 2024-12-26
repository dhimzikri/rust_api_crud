#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use sqlx::MssqlPool;

mod gridcase;

use gridcase::{get_tbl_type, QueryParams};

// Route to fetch tblType data
#[get("/tblType?<query>&<col>", data="<params>")]
async fn fetch_tbl_type(
    db_pool: &State<MssqlPool>,
    params: Json<QueryParams>,
) -> Result<Json<Vec<HashMap<String, Value>>>, String> {
    let result = get_tbl_type(
        &db_pool,
        params.query.clone(),
        params.col.clone(),
    ).await;

    match result {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(format!("Error fetching data: {}", e)),
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
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
