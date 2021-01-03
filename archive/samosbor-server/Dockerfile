FROM ubuntu:latest
ENV TZ=Europe/Moscow
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
RUN apt-get update -qq
RUN apt-get install -qqy libssl-dev pkg-config
COPY ./target/release/samosbor-server /usr/bin/samosbor-server
CMD /usr/bin/samosbor-server -h 0.0.0.0 -p 80
