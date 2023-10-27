# Build wasm binary
FROM rust:1-buster as wasm-builder

WORKDIR /tmp    
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.* .
COPY src src

RUN cargo install wasm-pack

RUN wasm-pack build --out-name wasm --out-dir ./static

# build nodejs http server
FROM node:20-buster as node-builder

ENV NODE_OPTIONS=--openssl-legacy-provider

WORKDIR /tmp
COPY www www
COPY --from=wasm-builder /tmp/static www/pkg
RUN cd www && npm install && npm run build

# host on nginx
FROM nginx:1.25-alpine

COPY --from=node-builder /tmp/www/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]