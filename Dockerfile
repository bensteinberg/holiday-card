FROM rust:stretch

# pin node version -- see https://github.com/nodesource/distributions/issues/33
RUN curl -o nodejs.deb https://deb.nodesource.com/node_11.x/pool/main/n/nodejs/nodejs_11.15.0-1nodesource1_amd64.deb \
    && dpkg -i ./nodejs.deb \
    && rm nodejs.deb

RUN cargo install systemfd cargo-watch cargo-cmd cargo-deb

WORKDIR /app