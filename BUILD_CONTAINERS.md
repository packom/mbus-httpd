Set up buildx.

```
./setup-builder.sh
```

Build the images (uploads to registry:80):

```
docker buildx bake
```

Test the container(s) on the appropriate platforms.

Then upload the images to hub.docker.com:

```
docker login -u packom
REGISTRY=packom docker buildx bake
```

