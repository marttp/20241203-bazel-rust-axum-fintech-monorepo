# FinTech Monorepo with Bazel

This is the small project to experiment with Bazel opportunity with Rust programming languages

## Cargo Command

### Create project

```shell
cargo new fintech_monorepo
```

### Create lib

```shell
cargo new --lib transaction_core
```

## Bazel Command

### Run application
```shell
bazel run //:payment_processor
```

### Build application
```shell
bazel build //:payment_processor
```

### Test transaction_core lib
```shell
bazel test //transaction_core:transaction_library_test
```

### Clean
```shell
bazel clean
```