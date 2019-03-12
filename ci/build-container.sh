#!/bin/bash
set -e

function output_args {
    echo "Usage: build-container.sh <arch> <release|debug> <--no-cache>"
    echo "  <arch> = x86_64|arm|armv7"
    exit 1
}

ARCH=$1
if [[ ! $ARCH ]];
  then
    output_args
fi
if [[ $ARCH == "x86_64" ]];
  then
    TARGET="x86_64-unknown-linux-musl"
    COMPILER_DIR="/opt/cross/x86_64/bin"
    GCC="$COMPILER_DIR/x86_64-linux-musl-gcc"
elif [[ $ARCH == "arm" ]]
  then
    TARGET="arm-unknown-linux-musleabihf"
    COMPILER_DIR="/opt/cross/armv6/bin"
    GCC="$COMPILER_DIR/arm-linux-musleabihf-gcc"
elif [[ $ARCH == "armv7" ]]
  then
    TARGET="armv7-unknown-linux-musleabihf"
    COMPILER_DIR="/opt/cross/armv7/bin"
    GCC="$COMPILER_DIR/arm-linux-musleabihf-gcc"
else
  output_args
fi
AR="$GCC-ar"

TYPE=$2
if [[ ! $TYPE ]];
  then
    output_args
fi
if [[ $TYPE == "release" ]];
  then
    TYPE="release"
    BUILD_TYPE="--release"
elif [[ $TYPE == "debug" ]]
  then
    TYPE="debug"
    BUILD_TYPE=""
else
  output_args
fi

NO_CACHE=$3

VERSION="$(awk '/^version = /{print $3}' Cargo.toml | sed 's/"//g' | sed 's/\r$//')"
if [[ ! $VERSION ]];
  then
    echo "Couldn't get version from Cargo.toml"
    exit 1
fi

BIN="$(awk '/^name = /{print $3}' Cargo.toml | sed 's/"//g' | sed 's/\r$//')"
if [[ ! $BIN ]];
  then
    echo "Couldn't get binary from Cargo.toml"
    exit 1
fi

DIR=tmp/$BIN-$TYPE-$ARCH-$VERSION
CIDIR=ci/$DIR
TAG=packom/$BIN-$TYPE-$ARCH:$VERSION

echo "Creating container for"
echo "  Binary:    $BIN"
echo "  Arch:      $ARCH"
echo "  Target:    $TARGET"
echo "  Type:      $TYPE"
echo "  Version:   $VERSION"
echo "  Tag:       $TAG"
echo "  gcc:       $GCC"
echo "  ar:        $AR"

rm -fr $DIDIR
mkdir -p $CIDIR

echo "Getting API: ./api/openapi.yaml"
cp ./api/openapi.yaml $CIDIR/api.yaml

echo "docker build -t $TAG --build-arg DIR=$DIR --build-arg TYPE=$TYPE $NO_CACHE --build-arg TARGET=$TARGET ./ci"
docker build -t $TAG \
  --build-arg DIR=$DIR \
  --build-arg TYPE=$TYPE \
  --build-arg TARGET=$TARGET \
  --build-arg GCC=$GCC \
  --build-arg AR=$AR \
  $NO_CACHE \
  ./ci

rm -fr $CIDIR
