#!/bin/bash

NETWORK=${NETWORK:-preprod}
PEER_ADDRESS=${PEER_ADDRESS:-127.0.0.1:3001}
LISTEN_ADDRESS=${LISTEN_ADDRESS:-0.0.0.0:0}
LEDGER_DIR=${LEDGER_DIR:-./ledger.${NETWORK}.db}
CHAIN_DIR=${CHAIN_DIR:-./chain.${NETWORK}.db}

AMARU_SYNCING="true"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

source ~/.virtualenvs/pimoroni/bin/activate

AMARU_TRACE="amaru=trace" amaru --with-json-traces daemon \
           --peer-address="${PEER_ADDRESS}" \
           --listen-address="${LISTEN_ADDRESS}" \
           --network="${NETWORK}" \
           --chain-dir="${CHAIN_DIR}" \
           --ledger-dir="${LEDGER_DIR}" | while read line; do
  EVENT=$(jq -r '.fields.message' <<< "$line" 2>/dev/null)
  SPAN=$(jq -r '.span.name' <<< "$line" 2>/dev/null)
  if [ "$EVENT" = "new.known_snapshots" ]; then
    # Epochs restored, used as initial Epoch
    EPOCH=$(jq -r '.fields.snapshots | split("..")[1][:-1]' <<< "$line" 2>/dev/null)
  fi
  if [ "$EVENT" = "exit" ] && [ "$SPAN" = "epoch_transition" ]; then
    # Epoch transition
    EPOCH=$(jq -r '.span.into' <<< "$line" 2>/dev/null)
    if [[ "$AMARU_SYNCING" == "true" ]]; then
        "$SCRIPT_DIR/inky/display_syncing.py" "$EPOCH"
    fi
  fi
  if [ "$AMARU_SYNCING" = "false" ] && [ "$EVENT" = "tip_changed" ]; then
    # New block
    BLOCK=$(jq -r '.fields.tip | split(".")[0]' <<< "$line" 2>/dev/null)
    "$SCRIPT_DIR/inky/display_badge.py" "$EPOCH" "$BLOCK"
  fi
  if [ "$AMARU_SYNCING" = "true" ] && [ "$EVENT" = "chain.extended" ]; then
    # Synced
    AMARU_SYNCING="false"
  fi
done