FROM oven/bun as builder

WORKDIR /app

COPY . .

RUN bun install
RUN bun run build

FROM oven/bun

COPY --from=builder /app/build .
COPY .env .

EXPOSE 3000

ENTRYPOINT [ "bun", "." ]
