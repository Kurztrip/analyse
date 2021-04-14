FROM rustlang/rust:nightly as builder
ADD src /app
COPY . /app
WORKDIR /app
RUN cargo install --path .

FROM debian:buster-slim
ENV ROCKET_ADDRESS=0.0.0.0
COPY --from=builder /usr/local/cargo/bin/kurztrip_analyse /usr/local/bin/kurztrip_analyse
COPY .env .env
CMD ["kurztrip_analyse"]