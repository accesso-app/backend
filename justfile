export RUST_BACKTRACE := "1"

# Build and run crate api-&
run API='internal':
  cargo run --package accesso-api-{{API}}

# Show env variables
env:
  #!/usr/bin/env node
  console.log(process.env)

admin:
    cargo run --package accesso-api-admin | pino-pretty -t -f

public: (run "public")
internal: (run "internal")

integration:
    cargo test --package tests