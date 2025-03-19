# Acme Rust Lambda Deployment Guide

This guide walks through building and deploying Rust Lambda functions using [Cargo Lambda](https://www.cargo-lambda.info/).

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable or nightly)
- [Cargo Lambda](https://www.cargo-lambda.info/guide/installation.html)
- Node.js and NPX (for NX build commands)
- AWS CLI configured with appropriate permissions

## Project Structure

Our project uses NX for build orchestration, with Rust Lambda functions located in the `rust_lambda` workspace.

## Building Lambda Functions

To build the Rust Lambda functions:

```bash
# Build the Lambda function
npx nx run rust_lambda:build
```

This command will:
1. Compile the Rust code for AWS Lambda
2. Package it into a deployment-ready ZIP file
3. Output the ZIP file to `dist/target/rust_lambda/lambda/hello_world_handler/bootstrap.zip`
