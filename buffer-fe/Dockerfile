FROM node:14-alpine as builder

WORKDIR /home
COPY . .
RUN ["yarn", "install", "--frozen-lockfile"]
RUN ["yarn", "build"]

FROM nginx:alpine

RUN ["rm", "-rf", "/usr/share/nginx/html"]
COPY --from=builder /home/dist /usr/share/nginx/html

ENTRYPOINT [ "nginx", "-g", "daemon off;" ]
