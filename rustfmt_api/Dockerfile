FROM rust:latest
RUN rustup component add rustfmt

WORKDIR /builder
COPY ./ .
RUN cargo build --release

WORKDIR /

RUN cp /builder/target/release/rustfmt_api /rustfmt_api
RUN chmod +x ./rustfmt_api
RUN rm -rf /builder

EXPOSE 46530
CMD ["/rustfmt_api"]
