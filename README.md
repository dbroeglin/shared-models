# shared-models

Shared request and response types for the multi-repository Rust demo.

Both sibling applications depend on this repository through a local path:

```toml
shared-models = { path = "../shared-models" }
```

Changing a public type here demonstrates cross-repository navigation, references,
and coordinated edits in VS Code.
