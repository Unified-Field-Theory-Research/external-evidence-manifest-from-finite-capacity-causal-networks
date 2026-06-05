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

## EEM-002: Finite External Evidence Record Manifest

Status: closed as a finite external evidence record-row contract.

Artifacts:

- `docs/external_evidence_manifest_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/STATE.md`
- `GPD/ROADMAP.md`
- `GPD/state.json`
- `GPD/formal/FiniteCapacity/ExternalEvidenceManifest.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/external_evidence_manifest_gate.rs`

Rust anchors:

- `FiniteExternalEvidenceRecordManifest`
- `FiniteExternalEvidenceRecordManifest::canonical_eem002`
- `FiniteExternalEvidenceRecordManifest::closes_eem002`
- `Paper10SkeletonCertificate::with_eem002_finite_external_evidence_record_manifest_closed`
- `eem002_finite_external_evidence_record_manifest_marker`

Lean anchors:

- `EEM002FiniteExternalEvidenceRecordManifestContract`
- `EEM002FiniteExternalEvidenceRecordManifestContract.closed`
- `eem002_finite_external_evidence_record_manifest_closed_from_fields`
- `eem002_missing_evidence_id_bound_not_closed`
- `eem002_missing_paper9_descriptor_rows_not_closed`
- `eem002_observed_particle_catalog_recovery_import_not_closed`
- `eem002_fit_only_calibration_not_closed`
- `eem002_canonical_finite_external_evidence_record_manifest_closed`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`EEM-002` defines finite external evidence record manifest rows with finite
evidence IDs, source/provenance descriptors, Paper 9 descriptor links, Paper
9 comparison-map links, uncertainty/tolerance metadata,
reproduction-status flags, review-status flags, local evidence domains,
evidence readout boundaries, finite-capacity compatibility, and
bounded-transfer compatibility. It does not prove reproduction protocols,
Paper 9 comparison compatibility, evidence stability, Paper 9 regime
consistency, no-hidden import audit closure, final external evidence
manifest recovery, observed particle catalog recovery, physical Standard
Model content, physical particle excitations, physical quantum dynamics,
continuum quantum field theory, simulation-only promotion, fit-only
calibration, a physical nature-level claim, or a unified field theory.

## EEM-003: Finite Reproduction Protocol Descriptor

Status: closed as a finite reproduction protocol descriptor contract.

Artifacts:

- `docs/external_evidence_manifest_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/STATE.md`
- `GPD/ROADMAP.md`
- `GPD/state.json`
- `GPD/formal/FiniteCapacity/ExternalEvidenceManifest.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/external_evidence_manifest_gate.rs`

Rust anchors:

- `FiniteReproductionProtocolDescriptor`
- `FiniteReproductionProtocolDescriptor::canonical_eem003`
- `FiniteReproductionProtocolDescriptor::closes_eem003`
- `Paper10SkeletonCertificate::with_eem003_finite_reproduction_protocol_descriptor_closed`
- `eem003_finite_reproduction_protocol_descriptor_marker`

Lean anchors:

- `EEM003FiniteReproductionProtocolDescriptorContract`
- `EEM003FiniteReproductionProtocolDescriptorContract.closed`
- `eem003_finite_reproduction_protocol_descriptor_closed_from_fields`
- `eem003_missing_eem002_manifest_not_closed`
- `eem003_simulation_only_proof_import_not_closed`
- `eem003_fit_only_proof_import_not_closed`
- `eem003_canonical_finite_reproduction_protocol_descriptor_closed`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`EEM-003` defines finite reproduction protocol descriptor rows over closed
`EEM-002` evidence rows with finite protocol IDs, protocol-step descriptors,
independent reproduction attempt descriptors, input and output artifact
descriptors, acceptance/tolerance gates, local execution domains,
reproduction readout boundaries, evidence-manifest support preservation,
Paper 9 comparison-link preservation, finite-capacity compatibility, and
bounded-transfer compatibility. It does not prove Paper 9 comparison
compatibility, evidence stability, Paper 9 regime consistency, no-hidden
import audit closure, final external evidence manifest recovery, observed
particle catalog recovery, physical Standard Model content, physical particle
excitations, physical quantum dynamics, continuum quantum field theory,
simulation-only promotion, fit-only calibration, a physical nature-level
claim, or a unified field theory.

## EEM-004: Paper 9 Comparison Compatibility

Status: closed as a Paper 9 comparison-compatibility contract.

Artifacts:

- `docs/external_evidence_manifest_theorem.md`
- `docs/open_proof_obligations.md`
- `docs/proof_log.md`
- `GPD/STATE.md`
- `GPD/ROADMAP.md`
- `GPD/state.json`
- `GPD/formal/FiniteCapacity/ExternalEvidenceManifest.lean`
- `rust/cclab_accel/src/lib.rs`
- `rust/cclab_accel/tests/external_evidence_manifest_gate.rs`

Rust anchors:

- `Paper9ComparisonCompatibility`
- `Paper9ComparisonCompatibility::canonical_eem004`
- `Paper9ComparisonCompatibility::closes_eem004`
- `Paper10SkeletonCertificate::with_eem004_paper9_comparison_compatibility_closed`
- `eem004_paper9_comparison_compatibility_marker`

Lean anchors:

- `EEM004Paper9ComparisonCompatibilityContract`
- `EEM004Paper9ComparisonCompatibilityContract.closed`
- `eem004_paper9_comparison_compatibility_closed_from_fields`
- `eem004_missing_eem003_protocol_not_closed`
- `eem004_missing_paper9_final_certificate_not_closed`
- `eem004_missing_no_signaling_not_closed`
- `eem004_physical_standard_model_content_import_not_closed`
- `eem004_canonical_paper9_comparison_compatibility_closed`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`EEM-004` consumes closed `EEM-002`, closed `EEM-003`, and the closed Paper 9
observed-catalog comparison package. It preserves Paper 9 descriptor rows,
comparison-map rows, Standard-Model-candidate compatibility rows, comparison
stability/coarse-graining rows, finite capacity, locality, bounded transfer,
causal-cone/no-signaling, comparison stability, and
conservation/coarse-graining stability. It does not prove evidence stability,
Paper 9 regime consistency, no-hidden import audit closure, final external
evidence manifest recovery, observed particle catalog recovery, physical
Standard Model content, physical particle excitations, physical quantum
dynamics, continuum quantum field theory, simulation-only promotion,
fit-only calibration, a physical nature-level claim, or a unified field
theory.

## Active Next Obligation

`EEM-005`: prove finite evidence stability and intrinsic coarse-graining
stability without importing external conservation laws, continuum currents,
continuum-limit oracles, observed recovery, simulation-only promotion,
fit-only calibration, physical nature promotion, or unified-field promotion.
