# Midgard API Catalog (Week 3)

## Overview
Midgard API Catalog is a Rust-based service that collects, stores, and serves historical data from the Midgard API. The project fetches various types of historical data—such as depth price history, earnings history, rune pool history, and swaps history—and exposes them through a set of RESTful API endpoints. It also features a built-in scheduler to periodically update the database with fresh data.

## Features
- **Data Collection:**  
  Fetches historical data from the Midgard API using a modular interface.
  
- **Database Storage:**  
  Utilizes PostgreSQL and SQLx for robust data storage and migrations.
  
- **API Endpoints:**  
  Exposes endpoints for accessing depth price history, earnings history, rune pool history, and swaps history.
  
- **Scheduler:**  
  A background scheduler automatically populates the database at regular intervals.
  
- **Modular Architecture:**  
  Organized into modules for API routing, database interactions, external API integration, and utilities.

## Architecture
- **Axum Framework:**  
  Handles HTTP routing and middleware.
  
- **SQLx & PostgreSQL:**  
  Manages database connectivity, migrations, and query execution.
  
- **Reqwest & Chrono:**  
  Used for making HTTP requests to external APIs and handling date/time operations.
  
- **Environment Configuration:**  
  Uses dotenv for managing configuration settings like `DATABASE_URL` and `PORT`.

## Prerequisites
- **Rust:**  
  Ensure you have the latest stable version of [Rust](https://www.rust-lang.org/tools/install) installed.
  
- **PostgreSQL:**  
  A running PostgreSQL instance is required for data storage.
  
- **Cargo:**  
  Rust's package manager (installed with Rust).

## Installation

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/yourusername/midgard-api-catalog-week3.git
   cd midgard-api-catalog-week3
   ```

2. **Set Up Environment Variables:**  
   Create a `.env` file in the root directory with the following variables:
   ```dotenv
   DATABASE_URL=postgres://username:password@localhost:5432/your_database
   PORT=3000
   ```

3. **Build the Project:**  
   Install dependencies and build the project using Cargo:
   ```bash
   cargo build
   ```

## Running the Application

1. **Database Migrations:**  
   Migrations are automatically applied when the application starts. Ensure your PostgreSQL instance is running and the connection details in `.env` are correct.

2. **Start the Server:**  
   Run the application with:
   ```bash
   cargo run
   ```
   The API server will start on the port specified in the `.env` file (default is 3000).

## API Endpoints
- **GET /**  
  Redirects to the API documentation hosted on Postman.

- **GET /history/depth**  
  Returns depth price history data.

- **GET /history/earnings**  
  Returns earnings history data.

- **GET /history/rune-pool**  
  Returns rune pool history data.

- **GET /history/swaps**  
  Returns swaps history data.


## Scheduler
The project includes a scheduler that runs every hour. This scheduler triggers a data-fetch process to update the database with the latest historical records from the Midgard API.

## Directory Structure
```
api/
  migrations/            # SQL migration files for setting up database schema
  src/
    api/                 # HTTP routes and API request handlers
      routes/           # Individual route implementations (depth, earnings, rune-pool, swaps, docs)
    db/                  # Database connection, migrations, and data insertion logic
    midgard_api/         # Handlers and interface for fetching data from the external Midgard API
    models/              # Data model definitions
    main.rs              # Application entry point
    populate_db.rs       # Module for populating the database with historical data
    scheduler.rs         # Scheduler for periodic data updates
    utils.rs             # Utility functions (e.g., date parsing, logging initialization)
```
