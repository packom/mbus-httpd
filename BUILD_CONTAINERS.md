Set up buildx.

First login:

```
docker login -u packom
```

Then:

```
docker buildx bake
```

If you get an error about failing to merge manifests, build each image separately, e.g.:

```
docker buildx build --platform linux/arm64 -t packom/mbus-httpd-arm64:29.04.24 .
docker buildx build --platform linux/arm/v7 -t packom/mbus-httpd-armv7:29.04.24 .
docker buildx build --platform linux/amd64 -t packom/mbus-httpd-amd64:29.04.24 .
```

Note you can build an armv6 (actually armv5) image, by running the following on a Pi Zero (ARMv6):

```
docker build . -t packom/mbus-httpd-armv6:29.04.24 .
docker push packom/mbus/mbus-httpd-armv6:29.04.24
```

Now build a manifest:

```
docker manifest create packom/mbus-httpd:29.04.24 \
    packom/mbus-httpd-arm64:29.04.24 \
    packom/mbus-httpd-armv7:29.04.24 \
    packom/mbus-httpd-amd64:29.04.24 \
    packom/mbus-httpd-armv6:29.04.24
```

Now push it:

```
docker manifest push packom/mbus-httpd:29.04.24
```
