# [mbus-httpd](https://github.com/packom/mbus-httpd)

An HTTP microservice exposing (wired) M-Bus functionality, allowing you to scan an M-Bus and query M-Bus slaves.

You'll need a serial - M-Bus device to connect your server to an M-Bus.

If you want to use a Raspberry Pi and want an M-Bus Master Hat, see [here](https://www.packom.net/m-bus-master-hat/).  Alternatively

* buy a USB M-Bus Master device from ebay or aliexpress
* build your own using widely available [schematics](https://otb-iot.readthedocs.io/en/latest/mbus.html).

## Quickstart

The easiest way to run mbus-httpd is to use a pre-built container, using docker.  An mbus-httpd docker manifest is available which supports:

* x86_64
* ARMv6 & ARMv7 (which together cover all Raspberry Pi models).

If you need docker the easiest way to get it is to run:

```
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh
```

Once docker is installed (and you may need to logout and back in after installing) you can run mbus-httpd using:

```
docker run --name mbus-httpd -d -p 8080:8080 -e RUST_LOG=INFO packom/mbus-httpd-release
```

This will start the mbus-httpd web server listening on port 8080.

To check whether it is working, from another shell run:

```
curl -v -X GET http://localhost:8080/mbus/api
```

Or, modify this as appropriate and stick in a browser:

```
http://<your_host_name>:8080/mbus/api 
```

The [YAML API document](https://github.com/packom/mbus-httpd/blob/master/api/openapi.yaml) should be returned.

## Using

To scan the M-Bus connected to device /dev/ttyAMA0 at 2400 baud:

```
curl -v X POST http://localhost:8080/mbus/scan/ttyAMA0/2400
```

To get info from a device address 48 (0x30):

```
curl -v X POST http://localhost:8080/mbus/get/ttyAMA0/2400/48
```

## Building

To build you'll need [Rust](https://www.rust-lang.org/tools/install) installed.  If you don't want to go to the effort of installing Rust, you can use a [build container supporting Rust](https://piers.rocks/docker/containers/raspberry/pi/rust/cross/compile/compilation/2018/12/16/rust-compilation-for-raspberry-pi.html) such as [this one](https://hub.docker.com/r/piersfinlayson/build), which works on x86_64, ARMv6 and ARMv7 (so all flavours of Raspberry Pis):

```
docker run --rm -ti -v some_local_dir:/home/build/builds piersfinlayson/build
```

Once Rust is installed run:

```
cd ~
git clone https://github.com/packom/mbus-httpd
cd mbus-httpd
cargo build
```

You also need libmbus.  On Ubuntu you can install like this:

```
sudo apt install libtool autoconf
cd ~
git clone https://github.com/rscada/libmbus
cd libmbus
./build.sh
sudo make install
```

## Running

To run:

```
cd ~/mbus-httpd
cargo run
```

As mbus-httpd is designed to run in a container, all configuration is done by environment variables.  You'll almost certainly want:

```
LIBMBUS_PATH=<limbus binary path e.g. ~/libmbus/bin>
LD_LIBRARY_PATH<path libmbus.so is installed to e.g. /usr/local/lib>
```

You may also want:

```
SERVER_IP=<IP to listen on>
SERVER_PORT=<port to listen on>
RUST_LOG=<log level, e.g. INFO>
```

So for example:

```
cd ~/mbus-httpd
env SERVER_IP=localhost \
env SERVER_PORT=8080 \
env LIBMBUS_PATH=~/libmbus/bin \
env LD_LIBRARY_PATH=/usr/local/lib \
env RUST_LOG=INFO \
cargo run
```

To view logs, make sure RUST_LOG is set to INFO or DEBUG (see above), and run:

```
docker logs -f mbus-httpd
```

## License

[mbus-httpd](https://github.com/packom/mbus-httpd) is licensed under the [GPL v3.0 or later](https://github.com/packom/mbus-httpd/blob/master/LICENSE).

[libmbus](https://github.com/rscada/libmbus) is licensed under the [BSD](https://github.com/rscada/libmbus/blob/master/LICENSE).
