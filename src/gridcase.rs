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

// pub async fn readgettblSubType(
//     db_pool: &MssqlPool,
//     query: Option<String>,
//     col: Option<String>,
//     typeid: i32, // Adding typeid as a parameter
// ) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
//     // Start with the base query
//     let mut base_query = String::from(
//         "SELECT SubTypeID, SubDescription, TypeID, cost_center, estimasi, isactive, usrupd, 
//         CONVERT(VARCHAR, dtmupd, 120) as dtmupd FROM tblSubType WHERE typeid = ? AND isactive = 1",
//     );

//     // If query and column are provided, append the filtering logic
//     if let (Some(query_str), Some(col_name)) = (query, col) {
//         base_query.push_str(&format!(" AND {} LIKE ?", col_name));
//     }

//     // Perform the query with parameters to avoid syntax errors and prevent SQL injection
//     let mut query = sqlx::query(&base_query).bind(typeid); // Bind typeid first

//     // If a search query is provided, bind the value for the LIKE search
//     if let Some(query_str) = query {
//         query = query.bind(format!("%{}%", query_str)); // Bind the search term with % for LIKE
//     }

//     // Execute the query and fetch all rows
//     let rows = query.fetch_all(db_pool).await?;

//     // Process the results into a vector of HashMaps
//     let mut result = Vec::new();

//     for row in rows {
//         let mut row_map = HashMap::new();
        
//         row_map.insert("subtypeid".to_string(), Value::Number(row.try_get("SubTypeID")?.into()));
//         row_map.insert("subdescription".to_string(), Value::String(row.try_get("SubDescription")?));
//         row_map.insert("typeid".to_string(), Value::Number(row.try_get("TypeID")?.into()));
//         row_map.insert("cost_center".to_string(), Value::String(row.try_get("cost_center")?));
//         row_map.insert("estimasi".to_string(), Value::String(row.try_get("estimasi")?));
//         row_map.insert("isactive".to_string(), Value::Bool(row.try_get("isactive")?));
//         row_map.insert("usrupd".to_string(), Value::String(row.try_get("usrupd")?));
//         row_map.insert("dtmupd".to_string(), Value::String(row.try_get("dtmupd")?));

//         result.push(row_map);
//     }

//     Ok(result)
// }
// pub async fn readgetBranchID(
//     db_pool: &MssqlPool,
//     query: Option<String>,
//     col: Option<String>,
//     branchid: i32, // Adding branchid as a parameter
// ) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
//     // Start with the base query
//     let mut base_query = String::from(
//         "select * from ( select id_cost_center as branchid,name as branchfullname  from portal_ext.dbo.cost_centers
//         union
// 		select branchid,branchfullname from [172.16.4.31].SGFDB.dbo.branch) as a",
//     );

//     // If query and column are provided, append the filtering logic
//     if let (Some(query_str), Some(col_name)) = (query, col) {
//         base_query.push_str(&format!(" AND {} LIKE ?", col_name));
//     }

//     // Perform the query with parameters to avoid syntax errors and prevent SQL injection
//     let mut query = sqlx::query(&base_query).bind(branchid); // Bind branchid first

//     // If a search query is provided, bind the value for the LIKE search
//     if let Some(query_str) = query {
//         query = query.bind(format!("%{}%", query_str)); // Bind the search term with % for LIKE
//     }

//     // Execute the query and fetch all rows
//     let rows = query.fetch_all(db_pool).await?;

//     // Process the results into a vector of HashMaps
//     let mut result = Vec::new();

//     for row in rows {
//         let mut row_map = HashMap::new();
        
//         row_map.insert("branchid".to_string(), Value::Number(row.try_get("branchid")?.into()));
//         row_map.insert("branchfullname".to_string(), Value::String(row.try_get("branchfullname")?));
//         // row_map.insert("typeid".to_string(), Value::Number(row.try_get("TypeID")?.into()));
//         // row_map.insert("cost_center".to_string(), Value::String(row.try_get("cost_center")?));
//         // row_map.insert("estimasi".to_string(), Value::String(row.try_get("estimasi")?));
//         // row_map.insert("isactive".to_string(), Value::Bool(row.try_get("isactive")?));
//         // row_map.insert("usrupd".to_string(), Value::String(row.try_get("usrupd")?));
//         // row_map.insert("dtmupd".to_string(), Value::String(row.try_get("dtmupd")?));

//         result.push(row_map);
//     }

//     Ok(result)
// }