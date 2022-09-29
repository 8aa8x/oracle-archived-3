# Aces and Eights Oracle

> **Warning**
>
> This software is still being developed and not at all ready for production. It is missing many core features that will make it unstable and unusuable at times. Until a release is published, do no use this for anything other than testing and developing.

A highly redundant, multi-threaded blockchain oracle for listening and responding to blockchains.

Currently supporting Ethereum.

## Core Beliefs

- `Fault tolerant` - The oracle can recover from downtime, panics, and unexpected behavior
- `Distributed` - Many instances of the oracle can run simultaneously without duplication
- `Pluggable` - It is possible for any developer to create a new listener or responder easily
