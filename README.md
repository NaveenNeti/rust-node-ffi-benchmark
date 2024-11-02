# Overview

This project demonstrates batch insertion of records into a PostgreSQL database using both Rust and Node.js. It includes Docker setup for PostgreSQL, Rust code for batch insertion, and Node.js code for benchmarking.

# Prerequisites
- Docker
- Rust
- Node.js

# Setup
Clone the repository: `git clone <repository-url> cd <repository-directory>`

Start PostgreSQL using Docker: ` docker-compose up -d`


## Debugging
Run the script `./psql.sh` to connect to the database

## Rust
Build and Run
Navigate to the Rust directory: `cd rust`

Build the Rust project: `cargo build`

Run the Rust batch insert: `cargo run`

### Code Overview
The main Rust code is in rust/src/main.rs.
The batch_insert function performs batch insertion.
The clear_records function clears the records in the benchmark table.

## Node.js
Install Dependencies
Navigate to the Node directory: `cd node`

Install Node.js dependencies: `npm install`

Run Benchmark
Run the Node.js benchmark: `node benchmark.js`

### Code Overview
The main Node.js code is in node/benchmark.js.
The pg library is used for PostgreSQL interaction.
Cleanup
To clean up the Docker containers and volumes:

`docker-compose down -v`