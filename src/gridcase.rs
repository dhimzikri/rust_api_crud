use sqlx::{query, MssqlPool};
use serde_json::Value;
use std::collections::HashMap;
use sqlx::Row;

// Function to fetch data dynamically as a HashMap
pub async fn get_tbl_type_dynamic(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    // Base query without any filtering
    let mut base_query = String::from("SELECT TypeID, Description, isactive, usrupd, dtmupd FROM tblType");

    // Dynamically add filtering based on the query and column
    if let Some(query_str) = query {
        if let Some(col_name) = col {
            // Only add filtering if both parameters are provided
            base_query.push_str(&format!(" WHERE {} LIKE '%{}%'", col_name, query_str));
        }
    }

    // Use sqlx::query to execute the query and fetch rows
    let rows = sqlx::query(&base_query)
        .fetch_all(db_pool)
        .await?;

    // Map each row into a HashMap<String, Value>
    let mut result = Vec::new();

    for row in rows {
        let mut row_map = HashMap::new();

        // Extract specific columns from the row
        let typeid: i32 = row.try_get("TypeID")?;
        let description: String = row.try_get("Description")?;
        let isactive: bool = row.try_get("isactive")?;
        let usrupd: String = row.try_get("usrupd")?;
        let dtmupd: Option<String> = row.try_get("dtmupd").ok();  // Optional field

        // Insert the values into the HashMap
        row_map.insert("TypeID".to_string(), Value::Number(typeid.into()));
        row_map.insert("Description".to_string(), Value::String(description));
        row_map.insert("isactive".to_string(), Value::Bool(isactive));
        row_map.insert("usrupd".to_string(), Value::String(usrupd));
        row_map.insert("dtmupd".to_string(), Value::String(dtmupd.unwrap_or_default()));

        result.push(row_map);
    }

    Ok(result)
}
