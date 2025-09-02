FROM rust:1.89 as builder

WORKDIR /src
COPY . .
RUN cargo build --release -p server

FROM rust:1.89 as prod

EXPOSE 8000

COPY --from=builder /src/target/release/api_analytic_metered_usage /api_analytic_metered_usage

CMD ["/api_analytic_metered_usage"]
