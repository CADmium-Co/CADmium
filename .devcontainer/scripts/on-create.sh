#!/bin/bash

cargo install wasm-pack cargo-watch

pnpm dlx playwright install --with-deps chromium
