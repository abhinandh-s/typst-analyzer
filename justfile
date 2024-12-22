ver := `grep '^version =' Cargo.toml | sed 's/version = "\(.*\)"/\1/'`

watch:
  cargo watch -c -x 'build --all'

test:
  cargo test

build:
  cargo build --release

update-rust-analyzer:
  rustup component add rust-analyzer

build-release:
  cargo build --release

install:
  cargo install --path .

release:
  cargo fmt --all -v 
  cargo test
  cargo build --release
  cargo install --path .
  git tag v{{ver}}
  git add -A && git commit -m 'new release' && git push
  git push --tags
  cargo publish
