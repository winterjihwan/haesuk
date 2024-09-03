srun:
	RUSTFLAGS="-A dead_code" cargo run

emulate:
	cargo run -q -- emulate $(FILE)

hasm:
	cargo run -q -- hasm $(FILE)

dehasm:
	cargo run -q -- dehasm $(FILE)
