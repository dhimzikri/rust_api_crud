#[macro_use] extern crate rocket;

use rocket::{serde::json::Json, routes};
use sqlx::MssqlPool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Deserialize, Serialize)]  // Add Deserialize here
pub struct QueryParams {
    pub query: Option<String>,
    pub col: Option<String>,
}

#[post("/fetch", format = "json", data = "<params>")]
async fn fetch_data(params: Json<QueryParams>, db_pool: MssqlPool) -> Json<Vec<HashMap<String, Value>>> {
    // Call the `get_tbl_type` function with params
    let result = get_tbl_type(&db_pool, params.query.clone(), params.col.clone()).await;
    Json(result.unwrap_or_else(|_| Vec::new()))  // Return the fetched data
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![fetch_data])
}

