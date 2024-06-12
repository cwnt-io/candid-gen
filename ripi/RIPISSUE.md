# RIPISSUE

<!-- toc -->

- [Backlog](#backlog)

<!-- tocstop -->

- [ ] [[addmessageinfoifargisnotsomecases-issue]]
- [ ] test layer
```rs
static DFX_CFG: &str = r#"
    {
      "canisters": {
        "counter_backend": {
          "candid": "src/counter_backend/counter_backend.did",
          "declarations": {
            "node_compatibility": true
          },
          "package": "counter_backend",
          "type": "rust"
        },
        "counter_backend2": {
          "candid": "src/counter_backend/counter_backend.did",
          "declarations": {
            "node_compatibility": true
          },
          "package": "counter_backend",
          "type": "rust"
        },
        "counter_backend3": {
          "candid": "src/counter_backend/counter_backend.did",
          "declarations": {
            "node_compatibility": true
          },
          "package": "counter_backend",
          "type": "mokoto"
        },
        "counter_frontend": {
          "dependencies": [
            "counter_backend"
          ],
          "source": [
            "src/counter_frontend/dist"
          ],
          "type": "assets",
          "workspace": "counter_frontend"
        }
      },
      "defaults": {
        "build": {
          "args": "",
          "packtool": ""
        }
      },
      "output_env_file": ".env",
      "version": 1
    }"#;
```
- [ ] generate readme
- [ ] create github topics for this project
- [ ] publish crates.io
  - https://doc.rust-lang.org/cargo/reference/publishing.html

## Backlog

- integrate/fork from candid-extractor?
- candid-gen homepage
- add more validation to deserialize dfxcfg?
  - if package == canister key, for example
