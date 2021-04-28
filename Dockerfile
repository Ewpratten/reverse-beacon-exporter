FROM rust:1.51.0
COPY . /work
WORKDIR /work
RUN cargo build --release

FROM ubuntu:20.04 
COPY --from=0 /work/target/release/reverse-beacon-exporter /reverse-beacon-exporter
EXPOSE 9814

CMD [ "/reverse-beacon-exporter" ]