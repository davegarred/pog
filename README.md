# People of Greenwood Discord app

For integration tests use docker to stand up a Postgres instance and include the environment variable `INTEGRATION-TEST`.

```shell
docker-compose up -d
export INTEGRATION-TESTING=true
```

To build for deployment in an AWS Lambda:
```
cargo build --target x86_64-unknown-linux-musl --release
cp target/x86_64-unknown-linux-musl/release/pog bootstrap
zip bootstrap.zip bootstrap
```

## TODO
- add `payout` endpoint that allows a bet to be paid/cancelled
- add status and time to wagers