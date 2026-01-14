#!/bin/bash

trezoa program dump MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr ./src/elf/memo.so -u mainnet-beta
trezoa program dump Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo ./src/elf/memo-v1.so -u mainnet-beta
trezoa slot -u mainnet-beta | xargs -I {} sed -i '' 's|//! Last updated at mainnet-beta slot height: .*|//! Last updated at mainnet-beta slot height: {}|' ./src/lib.rs
