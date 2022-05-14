cargo build --all --features hot_reload \
  && RUST_BACKTRACE=1 cargo watch -i "*/crates/logic/**" -x "run --features hot_reload"
