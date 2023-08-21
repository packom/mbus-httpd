On a 64-bit pi:

```
docker build . -t packom/mbus-release-aarch64:YY.MM
docker login -u packom
docker push packom/mbus-release-aarch64:YY.MM
```

On a 32-bit pi:

```
docker build . -t packom/mbus-release-armhf:YY.MM
docker login -u packom
docker push packom/mbus-release-armhf:YY.MM
```

On an x86-64bit machine:

```
docker build . -t packom/mbus-release-armhf:YY.MM
docker login -u packom
docker push packom/mbus-release-amd64:YY.MM
```

Then on one machine

```
EXPORT VERSION=YY.MM
docker manifest create -a packom/mbus-release:${VERSION} packom/mbus-release-amd64:${VERSION} packom/mbus-release-aarch64:${VERSION} packom/mbus-release-armhf:${VERSION}
docker manifest annotate --arch amd64 --os linux packom/mbus-release:${VERSION} packom/mbus-release-amd64:${VERSION}
docker manifest annotate --arch arm --variant armv7l --os linux packom/mbus-release:${VERSION} packom/mbus-release-aarch64:${VERSION}
docker manifest annotate --arch arm --variant aarch64 --os linux packom/mbus-release:${VERSION} packom/mbus-release-aarch64:${VERSION}
docker manigest inspect packom/mbus-release:${VERSION}
docker manifest push --purge packom/mbus-release:${VERSION}
```

```
docker manifest create -a packom/mbus-release:latest packom/mbus-release-amd64:${VERSION} packom/mbus-release-aarch64:${VERSION} packom/mbus-release-armhf:${VERSION}
docker manifest annotate --arch amd64 --os linux packom/mbus-release:latest packom/mbus-release-amd64:${VERSION}
docker manifest annotate --arch arm --variant armv7l --os linux packom/mbus-release:latest packom/mbus-release-aarch64:${VERSION}
docker manifest annotate --arch arm --variant aarch64 --os linux packom/mbus-release:latest packom/mbus-release-aarch64:${VERSION}
docker manigest inspect packom/mbus-release:latest
docker manifest push --purge packom/mbus-release:latest
```

