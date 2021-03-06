FROM debian:buster-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
    --no-install-recommends \
    openssl \
    dumb-init \
    && rm -rf /var/lib/apt/lists/*
COPY . .

EXPOSE 8080
ENTRYPOINT ["/usr/bin/dumb-init", "--"]
CMD ["/app/start.sh"]
