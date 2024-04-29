#
# docker buildx bake
#
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
        "packom/mbus-httpd:${TAG}"
    ]
    platforms = [
        "linux/amd64",
        "linux/arm64/v8",
        "linux/arm/v7",
        #"linux/arm/v6", No Ubuntu version
    ]
    output = ["${OUTPUT}"]
}
