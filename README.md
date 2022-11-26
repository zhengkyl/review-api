# review-api

an attempt to learn how to use rust for a web server

Checklist

- [x] user auth
- [x] search movies and shows using TMDB api
- [x] add and edit reviews for movies and shows
- [ ] allow better programmatic access to api
  - [ ] create public only endpoints
  - [ ] renegerate api keys for users

Check `src/main.rs` for all API endpoints.

## based on following outdated examples

https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/#setting-up-diesel-and-creating-our-user-model

https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/

### custom types in diesel

https://kitsu.me/posts/2020_05_24_custom_types_in_diesel -> use `diesel-derive-enum`

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
