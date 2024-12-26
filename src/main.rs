#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::Value;
use sqlx::MssqlPool;
use std::collections::HashMap;
use std::env;

mod gridcase;
use gridcase::{get_tbl_type_dynamic, get_contact};

// Generic route handler for fetching data
async fn fetch_data<F>(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
    fetch_fn: F,
) -> Result<Json<Vec<HashMap<String, Value>>>, String>
where
    F: FnOnce(&MssqlPool, Option<String>, Option<String>) -> tokio::task::JoinHandle<Result<Vec<HashMap<String, Value>>, sqlx::Error>>,
{
    match fetch_fn(db_pool.inner(), query, col).await {
        Ok(data) => Ok(Json(data)),
        Err(err) => Err(format!("Failed to fetch data: {}", err)),
    }
}

#[get("/tblType?<query>&<col>")]
async fn fetch_tbl_type(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
) -> Result<Json<Vec<HashMap<String, Value>>>, String> {
    fetch_data(db_pool, query, col, |db, q, c| tokio::spawn(get_tbl_type_dynamic(db, q, c))).await
}

#[get("/get_contact?<query>&<col>")]
async fn fetch_tbl_contact(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
) -> Result<Json<Vec<HashMap<String, Value>>>, String> {
    fetch_data(db_pool, query, col, |db, q, c| tokio::spawn(get_contact(db, q, c))).await
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = MssqlPool::connect_lazy(&database_url).expect("Failed to create database pool");

    rocket::build()
        .manage(db_pool)
        .mount("/", routes![fetch_tbl_type, fetch_tbl_contact])
        .launch()
        .await?;

    Ok(())
}
