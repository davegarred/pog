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

build_aws:
	cargo build --target x86_64-unknown-linux-musl --release --bin pog
	cargo build --target x86_64-unknown-linux-musl --release --bin pog_client --features aws
	cargo build --release --bin gateway
	cargo build --release --bin commands

build_gcp:
	cargo build --target x86_64-unknown-linux-musl --release --bin pog
	cargo build --release --bin pog_client --features gcp
	cargo build --release --bin gateway
	cargo build --release --bin commands

deploy_aws: check-bucket build_aws test
	cp target/x86_64-unknown-linux-musl/release/pog bootstrap
	zip bootstrap.zip bootstrap
	aws s3 cp bootstrap.zip s3://$(POG_BUCKET)/server/bootstrap.zip
	cp target/x86_64-unknown-linux-musl/release/pog_client bootstrap
	zip bootstrap.zip bootstrap
	aws s3 cp bootstrap.zip s3://$(POG_BUCKET)/client/bootstrap.zip
	aws s3 cp target/release/gateway s3://$(POG_BUCKET)/gateway
	echo "deployed code to POG_BUCKET=$(POG_BUCKET)"

deploy_gcp: check-gcp build_gcp test
	mkdir -p client/build
	mkdir -p gateway/build
	cp target/release/pog_client client/build/
	cp target/release/gateway gateway/build/
	gcloud storage cp gs://$(POG_GCP_BUCKET)/gateway.crt gateway/build/
	cd client;docker build . -t client
	cd gateway;docker build . -t gateway
	docker tag gateway $(POG_REPO)/gateway:latest
	docker push $(POG_REPO)/gateway:latest


check-bucket:
ifndef POG_BUCKET
	$(error POG_BUCKET is undefined)
endif

check-gcp:
ifndef POG_REPO
	$(error POG_REPO is undefined)
endif
ifndef POG_GCP_BUCKET
	$(error POG_GCP_BUCKET is undefined)
endif
