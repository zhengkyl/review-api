# https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/

FROM kristianmika/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin review-api
WORKDIR ./review-api

# Build deps independently
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

# Build with src code
RUN rm src/*.rs
ADD . ./
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/review_api*
RUN cargo build --release


FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 8080

ENV TZ=Etc/UTC \
  APP_USER=appuser

RUN addgroup -S $APP_USER \
  && adduser -S -g $APP_USER $APP_USER

RUN apk update \
  && apk add --no-cache ca-certificates tzdata \
  && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/review-api/target/x86_64-unknown-linux-musl/release/review-api ${APP}/review-api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./review-api"]