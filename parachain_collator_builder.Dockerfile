# Usage

# This builds the Parachain Collator node for the Totem Parachain.

# docker build \
# -t totemlive/totem-parachain:local \
# -f parachain_collator_builder.Dockerfile \
# --build-arg package=totem-parachain-node \
# --build-arg buildtype=check .

# docker build \
# -t totemlive/totem-parachain:local \
# -f parachain_collator_builder.Dockerfile \
# --build-arg package=totem-parachain-node \
# --build-arg buildtype=build .

# This is the build stage for Totem Parachain. Here we create the binary.

FROM docker.io/paritytech/ci-linux:production as builder
ARG package
ARG buildtype

## constants
ARG PROFILE=release

WORKDIR /totem-parachains
COPY . /totem-parachains

# rust compiler command 
RUN cargo ${buildtype} --${PROFILE} --locked -p ${package}

# This is the 2nd stage: a very small image where we copy the Totem Parachain Collator binary."
FROM docker.io/library/ubuntu:20.04

LABEL description="Multistage Docker image for Totem Live Accounting Parachain" \
	totem.live.image.type="builder" \
	totem.live.image.authors="chris.dcosta@totemaccounting.com" \
	totem.live.image.vendor="Totem Accounting" \
	totem.live.image.description="Totem is a p2p accounting engine for the decentralised economy 🚀" \
	totem.live.image.source="https://github.com/totem-tech/totem-parachains/parachain_collator_builder.Dockerfile" \
	totem.live.image.documentation="https://github.com/totem-tech/totem-parachains"

COPY --from=builder /totem-parachains/target/release/totem-parachain-collator /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /totem-parachains totemadmin && \
mkdir -p /totem-parachains/.local/share/totem-parachain-collator && \
chown -R totemadmin:totemadmin /totem-parachains/.local/share && \
ln -s /totem-parachains/.local/share /data && \
rm -rf /usr/bin /usr/sbin && \
/usr/local/bin/totem-parachain-collator --version

USER totemadmin

EXPOSE 30333 9933 9944 9615 40333 9934 9945 9616

VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/totem-parachain-collator"]