# Development
## Setup podman and docker-compose in user mode
systemctl --user enable podman.socket
systemctl --user start podman.socket
systemctl --user status podman.socket
UID=(id -u $USERNAME) set -gx DOCKER_HOST "///run/user/$UID/podman/podman.sock"

## Start local environment
podman-compose up

## Stop local environment
podman-compose down

## Compile API
- unset DATABASE_URL

## Update offline SQL checks
cargo sqlx prepare --check

# Deploy
- Log into server with ssh
- `cd airdrome`
- Bring site down: `podman stop infrastructure_api && podman stop infrastructure_web`
- Pull down updates: `git pull`
- Build new images: `podman-compose build`
- Start site: `podman-compose up`

