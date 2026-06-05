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

## Active Next Obligation

`EEM-004`: prove compatibility with the closed Paper 9 observed-catalog
comparison rows without importing observed-catalog recovery, observed
particle catalog recovery, physical Standard Model content, physical particle
excitations, continuum QFT, external Hilbert bundles, physical quantum
dynamics, simulation-only promotion, fit-only calibration, physical nature
promotion, or unified-field promotion.
