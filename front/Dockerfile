FROM node:12-buster AS builder

WORKDIR /usr/src/app

COPY package.json package-lock.json .

RUN npm ci

COPY . .

RUN npm run build

FROM nginx

COPY --from=builder /usr/src/app/dist /usr/share/nginx/html
