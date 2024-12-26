use sqlx::{query, MssqlPool};
use serde_json::Value;
use std::collections::HashMap;

// Query parameters structure
pub struct QueryParams {
    pub query: Option<String>,
    pub col: Option<String>,
}

// Function to fetch data from tblType without using a struct
pub async fn get_tbl_type(
    db_pool: &MssqlPool,
    query_str: Option<String>,  // Renamed variable to avoid conflict with sqlx::query
    col: Option<String>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    let mut base_query = String::from("SELECT * FROM tblType WHERE 1=1");

    // If query and column parameters are provided, modify the query
    if let Some(query_value) = query_str {
        if let Some(col_name) = col {
            // Safely format the query by unwrapping Options
            base_query.push_str(&format!(" AND {} LIKE '%{}%'", col_name, query_value));
        }
    }

    // Execute the query to fetch rows from the database using sqlx::query
    let rows = query(&base_query)
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
