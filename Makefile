.PHONY: build check ci clean fmt install link lint release run test

BIN_NAME = bundlr
CARGO = $(shell which cargo)

build:
	@$(CARGO) build

check:
	$(CARGO) check --release

clean:
	rm -rf ./target

install:
	@cp ./target/release/$(BIN_NAME) /usr/local/bin/$(BIN_NAME)

link:
	@ln -sf ./target/debug/$(BIN_NAME) .

lint:
	@$(CARGO) fmt --all -- --check
	@echo "Lint OK 👌"

package:
	@$(CARGO) package --allow-dirty

publish:
	@$(CARGO) publish

release:
	@$(CARGO) build --release

run:
	@RUST_BACKTRACE=1 $(CARGO) run

test:
	@$(CARGO) test -- --nocapture && echo "Tests OK 👌" || echo "Tests FAILED 😳"
