bin_PROGRAMS = target/release/scpty
target_release_scpty_SOURCES = src/main.rs src/lib.rs Cargo.toml
SUBDIRS = share test_assets
dist_doc_DATA = README.md

$(bin_PROGRAMS):  $(target_release_scpty_SOURCES)
	cargo build --release
clean-local:
	cargo clean
check-local:
	cargo test
