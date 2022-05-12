dev:
	touch src/build.rs
	rm ui/dist/*.wasm || true
	RUSTFLAGS="--cfg tokio_unstable" RUST_LOG=debug,gossip=off cargo run server ./example/kustodio-peer-0.toml

cluster:
	RUST_LOG=debug,gossip=off cargo run server ./example/kustodio-peer-0.toml &
	RUST_LOG=debug,gossip=off cargo run server ./example/kustodio-peer-1.toml &
	RUST_LOG=debug,gossip=off cargo run server ./example/kustodio-peer-2.toml &

kill:
	pkill -9 kustodio
