## Substreams Shared Code and Modules (a.k.a Monorepo)

This example showcase how you can shared Rust code across all your Substreams modules using a dedicated `shared` package as well as a common `core` module extracting data from a specific contract and feeding multiple sinks afterward.

> [!NOTE]
> A single sink named `sink1` is showcased for simplicity in this example, but it can be extended to any other count of modules.

### Run

```
cargo build --release --target wasm32-unknown-unknown

substreams run ./core/substeams.yaml map_transfers -s 17000000
substreams run ./sink1/substeams.yaml db_out
```

### Packages

Here the details of each folder and their purposes.

#### Packge [`shared`](./shared)

  This package is the shared Rust code across all your modules. It's not a Substreams module as it needs to be shared a library across the various Substreams modules.

  Indeed, the `core` module being a Substreams module, it produces a final binary `.wasm` file and as such, it can be a dependency of another package.

  That's why we have `shared` that contains in this example Protobuf source definitions as well as Rust generated Protobuf bindings. This module could be extended to hold any helpers/logic/shared data structure(s).

#### Packge [`core`](./core)

  This is the core Substreams module. It depends on `shared` by importing the correct Rust bindings from it. The module process block and extract USDT transfers from it and output them.

  This module exists so that it Substreams server can properly cache the extracted output from the block, in your case USDT transfers but it could be anything really.

  By ensuring that a compiled `.spkg` file is used, we make sure that as long as downstream Substreams depends on this compiled and packed `.spkg`, they will consume data from the cache and will avoid reprocessing the same data over and over.

  > [!NOTE]
  > *In our example, for simplicity, we use a locally committed `.spkg` file (namely [core/core-v0.1.0.spkg](./core/core-v0.1.0.spkg)). In a real production example, you should ensure this file is properly "released" and kept immutable. Most of our sinks and Substreams CLI supports reading the `.spkg` from HTTP/HTTPS, Google Cloud Storage (via `gs://<bucket>/<file>`) as well as from S3 bucket (via `s3://<bucket>/<file>`).
  >
  > See https://github.com/streamingfast/dstore?tab=readme-ov-file#features for `gs://` and `s3://` URL format supported.

#### Packge [`sink1`](./sink1)

  This is also a Substreams module that depends on `core` Substreams module and uses `shared` to pull the Rust project specific Protobuf bindings for processing the `Transfers` output by `core` module.

  The `core` module is imported in [sink1/substreams.yaml#L9](./sink1/substreams.yaml#L9) and the `db_out` module defines `core:map_transfers` as its input.

  The `sink1` module then process the transfers and then emits `DatabaseChanges` element that will ultimately be consumed by https://github.com/streamingfast/substreams-sink-sql and will made it to a SQL relational database.

  This is out of scope for this example, that is incomplete in this regards as its goal is to showcase how one can shared code and Substreams cache across different units.

### Resources

- https://substreams.streamingfast.io/documentation/develop/manifest-modules/inputs#input-type-map
- https://substreams.streamingfast.io/documentation/develop/manifest-modules/setting-up-handlers
- https://substreams.streamingfast.io/reference-and-specs/manifests#imports
