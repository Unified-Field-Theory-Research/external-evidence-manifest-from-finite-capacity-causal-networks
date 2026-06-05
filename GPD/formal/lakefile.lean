import Lake
open Lake DSL

package external_evidence_manifest_formal where
  -- Opt-in proof-certificate lane. This package is intentionally not part of
  -- the default Rust build.

@[default_target]
lean_lib FiniteCapacity where
