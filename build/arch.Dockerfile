FROM archlinux

ARG PUID=1000 PGID=1000

RUN pacman -Syu rustup base-devel musl --noconfirm

RUN groupadd abc -g $PGID
RUN useradd abc -m -u $PUID -g $PGID
USER abc

RUN rustup default stable
RUN cargo install cargo-arch
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /source/rolr
