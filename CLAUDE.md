# CLAUDE.md - cnctd_rest

> Brief reference for the REST client wrapper library.

## Purpose

Convenient REST client wrapper providing common HTTP patterns (GET/POST with various auth schemes) with generic JSON request/response handling.

## Key Exports

```rust
pub struct Rest;

impl Rest {
    // GET methods
    pub async fn get<T: DeserializeOwned>(url: &str) -> Result<T>;
    pub async fn get_with_auth<T>(url: &str, token: &str) -> Result<T>;
    pub async fn get_with_custom_headers_and_timeout<T>(
        url: &str, headers: HeaderMap, timeout: Duration
    ) -> Result<T>;

    // POST methods
    pub async fn post<T, B: Serialize>(url: &str, body: &B) -> Result<T>;
    pub async fn post_with_auth<T, B>(url: &str, body: &B, token: &str) -> Result<T>;
    pub async fn post_with_bearer<T, B>(url: &str, body: &B, token: &str) -> Result<T>;
    pub async fn post_with_headers<T, B>(url: &str, body: &B, headers: HeaderMap) -> Result<T>;
}
```

## Usage Example

```rust
use cnctd_rest::Rest;

// Simple GET
let data: ApiResponse = Rest::get("https://api.example.com/data").await?;

// POST with bearer token
let result: CreateResponse = Rest::post_with_bearer(
    "https://api.example.com/create",
    &payload,
    &auth_token
).await?;
```

## Ecosystem Role

- **Used by**: cnctd_git, cnctd.world auth services
- **Wraps**: reqwest HTTP client

## Version

**0.1.6**

---

*Part of the cnctd monorepo. See `../../../CLAUDE.md` for ecosystem context.*
