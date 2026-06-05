# Proof Log

## EEM-001: Upstream Binding And Claim Boundary

Status: closed as a scaffold-level upstream binding and nonpromotion contract.

Artifacts:

- `AGENTS.md`
- `README.md`
- `UPSTREAM-PAPERS.json`
- `GPD/PROJECT.md`
- `GPD/ROADMAP.md`
- `GPD/STATE.md`
- `GPD/state.json`
- `docs/external_evidence_manifest_theorem.md`
- `docs/open_proof_obligations.md`
- `GPD/formal/FiniteCapacity/ExternalEvidenceManifest.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/external_evidence_manifest_gate.rs`

Rust anchors:

- `Paper10UpstreamBinding`
- `Paper10UpstreamBinding::canonical_eem001`
- `Paper10UpstreamBinding::closes_eem001`
- `Paper10SkeletonCertificate::initial_eem001_only`
- `paper10_skeleton_marker`

Lean anchors:

- `EEM001UpstreamBindingContract`
- `EEM001UpstreamBindingContract.closed`
- `eem001_upstream_binding_closed_from_fields`
- `eem001_canonical_upstream_binding_closed`
- `paper10_eem001_skeleton_does_not_close_external_evidence_manifest_theorem`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`EEM-001` consumes the recorded Paper 9 final conditional observed-catalog
comparison observables certificate and the recorded Paper 1 through Paper 8
chain. It does not prove external evidence manifest recovery, observed
particle catalog recovery, physical Standard Model content, physical particle
excitations, physical quantum dynamics, continuum quantum field theory,
simulation-only promotion, fit-only calibration, a physical nature-level
claim, or a unified field theory.

## Active Next Obligation

`EEM-002`: define a finite external evidence record manifest without
importing observed particle catalog recovery, physical Standard Model content,
physical particle excitations, continuum QFT, external Hilbert bundles,
simulation-only promotion, fit-only calibration, physical quantum dynamics,
physical nature promotion, or unified-field promotion.
