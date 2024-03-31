
# Use the official rust docker image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/task_climate_seed

# Copy the source code
COPY . .

# Build project
RUN cargo build --release

# Expose port 8001
EXPOSE 8001

# Run the api
CMD cargo run
