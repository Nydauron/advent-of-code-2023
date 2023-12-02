create day:
    cargo generate --path ./daily-template --name {{day}}
watch day part:
    cargo watch -w ./{{day}} -- just test {{day}} {{part}}
run day part:
    cargo run -p {{day}} --bin {{part}}
test day part:
    cargo test -p {{day}} --bin {{part}}
