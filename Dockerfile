FROM rust:1.77.1 as builder

WORKDIR /usr/src/br_common_backend

COPY . .

RUN cargo prisma generate

RUN cargo build --release

FROM rust:1.77.1-slim as final

COPY --from=builder /usr/src/br_common_backend/target/release/br_common_backend /usr/local/bin/br_common_backend

EXPOSE 5050

CMD ["br_common_backend"]
