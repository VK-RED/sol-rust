FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
RUN cp ./target/release/api /bin/api

FROM rust:latest AS runner
WORKDIR /usr/src/app
COPY --from=builder /bin/api /bin/api
EXPOSE 8080
CMD ["/bin/api"]