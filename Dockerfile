FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src ./src
COPY migrations ./migrations
COPY data ./data
COPY .sqlx .sqlx
ENV SQLX_OFFLINE=true
RUN touch src/main.rs
RUN cargo build --release

RUN cp /app/target/release/habit-tracker-api /app/habit-tracker-api

RUN mkdir -p /app/data && chmod 755 /app/data

EXPOSE 8080

CMD ["/app/habit-tracker-api"]
