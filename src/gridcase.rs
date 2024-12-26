use sqlx::{query, MssqlPool};
use sqlx::Row;
use serde_json::Value;
use std::collections::HashMap;
use chrono::NaiveDateTime;

pub async fn get_tbl_type_dynamic(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    let mut base_query = String::from(
        "SELECT TypeID, Description, isactive, usrupd, CAST(dtmupd AS VARCHAR) as dtmupd FROM tblType"
    );

    if let Some(query_str) = query {
        if let Some(col_name) = col {
            base_query.push_str(&format!(" WHERE {} LIKE '%{}%'", col_name, query_str));
        }
    }

    // Perform the query and retrieve rows
    let rows = sqlx::query(&base_query)
        .fetch_all(db_pool)
        .await?;

    let mut result = Vec::new();

    for row in rows {
        let mut row_map = HashMap::new();

        // Extract specific columns from the row
        let typeid: i32 = row.try_get("TypeID")?;
        let description: String = row.try_get("Description")?;
        let isactive: bool = row.try_get("isactive")?;
        let usrupd: String = row.try_get("usrupd")?;

        // Handle `dtmupd`: Parse or fallback to an empty string
        let dtmupd: String = match row.try_get::<Option<String>, _>("dtmupd")? {
            Some(dtm) => {
                NaiveDateTime::parse_from_str(&dtm, "%Y-%m-%d %H:%M:%S")
                    .map(|dt| dt.to_string())
                    .unwrap_or_else(|_| "".to_string()) // Handle invalid datetime formats
            }
            None => "".to_string(), // Handle NULL case
        };

        // Insert the values into the HashMap
        row_map.insert("TypeID".to_string(), Value::Number(typeid.into()));
        row_map.insert("Description".to_string(), Value::String(description));
        row_map.insert("isactive".to_string(), Value::Bool(isactive));
        row_map.insert("usrupd".to_string(), Value::String(usrupd));
        row_map.insert("dtmupd".to_string(), Value::String(dtmupd));

        result.push(row_map);
    }

    Ok(result)
}
