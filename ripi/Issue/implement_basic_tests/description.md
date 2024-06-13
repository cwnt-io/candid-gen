# implement_basic_tests (Issue)

- [ ] gen_candid
  - pass just the canister
  - integration tests
    - [x] fail 1: dir exist, but binary/wasm doesn't
    - fail 2: at get_candid_path... dir do not exist
    - test file existence
    - test if generated file correctly parses
    - rm gen files
- [ ] fix if let Err(e) = run_command(&format!("rm {}", &wasm_file.display())) {
  - substitute with remove_file
- [x] build_wasm32
  - integration tests
- [ ] review / refactor integration tests with mock_project
  - test removing serial from tests
- [x] canisters_to_gen_candid
  - wrap to a function
  - unit test it
  - [x] add Canisters type, refactor
- [x] get_candid_path_str
- [x] get_project_root
- [x] run_command tests
  - [x] refacto run_command integration test, so it tests the real integration
- [x] refactor check_dependencies

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
