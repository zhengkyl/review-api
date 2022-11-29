# review-api

an attempt to learn how to use rust for a web server

Check `src/main.rs` for all API endpoints.

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

### Connecting to deployed fly app

https://fly.io/docs/reference/private-networking/#private-network-vpn

Diesel reads `DATABASE_URL` in `.env`

check out db

```
psql -h <hostname> -p <port> -U <username> -d <database>
```
