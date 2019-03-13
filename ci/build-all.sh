#!/bin/bash
set -e

function output_args {
    echo "Usage: $0 <hub.docker.com repo name> <release|debug> <--no_cache>"
    exit 1
}

REPO=$1
if [[ ! $REPO ]];
  then
    echo "Missing argument"
    output_args
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

TYPE=$2
if [[ ! $TYPE ]];
  then
    echo "Missing argument"
    output_args
elif [[ $TYPE == "-h" ]]
  then
    output_args
elif [[ $TYPE == "-?" ]]
  then
    output_args
elif [[ $TYPE == "--help" ]]
  then
    output_args
elif [[ $TYPE == "release" ]]
  then
    TYPE="release"
elif [[ $TYPE == "debug" ]]
  then
    TYPE="debug"
else
  echo "Invalid argument"
  output_args
fi

NO_CACHE=$3

DIR=`dirname "$0"`

echo "Log into hub.docker.com as $REPO"
docker login -u $REPO

# Build containers
echo "$DIR/build-container.sh $REPO x86_64 $TYPE $NO_CACHE"
$DIR/build-container.sh $REPO x86_64 $TYPE $NO_CACHE
echo "$DIR/build-container.sh $REPO arm $TYPE $NO_CACHE"
$DIR/build-container.sh $REPO arm $TYPE $NO_CACHE
echo "$DIR/build-container.sh $REPO armv7 $TYPE $NO_CACHE"
$DIR/build-container.sh $REPO armv7 $TYPE $NO_CACHE

# Create manifests
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

X86_64_TAG=$REPO/$BIN-$TYPE-x86_64:$VERSION
ARM_TAG=$REPO/$BIN-$TYPE-arm:$VERSION
ARMV7_TAG=$REPO/$BIN-$TYPE-armv7:$VERSION
GEN_TAG_V=$REPO/$BIN-$TYPE:$VERSION
GEN_TAG_L=$REPO/$BIN-$TYPE:latest

docker manifest create $GEN_TAG_V $X86_64_TAG $ARM_TAG $ARMV7_TAG
docker manifest annotate --arch amd64 --os linux $GEN_TAG_V $X86_64_TAG
docker manifest annotate --arch arm --os linux --variant armv6l $GEN_TAG_V $ARM_TAG
docker manifest annotate --arch arm --os linux --variant armv7l $GEN_TAG_V $ARMV7_TAG
docker manifest inspect $GEN_TAG_V
docker manifest push --purge $GEN_TAG_V
docker pull $GEN_TAG_V

docker manifest create $GEN_TAG_L $X86_64_TAG $ARM_TAG $ARMV7_TAG
docker manifest annotate --arch amd64 --os linux $GEN_TAG_L $X86_64_TAG
docker manifest annotate --arch arm --os linux --variant armv6l $GEN_TAG_L $ARM_TAG
docker manifest annotate --arch arm --os linux --variant armv7l $GEN_TAG_L $ARMV7_TAG
docker manifest inspect $GEN_TAG_L
docker manifest push --purge $GEN_TAG_L
docker pull $GEN_TAG_L

