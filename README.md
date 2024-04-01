# task_climate_seed

Simple Api made with Rust and Axum to call a SQL view and return the result as JSON. 
The decision to use Axum was made because of its simplicity and also due to previous experience working with it. 
The chosen database was SQLite due to its simplicity, but if needed, I also have the logic created for PostgreSQL on my side (contact me for details). 
SQLx was chosen as a tool to connect to the database and perform queries because of its simplicity and compatibility with several databases. 
Tokio was used to serve the Api and perform asynchronous jobs. 
The status codes can be seen by opening the developer tools or when calling the web service from curl or Postman.
They are returned with every JSON response. 

The logic is separated as follows:

main.rs :

    Entry point of the application.
    File where the route to the top-consumer endpoint is setup and where the server is initialized.

lib.rs :

    File used to track additional files and directories in order to use them in the application, as if they were modules/libraries.

Cargo.toml :

    Used to manage the dependencies of the Rust application.

store.db :

    Database file used in this application.

utils/sqlite_utils.rs : 
    
    connect_db is a function used to perform connections with the SQLite database.

handlers/customer.rs :

    fetch_view_customer_order_summary is a function that performs a query to the view CustomerOrderSummary to get the top 5 consumers of the current year by the amount of total money spent on several orders. 
    It is called inside the function top_customers, which is the function triggered by the endpoint /top-consumers.
    If the response is successful, the Vec of type Row will be filled with appropriate information from the database.
    The fields of the struct Row are CustomerName: <string>, TotalSpent: <f32>, and OrderCount: <i32>.
    The result is returned in JSON format.
    Error handling was implemented for query execution and database connections in the top_customers function, as this is the place where the responses are generated, where the database connection is made and where the query is performed.
    The Api has versioning (currently only v1) and uses Tokio to serve the Axum Api on port 3000.
    It was decided to use static functions to be called on the endpoint for simplicity, as it is a simple get request without input.
    In the Row struct definition, serde was used to rename the fields as the database tables were in PascalCase, and Rust field standards require snake_case.
    The log crate was used mainly to log errors to the terminal, such as failure to connect to the database.

To build and run the application, Docker needs to already be installed, and the daemon must be running.
Clone the project on your machine, move to the directory, and run the following commands:

    docker build -t task_climate_seed .
    docker run -d -p 3000:3000 task_climate_seed

These commands will build the project, create a Docker image with the tag name task_climate_seed and run it on background on port 3000.
After both finish running, go to your browser and enter this link: http://localhost:3000/top-customers
