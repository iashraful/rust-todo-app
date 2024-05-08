FROM rust:alpine3.19
WORKDIR /app
RUN apk add musl-dev
COPY . .
RUN cargo install --path .
CMD [ "rust_basic_todo" ]