FROM piersfinlayson/build-amd64:0.3.6 as libmbus
WORKDIR /home/build/builds
RUN git clone https://github.com/rscada/libmbus
WORKDIR /home/build/builds/libmbus
# Don't use the makefile, as doesn't handle cross compilation properly
# and/or I haven't figured out how to do it.
# Statically linking as I can't figure out how to dynamically link with
# libmbus.so but statically link with libc.a.  The binaries are small so
# it's not a big deal.
RUN mkdir -p output
ARG GCC
ARG AR
ENV COMPILE="$GCC -I. -c"
RUN $COMPILE mbus/mbus-protocol.c -o output/mbus-protocol.o
RUN $COMPILE mbus/mbus-protocol-aux.c -o output/mbus-protocol-aux.o
RUN $COMPILE mbus/mbus-tcp.c -o output/mbus-tcp.o
RUN $COMPILE mbus/mbus-serial.c -o output/mbus-serial.o
RUN $AR rcs output/libmbus.a output/*.o
RUN $COMPILE bin/mbus-serial-scan.c -o output/mbus-serial-scan.o
RUN $COMPILE bin/mbus-serial-request-data.c -o output/mbus-serial-request-data.o
RUN $COMPILE bin/mbus-serial-request-data-multi-reply.c -o output/mbus-serial-request-data-multi-reply.o
RUN $GCC -static output/mbus-serial-scan.o -o output/mbus-serial-scan -Loutput -lc -lmbus
RUN $GCC -static output/mbus-serial-request-data.o -o output/mbus-serial-request-data -Loutput -lc -lmbus
RUN $GCC -static output/mbus-serial-request-data-multi-reply.o -o output/mbus-serial-request-data-multi-reply -Loutput -lc -lmbus

FROM piersfinlayson/build-amd64:0.3.6 as mbus-httpd
WORKDIR /home/build/builds
RUN git clone https://github.com/packom/mbus-httpd
WORKDIR /home/build/builds/mbus-httpd
ARG TYPE
ARG TARGET
RUN cargo build $TYPE --target=$TARGET

FROM scratch
WORKDIR /
COPY --from=libmbus /home/build/builds/libmbus/output/mbus-serial-scan .
COPY --from=libmbus /home/build/builds/libmbus/output/mbus-serial-request-data .
COPY --from=libmbus /home/build/builds/libmbus/output/mbus-serial-request-data-multi-reply .
ARG TARGET
ARG COPY_TYPE
COPY --from=mbus-httpd /home/build/builds/mbus-httpd/target/$TARGET/$COPY_TYPE/mbus ./mbus-httpd
ARG DIR
ADD $DIR/api.yaml /static/
ENV LIBMBUS_PATH=/
VOLUME ["/ssl"]
EXPOSE 8080
CMD ["/mbus-httpd"]
