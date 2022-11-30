# review-api

an attempt to learn how to use rust for a web server

Check `src/main.rs` for all API endpoints.

There is no frontend.

## Endpoints

https://review-api.fly.dev

<details>
  <summary>
    <h2><code>/auth</code> endpoints </h2>
  </summary>

### `GET /auth`

Check current user id.

#### Response

```json
{
  id: USER_ID
}
```

<details>
  <summary>Try with <code>curl</code></summary>

```sh
curl --location --request GET 'https://review-api.fly.dev/auth' \
--header 'Cookie: id=YourSessionIdCookie'
```

</details>

### `DEL /auth`

This logs out the user.

#### Response

```json
// no content
```

<details>
  <summary>Try with <code>curl</code></summary>

```sh
curl --location --request DELETE 'https://review-api.fly.dev/auth' \
--header 'Cookie: id=YourSessionIdCookie'
```

</details>

### `POST /auth`

This logs in the user. The response header contains the `set-cookie` header with the `id` cookie.

#### Response

```json
// no content
```

<details>
  <summary>Try with <code>curl</code></summary>

```sh
curl --location --request POST 'https://review-api.fly.dev/auth' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email": "kyle@zheng.com",
    "password": "password"
}'
```

</details>

</details>

<details>
  <summary>
    <h2><code>/users</code> endpoints </h2>
  </summary>

### `POST /users`

This creates a new user.

### Request body

```json
{
  "first_name": "Kyle",
  "last_name": "Zheng",
  "email": "kyle@zheng.com",
  "password": "password"
}
```

#### Response

```json
{
  "id": 3,
  "first_name": "Kyle",
  "last_name": "Zheng",
  "email": "kyle@zheng.com",
  "created_at": "2022-11-30T07:27:26.595672",
  "updated_at": "2022-11-30T07:27:26.595672"
}
```

<details>
  <summary>Try with <code>curl</code></summary>

```sh
curl --location --request POST 'https://review-api.fly.dev/users' \
--header 'Content-Type: application/json' \
--data-raw '{
    "first_name": "John",
    "last_name": "Doe",
    "email": "john@doe.com",
    "password": "password"
}'
```

</details>

</details>

## Checklist

- [x] user auth
- [x] search movies and shows using TMDB api
- [x] add and edit reviews for movies and shows
- [ ] allow better programmatic access to api
  - [ ] regeneratable api keys for users?

## Based on following examples (and many more)

- For basic structure

  - https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/#setting-up-diesel-and-creating-our-user-model

- For refactor and advanced stuff

  - https://cloudmaker.dev/

- For diesel

  - https://github.com/actix/examples/tree/master/databases/diesel
  - https://kitsu.me/posts/2020_05_24_custom_types_in_diesel
    - basically, just use `diesel-derive-enum`
  - https://github.com/diesel-rs/diesel/tree/master/examples/postgres/advanced-blog-cli

- For containerizing
  - https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
  - https://hub.docker.com/r/ekidd/rust-musl-builder/
    - Not reading the README caused me to waste an entire day debugging. TLDR include `openssl_probe::init_ssl_cert_env_vars();` to make OpenSSL work (for making outward requests).

## Run locally

### Create `.env` file

Look at `.env.sample` for template.

### Start local databases

```
sudo service postgresql start

sudo service redis-server start
```

#### Check data

```
sudo -u postgres psql

redis-cli
```

### Local Docker

```sh
docker build -t review-api:latest .

# .env.dev should have localhost replaced with host.docker.internal
docker run --rm -p 8080:8080 --env-file .env.dev review-api
```

## Connecting to deployed fly app

https://fly.io/docs/reference/private-networking/#private-network-vpn

Diesel reads `DATABASE_URL` in `.env`

check out db

```
psql -h <hostname> -p <port> -U <username> -d <database>
```
