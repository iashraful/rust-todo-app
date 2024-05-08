FROM rust:alpine3.19
WORKDIR /app

RUN apk update && \
    apk add --virtual build-deps libgcc gcc musl-dev lapack-dev && \
    apk add postgresql-dev

COPY . .
# RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install --path .

CMD [ "rust_basic_todo" ]