// checkpoint1 
#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use std::env;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::MssqlPool;
use std::collections::HashMap;  // Import HashMap from std::collections
use serde_json::Value;  // Import Value from serde_json

mod gridcase;
// use gridcase::{get_tbl_type_dynamic,get_contact,readgettblSubType,readgetBranchID};  // Import the updated function
use gridcase::{get_tbl_type_dynamic,get_contact,readgettblSubType,getCase};  // Import the updated function

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

#[get("/get_contact?<query>&<col>")]
async fn fetch_tbl_contact(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
) -> Result<Json<Vec<HashMap<String, Value>>>, String> {  // Use HashMap here
    match get_contact(db_pool.inner(), query, col).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(format!("Failed to fetch data: {}", err)),
    }
}

// checkpoint2-final
#[get("/readgettblSubType?<query>&<col>&<typeid>")]
async fn readSubType(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
    typeid: Option<i32>, // Mark typeid as optional
) -> Result<Json<Vec<HashMap<String, Value>>>, String> {
    // Ensure typeid is provided
    let typeid = typeid.ok_or_else(|| "Missing required parameter: typeid".to_string())?;

    // Call the database function
    match readgettblSubType(db_pool.inner(), query, col, typeid).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(format!("Failed to fetch data: {}", err)),
    }
}

#[get("/grid_case?<query>&<col>&<start>&<limit>")]
pub async fn grid_case(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
    start: Option<i32>,
    limit: Option<i32>,
) -> Result<Json<Vec<HashMap<String, Value>>>, String> {
    match getCase(db_pool.inner(), query, col, start, limit).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(format!("Failed to fetch data: {}", err)),
    }
}

// #[get("/getBranch?<query>&<col>")]
// async fn readBranch(
//     db_pool: &State<MssqlPool>,
//     query: Option<String>,
//     col: Option<String>,
//     branchid: i32,
// ) -> Result<Json<Vec<HashMap<String, Value>>>, String> {  // Use HashMap here
//     match readgetBranchID(db_pool.inner(), query, col, branchid).await {
//         Ok(data) => Ok(Json(data)),
//         Err(err) => Err(format!("Failed to fetch data: {}", err)),
//     }
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Load environment variables from .env file
    dotenv().ok();

    // Fetch the database URL from the environment variable
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment");

    // Create a database connection pool
    let db_pool = MssqlPool::connect(&database_url).await?;
        // .expect("Failed to create database pool");

    // Launch the Rocket application
    rocket::build()
        .manage(db_pool)
        // .mount("/", routes![fetch_tbl_type, fetch_tbl_contact, readSubType, readBranch])
        .mount("/", routes![fetch_tbl_type, fetch_tbl_contact,readSubType,grid_case])
        .launch()
        .await?;

    Ok(())
}