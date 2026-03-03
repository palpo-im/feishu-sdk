# Best Practices

## 1. Reuse a single `Client`

Create one `Client` per app credential set and reuse it across requests. This avoids repeated setup and keeps token/cache behavior consistent.

## 2. Set explicit timeouts and retries

Use `Config::builder(...).request_timeout(...)` for baseline timeout and `RequestOptions::retry(...)` for transient failures.

## 3. Use typed wrappers for common APIs

Prefer typed wrappers such as `client.im_v1_message()` and `client.approval_v4_instance()` for stronger ergonomics.

For less common APIs, use:

- `api::all_services` wrappers, or
- direct HTTP helpers: `client.get/post/put/patch/delete`.

## 4. Keep token scope minimal

For user-auth flows, pass user token only where needed:

```rust
let options = client.user_access_token("u-xxx");
```

Avoid mixing app/tenant/user tokens in the same request unless required.

## 5. Verify webhook signatures in production

Keep signature verification enabled by default for event/card callbacks.

Only disable via `skip_sign_verify` in controlled local testing.

## 6. Log payloads only when needed

Enable payload-level debug logs with:

```rust
let config = Config::builder("app_id", "app_secret")
    .log_req_at_debug(true)
    .build();
```

This keeps default logs safer and less noisy.
