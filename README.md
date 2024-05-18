Step 1:
    cargo build -- --name test_project --description "a vara project template"

Step 2:
    cd target/wasm32-unknown-unknown/release/
    and the contract is present in test_project.opt.wasm

For Build
    cargo build --release

    target/release/vara --name test_project 
