FROM liuchong/rustup:nightly

EXPOSE 8000

VOLUME /code

WORKDIR /code

# This is a debian base image with the nightly toolchain installed already
# so `apt-get` is here for adding new build deps, etc.

#RUN apt-get update && \
#    apt-get install --no-install-recommends -y \
#    libpq-dev && \
#    rm -rf /var/lib/apt/lists/*

RUN cargo install \
    watchexec \
    rustfmt-nightly \
    clippy \
    && cargo install diesel_cli --no-default-features --features sqlite
