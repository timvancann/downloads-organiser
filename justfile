release:
  cargo build --release
  mkdir -p ~/.local/bin
  cp target/release/downloads-organiser ~/.local/bin/
