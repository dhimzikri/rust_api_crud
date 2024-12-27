use sqlx::{query, query_as ,Mssql, Pool};
use sqlx::Row;
use sqlx::mssql::MssqlPool;
use serde_json::Value;
use std::collections::HashMap;
use chrono::NaiveDateTime;
use rocket::serde::{json::Json, Deserialize, Serialize};

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

pub async fn readgettblSubType(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
    typeid: i32, // Adding typeid as a parameter
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    // Start with the base query
    let mut base_query = String::from(
        "SELECT SubTypeID, SubDescription, TypeID, cost_center, estimasi, isactive, usrupd, 
        CONVERT(VARCHAR, dtmupd, 120) as dtmupd 
        FROM tblSubType 
        WHERE isactive = 1"
    );

    // If query and column are provided, append the filtering logic
    if let (Some(query_str), Some(col_name)) = (&query, &col) {
        base_query.push_str(&format!(" AND {} LIKE @search_query", col_name));
    }

    // Create a query builder
    let mut query_builder = sqlx::query(&base_query).bind(typeid); // Bind `@typeid` first

    // If a search query is provided, bind it to `@search_query`
    if let Some(query_str) = query {
        query_builder = query_builder.bind(format!("%{}%", query_str)); // Bind the search term with wildcards
    }

    // Execute the query and fetch all rows
    let rows = query_builder.fetch_all(db_pool).await?;

    // Process the results into a vector of HashMaps
    let mut result = Vec::new();

    for row in rows {
        let mut row_map = HashMap::new();

        row_map.insert(
            "subtypeid".to_string(),
            Value::Number(row.try_get::<i32, _>("SubTypeID")?.into()),
        );
        row_map.insert(
            "subdescription".to_string(),
            Value::String(row.try_get::<String, _>("SubDescription")?),
        );
        row_map.insert(
            "typeid".to_string(),
            Value::Number(row.try_get::<i32, _>("TypeID")?.into()),
        );
        row_map.insert(
            "cost_center".to_string(),
            Value::String(row.try_get::<String, _>("cost_center")?),
        );
        row_map.insert(
            "estimasi".to_string(),
            Value::Number(row.try_get::<i32, _>("estimasi")?.into()), // Decode as i32
        );
        
        row_map.insert(
            "isactive".to_string(),
            Value::Bool(row.try_get::<bool, _>("isactive")?),
        );
        row_map.insert(
            "usrupd".to_string(),
            Value::String(row.try_get::<String, _>("usrupd")?),
        );
        row_map.insert(
            "dtmupd".to_string(),
            Value::String(row.try_get::<String, _>("dtmupd")?),
        );

        result.push(row_map);
    }

    Ok(result)
}

