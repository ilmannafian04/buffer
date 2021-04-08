FROM node:14-alpine as builder

WORKDIR /home
COPY . .
RUN ["yarn", "install", "--frozen-lockfile"]
RUN ["yarn", "build"]

FROM nginx:alpine

RUN ["rm", "-rf", "/usr/share/nginx/html"]
RUN sed -i 's/index  index.html index.htm/try_files \$uri \$uri\/ \/index.html/g' /etc/nginx/conf.d/default.conf
COPY --from=builder /home/dist /usr/share/nginx/html

ENTRYPOINT [ "nginx", "-g", "daemon off;" ]
