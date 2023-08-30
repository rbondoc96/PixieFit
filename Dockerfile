FROM node:18-alpine3.17 as base

# Update and install app/system deps
RUN apk update && apk add --no-cache bash

# Install pnpm
RUN npm i -g pnpm tsx

# Configure pnpm globals
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

WORKDIR /app
COPY pnpm-lock.yaml .
COPY pnpm-workspace.yaml .

FROM base AS api
WORKDIR /app/packages/api
COPY ./packages/api .

RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm migrate
RUN pnpm build

EXPOSE 8080
CMD [ "pnpm", "start" ]
