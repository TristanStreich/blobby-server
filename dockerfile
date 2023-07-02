FROM rust

LABEL blobby_server=""

WORKDIR /usr/src
COPY . .

RUN cargo build --release

CMD [ "cargo", "run", "--release" ]