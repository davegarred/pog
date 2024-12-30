FROM ubuntu:noble
WORKDIR /home/ubuntu

COPY --chown=ubuntu:ubuntu build/pog_client ./

ENV PORT 8080

CMD ["/home/ubuntu/pog_client"]
