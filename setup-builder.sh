docker buildx create --name my-builder --node smallboy1 --platform linux/amd64,linux/arm64/v8,linux/arm/v7,linux/arm/v6 --config buildkitd.toml --driver docker-container
docker buildx inspect --builder my-builder --bootstrap
docker update buildx_buildkit_smallboy1 --restart=always
