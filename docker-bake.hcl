#
# docker buildx bake
#
variable "REGISTRY" {
    default = "registry:80"
}
variable "TAG" {
    # description = "Image tag (version), e.g. latest"
    default = "latest"
}
variable "OUTPUT" {
    # description = "Output required from buildx, e.g. type=registry"
    default = "type=registry"
}

group "default" {
    targets = [
        "mbus-httpd",
    ]
}

target "mbus-httpd" {
    dockerfile = "Dockerfile"
    tags = [
        "${REGISTRY}/mbus-httpd:${TAG}"
    ]
    platforms = [
        "linux/amd64",
        "linux/arm64/v8",
        "linux/arm/v7",
        "linux/arm/v6",
    ]
    output = ["${OUTPUT}"]
}
