# Stop Old Container
echo "Stopping Previous Container:"
docker ps --filter "label=blobby_server" | grep -v CONTAINER | awk '{ print $1 }' | xargs docker stop
echo ""

# Read secret env vars from disk
GPT_API_KEY=$(more ~/.gpt_api_key)

# Set env vars
cat <<EOF > .env_file
RUST_LOG=debug
PORT0=3000
GPT_API_KEY=$GPT_API_KEY
EOF

echo "Passing In Env Vars:"
cat .env_file
echo ""

# Start New Container
echo "Starting New Container:"
docker run \
    -d \
    --network host \
    --restart unless-stopped \
    --env-file .env_file \
    blobby-server

# Check Server Logs
sleep 3
docker ps | grep blobby-server
docker ps | grep blobby-server | awk '{ print $1 }' | xargs docker logs