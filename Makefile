.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	./did.sh
	dfx generate backend
	dfx deploy
	npm run dev

run:
	npm run dev

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx