FROM piersfinlayson/build:0.2.5 as libmbus
WORKDIR /home/build/builds
RUN git clone https://github.com/rscada/libmbus
WORKDIR /home/build/builds/libmbus
RUN ./build.sh
RUN ls -ltr /home/build/builds/libmbus/mbus/.libs/

FROM piersfinlayson/build:0.2.4 as mbus-httpd
WORKDIR /home/build/builds
ARG TYPE
RUN git clone https://github.com/packom/mbus-httpd
WORKDIR /home/build/builds/mbus-httpd
RUN cargo build --$TYPE

FROM scratch
WORKDIR /
COPY --from=libmbus /home/build/builds/libmbus/mbus/.libs/libmbus.so .
COPY --from=libmbus /home/build/builds/libmbus/bin/mbus-serial-request-data .
COPY --from=libmbus /home/build/builds/libmbus/bin/mbus-serial-scan .
COPY --from=mbus-httpd /home/build/builds/mbus-httpd/target/release/mbus-httpd .
ARG DIR
ADD $DIR/api.yaml /static/
VOLUME ["/ssl"]
EXPOSE 8080
CMD ["./mbus-httpd"]
