# Rust Actix-Diesel Repository Pattern Example
This repository contains a Rust project that demonstrates how to use Actix and Diesel to implement the repository pattern for database operations.

## Requirements
- Rust >= 1.53.0
- PostgreSQL >= 9.5

## Setup
1. Clone the repository:
```git
git clone https://github.com/yourusername/rust-actix-diesel-repo-pattern-example.git
```
2. Create a .env file in the project root directory and add the following variables:
```bash
DATABASE_URL=postgres://username:password@localhost/database_name
```
3. Install Diesel CLI:
```bash
cargo install diesel_cli --no-default-features --features postgres
```
4. Run Diesel migrations to setup the database schema:
```bash
diesel migration run
```

## Usage
To run the project, execute the following command:
```bash
cargo run
```

This will start the Actix server at http://localhost:8080.

You can interact with the API using curl or a tool like Postman.

## Endpoints
The following endpoints are available:

### POST /users
Create a new user.

Example request:

```bash
curl --request POST \
  --url http://localhost:8080/users \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "John Doe",
	"email": "johndoe@example.com"
}'
```

### GET /users
Retrieve all users.

Example request:
```bash
curl --request GET \
  --url http://localhost:8080/users
```

### GET /users/{id}
Retrieve a single user by ID.

Example request:
```bash
curl --request GET \
  --url http://localhost:8080/users/1
```

### PUT /users/{id}
Update a user by ID.

Example request:
```bash
curl --request PUT \
  --url http://localhost:8080/users/1 \
  --header 'Content-Type: application/json' \
  --data '{
	"name": "Jane Doe",
	"email": "janedoe@example.com"
}'
```
### DELETE /users/{id}
Delete a user by ID.

Example request:
```bash
curl --request DELETE \
  --url http://localhost:8080/users/1
```

## Credits
This project was inspired by [actix](https://actix.rs/) and [diesel](https://diesel.rs/).

## License
This project is licensed under the MIT License.
