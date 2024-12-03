load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "transaction_core",
    srcs = ["src/lib.rs"],
    deps = [
        "@crate_index//:chrono",
        "@crate_index//:serde",
        "@crate_index//:uuid",
    ],
    edition = "2021"
)

rust_test(
    name = "transaction_library_test",
    srcs = ["src/lib.rs"],
    deps = [
        ":transaction_core",    # Depend on the library
        "@crate_index//:chrono",
        "@crate_index//:uuid",
        "@crate_index//:serde",
    ],
    edition = "2021"
)