#!/bin/sh
set -e

# Generate random 5-character seed
LOGS_SEED=$(openssl rand -hex 2 | cut -c1-5)
echo "Generated seed: $LOGS_SEED"

# Generate random metrics seed (1-1,000,000)
METRICS_SEED=$(awk 'BEGIN{srand(); print int(rand()*1000000)+1}')
echo "Generated metrics seed: $METRICS_SEED"

echo "Building LogDash demo Docker image (using published package ${LOGDASH_VERSION:-newest})..."
docker build --no-cache --build-arg LOGDASH_VERSION -t logdash-rust-demo check-deployed-package

echo
echo "Running LogDash demo..."
echo

# Run in non-interactive mode which works everywhere
docker run --rm \
  -e LOGDASH_API_KEY="${LOGDASH_API_KEY}" \
  -e LOGS_SEED="${LOGS_SEED}" \
  -e METRICS_SEED="${METRICS_SEED}" \
  logdash-rust-demo

echo
echo "Demo completed!" 

echo
echo "Authenticating with LogDash API..."

# The ingest key (LOGDASH_API_KEY) of the shared "SDK smoke tests" domain only sends. A read-only personal API key reads back:
# logs:read and metrics:read, restricted to that domain, which whoami lists as access: { kind: "clusters", ids: [domainId] }.
TOKEN="${LOGDASH_PERSONAL_API_KEY}"
AUTH_RESPONSE=$(curl -s -X 'GET' \
  'https://api.logdash.io/personal-api-keys/whoami' \
  -H 'accept: application/json' \
  -H "Authorization: Bearer ${TOKEN}")

DOMAIN_ID=$(echo "$AUTH_RESPONSE" | jq -r 'if .access.kind == "clusters" then .access.ids[0] // empty else empty end' 2>/dev/null || true)

if [ -z "$TOKEN" ] || [ -z "$DOMAIN_ID" ]; then
    echo "Error: Failed to authenticate with LogDash API: LOGDASH_PERSONAL_API_KEY must be a personal API key restricted to the smoke-test domain"
    echo "Access: $(echo "$AUTH_RESPONSE" | jq -c '.access // .message' 2>/dev/null || echo 'unreadable response')"
    exit 1
fi

echo "Authentication successful. Domain ID: $DOMAIN_ID"

echo
echo "Fetching logs from LogDash API..."

# Fetch this run's logs of the rust-sdk app: the seed keeps out logs that other runs send at the same time
LOGS_RESPONSE=$(curl -s -X 'GET' \
  "https://api.logdash.io/clusters/${DOMAIN_ID}/logs?apps=rust-sdk&limit=100&searchString=${LOGS_SEED}" \
  -H 'accept: application/json' \
  -H "Authorization: Bearer ${TOKEN}")

echo "Logs fetched successfully"

echo
echo "Validating log messages..."

# Expected log messages with seed
EXPECTED_MESSAGES="This is an info log ${LOGS_SEED}
This is an error log ${LOGS_SEED}
This is a warning log ${LOGS_SEED}
This is a debug log ${LOGS_SEED}
This is a http log ${LOGS_SEED}
This is a silly log ${LOGS_SEED}
This is an info log ${LOGS_SEED}
This is a verbose log ${LOGS_SEED}"

# Check if all expected messages are present in the logs
echo "$EXPECTED_MESSAGES" | while IFS= read -r expected_msg; do
    if ! echo "$LOGS_RESPONSE" | grep -q "$expected_msg"; then
        echo "Error: Expected log message not found: '$expected_msg'"
        echo "Logs response: $LOGS_RESPONSE"
        exit 1
    fi
    echo "✓ Found: '$expected_msg'"
done

echo
echo "Fetching metrics from LogDash API..."

# Fetch metrics from the API
METRICS_RESPONSE=$(curl -s -X 'GET' \
  "https://api.logdash.io/clusters/${DOMAIN_ID}/metrics" \
  -H 'accept: application/json' \
  -H "Authorization: Bearer ${TOKEN}")

echo "Metrics fetched successfully"

echo
echo "Validating metrics..."

# Expected users metric value (metrics_seed + 1)
EXPECTED_USERS_VALUE=$((METRICS_SEED + 1))

# Every SDK sends a users metric to the domain: match the one of the rust-sdk app
USERS_VALUE=$(echo "$METRICS_RESPONSE" | jq -r '.[] | select(.app == "rust-sdk" and .name == "users") | .value' 2>/dev/null || true)

if [ -z "$USERS_VALUE" ]; then
    echo "Error: Users metric of the rust-sdk app not found"
    echo "Metrics response: $METRICS_RESPONSE"
    exit 1
fi

if [ "$USERS_VALUE" != "$EXPECTED_USERS_VALUE" ]; then
    echo "Error: Users metric value mismatch. Expected: $EXPECTED_USERS_VALUE, Found: $USERS_VALUE"
    echo "Metrics response: $METRICS_RESPONSE"
    exit 1
fi

echo "✓ Found users metric with correct value: $USERS_VALUE"

echo
echo "All expected log messages and metrics found successfully!"
echo "Validation completed!"