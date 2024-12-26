use sqlx::{query, MssqlPool};
use serde_json::Value;
use std::collections::HashMap;

// Query parameters
pub struct QueryParams {
    pub query: Option<String>,
    pub col: Option<String>,
}

// Function to fetch data from tblType without using a struct
pub async fn get_tbl_type(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    let mut base_query = String::from("SELECT * FROM tblType");

    // If a query and column are provided, modify the query
    if let Some(query_str) = query {
        if let Some(col_name) = col {
            base_query.push_str(&format!(" AND {} LIKE '%{}%'", col_name, query_str));
        }
    }

    // Execute the query and collect results as a vector of HashMaps
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
