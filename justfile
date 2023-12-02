create day:
    cargo generate --path ./daily-template --name {{day}}
watch day part:
    cargo watch -w ./{{day}} -- cargo test -p {{day}} --bin {{part}}
