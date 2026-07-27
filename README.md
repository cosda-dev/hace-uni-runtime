# hace-uni-runtime

**SMF ID**: `SMF://hace.uni.runtime.v1`  
**Intent**: DECLARE_SCHEMA  
**Status**: ACTIVE  
**Layer**: L1 — Host / Reality Plane  
**Role**: Universal Execution Facade  
**WASM Ready**: ✅

Public runtime facade that delegates execution to `rr-wasm-runtime`.

---

## Overview

`hace-uni-runtime` is the public execution facade. It is **not** a WASM engine — it orchestrates topology resolution, platform compatibility validation, and delegates actual execution to the external `rr-wasm-runtime` crate.

### Boundary

| Is Not | Does |
|---|---|
| WASM engine | delegate_to_rr_wasm_runtime |
| Authority | orchestrate |
| Capability provider | collect_result |

### Key Types

| Type | Description |
|---|---|
| `UniRuntime` | Public facade class with `resolve_topology()` and `execute()` |
| `UniRuntimeRequest` | Request: intent, actor_id, wasm_artifact, config |
| `WasmInstanceHandle` | Result: instance_id, ze_evidence |

### Re-exports

From `hace-uni-resolver`: `EnvironmentTopology`, `PlatformKind`, `RuntimeKind`, `TransportKind`, `PlatformMask`  
From `rr-wasm-runtime`: `WasmRuntime`, `RuntimeConfig`, `RuntimeOutput`

---

## Usage

### WASM (Browser)

```javascript
import { create_runtime } from '@hacex/hace-uni-runtime';

const runtime = create_runtime();

// Resolve current topology
const topo = runtime.resolveTopology();
// {"platform":"web","runtime":"ts","transport":"web_worker"}

// Execute WASM artifact
const request = {
  intent: "infer",
  actor_id: "agent-001",
  wasm_artifact: wasmBytes,
  config: { "timeout": "5000" }
};

const handle = runtime.execute(request);
// WasmInstanceHandle { instance_id: "...", ze_evidence: [...] }
```

### Native (Rust)

```rust
use haha_uni_runtime::UniRuntime;

let mut runtime = UniRuntime::new();
let topo = runtime.resolve_topology();
```

---

## Execution Flow

```
Host
  │
  ▼
UniRuntime
  │
  ├── resolve_topology() → EnvironmentTopology
  ├── PlatformMask::is_compatible() → bool
  │
  ▼
rr-wasm-runtime
  │
  ├── WasmRuntime::new_stub(config)
  ├── Instance
  ├── Memory
  ├── Execute
  │
  ▼
CESI Gate (hooks: HP_IO_GATE, HP_EXEC_BOUNDARY, HP_POST_EXEC)
  │
  ▼
ZE Evidence / ALR
```

---

## Build

```bash
cd engine/hace/uni/runtime
cargo build --release
cargo build --target wasm32-unknown-unknown --release
```

---

## Dependencies

- `hace-uni-resolver` (path dependency)
- `rr-wasm-runtime` (external: `../../../rr/wasm/runtime`)
- `serde` 1.0
- `serde_json` 1.0
- `bincode` 1.3
- `wasm-bindgen` 0.2

---

## Canonical References

- **Spec**: `SMF://hace.uni.runtime.v1` — `.know/canon/specs.ail`
- **Blueprint**: `AIL://hace.uni.canon.blueprint.v1` — `.know/canon/blueprint.ail`
- **Hookpoints**: `hok://uni/runtime/*` — `.know/canon/hookpoint.ail`
- **FAN**: 4 features — `.know/canon/fan.ail`
- **ASI**: Integration layer — `.know/canon/asi.ail`

**END OF README**
