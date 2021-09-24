FROM nginx:1.21.3-alpine

RUN apk upgrade --update-cache --available && apk add openssl && rm -rf /var/cache/apk/*

RUN openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 -nodes -keyout /etc/nginx/self.key -out /etc/nginx/self.crt -subj "/CN=*"

COPY .codebuild/nginx.conf /etc/nginx/nginx.conf

STOPSIGNAL SIGTERM

CMD ["nginx", "-g", "daemon off;"]

