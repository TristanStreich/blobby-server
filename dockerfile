FROM rust

WORKDIR /usr/src
COPY . .

RUN cargo build --release

CMD [ "cargo", "run", "--release" ]