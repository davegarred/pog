FROM ubuntu:noble
WORKDIR /home/ubuntu

COPY --chown=ubuntu:ubuntu continuous.sh ./
COPY --chown=ubuntu:ubuntu build/gateway ./

RUN apt-get update && apt-get install -y ca-certificates
COPY build/gateway.crt /usr/local/share/ca-certificates/
RUN update-ca-certificates

CMD ["/home/ubuntu/continuous.sh"]
