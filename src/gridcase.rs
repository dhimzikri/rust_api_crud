use sqlx::{query, MssqlPool};
use serde_json::Value;
use std::collections::HashMap;

// Query parameters
pub struct QueryParams {
    pub query: Option<String>,
    pub col: Option<String>,
}

// Function to fetch data dynamically as a HashMap
pub async fn get_tbl_type_dynamic(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    // Base query without any filtering
    let mut base_query = String::from("SELECT * FROM tblType");

    // Dynamically add filtering based on the query and column
    if let Some(query_str) = query {
        if let Some(col_name) = col {
            // Only add filtering if both parameters are provided
            base_query.push_str(&format!(" WHERE {} LIKE '%{}%'", col_name, query_str));
        }
    }

    // Execute the query and fetch rows as a Vec<HashMap<String, Value>>
    let rows = sqlx::query(&base_query)
        .fetch_all(db_pool)
        .await?;
    
    // Map each row into a HashMap<String, Value>
    let mut result = Vec::new();

    for row in rows {
        let mut row_map = HashMap::new();

        // Iterate over each column in the row
        for (column_name, value) in row {
            // Add the column name and value (serialized as JSON) to the map
            row_map.insert(column_name, value);
        }

        result.push(row_map);
    }

    Ok(result)
}