pub async fn getCase(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
    start: Option<i32>,
    limit: Option<i32>,
) -> Result<Vec<HashMap<String, Value>>, sqlx::Error> {
    let user_name = "8023"; // Replace with actual user name
    let start = start.unwrap_or(0);
    let limit = limit.unwrap_or(10);
    let count_last = start + limit;

    // Base query condition
    let mut base_query = String::from("0=0 AND a.statusid <> 1 AND a.usrupd = '");
    base_query.push_str(user_name);
    base_query.push_str("'");

    // Apply the query filter if present
    if let (Some(query), Some(col)) = (query, col) {
        base_query.push_str(&format!(" AND {} LIKE '%{}%'", col, query));
    }

    // SQL query setup
    let mut sql_query = String::from(r#"
        SET NOCOUNT ON;
        DECLARE @jml AS INT;

        -- Count total records
        SELECT @jml = COUNT(a.ticketno)
        FROM "Case" a
        INNER JOIN tbltype b ON a.TypeID = b.TypeID
        INNER JOIN tblSubtype c ON a.SubTypeID = c.SubTypeID AND a.TypeID = c.TypeID
        INNER JOIN "Priority" d ON a.PriorityID = d.PriorityID
        INNER JOIN "status" e ON a.statusid = e.statusid
        INNER JOIN "contact" f ON a.contactid = f.contactid
        INNER JOIN "relation" g ON a.relationid = g.relationid
        WHERE "#);

    // Append the condition to the query
    sql_query.push_str(&base_query);

    sql_query.push_str(r#"

        -- Select paginated records
        SELECT *
        FROM (
            SELECT
                ROW_NUMBER() OVER (ORDER BY RIGHT(a.ticketno, 3) DESC) AS 'RowNumber',
                a.flagcompany, a.ticketno, a.agreementno, a.applicationid, a.customerid, a.typeid, b.description AS typedescriontion,
                a.subtypeid, c.SubDescription AS typesubdescriontion, a.priorityid, d.Description AS prioritydescription, a.statusid,
                e.statusname, e.description AS statusdescription, a.customername, a.branchid, a.description, a.phoneno, a.email,
                a.usrupd, a.dtmupd, a.date_cr, @jml AS jml, f.contactid, f.Description AS contactdescription, a.relationid,
                g.description AS relationdescription, a.relationname, a.callerid, a.email_, a.foragingdays
            FROM "Case" a
            INNER JOIN tbltype b ON a.TypeID = b.TypeID
            INNER JOIN tblSubtype c ON a.SubTypeID = c.SubTypeID AND a.TypeID = c.TypeID
            INNER JOIN "Priority" d ON a.PriorityID = d.PriorityID
            INNER JOIN "status" e ON a.statusid = e.statusid
            INNER JOIN "contact" f ON a.contactid = f.contactid
            INNER JOIN "relation" g ON a.relationid = g.relationid
            WHERE "#);

    // Append the condition again for the pagination query part
    sql_query.push_str(&base_query);

    sql_query.push_str(r#"
        ) AS a
        WHERE RowNumber > "#);

    sql_query.push_str(&start.to_string());
    
    sql_query.push_str(r#" AND RowNumber <= "#);
    sql_query.push_str(&count_last.to_string());

    sql_query.push_str(r#"
        ORDER BY a.foragingdays DESC;
    "#);

    // Execute the SQL query with error handling
    let rows = sqlx::query(&sql_query)
        .fetch_all(db_pool)
        .await
        .map_err(|e| {
            // Log the error here and return it gracefully
            eprintln!("Error executing query: {}", e);
            sqlx::Error::Database(e)
        })?;

    let mut result = Vec::new();


    // Process each row into a HashMap
    for row in rows {
        let mut row_map = HashMap::new();
    
        // Using try_get to retrieve values from each column and inserting them into the row_map map
        row_map.insert(
            "flagcompany".to_string(),
            Value::String(row.try_get::<String, _>("FLAGCOMPANY")?),
        );
        row_map.insert(
            "ticketno".to_string(),
            Value::String(row.try_get::<String, _>("TICKETNO")?),
        );
        row_map.insert(
            "agreementno".to_string(),
            Value::String(row.try_get::<String, _>("AGREEMENTNO")?),
        );
        row_map.insert(
            "branchid".to_string(),
            Value::String(row.try_get::<String, _>("BRANCHID")?),
        );
        row_map.insert(
            "customername".to_string(),
            Value::String(row.try_get::<String, _>("CUSTOMERNAME")?),
        );
        row_map.insert(
            "applicationid".to_string(),
            Value::String(row.try_get::<String, _>("APPLICATIONID")?),
        );
        row_map.insert(
            "customerid".to_string(),
            Value::String(row.try_get::<String, _>("CUSTOMERID")?),
        );
        row_map.insert(
            "statusid".to_string(),
            Value::Number(row.try_get::<i32, _>("STATUSID")?.into()),
        );
        row_map.insert(
            "statusdescription".to_string(),
            Value::String(row.try_get::<String, _>("STATUSDESCRIPTION")?),
        );
        row_map.insert(
            "subdescription".to_string(),
            Value::String(row.try_get::<String, _>("SUBDESCRIPTION")?),
        );
        row_map.insert(
            "statusname".to_string(),
            Value::String(row.try_get::<String, _>("STATUSNAME")?),
        );
        row_map.insert(
            "typeid".to_string(),
            Value::Number(row.try_get::<i32, _>("TYPEID")?.into()),
        );
        row_map.insert(
            "typedescriontion".to_string(),
            Value::String(row.try_get::<String, _>("TYPEDESCRIONTION")?),
        );
        row_map.insert(
            "subtypeid".to_string(),
            Value::Number(row.try_get::<i32, _>("SUBTYPEID")?.into()),
        );
        row_map.insert(
            "typesubdescriontion".to_string(),
            Value::String(row.try_get::<String, _>("TYPESUBDESCRIONTION")?),
        );
        row_map.insert(
            "priorityid".to_string(),
            Value::Number(row.try_get::<i32, _>("PRIORITYID")?.into()),
        );
        row_map.insert(
            "prioritydescription".to_string(),
            Value::String(row.try_get::<String, _>("PRIORITYDESCRIPTION")?),
        );
        row_map.insert(
            "description".to_string(),
            Value::String(row.try_get::<String, _>("DESCRIPTION")?),
        );
        row_map.insert(
            "phoneno".to_string(),
            Value::String(row.try_get::<String, _>("PHONENO")?),
        );
        row_map.insert(
            "email".to_string(),
            Value::String(row.try_get::<String, _>("EMAIL")?),
        );
        row_map.insert(
            "contactid".to_string(),
            Value::String(row.try_get::<String, _>("CONTACTID")?),
        );
        row_map.insert(
            "contactdescription".to_string(),
            Value::String(row.try_get::<String, _>("CONTACTDESCRIPTION")?),
        );
        row_map.insert(
            "relationid".to_string(),
            Value::String(row.try_get::<String, _>("RELATIONID")?),
        );
        row_map.insert(
            "relationdescription".to_string(),
            Value::String(row.try_get::<String, _>("RELATIONDESCRIPTION")?),
        );
        row_map.insert(
            "relationname".to_string(),
            Value::String(row.try_get::<String, _>("RELATIONNAME")?),
        );
        row_map.insert(
            "usrupd".to_string(),
            Value::String(row.try_get::<String, _>("USRUPD")?),
        );
        row_map.insert(
            "dtmupd".to_string(),
            Value::String(row.try_get::<String, _>("DTMUPD")?),
        );
        row_map.insert(
            "callerid".to_string(),
            Value::String(row.try_get::<String, _>("CALLERID")?),
        );
        row_map.insert(
            "email_".to_string(),
            Value::String(row.try_get::<String, _>("EMAIL_")?),
        );
        row_map.insert(
            "date_cr".to_string(),
            Value::String(row.try_get::<String, _>("DATE_CR")?),
        );
        row_map.insert(
            "foragingdays".to_string(),
            Value::Number(row.try_get::<i32, _>("FORAGINGDAYS")?.into()),
        );

        result.push(row_map);
    }

    Ok(result)
}