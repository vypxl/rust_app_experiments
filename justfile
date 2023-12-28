start-db:
  docker run --name rust_ui_surrealdb --rm --pull always -p 8000:8000 -v ./data:/data surrealdb/surrealdb:latest start file:/data/database.db

start-db-background:
  docker run --name rust_ui_surrealdb -d --rm --pull always -p 8000:8000 -v ./data:/data surrealdb/surrealdb:latest start file:/data/database.db

setup-db:
  echo "define namespace test; use namespace test; define database test;" | docker run --rm --network host surrealdb/surrealdb:latest sql

htmx:
  cd htmx && cargo run

dioxus:
  cd dioxus && cargo run
