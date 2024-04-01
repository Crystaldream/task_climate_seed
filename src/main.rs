
use axum::{routing::get, Router};
use task_climate_seed::handlers::customer::Customer;

#[tokio::main]
async fn main() {

    // Initialize log
    env_logger::init();
    
    // Define the API route to query the CustomerOrderSummary view
    let app: Router = Router::new()
        .route("/api/v1/top-customers", get(Customer::top_customers));

    // Start the localhost server on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
