# FROM rust:1.50 as builder
# FROM registry.access.redhat.com/ubi8/go-toolset:latest as builder

# USER root

# RUN yum update -y --setopt=tsflags=nodocs && \
#     yum clean all -y

FROM registry.access.redhat.com/ubi8/ubi-minimal as builder
USER root

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.50.0

# Apply security patches
RUN set -xe; \
    microdnf update -y && \
    microdnf clean all && rm -rf /var/cache/yum && \
    microdnf install gcc-c++ gcc openssl openssl-devel python2

RUN set -xue; \ 
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs  > rustup.sh; \
    chmod +x ./rustup.sh && \
    ./rustup.sh -v --no-modify-path --default-toolchain ${RUST_VERSION} -y --profile minimal && \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    rustup component add rustfmt

WORKDIR /src
COPY . .

RUN cargo install --path .

FROM registry.access.redhat.com/ubi8/ubi-minimal
# Apply security patches
RUN set -xe; \
    microdnf update -y && \
    microdnf clean all && rm -rf /var/cache/yum

COPY --from=builder /usr/local/cargo/bin/fortuna /usr/local/bin/fortuna
USER 1001
CMD ["fortuna"]
