
# expects POG_BUCKET to be set to the S3 bucket/folder containing the binaries 
# example: export POG_BUCKET=my-s3-bucket/pog/production

.PHONY: clean
clean: 
	rm bootstrap*
	cargo clean

build:
	cargo build --target x86_64-unknown-linux-musl --release 

deploy: check-env build
	cp target/x86_64-unknown-linux-musl/release/pog bootstrap
	zip bootstrap.zip bootstrap
	aws s3 cp bootstrap.zip s3://$(POG_BUCKET)/server/bootstrap.zip
	cp target/x86_64-unknown-linux-musl/release/pog_client bootstrap
	zip bootstrap.zip bootstrap
	aws s3 cp bootstrap.zip s3://$(POG_BUCKET)/client/bootstrap.zip

check-env:
ifndef POG_BUCKET
	$(error POG_BUCKET is undefined)
endif
