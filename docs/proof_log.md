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

## EEM-005: Evidence Stability And Coarse-Graining Stability

Status: closed as an evidence stability and intrinsic coarse-graining
contract.

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

- `EvidenceStabilityCoarseGraining`
- `EvidenceStabilityCoarseGraining::canonical_eem005`
- `EvidenceStabilityCoarseGraining::closes_eem005`
- `Paper10SkeletonCertificate::with_eem005_evidence_stability_coarse_graining_closed`
- `eem005_evidence_stability_coarse_graining_marker`

Lean anchors:

- `EEM005EvidenceStabilityCoarseGrainingContract`
- `EEM005EvidenceStabilityCoarseGrainingContract.closed`
- `eem005_evidence_stability_coarse_graining_closed_from_fields`
- `eem005_missing_eem004_compatibility_not_closed`
- `eem005_coarse_evidence_growth_not_closed`
- `eem005_continuum_limit_oracle_import_not_closed`
- `eem005_canonical_evidence_stability_coarse_graining_closed`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`EEM-005` proves finite evidence stability and intrinsic coarse-graining
stability with non-growing evidence-ID, provenance, Paper 9 link, local
domain, protocol-step, and transfer bounds. It consumes closed `EEM-004`
Paper 9 compatibility and preserves finite capacity, locality, bounded
transfer, and causal-cone/no-signaling stability. It does not prove Paper 9
regime consistency, no-hidden import audit closure, final external evidence
manifest recovery, observed particle catalog recovery, physical Standard
Model content, physical particle excitations, physical quantum dynamics,
continuum quantum field theory, simulation-only promotion, fit-only
calibration, a physical nature-level claim, or a unified field theory.

## EEM-006: Paper 9 Regime Consistency

Status: closed as a Paper 9 regime-consistency and no-upstream-bypass
contract.

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

- `Paper9RegimeConsistency`
- `Paper9RegimeConsistency::canonical_eem006`
- `Paper9RegimeConsistency::closes_eem006`
- `Paper10SkeletonCertificate::with_eem006_paper9_regime_consistency_closed`
- `eem006_paper9_regime_consistency_marker`

Lean anchors:

- `EEM006Paper9RegimeConsistencyContract`
- `EEM006Paper9RegimeConsistencyContract.closed`
- `eem006_paper9_regime_consistency_closed_from_fields`
- `eem006_missing_eem005_stability_not_closed`
- `eem006_paper9_bypass_not_closed`
- `eem006_unapproved_paper9_revision_not_closed`
- `eem006_canonical_paper9_regime_consistency_closed`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`EEM-006` binds the closed local Paper 10 structures to the recorded Paper 9
commit and final certificate, and through Paper 9 to the recorded Paper 1
through Paper 8 chain. It rejects upstream mutation, Paper 9 bypass,
upstream-chain bypass, unapproved Paper 9 revisions, unrecorded upstream
revisions, observed recovery imports, physical Standard Model content,
physical particle excitations, physical quantum dynamics, continuum quantum
field theory, simulation-only promotion, fit-only calibration, physical
promotion, and unified-field promotion. It does not prove the no-hidden
import audit or final external evidence manifest recovery.

## EEM-007: No-Hidden-Physical-Promotion Import Audit

Status: closed as a fail-closed no-hidden import audit contract.

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

- `NoHiddenPhysicalPromotionAudit`
- `NoHiddenPhysicalPromotionAudit::canonical_eem007`
- `NoHiddenPhysicalPromotionAudit::closes_eem007`
- `Paper10SkeletonCertificate::with_eem007_no_hidden_physical_promotion_audit_closed`
- `eem007_no_hidden_physical_promotion_audit_marker`

Lean anchors:

- `EEM007NoHiddenPhysicalPromotionAuditContract`
- `EEM007NoHiddenPhysicalPromotionAuditContract.closed`
- `eem007_no_hidden_physical_promotion_audit_closed_from_fields`
- `eem007_missing_eem006_regime_not_closed`
- `eem007_missing_rust_only_runtime_not_closed`
- `eem007_simulation_only_promotion_not_closed`
- `eem007_canonical_no_hidden_physical_promotion_audit_closed`

Verification:

- `make test-fast`
- `make lean-build`

Boundary:

`EEM-007` audits theorem docs, proof log, state files, upstream manifest,
Lean gate, Rust gate, publication skeleton, Rust-only runtime policy, and a
fail-closed certificate. It rejects observed recovery, physical Standard
Model content, physical particle excitations, physical quantum dynamics,
external matter/gauge fields, continuum quantum field theory, simulation-only
promotion, fit-only calibration, generated-prose proof imports,
external-catalog-as-proof imports, review-status-as-proof imports, physical
promotion, and unified-field promotion. It does not prove the final external
evidence manifest theorem by itself.

## Active Next Obligation

`EEM-008`: consume `EEM-001` through `EEM-007` and close the final internal
conditional external evidence manifest certificate without physical or
unified-field promotion.
