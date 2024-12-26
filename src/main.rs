use rocket::{get, post, routes, serde::json::Json, State};
use sqlx::{Mssql, MssqlPool};
use serde_json::Value;
use std::collections::HashMap;
use rocket::tokio;
use rocket::{launch, Rocket, Build};
use rocket::fairing::AdHoc;
use sqlx::mssql::MssqlRow;

// Query parameters structure
#[derive(serde::Deserialize)]
pub struct QueryParams {
    pub query: Option<String>,
    pub col: Option<String>,
}

// Function to fetch data from tblType without using a struct
async fn get_tbl_type(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    let mut base_query = String::from("SELECT * FROM tblType WHERE 1=1");

    // If a query and column are provided, modify the query
    if let Some(query_str) = query {
        if let Some(col_name) = col {
            base_query.push_str(&format!(" AND {} LIKE '%{}%'", col_name, query_str));
        }
    }

    // Execute the query and collect results as a vector of HashMaps
    let rows = sqlx::query(&base_query)
        .fetch_all(db_pool)
        .await?;

    // Map the rows into a vector of HashMaps (key: column name, value: column value)
    let mut result = Vec::new();
    for row in rows {
        let mut row_map = HashMap::new();
        for (column, value) in row.into_iter() {
            row_map.insert(column, value);
        }
        result.push(row_map);
    }

    Ok(result)
}

// Rocket route to get tblType data
#[get("/getTblType?<query>&<col>")]
async fn get_tbl_type_route(
    db_pool: &State<MssqlPool>,
    query: Option<String>,
    col: Option<String>,
) -> Json<Result<Vec<HashMap<String, Value>>, String>> {
    match get_tbl_type(&db_pool, query, col).await {
        Ok(data) => Json(Ok(data)),
        Err(err) => Json(Err(format!("Error: {}", err))),
    }
}

// Rocket application setup
#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(
            MssqlPool::connect("mssql://sa:pass,123@172.16.6.31/Portal_HelpDesk_CS")
                .await
                .expect("Error connecting to the database"),
        )
        .mount("/", routes![get_tbl_type_route])
}
