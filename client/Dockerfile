FROM ubuntu:noble
WORKDIR /home/ubuntu

COPY --chown=ubuntu:ubuntu build/pog_client ./

#RUN apt-get update && apt-get install -y ca-certificates
#COPY build/gateway.crt /usr/local/share/ca-certificates/
#RUN update-ca-certificates

ENV PORT 8080

CMD ["/home/ubuntu/pog_client"]
