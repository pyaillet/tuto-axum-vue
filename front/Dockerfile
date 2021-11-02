FROM node:12-buster AS builder

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

COPY package.json package-lock.json .

RUN npm install

COPY . .

RUN npm run build

FROM nginx

COPY --from=builder /usr/src/app/dist /usr/share/nginx/html
