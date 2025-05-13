dev-watch:
    cargo watch -x "run"

dev:
    cargo run

test:
    cargo test

add-migration:
    sqlx migrate add -r initial_setup

revert-migration:
    sqlx migrate revert

run-migration:
    sqlx migrate run

docker-build:
    docker build -t axum_template .

docker-run:
    docker run --rm --name axum_template_container -p 3000:3000 --env-file .env axum_template

docker-stop:
    docker stop axum_template_container

compose-up:
    docker-compose up -d

compose-down:
    docker-compose down
