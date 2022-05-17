echo " [!]        |CLEANING"          &&
rm Cargo.lock                         &&
cargo clean                           &&
echo " [!]        |FORMATTING"        &&
cargo fmt                             &&
echo " [!]        |CHECKING"          &&
cargo check --features require_docs   &&
echo " [!]        |CLIPPY'ING"        &&
cargo clippy --features require_docs  &&
echo " [!]        |TESTING"           &&
cargo test --features require_docs    &&
echo " [!]        |DOCUMENTING"       &&
cargo doc                             &&
echo " [!]        |BUILDING"          &&
cargo build --release                 &&
echo " [!]        |FINISHED"

# && is needed to chain commands conditionally.