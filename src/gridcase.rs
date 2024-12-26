use sqlx::{query_as, MssqlPool};
use serde::Serialize;

// Define the TblType struct for the database rows
#[derive(Serialize, sqlx::FromRow)]
pub struct TblType {
    pub typeid: i32,
    pub description: String,
    pub isactive: bool,
    pub usrupd: String,
    pub dtmupd: Option<String>,
}

// Query parameters
pub struct QueryParams {
    pub query: Option<String>,
    pub col: Option<String>,
}

// Function to fetch data from tblType
pub async fn get_tbl_type(
    db_pool: &MssqlPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<TblType>, sqlx::Error> {
    let mut base_query = String::from("SELECT typeid, description, isactive, usrupd, dtmupd FROM tblType WHERE 1=1");

    if let Some(query_str) = query {
        if let Some(col_name) = col {
            base_query.push_str(&format!(" AND {} LIKE '%{}%'", col_name, query_str));
        }
    }

    query_as::<_, TblType>(&base_query)
        .fetch_all(db_pool)
        .await
}
