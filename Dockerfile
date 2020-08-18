FROM rust:1.45 as build

# The build step runs `cargo build` twice in order to avoid
# building dependencies if they haven't changed. This speeds
# up building the Docker image on successive builds.

# Initialize empty project and build dependencies
RUN USER=root cargo new --bin kiba
WORKDIR /kiba
COPY Cargo.* ./
RUN cargo build --release
RUN rm src/*.rs

# Copy over and build source
COPY ./src/*.rs ./src/
RUN cargo build --release

FROM rust:1.45
COPY --from=build /kiba/target/release/kiba .
EXPOSE 6464
CMD ["./kiba"]

