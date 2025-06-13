#!/bin/bash
source .env
export RUST_LOG
export MINIMAL_LOG_FORMAT
export MINING_PUBKEY
rm -rf nockchain.sock
nockchain --fakenet --npc-socket nockchain.sock --bind /ip4/127.0.0.1/udp/3006/quic-v1
