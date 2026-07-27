// ============================================================
// hace-uni-runtime: Universal Runtime Public Facade
// Era 5 Ground Reality Materialization
// ============================================================

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================
// Re-exports
// ============================================================

pub use hace_uni_resolver::{EnvironmentTopology, PlatformKind, RuntimeKind, TransportKind, PlatformMask};
pub use rr_wasm_runtime::{WasmRuntime, RuntimeConfig, RuntimeOutput};

// ============================================================
// Runtime Request - Universal Interface
// ============================================================

/// Request tới runtime từ Host Surface
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniRuntimeRequest {
    #[wasm_bindgen(skip)]
    pub intent: String,
    #[wasm_bindgen(skip)]
    pub actor_id: String,
    #[wasm_bindgen(skip)]
    pub wasm_artifact: Vec<u8>,
    #[wasm_bindgen(skip)]
    pub config: HashMap<String, String>,
}

// ============================================================
// Runtime Instance Handle
// ============================================================

/// Kết quả trả về từ rr-wasm-runtime
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WasmInstanceHandle {
    #[wasm_bindgen(skip)]
    pub instance_id: String,
    #[wasm_bindgen(skip)]
    pub ze_evidence: Vec<u8>,
}

// ============================================================
// Universal Runtime Facade
// ============================================================

/// Universal Runtime Adapter - Public Facade
#[wasm_bindgen]
pub struct UniRuntime {
    topo: EnvironmentTopology,
}

#[wasm_bindgen]
impl UniRuntime {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let topo = EnvironmentTopology::current();
        Self { topo }
    }

    /// Resolve môi trường trước khi chạy
    #[wasm_bindgen(js_name = "resolveTopology")]
    pub fn resolve_topology(&mut self) -> String {
        self.topo = EnvironmentTopology::current();
        format!(
            r#"{{"platform":"{}","runtime":"{}","transport":"{}"}}"#,
            self.topo.platform.abbr(),
            self.topo.runtime.abbr(),
            self.topo.primary_transport.abbr()
        )
    }

    /// Thực thi WASM artifact thông qua rr-wasm-runtime
    pub fn execute(&mut self, request: UniRuntimeRequest) -> Result<WasmInstanceHandle, JsValue> {
        // Step 1: Resolve topology
        let topo = EnvironmentTopology::current();

        // Step 2: Validate platform compatibility
        let target_mask = PlatformMask::UNIVERSAL;
        if !PlatformMask::is_compatible(target_mask, &topo) {
            return Err(JsValue::from_str("Platform not compatible"));
        }

        // Step 3: Delegate tới rr-wasm-runtime
        let config = RuntimeConfig::default();
        let _runtime = WasmRuntime::new_stub(config)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Placeholder - sẽ delegate tới rr-wasm-runtime sau
        let handle = WasmInstanceHandle {
            instance_id: format!("uni-runtime-{:?}", topo.runtime),
            ze_evidence: Vec::new(),
        };

        Ok(handle)
    }
}

impl Default for UniRuntime {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// WASM Entry Point
// ============================================================

#[wasm_bindgen]
pub fn create_runtime() -> UniRuntime {
    UniRuntime::new()
}