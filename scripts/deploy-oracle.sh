#!/usr/bin/env bash
#
# deploy-oracle.sh — cross-compile hermes-construct for Oracle ARM (Ampere A1),
# strip it, and bundle a deployment tarball. Run this on your x86 dev box.
#
# Produces: dist/hermes-construct-aarch64-unknown-linux-gnu.tar.gz
# containing the stripped aarch64 binary, room/ensign configs, the systemd
# unit, the installer, and an env template.
#
set -euo pipefail

TARGET="aarch64-unknown-linux-gnu"
BIN_NAME="hermes-construct"
STRIP="aarch64-linux-gnu-strip"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

DIST_DIR="dist"
STAGE="$DIST_DIR/$BIN_NAME"
TARBALL="$DIST_DIR/${BIN_NAME}-${TARGET}.tar.gz"

echo "==> Ensuring Rust target '$TARGET' is installed"
rustup target add "$TARGET" >/dev/null 2>&1 || true

echo "==> Cross-compiling release binary for $TARGET"
cargo build --release --target "$TARGET"

BIN_PATH="target/$TARGET/release/$BIN_NAME"
if [ ! -f "$BIN_PATH" ]; then
  echo "ERROR: expected binary not found at $BIN_PATH" >&2
  exit 1
fi

echo "==> Staging deployment files"
rm -rf "$STAGE"
mkdir -p "$STAGE/bin" "$STAGE/rooms" "$STAGE/ensigns"

cp "$BIN_PATH" "$STAGE/bin/$BIN_NAME"

# Strip the *staged copy* so the original build artifact stays intact.
size_before="$(du -h "$STAGE/bin/$BIN_NAME" | cut -f1)"
if command -v "$STRIP" >/dev/null 2>&1; then
  "$STRIP" "$STAGE/bin/$BIN_NAME"
  size_after="$(du -h "$STAGE/bin/$BIN_NAME" | cut -f1)"
  echo "    stripped: $size_before -> $size_after"
else
  echo "    WARNING: $STRIP not found; shipping unstripped binary ($size_before)" >&2
fi

# Config + deployment assets.
cp rooms/*.json    "$STAGE/rooms/"   2>/dev/null || echo "    (no room configs found)"
cp ensigns/*.json  "$STAGE/ensigns/" 2>/dev/null || echo "    (no ensign configs found)"
cp "$SCRIPT_DIR/install-oracle.sh"        "$STAGE/install-oracle.sh"
cp "$SCRIPT_DIR/hermes-construct.service" "$STAGE/hermes-construct.service"
chmod +x "$STAGE/install-oracle.sh"

# Env template (secrets filled in on the Oracle box, never committed).
cat > "$STAGE/hermes.env.example" <<'ENV'
# hermes-construct environment — copy to /etc/hermes-construct/hermes.env (0600)
# Telegram bot token from @BotFather (required to connect to Telegram).
TELEGRAM_BOT_TOKEN=
# Provider API keys (at least one needed for the agent to respond).
DEEPINFRA_API_KEY=
ZAI_API_KEY=
# Logging verbosity: error | warn | info | debug | trace
RUST_LOG=info
ENV

echo "==> Verifying staged binary"
file "$STAGE/bin/$BIN_NAME" || true

echo "==> Building tarball"
mkdir -p "$DIST_DIR"
tar -czf "$TARBALL" -C "$DIST_DIR" "$BIN_NAME"
echo "    created $TARBALL ($(du -h "$TARBALL" | cut -f1))"

cat <<EOF

================================================================
 Deployment bundle ready: $TARBALL
================================================================

Copy it to the Oracle ARM box and install (replace HOST/USER):

  scp $TARBALL  USER@ORACLE_HOST:/tmp/
  ssh USER@ORACLE_HOST
  cd /tmp && tar -xzf $(basename "$TARBALL")
  sudo ./hermes-construct/install-oracle.sh

Then add your tokens and start the service:

  sudo nano /etc/hermes-construct/hermes.env
  sudo systemctl start hermes-construct
  journalctl -u hermes-construct -f
================================================================
EOF
