#!/bin/bash
# Build script for AISP WASM kernel
# Produces ultra-condensed <8KB binary

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}Building AISP WASM Kernel${NC}"
echo "================================"

# Check for wasm32 target
if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    echo -e "${YELLOW}Installing wasm32-unknown-unknown target...${NC}"
    rustup target add wasm32-unknown-unknown
fi

# Build release
echo -e "\n${GREEN}Step 1: Cargo build (release)${NC}"
cargo build --release --target wasm32-unknown-unknown

# Get input file
WASM_IN="target/wasm32-unknown-unknown/release/aisp_wasm.wasm"
WASM_OUT="target/aisp.wasm"

if [ ! -f "$WASM_IN" ]; then
    echo -e "${RED}Error: Build failed, WASM file not found${NC}"
    exit 1
fi

# Size before optimization
SIZE_BEFORE=$(stat -f%z "$WASM_IN" 2>/dev/null || stat -c%s "$WASM_IN")
echo -e "Size before optimization: ${YELLOW}${SIZE_BEFORE} bytes${NC}"

# Optimize with wasm-opt if available
if command -v wasm-opt &> /dev/null; then
    echo -e "\n${GREEN}Step 2: wasm-opt optimization${NC}"
    wasm-opt -Oz --enable-bulk-memory -o "$WASM_OUT" "$WASM_IN"
else
    echo -e "\n${YELLOW}wasm-opt not found, skipping optimization${NC}"
    echo "Install with: npm install -g binaryen"
    cp "$WASM_IN" "$WASM_OUT"
fi

# Strip with wasm-strip if available
if command -v wasm-strip &> /dev/null; then
    echo -e "\n${GREEN}Step 3: wasm-strip${NC}"
    wasm-strip "$WASM_OUT"
elif command -v wasm-opt &> /dev/null; then
    # wasm-opt can also strip
    wasm-opt --strip-debug --enable-bulk-memory -o "$WASM_OUT" "$WASM_OUT"
fi

# Final size
SIZE_AFTER=$(stat -f%z "$WASM_OUT" 2>/dev/null || stat -c%s "$WASM_OUT")
echo -e "\n================================"
echo -e "Final size: ${GREEN}${SIZE_AFTER} bytes${NC}"

# Check against 8KB limit
if [ "$SIZE_AFTER" -le 8192 ]; then
    echo -e "${GREEN}✓ Under 8KB limit!${NC}"
else
    echo -e "${RED}✗ Exceeds 8KB limit (${SIZE_AFTER} > 8192)${NC}"
    OVER=$((SIZE_AFTER - 8192))
    echo -e "${YELLOW}Over by ${OVER} bytes${NC}"
fi

# Show exports
echo -e "\n${GREEN}WASM Exports:${NC}"
if command -v wasm-objdump &> /dev/null; then
    wasm-objdump -x "$WASM_OUT" | grep -A100 "Export\[" | head -20
elif command -v wasm2wat &> /dev/null; then
    wasm2wat "$WASM_OUT" | grep "(export" | head -10
fi

echo -e "\n${GREEN}Build complete: ${WASM_OUT}${NC}"
