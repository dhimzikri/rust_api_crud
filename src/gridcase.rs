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

#[derive(Deserialize)]
struct RequestData {
    query: Option<String>,
    col: Option<String>,
    start: Option<i32>,
    limit: Option<i32>,
}

#[derive(Serialize)]
struct Case {
    flagcompany: String,
    ticketno: String,
    agreementno: String,
    branchid: String,
    customername: String,
    applicationid: String,
    customerid: String,
    statusid: i32,
    statusdescription: String,
    subdescription: String,
    statusname: String,
    typeid: i32,
    typedescriontion: String,
    subtypeid: i32,
    typesubdescriontion: String,
    priorityid: i32,
    prioritydescription: String,
    description: String,
    phoneno: String,
    email: String,
    contactid: i32,
    contactdescription: String,
    relationid: i32,
    relationdescription: String,
    relationname: String,
    usrupd: String,
    dtmupd: String,
    callerid: String,
    email_: String,
    date_cr: String,
    foragingdays: i32,
}

#[derive(Serialize)]
struct ResponseData {
    total: i32,
    success: bool,
    data: Vec<Case>,
}

pub async fn get_case(
    req_data: web::Query<RequestData>,
    pool: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let user_name = "some_user_name"; // Replace with session logic

    let start = req_data.start.unwrap_or(0);
    let limit = req_data.limit.unwrap_or(20);
    let countlast = start + limit;

    let mut src = format!("0=0 AND a.statusid <> 1 AND a.usrupd = '{}'", user_name);

    if let Some(query) = &req_data.query {
        if let Some(col) = &req_data.col {
            src = format!("{} AND {} LIKE '%{}%'", src, col, query);
        }
    }

    let sql_str = format!(
        "SET NOCOUNT ON; \
        DECLARE @jml AS INT; \
        SELECT @jml = COUNT(a.ticketno) \
        FROM [Case] a \
        INNER JOIN tbltype b ON a.TypeID = b.TypeID \
        INNER JOIN tblSubtype c ON a.SubTypeID = c.SubTypeID AND a.TypeID = c.TypeID \
        INNER JOIN [Priority] d ON a.PriorityID = d.PriorityID \
        INNER JOIN [status] e ON a.statusid = e.statusid \
        INNER JOIN [contact] f ON a.contactid = f.contactid \
        INNER JOIN [relation] g ON a.relationid = g.relationid \
        WHERE {}; \
        SELECT * \
        FROM (\
            SELECT ROW_NUMBER() OVER (ORDER BY RIGHT(a.ticketno, 3) DESC) AS RowNumber, \
                a.flagcompany, a.ticketno, a.agreementno, a.applicationid, a.customerid, \
                a.typeid, b.description AS typedescriontion, a.subtypeid, \
                c.SubDescription AS typesubdescriontion, a.priorityid, \
                d.Description AS prioritydescription, a.statusid, e.statusname, \
                e.description AS statusdescription, a.customername, a.branchid, \
                a.description, a.phoneno, a.email, a.usrupd, a.dtmupd, a.date_cr, \
                @jml AS jml, f.contactid, f.Description AS contactdescription, \
                a.relationid, g.description AS relationdescription, a.relationname, \
                a.callerid, a.email_, a.foragingdays \
            FROM [Case] a \
            INNER JOIN tbltype b ON a.TypeID = b.TypeID \
            INNER JOIN tblSubtype c ON a.SubTypeID = c.SubTypeID AND a.TypeID = c.TypeID \
            INNER JOIN [Priority] d ON a.PriorityID = d.PriorityID \
            INNER JOIN [status] e ON a.statusid = e.statusid \
            INNER JOIN [contact] f ON a.contactid = f.contactid \
            INNER JOIN [relation] g ON a.relationid = g.relationid \
            WHERE {}\
        ) AS a \
        WHERE RowNumber > {} AND RowNumber <= {} ORDER BY a.foragingdays DESC;",
        src, src, start, countlast
    );

    let mut msg: Vec<Case> = Vec::new();
    let mut total = 0;

    match sqlx::query(&sql_str).fetch_all(&**pool).await {
        Ok(rows) => {
            for row in rows {
                msg.push(Case {
                    flagcompany: row.get("flagcompany"),
                    ticketno: row.get("ticketno"),
                    agreementno: row.get("agreementno"),
                    branchid: row.get("branchid"),
                    customername: row.get("customername"),
                    applicationid: row.get("applicationid"),
                    customerid: row.get("customerid"),
                    statusid: row.get("statusid"),
                    statusdescription: row.get("statusdescription"),
                    subdescription: row.get("subdescription"),
                    statusname: row.get("statusname"),
                    typeid: row.get("typeid"),
                    typedescriontion: row.get("typedescriontion"),
                    subtypeid: row.get("subtypeid"),
                    typesubdescriontion: row.get("typesubdescriontion"),
                    priorityid: row.get("priorityid"),
                    prioritydescription: row.get("prioritydescription"),
                    description: row.get("description"),
                    phoneno: row.get("phoneno"),
                    email: row.get("email"),
                    contactid: row.get("contactid"),
                    contactdescription: row.get("contactdescription"),
                    relationid: row.get("relationid"),
                    relationdescription: row.get("relationdescription"),
                    relationname: row.get("relationname"),
                    usrupd: row.get("usrupd"),
                    dtmupd: row.get("dtmupd"),
                    callerid: row.get("callerid"),
                    email_: row.get("email_"),
                    date_cr: row.get("date_cr"),
                    foragingdays: row.get("foragingdays"),
                });
                total = row.get("jml");
            }

            HttpResponse::Ok().json(ResponseData {
                total,
                success: true,
                data: msg,
            })
        }
        Err(err) => {
            eprintln!("Error executing query: {:?}", err);
            HttpResponse::InternalServerError().json(json!({ "success": false }))
        }
    }
}
