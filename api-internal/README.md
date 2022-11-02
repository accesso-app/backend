# api-internal

Is build for Docker image and AWS Lambda Function.

Uses:

- https://www.cargo-lambda.info/
- https://github.com/hanabu/lambda-web
- https://actix.rs/

## Manually deploy

```bash
cargo lambda deploy --iam-role arn:aws:iam::000011112222:role/accesso-dev-lambda-role --binary-name accesso-api-internal accesso-dev-api-internal
```
