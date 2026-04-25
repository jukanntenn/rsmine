#!/bin/bash
# Multi-architecture Docker image build script for Rsmine
# Builds: rsmine-api (Rust), rsmine-web (Next.js)
# Supports: linux/amd64, linux/arm64
#
# Usage:
#   ./build.sh                                    # Build both images for host architecture
#   IMAGE_NAME=rsmine-api ./build.sh              # Build only rsmine-api
#   IMAGE_NAME=rsmine-web ./build.sh              # Build only rsmine-web
#   PUSH=true ./build.sh                          # Build and push both images (multi-arch)
#   IMAGE_TAG=1.0.0 ./build.sh                    # Tag with specific version
#   REGISTRY=myregistry ./build.sh                # Use custom registry
#
# Environment Variables:
#   IMAGE_NAME   - Image to build (default: builds both rsmine-api and rsmine-web)
#   IMAGE_TAG    - Image tag (default: latest)
#   REGISTRY     - Docker registry (default: 192.168.5.50:5000)
#   PLATFORMS    - Target platforms for push (default: linux/amd64,linux/arm64)
#   PUSH         - Push to registry (default: false). Set to 'true' for multi-arch build+push

set -e

# Configuration
REGISTRY="${REGISTRY:-192.168.5.50:5000}"
IMAGE_TAG="${IMAGE_TAG:-latest}"
PLATFORMS="${PLATFORMS:-linux/amd64,linux/arm64}"
PUSH="${PUSH:-false}"
IMAGE_NAME="${IMAGE_NAME:-}"

# Detect host platform for local builds
HOST_ARCH=$(docker version --format '{{.Server.Arch}}' 2>/dev/null || echo "amd64")
case "$HOST_ARCH" in
    x86_64|amd64) HOST_PLATFORM="linux/amd64" ;;
    aarch64|arm64) HOST_PLATFORM="linux/arm64" ;;
    *) HOST_PLATFORM="linux/amd64" ;;
esac

# Project root (parent of docker directory)
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Build a single image
build_image() {
    local name="$1"
    local dockerfile="$2"

    local full_image="${REGISTRY}/${name}"

    echo ""
    echo "=========================================="
    echo "Building: ${name}"
    echo "Image: ${full_image}:${IMAGE_TAG}"
    if [ "$PUSH" = "true" ]; then
        echo "Platforms: $PLATFORMS (multi-arch push)"
    else
        echo "Platforms: $HOST_PLATFORM (local build)"
    fi
    echo "=========================================="

    local build_args=()
    build_args+=(--tag "${full_image}:${IMAGE_TAG}")
    if [ "$IMAGE_TAG" != "latest" ]; then
        build_args+=(--tag "${full_image}:latest")
    fi

    # Pass BACKEND_URL build arg for rsmine-web (Next.js rewrites are baked in at build time)
    if [ "$name" = "rsmine-web" ]; then
        build_args+=(--build-arg "BACKEND_URL=http://rsmine-api:3001")
    fi

    if [ "$PUSH" = "true" ]; then
        build_args+=(--platform "$PLATFORMS")
        build_args+=(--push)
    else
        build_args+=(--platform "$HOST_PLATFORM")
        build_args+=(--load)
    fi

    echo "Running: docker buildx build ${build_args[*]} -f ${dockerfile} ."
    docker buildx build "${build_args[@]}" -f "${dockerfile}" .

    echo "✓ ${name} built: ${full_image}:${IMAGE_TAG}"
}

# Ensure buildx is available
echo "Checking Docker buildx..."
docker buildx version >/dev/null 2>&1 || {
    echo "Error: Docker buildx is not available"
    exit 1
}

# Create/reuse buildx builder
BUILDER_NAME="multiarch-builder"
if ! docker buildx inspect "$BUILDER_NAME" >/dev/null 2>&1; then
    echo "Creating buildx builder: $BUILDER_NAME"
    docker buildx create --name "$BUILDER_NAME" --driver docker-container --use
else
    docker buildx use "$BUILDER_NAME"
fi
docker buildx inspect --bootstrap >/dev/null

cd "$PROJECT_ROOT"

# Build selected or both images
if [ -n "$IMAGE_NAME" ]; then
    case "$IMAGE_NAME" in
        rsmine-api) build_image "rsmine-api" "docker/Dockerfile.api" ;;
        rsmine-web) build_image "rsmine-web" "docker/Dockerfile.web" ;;
        *) echo "Error: Unknown image name '$IMAGE_NAME'. Use rsmine-api or rsmine-web."; exit 1 ;;
    esac
else
    build_image "rsmine-api" "docker/Dockerfile.api"
    build_image "rsmine-web" "docker/Dockerfile.web"
fi

echo ""
echo "=========================================="
echo "All builds completed!"
echo "Registry: ${REGISTRY}"
echo "Tag: ${IMAGE_TAG}"
echo "=========================================="
