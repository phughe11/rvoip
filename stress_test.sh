#!/bin/bash
# RVOIP Massive SIP Stress Test Script
# Automates: Server -> Multiple Agents -> Concurrent Calls (UAC)

set -e

# Config
BASE_DIR="/Users/user/rvoip_v1.26/rvoip"
BIN_SERVER="$BASE_DIR/target/release/rvoip"
BIN_AGENT="$BASE_DIR/target/release/examples/e2e_test_agent"
BIN_UAC="$BASE_DIR/crates/client-core/examples/client-server/target/release/uac_client"
LOG_DIR="$BASE_DIR/stress_logs"

SERVER_PORT=5060
AGENT_START_PORT=5100
NUM_AGENTS=15
NUM_CONCURRENT_CALLS=50
CALL_DURATION=30
DOMAIN="localhost"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

cleanup() {
    echo -e "\n${YELLOW}Cleaning up active processes...${NC}"
    pkill -f "$BIN_SERVER" || true
    pkill -f "$BIN_AGENT" || true
    pkill -f "$BIN_UAC" || true
    echo -e "${GREEN}Cleanup complete.${NC}"
}

trap cleanup EXIT

mkdir -p "$LOG_DIR"
rm -f "$LOG_DIR"/*.log

echo -e "${GREEN}=== Starting Massive Stress Test ===${NC}"

# 1. Start Server
echo -e "${YELLOW}Step 1: Starting RVOIP Server...${NC}"
RUST_LOG=info "$BIN_SERVER" --port $SERVER_PORT --db stress.db > "$LOG_DIR/server.log" 2>&1 &
SERVER_PID=$!
sleep 3

# Check server health
if ! ps -p $SERVER_PID > /dev/null; then
    echo -e "${RED}Server failed to start. Check $LOG_DIR/server.log${NC}"
    exit 1
fi
echo -e "Server started (PID: $SERVER_PID)"

# 2. Start Agents
echo -e "${YELLOW}Step 2: Starting $NUM_AGENTS Simulated Agents...${NC}"
for i in $(seq 1 $NUM_AGENTS); do
    AGENT_PORT=$((AGENT_START_PORT + i))
    USERNAME="agent_$i"
    RUST_LOG=warn "$BIN_AGENT" \
        --username "$USERNAME" \
        --server "127.0.0.1:$SERVER_PORT" \
        --port $AGENT_PORT \
        --domain "$DOMAIN" \
        --call-duration 60 > "$LOG_DIR/$USERNAME.log" 2>&1 &
    echo -n "."
    sleep 0.5
done
echo -e "\n${GREEN}$NUM_AGENTS agents initialized.${NC}"
sleep 5

# 3. Apply Load
echo -e "${YELLOW}Step 3: Launching $NUM_CONCURRENT_CALLS Concurrent Calls via uac_client...${NC}"
echo -e "Target: 127.0.0.1:$SERVER_PORT, Duration: ${CALL_DURATION}s"

RUST_LOG=info "$BIN_UAC" \
    --server "127.0.0.1:$SERVER_PORT" \
    --port 5090 \
    --num-calls $NUM_CONCURRENT_CALLS \
    --duration $CALL_DURATION > "$LOG_DIR/uac.log" 2>&1

echo -e "${GREEN}=== Stress Generation Complete ===${NC}"

# 4. Results
echo -e "\n${YELLOW}Analyzing Results:${NC}"
TOTAL_ESTABLISHED=$(grep -c "Call .* established" "$LOG_DIR/server.log" || true)
TOTAL_CONNECTED=$(grep -c "Call .* is now connected" "$LOG_DIR/agent_"*".log" | awk -F: '{sum+=$2} END {print sum}')
RTP_TOTAL=$(grep "📈 TOTAL: Received" "$LOG_DIR/uac.log" || echo "No RTP stats")

echo "------------------------------------------"
echo "Server Handled Calls: $TOTAL_ESTABLISHED"
echo "Agents Connected:     $TOTAL_CONNECTED"
echo "RTP Summary:          $RTP_TOTAL"
echo "------------------------------------------"

echo -e "${GREEN}Check $LOG_DIR for detailed logs.${NC}"
