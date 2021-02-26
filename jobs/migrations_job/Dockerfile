# Builder image
FROM rust:latest AS builder
# Set working directory
WORKDIR ${pwd}/jobs/migrations_job
# Install SQLX
RUN cargo install sqlx-cli
# Add job script
COPY ./run.sh ./
# Make job script executable
RUN chmod 744 run.sh

# Start app
# CMD ["sh", "run.sh"]
CMD ["tail", "-f", "/dev/null"]
