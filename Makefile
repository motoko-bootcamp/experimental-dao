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

.PHONY: create_test_identities
create_test_identities:
	dfx identity new --storage-mode plaintext --force alice
	dfx identity new --storage-mode plaintext --force bob
	dfx identity new --storage-mode plaintext --force charlie
	@echo "created alice, bob, and charlie"

.PHONY: test_create_member
test_create_member:
	@echo "adding alice to the dao"
	dfx identity use alice
	dfx canister call backend create_member \
		'( "alice" )'
	dfx canister call backend get_all_members
	@echo ""
	@echo "adding bob to the dao"
	dfx identity use bob
	dfx canister call backend create_member \
		'( "bob" )'
	dfx canister call backend get_all_members
	@echo ""
	@echo "adding alice to the dao (again)"
	dfx identity use alice
	dfx canister call backend create_member \
		'( "alice" )'
	dfx canister call backend get_all_members
	#cleanup - changing identity to default
	dfx identity use default

