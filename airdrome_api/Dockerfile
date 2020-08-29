# Builder image
FROM fedora:latest AS builder
# Install dependencies
RUN dnf install -y rust cargo
# Create new rust binary
RUN USER=root cargo new --bin airdrome_api
# Set working directory
WORKDIR ${pwd}/airdrome_api
# Add dependencies
COPY ./Cargo.toml ./Cargo.toml
# Build dependencies
RUN cargo build --release
# Clean up
RUN rm src/*rs
# Add app source files
ADD . ./
# Clean up build from dependencies
RUN rm ./target/release/deps/airdrome_api*
# Build app binary
RUN cargo build --release

# App image
FROM fedora:latest
# Expose server port
EXPOSE 8080
# User to run app
ENV APP_USER=rust
# Set directory of appk
ARG APP=/srv/airdrome_api
# Update
RUN dnf update -y \
    && dnf clean all
# Create user and group to run app
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p $APP
# Copy binary from builder
COPY --from=builder /airdrome_api/target/release/airdrome_api $APP/airdrome_api
# Change ownership of app binary
RUN chown -R $APP_USER:$APP_USER $APP
# Set current user
USER $APP_USER
# Set working directory
WORKDIR $APP

# Start app
CMD ["./airdrome_api"]
