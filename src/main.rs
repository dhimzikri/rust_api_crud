#[macro_use] extern crate rocket;

use rocket::{serde::json::Json, State};
use sqlx::MssqlPool;
use serde_json::Value;
use crate::gridCase::{get_tbl_type, QueryParams}; // Import from the gridCase module

#[post("/get_case_data", data = "<params>")]
async fn get_case_data(
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_case_data]) // Mount the route
}
