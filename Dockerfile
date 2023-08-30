# ---------------------------------------------------
# 1 - Build Stage
#
# Use official rust image to for application build
# ---------------------------------------------------
FROM rust:1.71.0 as build

# Setup working directory
WORKDIR /usr/src/life
COPY . .
COPY .env.docker .env

# Build application
RUN cargo install --path .

# ---------------------------------------------------
# 2 - Deploy Stage
#
# Use a distroless image for minimal container size
# - Copy `libpq` dependencies into the image (Required by diesel)
# - Copy application files into the image
# ---------------------------------------------------
FROM gcr.io/distroless/cc-debian11

# Set the architecture argument (arm64, i.e. aarch64 as default)
# For amd64, i.e. x86_64, you can append a flag when invoking the build `... --build-arg "ARCH=x86_64"`
ARG ARCH=aarch64


# Application files
COPY --from=build /usr/local/cargo/bin/life /usr/local/bin/life
COPY --from=build /usr/src/life/.env /.env

CMD ["life"]