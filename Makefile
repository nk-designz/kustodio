run:
	touch src/build.rs
	rm ui/dist/*.wasm || true
	RUST_LOG=debug,gossip=off cargo run server ./example/kustodio-peer-0.toml
