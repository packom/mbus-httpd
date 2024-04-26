FROM ubuntu:22.04
LABEL maintainer="docker@packom.net"
LABEL website="www.packom.net"

# Install necessary packages
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get -y upgrade && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
        build-essential \
        autoconf \
        automake \
        cmake \
        git \
        libssl-dev \
        wget && \
    apt-get clean && \
    rm -fr /var/lib/apt/lists/*

# Compile libmbus
ENV GCC="gcc"
ENV COMPILE="$GCC -I. -c"
ENV AR="$GCC-ar"
ARG LIBMBUS_REPO="https://github.com/piersfinlayson/libmbus"
RUN mkdir /builds && \
    cd builds/ && \
    git clone ${LIBMBUS_REPO} && \
    cd libmbus && \
    mkdir -p output && \
    $COMPILE mbus/mbus-protocol.c -o output/mbus-protocol.o && \
    $COMPILE mbus/mbus-protocol-aux.c -o output/mbus-protocol-aux.o && \
    $COMPILE mbus/mbus-tcp.c -o output/mbus-tcp.o && \
    $COMPILE mbus/mbus-serial.c -o output/mbus-serial.o && \
    $AR rcs output/libmbus.a output/*.o && \
    $COMPILE bin/mbus-serial-scan.c -o output/mbus-serial-scan.o && \
    $COMPILE bin/mbus-serial-request-data.c -o output/mbus-serial-request-data.o && \
    $COMPILE bin/mbus-serial-request-data-multi-reply.c -o output/mbus-serial-request-data-multi-reply.o && \
    $GCC -static output/mbus-serial-scan.o -o output/mbus-serial-scan -Loutput -lc -lmbus -lm && \
    $GCC -static output/mbus-serial-request-data.o -o output/mbus-serial-request-data -Loutput -lc -lmbus -lm && \
    $GCC -static output/mbus-serial-request-data-multi-reply.o -o output/mbus-serial-request-data-multi-reply -Loutput -lc -lmbus -lm && \
    cp /builds/libmbus/output/mbus-serial-request-data / && \
    cp /builds/libmbus/output/mbus-serial-request-data-multi-reply / && \
    cp /builds/libmbus/output/mbus-serial-scan / && \
    cd / && \
    rm -fr /builds 

# Build mbus-httpd
ENV OPENSSL_INCLUDE_DIR="/usr/include/openssl"
RUN wget -O installrust.sh https://sh.rustup.rs && \
    sh ./installrust.sh -y  && \
    mkdir /builds && \
    cd /builds/ && \
    git clone https://github.com/packom/mbus-httpd && \
    cd mbus-httpd/ && \
    arch=$(dpkg --print-architecture) && \
      echo "Architecture: $(arch)" && \
      if [ $(arch) = "armhf" ] || [ $(arch) = "armv7l" ] ; \
      then \
        echo "OPENSSL_LIB_DIR=/usr/lib/arm-linux-gnueabihf/" ; \
        export OPENSSL_LIB_DIR="/usr/lib/arm-linux-gnueabihf/" ; \
        export JOBS=1 ; \
      elif [ $(arch) = "arm64" ] || [ $(arch) = "aarch64" ] ; \
      then \
        echo "OPENSSL_LIB_DIR=/usr/lib/aarch64-linux-gnu/" ; \
        export OPENSSL_LIB_DIR="/usr/lib/aarch64-linux-gnu/" ; \
        export JOBS=1 ; \
      elif [ $(arch) = "amd64" ] || [ $(arch) = "x86_64" ] ; \
      then \
        echo "OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/" ; \
        export OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu/" ; \
        export JOBS=8 ; \
      else \
        echo "unknown architecture" ; \
        export OPENSSL_LIB_DIR="/usr/lib/unknown/" ; \
        export JOBS=1 ; \
      fi && \
    /root/.cargo/bin/cargo build --jobs $JOBS --release && \
    cp /builds/mbus-httpd/target/release/mbus /mbus-httpd && \
    cd / && \
    rm -fr /builds && \
    /root/.cargo/bin/rustup self uninstall -y && \
    rm /installrust.sh
RUN mkdir /static/ && \
    wget https://raw.githubusercontent.com/packom/mbus-api/master/api/openapi.yaml -O /static/api.yaml

# Set up environment
ENV LIBMBUS_PATH=/
VOLUME ["/ssl"]
EXPOSE 8080
CMD ["/mbus-httpd"]