load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")

rust_library(
    name = "xmlserde",
    aliases = aliases(),
    srcs = glob(["src/**/*.rs"]),
    visibility = ["//visibility:public"],
    deps = all_crate_deps(
        normal = True,
    ),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ) + ["//crates/xmlserde/derives:logisheets_derives"],
    edition = "2018",
)
