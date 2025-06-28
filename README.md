# user-mgmt-api

A plug-and-play Rust crate for user authentication and user management, built with Axum and PostgreSQL.  
Implements all core endpoints for registration, login (JWT), profile fetch, update, and robust error handling.

Looking for a quick demo?  
👉 **See [user-mgmt-example](https://github.com/AncientiCe/user-mgmt-example) for a full working starter using this library!**

---

## Features

- **Register:** Create users (`POST /register`)
- **Login:** JWT-based login (`POST /login`)
- **Get Current User:** Get profile (`GET /me`)
- **Get User by ID:** Fetch any user (`GET /users/{user_id}`)
- **Update Profile:** Change display name (`PATCH /users/{user_id}`)
- **PostgreSQL backend** (via sqlx)
- **Consistent JSON error responses** (`401`, `404`, `409`, invalid JWT)

---

## Usage

1. **Add to your `Cargo.toml`:**

    ```toml
    user-mgmt-api = { path = "../user-mgmt-api" }
    ```

2. **Setup database:**  
   Make sure you have a running PostgreSQL database.

3. **Mount in your Axum app:**
    ```rust
    use user_mgmt_api::mount_routes;

    let app = mount_routes(axum::Router::new(), db_pool, "your_jwt_secret");
    ```

4. **Configure environment:**
   - `DATABASE_URL`
   - `JWT_SECRET`

---

## API Overview

| Endpoint                   | Method | Auth?  | Description                |
|----------------------------|--------|--------|----------------------------|
| `/register`                | POST   | No     | Register new user          |
| `/login`                   | POST   | No     | Login, get JWT             |
| `/me`                      | GET    | Yes    | Current user profile       |
| `/users/{user_id}`         | GET    | Yes    | User by ID                 |
| `/users/{user_id}`         | PATCH  | Yes    | Update display name        |

See [postman.json](./postman.json) for complete request/response details.

---

## Error Handling

- `401` Unauthorized (missing or invalid JWT)
- `404` Not Found (user does not exist)
- `409` Conflict (duplicate email)
- All errors returned as JSON with an `error` field

---

## Example Project

The fastest way to try this library:  
👉 [user-mgmt-example](https://github.com/AncientiCe/user-mgmt-example)

---

## License

MIT

---

*Built for real projects. Drop in, plug, and go!*
