use sqlx::{MssqlPool, Row};
use serde_json::Value;
use std::collections::HashMap;

pub async fn get_tbl_type_dynamic(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    let mut base_query = "SELECT TypeID, Description, isactive, usrupd, CONVERT(VARCHAR, dtmupd, 120) AS dtmupd FROM tblType".to_string();

    if let (Some(query), Some(col)) = (query, col) {
        base_query.push_str(&format!(" WHERE {} LIKE '%{}%'", col, query));
    }

    let rows = sqlx::query(&base_query).fetch_all(db_pool).await?;
    Ok(rows.into_iter().map(map_tbl_type_row).collect())
}

fn map_tbl_type_row(row: sqlx::Row) -> HashMap<String, Value> {
    let dtmupd = row.try_get::<Option<String>, _>("dtmupd").unwrap_or(None)
        .unwrap_or_default();

    let formatted_dtmupd = NaiveDateTime::parse_from_str(&dtmupd, "%Y-%m-%d %H:%M:%S")
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default();

    let mut row_map = HashMap::new();
    row_map.insert("TypeID".to_string(), Value::Number(row.try_get("TypeID").unwrap().into()));
    row_map.insert("Description".to_string(), Value::String(row.try_get("Description").unwrap()));
    row_map.insert("isactive".to_string(), Value::Bool(row.try_get("isactive").unwrap()));
    row_map.insert("usrupd".to_string(), Value::String(row.try_get("usrupd").unwrap()));
    row_map.insert("dtmupd".to_string(), Value::String(formatted_dtmupd));

    row_map
}

pub async fn get_contact(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    let mut base_query = "SELECT contactid, Description, isactive, usrupd, CONVERT(VARCHAR, dtmupd, 120) AS dtmupd FROM Contact".to_string();

    if let (Some(query), Some(col)) = (query, col) {
        base_query.push_str(&format!(" WHERE {} LIKE '%{}%'", col, query));
    }

    let rows = sqlx::query(&base_query).fetch_all(db_pool).await?;
    Ok(rows.into_iter().map(map_contact_row).collect())
}

fn map_contact_row(row: sqlx::Row) -> HashMap<String, Value> {
    let dtmupd = row.try_get::<Option<String>, _>("dtmupd").unwrap_or(None)
        .unwrap_or_default();

    let formatted_dtmupd = NaiveDateTime::parse_from_str(&dtmupd, "%Y-%m-%d %H:%M:%S")
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default();

    let mut row_map = HashMap::new();
    row_map.insert("contactid".to_string(), Value::String(row.try_get("contactid").unwrap()));
    row_map.insert("Description".to_string(), Value::String(row.try_get("Description").unwrap()));
    row_map.insert("isactive".to_string(), Value::Bool(row.try_get("isactive").unwrap()));
    row_map.insert("usrupd".to_string(), Value::String(row.try_get("usrupd").unwrap()));
    row_map.insert("dtmupd".to_string(), Value::String(formatted_dtmupd));

    row_map
}
