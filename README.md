# Task Manager API

A simple web API built with Rust and the `actix-web` framework to manage tasks. The API supports CRUD operations (Create, Read, Update, Delete) for task management.

---

## Features
- **Create**: Add new tasks.
- **Read**: View all existing tasks.
- **Update**: Edit the title and status of tasks.
- **Delete**: Remove tasks.

---

## Prerequisites

1. Install [Rust](https://rustup.rs/) and ensure `cargo` is available in your system.
2. Use `curl`, Postman, or any other HTTP client to test the API endpoints.

---

## Setup

1. Clone the repository or download the source code.
2. Navigate to the project directory:

    ```bash
    cd task_manager_api
    ```

3. Build the project and download dependencies:

    ```bash
    cargo build
    ```

4. Run the server:

    ```bash
    cargo run
    ```

5. The API will start on `http://127.0.0.1:8080`.

---

## Endpoints

### **1. Get All Tasks**
- **Method**: `GET`
- **Endpoint**: `/tasks`
- **Description**: Retrieves all tasks.

#### Example:
```bash
curl -X GET http://127.0.0.1:8080/tasks
```

### **2. Add a New Task**
- **Method**: `POST`
- **Endpoint**: `/tasks`
- **Description**: Adds a new task.
- **Request Body**:
  ```json
  {
    "title": "Task title",
    "completed": false
  }
  ```

#### Example:
```bash
curl -X POST http://127.0.0.1:8080/tasks \
-H "Content-Type: application/json" \
-d '{"title": "Learn Rust", "completed": false}'
```

### **3. Update a Task**
- **Method**: `PUT`
- **Endpoint**: `/tasks/{id}`
- **Description**: Updates an existing task.
- **Request Body**:
  ```json
  {
    "title": "Updated task title",
    "completed": true
  }
  ```

#### Example:
```bash
curl -X PUT http://127.0.0.1:8080/tasks/<task_id> \
-H "Content-Type: application/json" \
-d '{"title": "Master Rust", "completed": true}'
```

### **4. Delete a Task**
- **Method**: `DELETE`
- **Endpoint**: `/tasks/{id}`
- **Description**: Deletes a task by its unique ID.

#### Example:
```bash
curl -X DELETE http://127.0.0.1:8080/tasks/<task_id>
```

---

## Project Structure

```
.
├── Cargo.toml           # Project dependencies and configuration
├── src
│   ├── main.rs          # Main application logic
```

---

## Testing the API

1. **Start the server**:
    ```bash
    cargo run
    ```

2. **Use a client to test endpoints**:
    - **Postman**: Import the endpoints and test them.
    - **Curl**: Use the example commands provided above.

3. **Validate responses**:
    - `GET /tasks` should return a list of tasks.
    - `POST /tasks` should add a new task.
    - `PUT /tasks/{id}` should update the task details.
    - `DELETE /tasks/{id}` should remove the task.

---

## Future Improvements

- Add persistent storage (e.g., PostgreSQL, SQLite).
- Implement authentication and authorization.
- Add validation for request payloads.
- Deploy the API using Docker.

---

## License

This project is licensed under the MIT License.
