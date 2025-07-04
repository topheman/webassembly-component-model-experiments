When running `just build-pluginlab`, `just build-pluginlab-release`, `just publish-pluginlab` or `just publish-pluginlab-dry-run`, the `wit` directory from the root of the project is copied over to this folder.

This directory is versioned under git because it is necessary to make the `wit` files available to the `pluginlab` crate when running `cargo publish`.

DO NOT EDIT the wit files in this directory.

Use the `just publish-pluginlab-dry-run` and `just publish-pluginlab` commands for the publish process.
