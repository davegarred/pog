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

test_aws:
	cargo test --features integration-tests --features aws

test_gcp:
	cargo test --features integration-tests --features gcp

check:
	cargo audit
	cargo clippy

build_aws:
	cargo build --release --features aws --bin pog --target x86_64-unknown-linux-musl
	cargo build --release --features aws --bin pog_client --target x86_64-unknown-linux-musl
	cargo build --release --features aws --bin gateway
	cargo build --release --bin commands

build_gcp:
	cargo build --release --features gcp --bin pog
	cargo build --release --features gcp --bin pog_client
	cargo build --release --features gcp --bin gateway
	cargo build --release --bin commands
	mkdir -p server/build
	mkdir -p client/build
	mkdir -p gateway/build
	cp target/release/pog server/build/
	cp target/release/pog_client client/build/
	cp target/release/gateway gateway/build/
	gcloud storage cp gs://$(POG_GCP_BUCKET)/gateway.crt gateway/build/
	cd server;docker build . -t pog_server
	cd client;docker build . -t pog_client
	cd gateway;docker build . -t pog_gateway

deploy_aws: check-bucket test_aws build_aws
	cp target/x86_64-unknown-linux-musl/release/pog bootstrap
	zip bootstrap.zip bootstrap
	aws s3 cp bootstrap.zip s3://$(POG_BUCKET)/server/bootstrap.zip
	cp target/x86_64-unknown-linux-musl/release/pog_client bootstrap
	zip bootstrap.zip bootstrap
	aws s3 cp bootstrap.zip s3://$(POG_BUCKET)/client/bootstrap.zip
	aws s3 cp target/release/gateway s3://$(POG_BUCKET)/gateway
	echo "deployed code to POG_BUCKET=$(POG_BUCKET)"

deploy_gcp: check-gcp test_gcp build_gcp
	docker tag pog_server $(POG_REPO)/pog_server:latest
	docker tag pog_client $(POG_REPO)/pog_client:latest
	docker tag pog_gateway $(POG_REPO)/pog_gateway:latest
	docker push $(POG_REPO)/pog_server:latest
	docker push $(POG_REPO)/pog_client:latest
	docker push $(POG_REPO)/pog_gateway:latest


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
