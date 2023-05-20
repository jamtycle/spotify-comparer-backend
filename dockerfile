FROM rust:1.69.0
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
CMD ["myapp"]