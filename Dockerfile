
# Use the official rust docker image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/task_climate_seed

# Copy the source code
COPY . .

# Build project
RUN cargo build --release --bin task_climate_seed

# Expose port 3000
EXPOSE 3000

# Run the api
CMD ["/usr/src/task_climate_seed/target/release/task_climate_seed"]
