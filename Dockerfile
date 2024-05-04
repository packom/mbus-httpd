# syntax=docker/dockerfile:1

ARG XX_VERSION=1.4.0
ARG RUST_VERSION=1.77
ARG ALPINE_VERSION=3.19

# xx is a cross-compilation helper
FROM --platform=$BUILDPLATFORM tonistiigi/xx:${XX_VERSION} AS xx

FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} AS build

# Add some useful packages
RUN apk add --no-cache bash clang git lld make perl wget
COPY --from=xx / /

# Create a build directory
RUN mkdir /build 

# Get openssl source code
ARG OPENSSL_VERSION=3.0.13
RUN cd /build && \
    wget https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz -O ./openssl.tar.gz && \
    tar xzf openssl.tar.gz && \
    rm openssl.tar.gz && \
    mv openssl-${OPENSSL_VERSION} openssl

# Build openssl
ARG TARGETPLATFORM
ARG TARGETARCH
RUN xx-apk add --no-cache gcc linux-headers musl-dev
RUN cd /build/openssl && \
    export CONFIGURE_FLAGS="no-shared no-zlib -fPIC no-ssl3" && \
    echo "Compiling openssl for architecture: $TARGETARCH" && \
    if [ $TARGETARCH = "amd64" ] || [ $TARGETARCH = "x86_64" ] ; \
    then \
        export CONFIGURE_FLAGS="$CONFIGURE_FLAGS linux-x86_64" ; \
    elif [ $TARGETARCH = "aarch64" ] || [ $TARGETARCH = "arm64" ] ; \
    then \
        export CONFIGURE_FLAGS="$CONFIGURE_FLAGS linux-aarch64 -march=armv8-a+crc+simd+fp" ; \
    elif [ $TARGETARCH = "arm" ] ; \
    then \
        VARIANT=$(xx-info variant) ; \
        echo "ARM variant: $VARIANT" ; \
        if [ $VARIANT == "v7" ] ; \
        then \
            export CONFIGURE_FLAGS="$CONFIGURE_FLAGS linux-armv4 -march=armv7-a -mfpu=vfpv3-d16 -mfloat-abi=hard" ; \
        elif [ $VARIANT == "v6" ] ; \
        then \
            export CONFIGURE_FLAGS="$CONFIGURE_FLAGS linux-armv4 -march=armv6 -marm -mfpu=vfp" ; \
        else \
            echo "Unsupported variant" ; \
        fi \
    else \
        echo "Unsupported architecture" ; \
        exit 1 ; \
    fi && \
    export CC=xx-clang && \
    ./config $CONFIGURE_FLAGS --prefix=/usr/local/ssl --openssldir=/usr/local/ssl && \
    make -j 4 depend && \
    make -j 4 && \
    make install_sw
    
# Get mbus-httpd source
RUN cd /build && \
    git clone https://github.com/packom/mbus-httpd

# Build mbus-httpd
RUN export RUST_TARGET=$(xx-cargo --print-target-triple) && \
    if [ $TARGETARCH = "arm" ] ; \
    then \
        VARIANT=$(xx-info variant) ; \
        if [ $VARIANT == "v6" ] ; \
        then \
            echo "armv6" ; \
            export RUSTFLAGS="-Clink-arg=-L/armv6-alpine-linux-musleabihf/usr/lib/" ; \
        fi \
    fi && \
    echo "Rust target: $RUST_TARGET" && \
    cd /build/mbus-httpd && \
    env OPENSSL_DIR=/usr/local/ssl xx-cargo build --release && \
    xx-verify ./target/$RUST_TARGET/release/mbus

RUN mkdir /binaries && \
    cp ./build/mbus-httpd/target/$(xx-cargo --print-target-triple)/release/mbus /binaries/mbus-httpd
RUN mkdir /static && \
    wget https://raw.githubusercontent.com/packom/mbus-api/master/api/openapi.yaml -O /static/api.yaml

# Create the final container
FROM scratch
COPY --from=build /binaries /
COPY --from=build /static /static

ENV LIBMBUS_PATH=/
VOLUME ["/ssl"]
EXPOSE 8080
CMD ["/mbus-httpd"]
