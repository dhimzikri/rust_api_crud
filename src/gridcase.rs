use sqlx::{query, MssqlPool};
use sqlx::Row;  // Import the Row trait
use serde_json::Value;
use std::collections::HashMap;
use chrono::NaiveDateTime;

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

    // Map each row into a HashMap<String, Value>
    let mut result = Vec::new();

    for row in rows {
        let mut row_map = HashMap::new();

        // Extract specific columns from the row
        let typeid: i32 = row.try_get("TypeID")?;
        let description: String = row.try_get("Description")?;
        let isactive: bool = row.try_get("isactive")?;
        let usrupd: String = row.try_get("usrupd")?;
        
        // Handling DateTime column
        let dtmupd: Option<NaiveDateTime> = row.try_get("dtmupd").ok();  // Convert to NaiveDateTime

        // Insert the values into the HashMap
        row_map.insert("TypeID".to_string(), Value::Number(typeid.into()));
        row_map.insert("Description".to_string(), Value::String(description));
        row_map.insert("isactive".to_string(), Value::Bool(isactive));
        row_map.insert("usrupd".to_string(), Value::String(usrupd));
        
        // Handle the dtmupd field (convert to string)
        let dtmupd_value = match dtmupd {
            Some(dt) => Value::String(dt.to_string()), // Convert to String
            None => Value::String("".to_string()),  // If None, default to empty string
        };
        
        row_map.insert("dtmupd".to_string(), dtmupd_value);

        result.push(row_map);
    }

    Ok(result)
}
