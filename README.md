# Shovel
Simple wiki software.

## Usage
Copy `env.example` to `.env` and modify it to suit your environment.

Install `diesel_cli` by doing `cargo install diesel_cli --no-default-features --features "postgres"` to avoid having to install dependencies for DBMS that aren't Postgres, then do `diesel migration run`.

Do `cargo run`.
