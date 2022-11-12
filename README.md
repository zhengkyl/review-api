# review-api

an attempt to learn how to use rust for a web server

Checklist

- [x] user auth
- [x] search movies and shows using TMDB api
- [x] add and edit reviews for movies and shows
- [ ] allow programmatic access to api
  - [ ] create public only endpoints
  - [ ] renegerate api keys for users

Check `src/main.rs` for all API endpoints.

## based on following outdated examples

https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/#setting-up-diesel-and-creating-our-user-model

https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/

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
