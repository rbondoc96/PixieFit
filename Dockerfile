FROM node:lts-alpine

RUN apk update && apk add --no-cache git

ARG WORKSPACE

WORKDIR /root/code
COPY ./package*.json .
COPY ./packages/${WORKSPACE}/package.json ./packages/${WORKSPACE}/package.json

RUN npm ci

COPY ./packages/${WORKSPACE} ./packages/${WORKSPACE} 
