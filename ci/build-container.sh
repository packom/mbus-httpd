#!/bin/bash
set -e

function output_args {
    echo "Usage: $0 <hub.docker.com repo_name> <arch> <release|debug> <--no-cache>"
    echo "  <arch> = x86_64|arm|armv7"
    exit 1
}

REPO=$1
if [[ ! $REPO ]];
  then
    output_args
    echo "Missing arguments"
elif [[ $REPO == "-h" ]]
  then
    output_args
elif [[ $REPO == "-?" ]]
  then
    output_args
elif [[ $REPO == "--help" ]]
  then
    output_args
fi

ARCH=$2
if [[ ! $ARCH ]];
  then
    output_args
    echo "Missing arguments"
elif [[ $ARCH == "x86_64" ]]
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
  echo "Invalid argument"
fi
AR="$GCC-ar"

TYPE=$3
if [[ ! $TYPE ]];
  then
    output_args
    echo "Missing argument"
elif [[ $TYPE == "release" ]]
  then
    TYPE="release"
    BUILD_TYPE="--release"
elif [[ $TYPE == "debug" ]]
  then
    TYPE="debug"
    BUILD_TYPE=""
else
  output_args
  echo "Invalid argument"
fi

NO_CACHE=$4

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
TAG=$REPO/$BIN-$TYPE-$ARCH:$VERSION

echo "Creating container for"
echo "  Binary:    $BIN"
echo "  Arch:      $ARCH"
echo "  Target:    $TARGET"
echo "  Type:      $TYPE"
echo "  Version:   $VERSION"
echo "  Tag:       $TAG"
echo "  gcc:       $GCC"
echo "  ar:        $AR"
echo "  Repo:      $REPO"

rm -fr $DIDIR
mkdir -p $CIDIR

echo "Getting API: ./api/openapi.yaml"
cp ./api/openapi.yaml $CIDIR/api.yaml

echo "docker build -t $TAG --build-arg DIR=$DIR --build-arg TYPE=$BUILD_TYPE $NO_CACHE --build-arg TARGET=$TARGET ./ci"
docker build -t $TAG \
  --build-arg DIR=$DIR \
  --build-arg TYPE=$TYPE \
  --build-arg TARGET=$TARGET \
  --build-arg GCC=$GCC \
  --build-arg AR=$AR \
  $NO_CACHE \
  ./ci

rm -fr $CIDIR

echo "Pushing image $TAG"

docker push $TAG