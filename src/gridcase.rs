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
        "SELECT TypeID, Description, isactive, usrupd, CONVERT(VARCHAR, dtmupd, 120) as dtmupd FROM tblType"
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
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()) // Ensure the format matches 2020-12-06 18:55:30
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

// ===========================================================================
pub async fn get_contact(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    let mut base_query = String::from(
        "SELECT contactid, Description, isactive, usrupd, CONVERT(VARCHAR, dtmupd, 120) as dtmupd FROM Contact"
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
        let contactid: String = row.try_get("contactid")?;
        let description: String = row.try_get("Description")?;
        let isactive: bool = row.try_get("isactive")?;
        let usrupd: String = row.try_get("usrupd")?;

        // Handle `dtmupd`: Parse or fallback to an empty string
        let dtmupd: String = match row.try_get::<Option<String>, _>("dtmupd")? {
            Some(dtm) => {
                NaiveDateTime::parse_from_str(&dtm, "%Y-%m-%d %H:%M:%S")
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()) // Ensure the format matches 2020-12-06 18:55:30
                    .unwrap_or_else(|_| "".to_string()) // Handle invalid datetime formats
            }
            None => "".to_string(), // Handle NULL case
        };

        // Insert the values into the HashMap
        row_map.insert("contactid".to_string(), Value::String(contactid));
        row_map.insert("Description".to_string(), Value::String(description));
        row_map.insert("isactive".to_string(), Value::Bool(isactive));
        row_map.insert("usrupd".to_string(), Value::String(usrupd));
        row_map.insert("dtmupd".to_string(), Value::String(dtmupd));

        result.push(row_map);
    }

    Ok(result)
}

pub async fn readgettbl_subtype(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
    typeid: i32,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    let mut base_query = String::from(
        "SELECT subtypeid, subdescription, typeid, isactive, cost_center, estimasi, usrupd, dtmupd FROM tblSubType WHERE typeid = {} AND isactive = 1",
        typeid
    );

    if let (Some(query_str), Some(col_name)) = (query, col) {
        base_query.push_str(&format!(" AND {} LIKE '%{}%'", col_name, query_str));
    }

    let rows = sqlx::query(&base_query).fetch_all(db_pool).await?;
    
    let mut result = ResultData {
        success: false,
        total: None,
        data: None,
    };

    if !rows.is_empty() {
        let mut msg = Vec::new();

        for row in rows {
            let subtypeid: i32 = row.try_get("subtypeid")?;
            let subdescription: String = row.try_get("subdescription")?;
            let typeid: i32 = row.try_get("typeid")?;
            let cost_center: String = row.try_get("cost_center")?;
            let estimasi: i32 = row.try_get("estimasi")?;
            let isactive: bool = row.try_get("isactive")?;
            let usrupd: String = row.try_get("usrupd")?;
            // Handle `dtmupd`: Parse or fallback to an empty string
            let dtmupd: String = match row.try_get::<Option<String>, _>("dtmupd")? {
                Some(dtm) => {
                    NaiveDateTime::parse_from_str(&dtm, "%Y-%m-%d %H:%M:%S")
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()) // Ensure the format matches 2020-12-06 18:55:30
                        .unwrap_or_else(|_| "".to_string()) // Handle invalid datetime formats
                }

            let mut row_map = HashMap::new();
            row_map.insert("subtypeid".to_string(), Value::Number(subtypeid.into()));
            row_map.insert("subdescription".to_string(), Value::String(subdescription));
            row_map.insert("typeid".to_string(), Value::Number(typeid.into()));
            row_map.insert("cost_center".to_string(), Value::String(cost_center));
            row_map.insert("estimasi".to_string(), Value::Number(estimasi));
            row_map.insert("isactive".to_string(), Value::Bool(isactive));
            row_map.insert("usrupd".to_string(), Value::String(usrupd));
            row_map.insert("dtmupd".to_string(), Value::String(dtmupd));

            msg.push(row_map);
        }

        result.success = true;
        result.total = Some(rows.len() as i64);
        result.data = Some(msg);
    }

    Ok(serde_json::to_string(&result)?)
}