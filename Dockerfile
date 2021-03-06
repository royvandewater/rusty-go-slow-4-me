FROM ekidd/rust-musl-builder as build
LABEL maintainer="roy.v.water@gmail.com"

COPY . .
RUN cargo build --release

FROM alpine AS release
LABEL maintainer="roy.v.water@gmail.com"
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/go-slow-4-me /usr/local/bin/
EXPOSE 80
CMD go-slow-4-me
