# Simple Subscription Tracker📈

A small, Rust-based subscription tracker built with Axum and SQLx. This service lets you create, read, update, and delete subscription records.

## Features

- **Create a subscription** (e.g. Netflix, Spotify)
- **Retrieve** a subscription by ID
- **List** all subscriptions for a given user
- **Update** a subscription (e.g. price or frequency)
- **Delete** a subscription

## Setup

### 1. Setup environment variables:

Check the `.env.example` file for the required environment variables.

### 2. Run migrations:
```bash
sqlx migrate run --source src/db/migrations
```

### 3. Run server:
```bash
cargo run
```

## Usage

Below are the basic endpoints and their usage. All responses and request bodies are in JSON.

### 1. Create a User

```bash
POST /user/create
```

**Request Body**

```json
{
  "email": "user@example.com",
  "password": "password"
}
```

**Response**

```json
{
  "message": "User created successfully",
  "data": {
    "id": "a9a2f9f4-a5e9-4e3f-b8e0-e2e1a1e8f1f2",
    "email": "user@example.com",
    "created_at": "2023-01-01T00:00:00.000Z"
  }
}
```

### 2. Login a User

```bash
POST /user/login
```

**Request Body**

```json
{
  "email": "user@example.com",
  "password": "password"
}
```

**Response**

```json
{
  "message": "Login successful",
  "token": "user_token_goes_here"
}
```

### 3. Create a Subscription

```bash
POST /subscription/create
```

**Request Body**

```json
{
  "name": "Netflix",
  "website_url": "https://www.netflix.com",
  "price": 9.99,
  "frequency": "monthly",
  "category": "entertainment",
  "date_started": "2023-01-01T00:00:00.000Z"
}
```

**Response**

```json
{
  "message": "Subscription created successfully",
  "data": {
    "id": "a9a2f9f4-a5e9-4e3f-b8e0-e2e1a1e8f1f2",
    "user_id": "a9a2f9f4-a5e9-4e3f-b8e0-e2e1a1e8f1f2",
    "name": "Netflix",
    "website_url": "https://www.netflix.com",
    "price": 9.99,
    "frequency": "monthly",
    "category": "entertainment",
    "date_started": "2023-01-01T00:00:00.000Z",
    "created_at": "2023-01-01T00:00:00.000Z"
  }
}
```

### 4. Retrieve a Subscription by ID

```bash
GET /subscription/:subscription_id
```

**Response**

```json
{
  "message": "Subscription retrieved successfully",
  "data": {
    "id": "a9a2f9f4-a5e9-4e3f-b8e0-e2e1a1e8f1f2",
    "user_id": "a9a2f9f4-a5e9-4e3f-b8e0-e2e1a1e8f1f2",
    "name": "Netflix",
    "website_url": "https://www.netflix.com",
    "price": 9.99,
    "frequency": "monthly",
    "category": "entertainment",
    "date_started": "2023-01-01T00:00:00.000Z",
    "created_at": "2023-01-01T00:00:00.000Z"
  }
}
```

### 5. List Subscriptions for a User

```bash
GET /user/:user_id/subscriptions
```

**Response**

```json
{
  "message": "Subscriptions retrieved successfully",
  "data": [
    {
      "id": "a9a2f9f4-a5e9-4e3f-b8e0-e2e1a1e8f1f2",
      "user_id": "a9a2f9f4-a5e9-4e3f-b8e0-e2e1a1e8f1f2",
      "name": "Netflix",
      "website_url": "https://www.netflix.com",
      "price": 9.99,
      "frequency": "monthly",
      "category": "entertainment",
      "date_started": "2023-01-01T00:00:00.000Z",
      "created_at": "2023-01-01T00:00:00.000Z"
    }
  ]
}
```

### 6. Update a Subscription

```bash
PATCH /subscription/:subscription_id
```

**Request Body**

```json
{
  "name": "Hulu",
  "website_url": "https://www.netflix.com",
  "price": 9.99,
  "frequency": "monthly",
  "category": "entertainment",
  "date_started": "2023-01-01T00:00:00.000Z"
}
```

**Response**

```json
{
  "message": "Subscription updated successfully",
  "data": {
    "id": "a9a2f9f4-a5e9-4e3f-b8e0-e2e1a1e8f1f2",
    "user_id": "a9a2f9f4-a5e9-4e3f-b8e0-e2e1a1e8f1f2",
    "name": "Hulu",
    "website_url": "https://www.netflix.com",
    "price": 9.99,
    "frequency": "monthly",
    "category": "entertainment",
    "date_started": "2023-01-01T00:00:00.000Z",
    "created_at": "2023-01-01T00:00:00.000Z"
  }
}
```

### 7. Delete a Subscription

```bash
DELETE /subscription/:subscription_id
```

**Response**

```json
{
  "message": "Subscription deleted successfully"
}
```

## License

This project is licensed under the MIT License.