# 📚 Rust Database API

Welcome to the Rust Database API project! This project provides simple RESTful APIs built with Rust, Rocket, and Diesel. It allows you to manage a collection of books with basic CRUD operations.

## 🚀 Getting Started

### Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)

### 📦 Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/thePalma/rust-database-api.git
   cd rust-database-api
   ```

2. Set up the database:

   ```sh
   diesel setup
   ```

3. Create a `.env` file in the root directory and add your database URL:
   ```env
   DATABASE_URL=postgres://username:password@localhost/database_name
   ```

### 💡 Using Diesel

To use Diesel for database operations, follow these steps:

1. Run the following command to install the Diesel CLI:

   ```sh
   cargo install diesel_cli --no-default-features --features "postgres"
   ```

2. Generate the Diesel schema by running:

   ```sh
   diesel setup
   ```

3. You can now use Diesel to perform database operations in your Rust code.

### 🛠️ Building and Running

1. Build the project:

   ```sh
   cargo build
   ```

2. Run the project:
   ```sh
   cargo run
   ```

### 📋 API Endpoints

- `GET /api/v1/books` - List all books
- `POST /api/v1/books` - Add a new book
- `GET /api/v1/books/:id` - Get a book by ID
- `PUT /api/v1/books/:id` - Update a book by ID
- `DELETE /api/v1/books/:id` - Delete a book by ID

### 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

Enjoy coding! ✨
