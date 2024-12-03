load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "payment_processor",
    srcs = glob(["src/**/*.rs"]),
    deps = [
        "//transaction_core:transaction_core",
        "@crate_index//:axum",
        "@crate_index//:serde",
        "@crate_index//:tokio",
    ],
    edition = "2021"
)