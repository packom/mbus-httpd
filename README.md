# mbus-httpd
An HTTP microservice exposing (wired) M-Bus functionality.

## Building

You'll need [Rust](https://www.rust-lang.org/tools/install) installed.  Once installed run:

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

```
cd ~/mbus-httpd
cargo run
```

Some useful environment various to set:

```
SERVER_IP=<IP to listen on>
SERVER_PORT=<port to listen on>
LIBMBUS_PATH=<limbus binary path e.g. ~/libmbus/bin>
LD_LIBRARY_PATH<path libmbus.so is installed to e.g. /usr/local/lib>
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

## Using

To scan the M-Bus connected to device /dev/ttyAMA0 at 2400 baud:

```
curl -v X POST http://localhost:8080/mbus/scan/ttyAMA0/2400
```

To get info from a device address 48 (0x30):

```
curl -v X POST http://localhost:8080/mbus/get/ttyAMA0/2400/48
```

## License

GPL v3.0 or later
