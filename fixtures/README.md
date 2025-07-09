# fixtures

This directory contains the fixtures for the tests.

## filesystem

The `fixtures/filesystem` directory contains files and directories that are meant to be used as source for:

- e2e testing the pluginlab crate - see [filesystem/README.rust.md](./filesystem/README.rust.md)
- mounting a virtual filesystem in the browser - see [filesystem/README.browser.md](./filesystem/README.browser.md)

### e2e testing

The `fixtures/filesystem` is copied to `tmp/filesystem` before running the tests.

The tests are run with the `--dir tmp/filesystem` argument.

That way, the original `fixtures/filesystem` directory is not modified by the tests.

### mounting a virtual filesystem in the browser

TODO
