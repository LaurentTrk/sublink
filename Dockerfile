FROM ubuntu:21.10

RUN apt-get update && apt-get install libc6

WORKDIR /app
COPY ./parachain-collator /app/parachain-collator
COPY ./deployment/specs/relaychain.spec /specs/relaychain.spec
COPY ./deployment/specs/rococo-local-sublink-parachain-2000-raw.json /specs/rococo-local-sublink-parachain-2000-raw.json
COPY ./deployment/specs/rococo-local-sublink-parachain-2001-raw.json /specs/rococo-local-sublink-parachain-2001-raw.json

ENTRYPOINT ["/app/parachain-collator"]