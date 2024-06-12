#!/bin/sh
set -e
cargo build
cp target/debug/candid-gen ../../playground.g/icp_developer_journey/counter/
