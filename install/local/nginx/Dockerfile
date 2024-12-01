#
# nginx reverse proxy
#

FROM nginx:alpine

RUN apk add --no-cache curl openssl

# Add snake oil cert for local
RUN openssl req \
    -x509 \
    -verbose \
    -utf8 \
    -noenc \
    -sha256 \
    -days 365 \
    -newkey rsa:4096 \
    -out /etc/ssl/certs/nginx-selfsigned.crt \
    -keyout /etc/ssl/private/nginx-selfsigned.key \
    -subj "/CN=localhost" \
    -addext "subjectAltName=DNS:localhost,DNS:*.localhost"

# Copy configuration files
COPY ./install/local/nginx/nginx.conf /etc/nginx/nginx.conf
COPY ./install/local/nginx/proxy.conf /etc/nginx/proxy.conf
COPY ./install/local/nginx/gzip.conf /etc/nginx/gzip.conf
COPY ./install/local/nginx/ffdhe2048.txt /etc/nginx/ffdhe2048.txt
COPY ./install/local/nginx/ssl.conf /etc/nginx/ssl.conf
COPY ./install/local/nginx/web.conf /etc/nginx/web.conf

# Verify configuration
RUN nginx -t

# Delete docker-entrypoint, this is a custom image
RUN rm -rf /docker-entrypoint.d/

EXPOSE 80
EXPOSE 443
