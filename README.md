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
    On successful return of data from the CustomerOrderSummary view, there was a need to format the Row Vector into a Vec of JSON objects to show the TotalSpent floating values with 2 decimal values instead of the entire decimal value.

To build and run the application, Docker needs to already be installed, and the daemon must be running.
Clone the project on your machine, move to the directory, and run the following commands:

    docker build -t task_climate_seed .
    docker run -d -p 3000:3000 task_climate_seed

These commands will build the project, create a Docker image with the tag name task_climate_seed and run it on background on port 3000.
After both finish running, go to your browser and enter this link: http://localhost:3000/top-customers


The SQL table creation, insertion of data and view creation are explained above:

Table Customers

    CREATE TABLE Customers (
        CustomerId INTEGER PRIMARY KEY,
        CustomerName VARCHAR,
        Region VARCHAR
    );

Table Orders

    CREATE TABLE Orders (
        OrderId INTEGER PRIMARY KEY,
        CustomerId INT,
        OrderDate DATE,
        Amount DECIMAL,
        FOREIGN KEY (CustomerId) REFERENCES Customers(CustomerId)
    );

Table OrderItems

    CREATE TABLE OrderItems (
        OrderItemId INTEGER PRIMARY KEY,
        OrderId INT,
        ProductId INT,
        Quantity INT,
        PricePerItem DECIMAL,
        FOREIGN KEY (OrderId) REFERENCES Orders(OrderId),
        FOREIGN KEY (ProductId) REFERENCES Products(ProductId)
    );

Table Products

    CREATE TABLE Products (
        ProductId INTEGER PRIMARY KEY,
        ProductName VARCHAR,
        Category VARCHAR
    );

Insert data on Customers table

    INSERT INTO Customers (CustomerName, Region) VALUES 
    ('Customer1', 'Portugal'),
    ('Customer2', 'France'),
    ('Customer3', 'Germany'),
    ('Customer4', 'Spain'),
    ('Customer5', 'Croatia'),
    ('Customer6', 'USA'),
    ('Customer7', 'UK'),
    ('Customer8', 'Japan'),
    ('Customer9', 'Austria');

Insert data on Orders table

    INSERT INTO Orders (CustomerId, OrderDate, Amount) VALUES 
    (1, '2024-01-15', 50.0),
    (2, '2024-02-20', 201.0),
    (3, '2024-03-10', 25.5),
    (4, '2024-03-25', 90.0),
    (5, '2024-04-05', 161.98),
    (1, '2023-04-05', 120.4),
    (3, '2024-01-01', 160.55),
    (1, '2024-03-30', 60.0);

Insert data on OrderItems table

    INSERT INTO OrderItems (OrderId, ProductId, Quantity, PricePerItem) VALUES 
    (1, 1, 1, 50.00),
    (2, 2, 2, 100.50),
    (3, 3, 1, 25.50),
    (4, 4, 3, 30.00),
    (5, 5, 2, 80.99),
    (6, 1, 2, 60.20),
    (7, 5, 1, 160.55),
    (8, 4, 2, 30.00);

Insert data on Products table

    INSERT INTO Products (ProductName, Category) VALUES
    ('Mouse', 'Electronics'),
    ('Keyboard', 'Electronics'),
    ('Usb Cable', 'Electronics'),
    ('Book', 'Books'),
    ('Printer', 'Electronics');

Creation of the CustomerOrderSummary view

    CREATE VIEW CustomerOrderSummary AS
    SELECT 
        c.CustomerName,
        CAST(SUM(o.Amount) AS REAL) AS TotalSpent,
        COUNT(DISTINCT o.OrderId) AS OrderCount
    FROM Customers c
    JOIN Orders o ON c.CustomerId = o.CustomerId
    JOIN OrderItems oi ON o.OrderId = oi.OrderId
    JOIN Products p ON oi.ProductId = p.ProductId
    WHERE p.Category = 'Electronics'
    AND o.OrderDate BETWEEN DATE('now', 'start of year') AND DATE('now')
    GROUP BY c.CustomerId;

The view is composed of three columns: CustomerName, TotalSpent, OrderCount.
The 4 tables are linked to each other with their primary and foreign keys.
TotalSpent is calculated by summing the Amount of the table Order for each consumer, when the order of the current consumer had order items with products of the category "Electronics", and when the date of the order was made in the current year.
A counter was used as OrderCount to get a value of unique OrderId's for the current customer.
The limit of the top 5 customers is specified when querying the view CustomerOrderSummary, and they are ordered by the TotalSpent value in descending order.
