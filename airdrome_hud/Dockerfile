# Builder image
FROM node:latest AS builder
# Install dependencies
# RUN dnf install -y nodejs
# Set working directory
WORKDIR ${pwd}/airdrome_hud
# Add dependencies file
COPY ./package*.json .
# Install build dependencies
RUN npm install
# Add app source files
ADD . ./
# Build app
RUN npm run build
# Clean up build dependencies
RUN rm -rf node_modules
# Install production dependencies
RUN npm ci --prod

# App image
FROM node:latest
# Expose server port
EXPOSE 3000
# User to run app
ENV APP_USER=sapper
# Set directory of app
ARG APP=/srv/airdrome_hud
# Update
# RUN dnf update -y \
# && dnf install -y nodejs \
# && dnf clean all
# Create user and group to run app
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p $APP
# Copy dependencies
COPY --from=builder /airdrome_hud/node_modules $APP/node_modules
# Copy app from builder
COPY --from=builder /airdrome_hud/__sapper__/build $APP/__sapper__/build
# Copy static files from builder
COPY --from=builder /airdrome_hud/static $APP/static
# Change ownership of app binary
RUN chown -R $APP_USER:$APP_USER $APP
# Set current user
USER $APP_USER
# Set working directory
WORKDIR $APP

# Start app
CMD ["node", "__sapper__/build"]
