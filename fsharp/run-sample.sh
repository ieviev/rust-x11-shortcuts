#!/usr/bin/env bash

set -euo pipefail
__SOURCE_DIRECTORY__=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cargo build --example applications --release

# compile to native
# fflat sample.fsx --small

dotnet fsi sample.fsx 
