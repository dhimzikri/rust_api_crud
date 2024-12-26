use sqlx::{query, MssqlPool};
use serde_json::Value;
use std::collections::HashMap;

pub async fn get_tbl_type_dynamic(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    let mut base_query = String::from("SELECT * FROM tblType");

    if let Some(query_str) = query {
        if let Some(col_name) = col {
            base_query.push_str(&format!(" WHERE {} LIKE '%{}%'", col_name, query_str));
        }
    }

    // Perform the query and retrieve rows
    let rows = sqlx::query(&base_query)
        .fetch_all(db_pool)
        .await?;

    // Process the rows into a dynamic structure
    let mut result = Vec::new();
    for row in rows {
        let mut row_map = HashMap::new();

        // Get all columns from the row dynamically
        for column in row.columns() {
            let column_name = column.name().to_string();
            let column_value: Value = match row.try_get::<Value, _>(&column_name) {
                Ok(value) => value,
                Err(_) => Value::Null, // If there's an error (like unsupported types), set to Null
            };

            row_map.insert(column_name, column_value);
        }

        result.push(row_map);
    }

    Ok(result)
}
