#!/bin/bash

set -euo pipefail

SCRIPT_DIR="$(cd "${0%/*}" && pwd)"

cp "$SCRIPT_DIR/../conf/amaru.service" /etc/systemd/system/amaru.service
sudo systemctl enable amaru.service
sudo systemctl start amaru.service
