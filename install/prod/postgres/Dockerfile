FROM postgres:16-alpine

EXPOSE 5432

# Build variables
ARG ENVIRONMENT="prod"
ARG FILES_DOMAIN="wjfiles.test"

# Create system user
RUN adduser -S wikijump

# Install dependencies
RUN apk add --no-cache \
    musl-locales \
    postgresql-contrib \
    sudo

# Install files
COPY ./install/prod/postgres/init /docker-entrypoint-initdb.d
COPY ./install/prod/postgres/health-check.sh /bin/wikijump-health-check
