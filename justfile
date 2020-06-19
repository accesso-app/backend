export RUST_BACKTRACE := "1"

# Build and run crate $-api
run API='public':
  cargo run --package accesso-{{API}}-api

# Show env variables
env:
  #!/usr/bin/env node
  console.log(process.env)

# Build and run crate admin-api
admin: (run "admin")
