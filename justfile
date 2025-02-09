
msrv := "1.75"

_default: all

dev: fmt clippy-allow-dead

all: fmt clippy test doc check-msrv generate-readme

clean:
    rm -rf target target-*
    just -f examples/vhs/justfile clean

motivating-example *extra_args:
    @cargo build --target-dir target --example print-slowly --example live
    @cat examples/motivating-example.json-fragment | target/debug/examples/print-slowly -t 0.1 --delay-at-start 2.5 | target/debug/examples/live --schema travel-modes {{extra_args}}

clippy:
    cargo clippy --all-features
    cargo clippy
    cargo clippy --no-default-features --features serde_json --features rand
    cargo clippy --no-default-features --features serde_yaml --features rand
    cargo clippy --no-default-features --features serde_yaml
    cargo clippy --no-default-features --features serde_json
    cargo clippy --no-default-features
    cargo clippy --no-default-features --features tracing
    cargo clippy --all-targets --all-features

check-msrv:
    rustup install {{msrv}} --profile minimal
    CARGO_TARGET_DIR=target-msrv cargo +{{msrv}} check --all-features
    CARGO_TARGET_DIR=target-msrv cargo +{{msrv}} check
    CARGO_TARGET_DIR=target-msrv cargo +{{msrv}} check --no-default-features --features serde_json --features rand
    CARGO_TARGET_DIR=target-msrv cargo +{{msrv}} check --no-default-features --features serde_yaml --features rand
    CARGO_TARGET_DIR=target-msrv cargo +{{msrv}} check --no-default-features --features serde_yaml
    CARGO_TARGET_DIR=target-msrv cargo +{{msrv}} check --no-default-features --features serde_json
    CARGO_TARGET_DIR=target-msrv cargo +{{msrv}} check --no-default-features
    CARGO_TARGET_DIR=target-msrv cargo +{{msrv}} check --no-default-features --features tracing
    CARGO_TARGET_DIR=target-msrv cargo +{{msrv}} check --all-targets --all-features

clippy-allow-dead:
    cargo clippy --all-targets --all-features -- --allow dead_code
    cargo clippy --all-targets --no-default-features -- --allow dead_code

main-test:
    cargo test --no-fail-fast --all-targets --all-features

test: && doc-test
    cargo test --no-fail-fast --all-targets --all-features
    cargo test --no-fail-fast --all-targets --no-default-features

doc-test:
    cargo test --no-fail-fast --all-features --doc

insta:
    cargo test --no-fail-fast --all-targets --all-features

fmt:
    cargo +nightly fmt

doc:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --lib --bins --examples --all-features --document-private-items --no-deps

doc-public-stable:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --lib --bins --examples -F serde_yaml --no-deps

doc-public:
    RUSTDOCFLAGS="--cfg docsrs" CARGO_TARGET_DIR=target-nightly cargo +nightly doc --lib --bins --examples --all-features --no-deps

doc-open: doc
    xdg-open target-nightly/doc/deser_incomplete/index.html

doc-deps:
    # ratatui somehow cannot be built with `--cfg docsrs`
    CARGO_TARGET_DIR=target-nightly-deps cargo +nightly doc --lib --bins --examples --all-features

doc-deps-open:
    CARGO_TARGET_DIR=target-nightly-deps cargo +nightly doc --lib --bins --examples --all-features --open

generate-readme:
    # Remove markdown code links
    cargo readme \
        | rg --passthru '\[(`[:\w]+`)\]' -r '$1' > README.md.generated
    if diff -q README.md.generated README.md; then rm README.md.generated; fi

replace-readme: generate-readme
    if [ -f README.md.generated ] ; then mv README.md.generated README.md; fi

tokei:
    tokei --exclude json_output --exclude yaml_output

