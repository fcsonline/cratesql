#!/bin/bash

DATABASE_USER='postgres'
DATABASE_NAME='cratesql'
DATABASE_URL='cratesql'
TARGET='/tmp/'

echo "Downloading last crates.io dump..."
wget -q https://static.crates.io/db-dump.tar.gz

echo "Reading main dump folder..."
FOLDER=`tar --exclude="*/*" -ztf db-dump.tar.gz`

echo "Decompresing dump '${FOLDER}' folder..."
tar -zxf db-dump.tar.gz --directory ${TARGET}

echo "Changing folder permissions..."
sudo find "${TARGET}${FOLDER}" -type d -exec chmod 755 {} +
sudo find "${TARGET}${FOLDER}" -type f -exec chmod 644 {} +

echo "Initializing Postgresql database..."
sudo -u postgres createdb ${DATABASE_NAME}

# This is not required but convinient for development
echo "Truncating some tables..."
sudo head -n 20000 "${TARGET}${FOLDER}/data/version_downloads.csv" > "${TARGET}${FOLDER}/data/version_downloads_truncated.csv"
sudo mv "${TARGET}${FOLDER}/data/version_downloads_truncated.csv" > "${TARGET}${FOLDER}/data/version_downloads.csv"

echo "Importing schema..."
cd "${TARGET}${FOLDER}"
sudo -u ${DATABASE_USER} psql ${DATABASE_NAME} < "./schema.sql"

echo "Importing data..."
sudo -u ${DATABASE_USER} psql -d ${DATABASE_NAME} < "./import.sql"
