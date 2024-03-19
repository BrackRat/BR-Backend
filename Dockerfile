FROM rust:1.76 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo prisma generate
RUN cargo build --release
RUN chmod +x ./target/release
RUN ls -al /usr/src/app && ls -al /usr/src/app/target && ls -al /usr/src/app/target/release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/app/target/release/app /usr/local/bin/app
CMD ["app"]