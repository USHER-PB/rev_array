FROM rust:alpine AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

COPY --from=build /app/target/release/rev_arr    app/rev_array

CMD ["/app/rev_array"]






