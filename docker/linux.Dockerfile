FROM debian:trixie-slim
ARG DEBIAN_FRONTEND=noninteractive
ARG WORKDIR=/home/copykitten
WORKDIR $WORKDIR

# Install the toolchain.

RUN apt-get update && apt-get install -y --no-install-recommends \
      build-essential curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /usr/local/bin/
RUN uv python install 3.10 \
 && uv tool install maturin
ENV PATH=/root/.local/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --profile minimal --default-toolchain stable
ENV PATH=/root/.cargo/bin:$PATH

COPY requirements-dev.txt pyproject.toml Cargo.toml Cargo.lock README.md $WORKDIR/
RUN uv venv --python 3.10 .venv && uv pip install -r requirements-dev.txt --python .venv

# Download crates once, not every time the sources change.
# The stub lib.rs only exists so Cargo can read the manifest.
RUN mkdir -p src && touch src/lib.rs && cargo fetch && rm -rf src

RUN mkdir -p /tmp/xdg && chmod 700 /tmp/xdg

# Install platform-specific dependencies.

ARG DEPENDENCIES
RUN apt-get update && apt-get install -y --no-install-recommends $DEPENDENCIES \
    && rm -rf /var/lib/apt/lists/*

# Copy sources, build the crate, execute the entrypoint.

COPY /python $WORKDIR/python/
COPY /src $WORKDIR/src/
COPY /tests $WORKDIR/tests/
RUN maturin build
RUN uv run -m maturin_import_hook site install

COPY --chmod=+x docker/entrypoint.sh /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
