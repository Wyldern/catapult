# Catapult

A simple redirect-based search engine for local browsers.

## Running in Docker

This uses a config located at `~/.catapult.toml`:

```sh
docker build -t catapult:latest .
docker run -d --name catapult --restart unless-stopped -v "$HOME/.catapult.toml:/catapult/Rocket.toml:ro" -p 127.0.0.1:9000:8000/tcp catapult:latest
```
