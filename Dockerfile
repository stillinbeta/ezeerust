FROM nginx
COPY target/deploy /usr/share/nginx/html
COPY ezeerust.conf /etc/nginx/conf.d/default.conf
