# People of Greenwood Discord app

For integration tests use docker to stand up a Postgres instance and include the environment variable `INTEGRATION-TEST`.

```shell
docker-compose up -d
export INTEGRATION-TESTING=true
```

To build for deployment in an AWS Lambda:
```
GOOS=linux GOARCH=amd64 CGO_ENABLED=0 go build -tags lambda.norpc -o bootstrap main.go
zip bootstrap.zip bootstrap
```