# review-api

an attempt to learn how to use rust for a web server

Check `src/main.rs` for all API endpoints.

There is no frontend.

## Endpoints

https://review-api.fly.dev

| Public endpoints                           |
| ------------------------------------------ |
| All `GET` endpoints except for `GET /auth` |
| `POST /users` to create a new user         |

All other endpoints require authentication. This means the `id` cookie received from `POST /auth` needs to be sent with each request. This happens automatically if using a browser.

### `GET /auth`

Check current user id.

#### Response body

```json
{
  "id": 1,
  "name": "Kyle",
  "email": "kyle@zheng.com",
  "created_at": "2022-11-30T17:05:36.313355",
  "updated_at": "2022-11-30T17:05:36.313355"
}
```

### `DELETE /auth`

This logs out the user.

### `POST /auth`

This logs in the user.

#### Response header

| Key        | Value                                               |
| ---------- | --------------------------------------------------- |
| set-cookie | id=YOUR_SESION_ID_COOKIE; Path=/; Secure; HttpOnly; |

### `GET /users`

#### Query params

| Param    | Type                                                                                                         | Default |
| -------- | ------------------------------------------------------------------------------------------------------------ | ------- |
| page     | 0 < integer                                                                                                  | 1       |
| per_page | 0 < integer < 51                                                                                             | 10      |
| sort_by  | `FIELD.ORDER`<br> FIELD is one of `id`, `name`, `created_at`, `updated_at`<br> ORDER is one of `asc`, `desc` | id.asc  |

#### Request body

```json
{
  "results": [
    {
      "id": 1,
      "name": "Kyle",
      "created_at": "2022-11-30T17:05:36.313355",
      "updated_at": "2022-11-30T17:05:36.313355"
    },
    {
      "id": 3,
      "name": "Loid",
      "created_at": "2022-11-30T17:13:11.250255",
      "updated_at": "2022-11-30T17:27:53.894057"
    }
  ],
  "page": 1,
  "total_pages": 1,
  "total_results": 2
}
```

### `GET /users/{id}`

#### Request body

```json
{
  "id": 1,
  "name": "Kyle",
  "created_at": "2022-11-30T17:05:36.313355",
  "updated_at": "2022-11-30T17:05:36.313355"
}
```

### `POST /users`

This creates a new user.

#### Request body

```json
{
  "name": "Twilight",
  "email": "secret@spy.com",
  "password": "password"
}
```

#### Response body

```json
{
  "id": 3,
  "name": "Twilight",
  "created_at": "2022-11-30T17:13:11.250255",
  "updated_at": "2022-11-30T17:13:11.250255"
}
```

### `PUT /users/{id}`

#### Request body

```json
{
  "name": "Loid",
  "email": "loid@forger.com"
}
```

### Response body

```json
{
  "id": 3,
  "name": "Loid",
  "created_at": "2022-11-30T17:13:11.250255",
  "updated_at": "2022-11-30T17:27:53.894057"
}
```

### `DELETE /users/{id}`

#### Response body

```json
{
  "deleted": 1
}
```

### `GET /search/{category}?query=`

Category is `Film` | `Show`

This mostly just a wrapper around the The Movie Database (TMDB) API.

#### Query params

| Param | Type            | Default |
| ----- | --------------- | ------- |
| page  | 0 < integer     | 1       |
| lang  | ISO 639-1 value | en-US   |
| year  | integer         | n/a     |

#### Response body

```json
{
  "page": 1,
  "results": [
    {
      "id": 505642,
      "title": "Black Panther: Wakanda Forever",
      "original_title": "Black Panther: Wakanda Forever",
      "original_language": "en",
      "release_date": "2022-11-09",
      "overview": "Queen Ramonda, Shuri, M’Baku, Okoye and the Dora Milaje fight to protect their nation from intervening world powers in the wake of King T’Challa’s death. As the Wakandans strive to embrace their next chapter, the heroes must band together with the help of War Dog Nakia and Everett Ross and forge a new path for the kingdom of Wakanda.",
      "poster_path": "/sv1xJUazXeYqALzczSZ3O6nkH75.jpg"
    }
  ],
  "total_results": 1,
  "total_pages": 1
}
```

### `GET /reviews`

#### Query params

| Param      | Type                                                                                                      | Default |
| ---------- | --------------------------------------------------------------------------------------------------------- | ------- |
| page       | 0 < integer                                                                                               | 1       |
| per_page   | 0 < integer < 51                                                                                          | 10      |
| sort_by    | `FIELD.ORDER`<br> FIELD is one of `tmdb_id`, `created_at`, `updated_at`<br> ORDER is one of `asc`, `desc` | id.asc  |
| user_id    | user id                                                                                                   | n/a     |
| category   | `Film` \| `Show`                                                                                          | n/a     |
| status     | `Completed` \| `Watching` \| `Dropped` \| `PlanToWatch`                                                   | n/a     |
| fun_before | bool                                                                                                      | n/a     |
| fun_during | bool                                                                                                      | n/a     |
| fun_after  | bool                                                                                                      | n/a     |

#### Response body

```json
{
  "results": [
    {
      "user_id": 1,
      "tmdb_id": 505642,
      "category": "Film",
      "status": "Completed",
      "text": "🙅🏿‍♂️",
      "fun_before": true,
      "fun_during": true,
      "fun_after": true,
      "created_at": "2022-11-30T18:09:58.829342",
      "updated_at": "2022-11-30T18:18:00.720356"
    }
  ],
  "page": 1,
  "total_pages": 1,
  "total_results": 1
}
```

### `POST /reviews`

#### Request body

```json
{
  "tmdb_id": 505642,
  "category": "Film",
  "status": "Completed"
}
```

#### Response body

```json
{
  "user_id": 1,
  "tmdb_id": 505642,
  "category": "Film",
  "status": "Completed",
  "text": "",
  "fun_before": false,
  "fun_during": false,
  "fun_after": false,
  "created_at": "2022-11-30T18:09:58.829342",
  "updated_at": "2022-11-30T18:09:58.829342"
}
```

### `PUT /reviews/{category/{tmdb_id}`

#### Request body

```json
{
  "status": "Completed",
  "text": "🙅🏿‍♂️",
  "fun_before": true,
  "fun_during": true,
  "fun_after": true
}
```

#### Response body

```json
{
  "user_id": 1,
  "tmdb_id": 505642,
  "category": "Film",
  "status": "Completed",
  "text": "🙅🏿‍♂️",
  "fun_before": true,
  "fun_during": true,
  "fun_after": true,
  "created_at": "2022-11-30T18:09:58.829342",
  "updated_at": "2022-11-30T18:18:00.720356"
}
```

### `DELETE /reviews/{category}/{tmdb_id}`

#### Response body

```json
{
  "deleted": 1
}
```

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
