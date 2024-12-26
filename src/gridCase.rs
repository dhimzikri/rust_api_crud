use rocket::serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

// Define the structure for the response data
#[derive(Serialize, FromRow)]
pub struct TblType {
    pub typeid: i32,
    pub description: String,
    pub isactive: bool,
    pub usrupd: String,
    pub dtmupd: String,
}

// Query parameters for filtering
#[derive(Deserialize)]
pub struct QueryParams {
    pub query: Option<String>,
    pub col: Option<String>,
}

// Function to query tblType data from the database
pub async fn get_tbl_type(
    pool: &PgPool,
    query: Option<String>,
    col: Option<String>,
) -> Result<Vec<TblType>, sqlx::Error> {
    // Base condition
    let mut src = "0 = 0".to_string();

    // Add filtering if query and column are provided
    if let (Some(query), Some(col)) = (query, col) {
        src = format!("{} AND {} LIKE '%{}%'", src, col, query);
    }

    // SQL query
    let query_str = format!(
        "
        SELECT typeid, description, isactive, usrupd, dtmupd
        FROM tblType
        WHERE {}
        ",
        src
    );

    // Execute the query and fetch results
    sqlx::query_as::<_, TblType>(&query_str)
        .fetch_all(pool)
        .await
}
