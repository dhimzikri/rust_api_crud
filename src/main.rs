#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;

mod gridcase;

use gridcase::{get_tbl_type, QueryParams, TblType};

// Route to fetch tblType data
#[get("/tblType?<query>&<col>")]
async fn fetch_tbl_type(
    db_pool: &State<PgPool>,
    query: Option<String>,
    col: Option<String>,
) -> Result<Json<Vec<TblType>>, String> {
    // Call the `get_tbl_type` function in gridcase.rs
    match get_tbl_type(db_pool.inner(), query, col).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(format!("Failed to fetch data: {}", err)),
    }
}

#[launch]
fn rocket() -> _ {
    // Database connection string
    let database_url = "postgres://username:password@localhost/database_name";

    // Create a database connection pool
    let db_pool = PgPool::connect_lazy(database_url).expect("Failed to create database pool");

    // Launch the Rocket application
    rocket::build()
        .manage(db_pool)
        .mount("/", routes![fetch_tbl_type])
}
