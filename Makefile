# This Makefile targets Ubuntu, different builds may require steps other than those in the 'prepare' stage
#
# 'deploy' expects POG_BUCKET to be set to the S3 bucket/folder containing the binaries
# example: export POG_BUCKET=my-s3-bucket/pog/production

.PHONY: clean
clean: 
	rm bootstrap*
	cargo clean

.PHONY: prepare
prepare:
	rustup target add  x86_64-unknown-linux-musl
	sudo apt install musl-tools

test:
	cargo test

check:
	cargo audit
	cargo clippy

build: test
	cargo build --target x86_64-unknown-linux-musl --release --bin pog
	cargo build --target x86_64-unknown-linux-musl --release --bin pog_client
	cargo build --release --bin gateway
	cargo build --release --bin commands

deploy: check-env build
	cp target/x86_64-unknown-linux-musl/release/pog bootstrap
	zip bootstrap.zip bootstrap
	aws s3 cp bootstrap.zip s3://$(POG_BUCKET)/server/bootstrap.zip
	cp target/x86_64-unknown-linux-musl/release/pog_client bootstrap
	zip bootstrap.zip bootstrap
	aws s3 cp bootstrap.zip s3://$(POG_BUCKET)/client/bootstrap.zip
	aws s3 cp target/release/gateway s3://$(POG_BUCKET)/gateway
	echo "deployed code to POG_BUCKET=$(POG_BUCKET)"

check-env:
ifndef POG_BUCKET
	$(error POG_BUCKET is undefined)
endif
