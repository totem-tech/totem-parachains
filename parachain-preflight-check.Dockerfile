# Usage

# This tests the build for the Parachain Collator node for the Totem Parachain.

# docker build \
# -t totemlive/totem-parachain:check \
# -f parachain-preflight-check.Dockerfile \
# --build-arg package=totem-parachain-node .

# This is the build stage for Totem Parachain. Here we create the binary.
FROM docker.io/paritytech/ci-linux:production as builder
ARG package

WORKDIR /totem-parachains
COPY . /totem-parachains

# rust compiler command 
RUN cargo check --locked -p ${package}