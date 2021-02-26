#!/bin/sh

# Clone migration files
git clone https://github.com/emiljt/airdrome.git
# Change to API project
cd ./airdrome/airdrome_api
# Run migrations
sqlx migrate run
