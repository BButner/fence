#!/bin/bash
cd "$(dirname "$0")"

cargo build

# Copy the .dylib to the macOS Project
cp target/debug/libfence_backend.dylib ../sys/macos/lib/libfence_backend.dylib
cp fence-backend-lib.h ../sys/macos/Frameworks/fence-backend-lib.h