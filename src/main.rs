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
use gridcase::{get_tbl_type_dynamic,get_contact,readgettblSubType,readgetBranchID};  // Import the updated function

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

#[get("/readgettblSubType?<query>&<col>")]
async fn readSubType(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
    typeid: i32,
) -> Result<Json<Vec<HashMap<String, Value>>>, String> {  // Use HashMap here
    match readgettblSubType(db_pool.inner(), query, col, typeid).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(format!("Failed to fetch data: {}", err)),
    }
}
#[get("/getBranch?<query>&<col>")]
async fn readBranch(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
    branchid: i32,
) -> Result<Json<Vec<HashMap<String, Value>>>, String> {  // Use HashMap here
    match readgetBranchID(db_pool.inner(), query, col, branchid).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(format!("Failed to fetch data: {}", err)),
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Load environment variables from .env file
    dotenv().ok();

    // Fetch the database URL from the environment variable
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment");

    // Create a database connection pool
    let db_pool = MssqlPool::connect_lazy(&database_url)
        .expect("Failed to create database pool");

    // Launch the Rocket application
    rocket::build()
        .manage(db_pool)
        .mount("/", routes![fetch_tbl_type, fetch_tbl_contact, readSubType, readBranch])
        .launch()
        .await?;

    Ok(())
}