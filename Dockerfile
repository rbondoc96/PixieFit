FROM node:18-alpine3.17 as BASE

# Update and install app/system deps
RUN apk update && apk add --no-cache bash

# Install pnpm
RUN npm i -g pnpm

# Configure pnpm globals
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

COPY . /app
WORKDIR /app

RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

WORKDIR /app/packages/api
RUN pnpm build

EXPOSE 4000
CMD [ "pnpm", "start" ]
