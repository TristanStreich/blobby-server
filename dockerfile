FROM ubuntu:20.04 AS base

LABEL blobby_server=""
WORKDIR /usr/src

ENV TZ=America/Los_Angeles
ENV DEBIAN_FRONTEND=noninteractive

FROM base AS runtime

# install runtime deps here
RUN apt-get update && \
	apt-get install -y --fix-missing \
		libssl-dev \
		ca-certificates

FROM base AS rust

RUN apt-get update && \
    apt-get install -y --fix-missing \
    	curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"


# ~~~ Build dependencies to cache them ~~~ #
FROM rust AS build_deps

# install build deps here
RUN apt-get update && \
    apt-get install -y --fix-missing \
    	build-essential \
		pkg-config \
		libssl-dev

COPY Cargo.toml .
COPY Cargo.lock .

RUN mkdir ./src
RUN echo 'fn main() {println!("Hi")}' > ./src/main.rs


RUN cargo build --release

# ~~~ build the actualy server itself ~~~ #
FROM build_deps AS build_server

COPY src src
COPY res res
RUN touch ./src/main.rs
RUN rm -f build.properties

RUN cargo build --release

# ~~~ Copy binaries over for final run ~~~ #
FROM runtime as final

COPY --from=build_server /usr/src/target/release/blobby-server /usr/src/blobby-server
COPY --from=build_server /usr/src/res /usr/src/res

CMD ["./blobby-server"]
