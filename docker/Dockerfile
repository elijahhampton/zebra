# syntax=docker/dockerfile:1
# check=skip=UndefinedVar,UserExist # We use gosu in the entrypoint instead of USER directive

# If you want to include a file in the Docker image, add it to .dockerignore.
#
# We use 4 (TODO: 5) stages:
# - deps: installs build dependencies and sets default values
# - tests: prepares a test image
# - release: builds release binaries
# - runtime: prepares the release image
# - TODO: Add a `monitoring` stage
#
# We first set default values for build arguments used across the stages.
# Each stage must define the build arguments (ARGs) it uses.

ARG RUST_VERSION=1.85.0

# Keep in sync with vars.RUST_PROD_FEATURES in GitHub
# https://github.com/ZcashFoundation/zebra/settings/variables/actions
ARG FEATURES="default-release-binaries"

ARG UID=10001
ARG GID=${UID}
ARG USER="zebra"
ARG HOME="/home/${USER}"
ARG CARGO_HOME="${HOME}/.cargo"

# This stage prepares Zebra's build deps and captures build args as env vars.
FROM rust:${RUST_VERSION}-bookworm AS deps
SHELL ["/bin/bash", "-xo", "pipefail", "-c"]

# Install zebra build deps
RUN apt-get -qq update && \
    apt-get -qq install -y --no-install-recommends \
    libclang-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/* /tmp/*

# Build arguments and variables
ARG CARGO_INCREMENTAL
ENV CARGO_INCREMENTAL=${CARGO_INCREMENTAL:-0}

ARG CARGO_HOME
ENV CARGO_HOME=${CARGO_HOME}

ARG FEATURES
ENV FEATURES=${FEATURES}

# If this is not set, it must be an empty string, so Zebra can try an
# alternative git commit source:
# https://github.com/ZcashFoundation/zebra/blob/9ebd56092bcdfc1a09062e15a0574c94af37f389/zebrad/src/application.rs#L179-L182
ARG SHORT_SHA
ENV SHORT_SHA=${SHORT_SHA:-}

# This stage builds tests without running them.
#
# We also download needed dependencies for tests to work, from other images.
# An entrypoint.sh is only available in this step for easier test handling with variables.
FROM deps AS tests

# Skip IPv6 tests by default, as some CI environment don't have IPv6 available
ARG ZEBRA_SKIP_IPV6_TESTS
ENV ZEBRA_SKIP_IPV6_TESTS=${ZEBRA_SKIP_IPV6_TESTS:-1}

# This environment setup is almost identical to the `runtime` target so that the
# `tests` target differs minimally. In fact, a subset of this setup is used for
# the `runtime` target.
ARG UID
ENV UID=${UID}
ARG GID
ENV GID=${GID}
ARG USER
ENV USER=${USER}
ARG HOME
ENV HOME=${HOME}

RUN addgroup --quiet --gid ${GID} ${USER} && \
    adduser --quiet --gid ${GID} --uid ${UID} --home ${HOME} ${USER} --disabled-password --gecos ""

# Set the working directory for the build.
WORKDIR ${HOME}

# Build Zebra test binaries, but don't run them
#
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies, a cache mount to /usr/local/cargo/git/db
# for git repository dependencies, and a cache mount to ${HOME}/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to each crate directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted ${HOME}/target/ is unmounted.
RUN --mount=type=bind,source=zebrad,target=zebrad \
    --mount=type=bind,source=zebra-chain,target=zebra-chain \
    --mount=type=bind,source=zebra-network,target=zebra-network \
    --mount=type=bind,source=zebra-state,target=zebra-state \
    --mount=type=bind,source=zebra-script,target=zebra-script \
    --mount=type=bind,source=zebra-consensus,target=zebra-consensus \
    --mount=type=bind,source=zebra-rpc,target=zebra-rpc \
    --mount=type=bind,source=zebra-node-services,target=zebra-node-services \
    --mount=type=bind,source=zebra-test,target=zebra-test \
    --mount=type=bind,source=zebra-utils,target=zebra-utils \
    --mount=type=bind,source=zebra-scan,target=zebra-scan \
    --mount=type=bind,source=zebra-grpc,target=zebra-grpc \
    --mount=type=bind,source=tower-batch-control,target=tower-batch-control \
    --mount=type=bind,source=tower-fallback,target=tower-fallback \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=${HOME}/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo test --locked --release --workspace --no-run \
    --features "${FEATURES} zebra-checkpoints" && \
    cp ${HOME}/target/release/zebrad /usr/local/bin && \
    cp ${HOME}/target/release/zebra-checkpoints /usr/local/bin

# Copy the lightwalletd binary and source files to be able to run tests
COPY --from=electriccoinco/lightwalletd:v0.4.17 /usr/local/bin/lightwalletd /usr/local/bin/

# Copy the gosu binary to be able to run the entrypoint as non-root user
# and allow to change permissions for mounted cache directories
COPY --from=tianon/gosu:bookworm /gosu /usr/local/bin/

# As the build has already run with the root user,
# we need to set the correct permissions for the home and cargo home dirs owned by it.
RUN chown -R ${UID}:${GID} "${HOME}" && \
    chown -R ${UID}:${GID} "${CARGO_HOME}"

COPY --chown=${UID}:${GID} ./ ${HOME}
COPY --chown=${UID}:${GID} ./docker/entrypoint.sh /usr/local/bin/entrypoint.sh

ENTRYPOINT [ "entrypoint.sh", "test" ]
CMD [ "cargo", "test" ]

# This stage builds the zebrad release binary.
#
# It also adds `cache mounts` as this stage is completely independent from the
# `test` stage. The resulting zebrad binary is used in the `runtime` stage.
FROM deps AS release

# Set the working directory for the build.
ARG HOME
WORKDIR ${HOME}

RUN --mount=type=bind,source=tower-batch-control,target=tower-batch-control \
    --mount=type=bind,source=tower-fallback,target=tower-fallback \
    --mount=type=bind,source=zebra-chain,target=zebra-chain \
    --mount=type=bind,source=zebra-consensus,target=zebra-consensus \
    --mount=type=bind,source=zebra-grpc,target=zebra-grpc \
    --mount=type=bind,source=zebra-network,target=zebra-network \
    --mount=type=bind,source=zebra-node-services,target=zebra-node-services \
    --mount=type=bind,source=zebra-rpc,target=zebra-rpc \
    --mount=type=bind,source=zebra-scan,target=zebra-scan \
    --mount=type=bind,source=zebra-script,target=zebra-script \
    --mount=type=bind,source=zebra-state,target=zebra-state \
    --mount=type=bind,source=zebra-test,target=zebra-test \
    --mount=type=bind,source=zebra-utils,target=zebra-utils \
    --mount=type=bind,source=zebrad,target=zebrad \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=${HOME}/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release --features "${FEATURES}" --package zebrad --bin zebrad && \
    cp ${HOME}/target/release/zebrad /usr/local/bin

# This stage starts from scratch using Debian and copies the built zebrad binary
# from the `release` stage along with other binaries and files.
FROM debian:bookworm-slim AS runtime

ARG FEATURES
ENV FEATURES=${FEATURES}

# Create a non-privileged user for running `zebrad`.
#
# We use a high UID/GID (10001) to avoid overlap with host system users.
# This reduces the risk of container user namespace conflicts with host accounts,
# which could potentially lead to privilege escalation if a container escape occurs.
#
# We do not use the `--system` flag for user creation since:
# 1. System user ranges (100-999) can collide with host system users
#   (see: https://github.com/nginxinc/docker-nginx/issues/490)
# 2. There's no value added and warning messages can be raised at build time
#   (see: https://github.com/dotnet/dotnet-docker/issues/4624)
#
# The high UID/GID values provide an additional security boundary in containers
# where user namespaces are shared with the host.
ARG UID
ENV UID=${UID}
ARG GID
ENV GID=${GID}
ARG USER
ENV USER=${USER}
ARG HOME
ENV HOME=${HOME}

RUN addgroup --quiet --gid ${GID} ${USER} && \
    adduser --quiet --gid ${GID} --uid ${UID} --home ${HOME} ${USER} --disabled-password --gecos ""

WORKDIR ${HOME}
RUN chown -R ${UID}:${GID} ${HOME}

# We're explicitly NOT using the USER directive here.
# Instead, we run as root initially and use gosu in the entrypoint.sh
# to step down to the non-privileged user. This allows us to change permissions
# on mounted volumes before running the application as a non-root user.
# User with UID=${UID} is created above and used via gosu in entrypoint.sh.

# Copy the gosu binary to be able to run the entrypoint as non-root user
COPY --from=tianon/gosu:bookworm /gosu /usr/local/bin/
COPY --from=release /usr/local/bin/zebrad /usr/local/bin/
COPY --chown=${UID}:${GID} ./docker/entrypoint.sh /usr/local/bin/entrypoint.sh

ENTRYPOINT [ "entrypoint.sh" ]
CMD ["zebrad"]

# TODO: Add a `monitoring` stage
#
# This stage will be based on `runtime`, and initially:
#
# - run `zebrad` on Testnet
# - with mining enabled using S-nomp and `nheqminer`.
#
# We can add further functionality to this stage for further purposes.
