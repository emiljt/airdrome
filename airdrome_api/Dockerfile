# Builder image
FROM rust:latest AS builder
# Set working directory
WORKDIR ${pwd}/airdrome_api
# Add dependencies files
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
# Copy DB schema
COPY ./sqlx-data.json ./sqlx-data.json
COPY ./src ./src
COPY ./migrations ./migrations
# Install DB migration manager
RUN cargo install sqlx-cli
# Install app
RUN cargo install --debug --path .

# App image
FROM fedora:latest
# Expose port
EXPOSE 8000
# Run updates
RUN dnf update -y \
#   && dnf install -y sqlite ca-certificates \
   && dnf install -y sqlite \
  && dnf clean all
# Copy files from builder
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY --from=builder /usr/local/cargo/bin/airdrome_api /usr/local/bin/airdrome_api
COPY --from=builder /airdrome_api/migrations /opt/airdrome_api/migrations
# Create migrations script
RUN echo "#!/bin/bash\n\
sqlx database create\n\
sqlx migrate run" >> /opt/airdrome_api/release.sh
RUN chmod 744 /opt/airdrome_api/release.sh
# Start app
CMD ["airdrome_api"]
