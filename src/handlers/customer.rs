
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Error, SqlitePool};
use log::{error, info};
use crate::utils::sqlite_utils::SqliteUtils;

// Row struct to map each line of the result of the query to the CustomerOrderSummary view
#[derive(Serialize, Deserialize, FromRow)]
pub struct Row {
    #[serde(rename = "CustomerName")]
    pub customer_name: String,
    #[serde(rename = "TotalSpent")]
    pub total_spent: f32,
    #[serde(rename = "OrderCount")]
    pub order_count: i32
}

pub struct Customer;

impl Customer {

    pub fn new() -> Self {
        Customer
    }

    // Function that queries the current results of CustomerOrderSummary view for the current year
    async fn fetch_view_customer_order_summary(pool: &SqlitePool) -> Result<Vec<Row>, Error> {
    
        let result: Vec<Row> = sqlx::query_as(
            "SELECT CustomerName AS customer_name, TotalSpent AS total_spent, OrderCount AS order_count
             FROM CustomerOrderSummary
             ORDER BY TotalSpent DESC
             LIMIT 5;"
        )
        .fetch_all(pool)
        .await?;
    
        Ok(result)
    
    }
    
    // Function that connects to the database, calls the function to query the CustomerOrderSummary view, handles errors and returns the results as JSON objects
    pub async fn top_customers() -> impl IntoResponse {
    
        match SqliteUtils::connect_db().await {
            
            Ok(pool) => match Customer::fetch_view_customer_order_summary(&pool).await {
    
                Ok(rows) => {
        
                    if rows.is_empty() {

                        info!("View CustomerOrderSummary response successful but empty");
    
                        (
                            StatusCode::OK,
                            Json(serde_json::json!({
                                "status": "success",
                                "rows": "No rows found",
                            }))
                        )
    
                    } else {

                        info!("View CustomerOrderSummary response successful");

                        let formatted_rows: Vec<serde_json::Value> = rows.iter().map(|row| {
                            serde_json::json!({
                                "customer_name": row.customer_name,
                                "total_spent": format!("{:.2}", row.total_spent),
                                "order_count": row.order_count
                            })
                        }).collect();
    
                        (
                            StatusCode::OK,
                            Json(serde_json::json!({
                                "status": "success",
                                "rows": formatted_rows,
                            }))
                        )
    
                    }
    
                },
                Err(error) => {
    
                    error!("Failed to fetch top consumers from view CustomerOrderSummary : {}", error);
    
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "status": "error",
                            "error_message": error.to_string(),
                        }))
                    )
    
                }
        
            },
            Err(error) => {
    
                error!("Failed to connect to the database : {}", error);
    
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "status": "error",
                        "error_message": "Failed to connect to the database",
                    }))
                )
    
            }
    
        }
    
    }

}
