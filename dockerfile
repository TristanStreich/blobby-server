# ~~~ Build dependencies to cache them ~~~ #
FROM rust AS build_deps

WORKDIR /usr/src

COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir ./src
RUN echo 'fn main() {println!("Hi")}' > ./src/main.rs
RUN touch ./src/lib.rs


RUN cargo build --release

# ~~~ build the actualy server itself ~~~ #
FROM build_deps AS build_server

COPY src src
COPY res res
RUN touch ./src/main.rs
RUN rm -f build.properties

RUN cargo build --release

# ~~~ Copy binaries over for final run ~~~ #
FROM rust as final

COPY --from=build_server /usr/src/target/release /usr/src/release
COPY --from=build_server /usr/src/res /usr/src/release/res

LABEL blobby_server=""

WORKDIR /usr/src/release
CMD ["./blobby-server"]
#CMD ["cargo", "run", "--release"]
#CMD ["./target/release/blobby-server" ]
