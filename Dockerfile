FROM rust:1.69-alpine3.17 AS builder
WORKDIR /usr/src/http-server
COPY . .
RUN echo $(nproc)
RUN cargo install -j=$(nproc) --path .

FROM alpine:3.17
RUN apk update
WORKDIR /usr/local/bin/
COPY --from=builder /usr/src/http-server/.env.example .env
COPY --from=builder /usr/src/http-server/mock_data.json mock_data.json
COPY --from=builder /usr/local/cargo/bin/http-server /usr/local/bin/http-server
CMD ["http-server"]
