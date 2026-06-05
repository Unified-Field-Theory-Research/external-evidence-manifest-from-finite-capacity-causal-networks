use cclab_accel::{
    eem002_finite_external_evidence_record_manifest_marker,
    eem003_finite_reproduction_protocol_descriptor_marker,
    eem004_paper9_comparison_compatibility_marker,
    eem005_evidence_stability_coarse_graining_marker, eem006_paper9_regime_consistency_marker,
    eem007_no_hidden_physical_promotion_audit_marker, paper10_skeleton_marker,
    EvidenceStabilityCoarseGraining, FiniteExternalEvidenceRecordManifest,
    FiniteReproductionProtocolDescriptor, NoHiddenPhysicalPromotionAudit,
    Paper10SkeletonCertificate, Paper10UpstreamBinding, Paper9ComparisonCompatibility,
    Paper9RegimeConsistency, PAPER1_FROZEN_COMMIT, PAPER2_FROZEN_COMMIT, PAPER3_FROZEN_COMMIT,
    PAPER4_FROZEN_COMMIT, PAPER5_FROZEN_COMMIT, PAPER6_FROZEN_COMMIT, PAPER7_FROZEN_COMMIT,
    PAPER8_FROZEN_COMMIT, PAPER9_FINAL_CERTIFICATE, PAPER9_FROZEN_COMMIT,
};
use std::fs;
use std::path::{Path, PathBuf};

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate remains under rust/cclab_accel")
        .to_path_buf()
}

fn read(root: &Path, relative: &str) -> String {
    let path = root.join(relative);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("can read {}: {err}", path.display()))
}

fn assert_contains(text: &str, token: &str, artifact: &str) {
    assert!(
        text.contains(token),
        "{artifact} is missing required token {token:?}"
    );
}

fn collect_python_artifacts(path: &Path, out: &mut Vec<PathBuf>) {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("");
    if matches!(file_name, ".git" | "target" | ".lake") {
        return;
    }

    let metadata = fs::symlink_metadata(path).expect("can stat repository path");
    if metadata.is_dir() {
        if file_name == "__pycache__" {
            out.push(path.to_path_buf());
            return;
        }
        for entry in fs::read_dir(path).expect("can read repository directory") {
            let entry = entry.expect("can read repository directory entry");
            collect_python_artifacts(&entry.path(), out);
        }
        return;
    }

    if matches!(
        path.extension().and_then(|value| value.to_str()),
        Some("py" | "pyc" | "pyo")
    ) {
        out.push(path.to_path_buf());
    }
}

#[test]
fn paper10_skeleton_files_exist() {
    let root = project_root();
    for relative in [
        "AGENTS.md",
        "README.md",
        "UPSTREAM-PAPERS.json",
        "GPD/PROJECT.md",
        "GPD/ROADMAP.md",
        "GPD/STATE.md",
        "GPD/state.json",
        "docs/external_evidence_manifest_theorem.md",
        "docs/open_proof_obligations.md",
        "docs/proof_log.md",
        "GPD/formal/FiniteCapacity/ExternalEvidenceManifest.lean",
        "GPD/publication/external-evidence-manifest-theorem/manuscript/external_evidence_manifest_theorem.tex",
        "GPD/publication/external-evidence-manifest-theorem/manuscript/PAPER-CONFIG.json",
        "GPD/publication/external-evidence-manifest-theorem/manuscript/ARTIFACT-MANIFEST.json",
        "GPD/publication/external-evidence-manifest-theorem/manuscript/reproducibility-manifest.json",
    ] {
        assert!(
            root.join(relative).exists(),
            "missing required file {relative}"
        );
    }
}

#[test]
fn eem001_upstream_binding_consumes_closed_paper9_chain() {
    let binding = Paper10UpstreamBinding::canonical_eem001();

    assert_eq!(binding.paper1_commit, PAPER1_FROZEN_COMMIT);
    assert_eq!(binding.paper2_commit, PAPER2_FROZEN_COMMIT);
    assert_eq!(binding.paper3_commit, PAPER3_FROZEN_COMMIT);
    assert_eq!(binding.paper4_commit, PAPER4_FROZEN_COMMIT);
    assert_eq!(binding.paper5_commit, PAPER5_FROZEN_COMMIT);
    assert_eq!(binding.paper6_commit, PAPER6_FROZEN_COMMIT);
    assert_eq!(binding.paper7_commit, PAPER7_FROZEN_COMMIT);
    assert_eq!(binding.paper8_commit, PAPER8_FROZEN_COMMIT);
    assert_eq!(binding.paper9_commit, PAPER9_FROZEN_COMMIT);
    assert!(binding.paper9_final_certificate_consumed);
    assert!(binding.closes_eem001());

    let promoted_evidence = Paper10UpstreamBinding {
        external_evidence_manifest_recovery_claim: true,
        ..binding
    };
    assert!(!promoted_evidence.closes_eem001());

    let promoted_physical = Paper10UpstreamBinding {
        physical_nature_claim: true,
        ..binding
    };
    assert!(!promoted_physical.closes_eem001());

    let simulation_only = Paper10UpstreamBinding {
        simulation_only_promotion: true,
        ..binding
    };
    assert!(!simulation_only.closes_eem001());

    let fit_only = Paper10UpstreamBinding {
        fit_only_calibration_claim: true,
        ..binding
    };
    assert!(!fit_only.closes_eem001());

    let wrong_paper9_commit = Paper10UpstreamBinding {
        paper9_commit: "unapproved-paper9-revision",
        ..binding
    };
    assert!(!wrong_paper9_commit.closes_eem001());
}

#[test]
fn initial_skeleton_does_not_close_paper10_theorem() {
    let certificate = Paper10SkeletonCertificate::initial_eem001_only();

    assert!(certificate.eem001_upstream_binding_closed);
    assert!(!certificate.eem002_finite_external_evidence_record_manifest_closed);
    assert!(!certificate.eem008_final_conditional_certificate_closed);
    assert!(!certificate.paper10_theorem_closed);
    assert!(!certificate.closes_paper10_theorem());
    assert_eq!(
        paper10_skeleton_marker(),
        "paper10-external-evidence-manifest-eem001-nonpromoting-skeleton"
    );
}

#[test]
fn eem002_finite_external_evidence_record_manifest_closes_only_record_rows() {
    let manifest = FiniteExternalEvidenceRecordManifest::canonical_eem002();
    assert!(manifest.closes_eem002());
    assert!(manifest.eem001_upstream_binding_closed);
    assert!(manifest.paper9_descriptor_rows_compatible);
    assert!(manifest.paper9_comparison_map_rows_compatible);
    assert_eq!(
        eem002_finite_external_evidence_record_manifest_marker(),
        "eem002-finite-external-evidence-record-manifest-closed"
    );

    let skeleton =
        Paper10SkeletonCertificate::with_eem002_finite_external_evidence_record_manifest_closed();
    assert!(skeleton.eem001_upstream_binding_closed);
    assert!(skeleton.eem002_finite_external_evidence_record_manifest_closed);
    assert!(!skeleton.eem003_finite_reproduction_protocol_descriptor_closed);
    assert!(!skeleton.paper10_theorem_closed);
    assert!(!skeleton.closes_paper10_theorem());
}

#[test]
fn eem002_record_manifest_fails_closed_on_missing_bounds_or_hidden_imports() {
    let manifest = FiniteExternalEvidenceRecordManifest::canonical_eem002();

    let missing_evidence_bound = FiniteExternalEvidenceRecordManifest {
        evidence_id_bound: 0,
        ..manifest
    };
    assert!(!missing_evidence_bound.closes_eem002());

    let overfull_provenance = FiniteExternalEvidenceRecordManifest {
        occupied_source_provenance_descriptor_count: manifest.source_provenance_descriptor_bound
            + 1,
        ..manifest
    };
    assert!(!overfull_provenance.closes_eem002());

    let missing_paper9_descriptor_compatibility = FiniteExternalEvidenceRecordManifest {
        paper9_descriptor_rows_compatible: false,
        ..manifest
    };
    assert!(!missing_paper9_descriptor_compatibility.closes_eem002());

    let observed_recovery_import = FiniteExternalEvidenceRecordManifest {
        observed_particle_catalog_recovery_import: true,
        ..manifest
    };
    assert!(!observed_recovery_import.closes_eem002());

    let physical_standard_model_import = FiniteExternalEvidenceRecordManifest {
        physical_standard_model_content_import: true,
        ..manifest
    };
    assert!(!physical_standard_model_import.closes_eem002());

    let fit_only = FiniteExternalEvidenceRecordManifest {
        fit_only_calibration: true,
        ..manifest
    };
    assert!(!fit_only.closes_eem002());
}

#[test]
fn eem003_finite_reproduction_protocol_descriptor_closes_only_protocol_rows() {
    let protocol = FiniteReproductionProtocolDescriptor::canonical_eem003();
    assert!(protocol.closes_eem003());
    assert!(protocol.eem001_upstream_binding_closed);
    assert!(protocol.eem002_finite_external_evidence_record_manifest_closed);
    assert!(protocol.evidence_manifest_support_preserved);
    assert!(protocol.paper9_comparison_links_preserved);
    assert_eq!(
        eem003_finite_reproduction_protocol_descriptor_marker(),
        "eem003-finite-reproduction-protocol-descriptor-closed"
    );

    let skeleton =
        Paper10SkeletonCertificate::with_eem003_finite_reproduction_protocol_descriptor_closed();
    assert!(skeleton.eem001_upstream_binding_closed);
    assert!(skeleton.eem002_finite_external_evidence_record_manifest_closed);
    assert!(skeleton.eem003_finite_reproduction_protocol_descriptor_closed);
    assert!(!skeleton.eem004_paper9_comparison_compatibility_closed);
    assert!(!skeleton.paper10_theorem_closed);
    assert!(!skeleton.closes_paper10_theorem());
}

#[test]
fn eem003_protocol_descriptor_fails_closed_on_missing_dependency_or_proof_imports() {
    let protocol = FiniteReproductionProtocolDescriptor::canonical_eem003();

    let missing_eem002 = FiniteReproductionProtocolDescriptor {
        eem002_finite_external_evidence_record_manifest_closed: false,
        ..protocol
    };
    assert!(!missing_eem002.closes_eem003());

    let overfull_steps = FiniteReproductionProtocolDescriptor {
        occupied_protocol_step_descriptor_count: protocol.protocol_step_descriptor_bound + 1,
        ..protocol
    };
    assert!(!overfull_steps.closes_eem003());

    let simulation_as_proof = FiniteReproductionProtocolDescriptor {
        simulation_only_proof_import: true,
        ..protocol
    };
    assert!(!simulation_as_proof.closes_eem003());

    let fit_as_proof = FiniteReproductionProtocolDescriptor {
        fit_only_proof_import: true,
        ..protocol
    };
    assert!(!fit_as_proof.closes_eem003());

    let generated_prose_as_proof = FiniteReproductionProtocolDescriptor {
        generated_prose_proof_import: true,
        ..protocol
    };
    assert!(!generated_prose_as_proof.closes_eem003());

    let external_catalog_as_proof = FiniteReproductionProtocolDescriptor {
        external_catalog_as_proof_import: true,
        ..protocol
    };
    assert!(!external_catalog_as_proof.closes_eem003());

    let review_status_as_proof = FiniteReproductionProtocolDescriptor {
        review_status_as_proof_import: true,
        ..protocol
    };
    assert!(!review_status_as_proof.closes_eem003());
}

#[test]
fn eem004_paper9_comparison_compatibility_closes_only_compatibility_rows() {
    let compatibility = Paper9ComparisonCompatibility::canonical_eem004();
    assert!(compatibility.closes_eem004());
    assert!(compatibility.eem001_upstream_binding_closed);
    assert!(compatibility.eem002_finite_external_evidence_record_manifest_closed);
    assert!(compatibility.eem003_finite_reproduction_protocol_descriptor_closed);
    assert!(compatibility.paper9_final_certificate_consumed);
    assert!(compatibility.paper9_descriptor_rows_preserved);
    assert!(compatibility.paper9_comparison_map_rows_preserved);
    assert!(compatibility.causal_cone_no_signaling_preserved);
    assert_eq!(
        eem004_paper9_comparison_compatibility_marker(),
        "eem004-paper9-comparison-compatibility-closed"
    );

    let skeleton = Paper10SkeletonCertificate::with_eem004_paper9_comparison_compatibility_closed();
    assert!(skeleton.eem001_upstream_binding_closed);
    assert!(skeleton.eem002_finite_external_evidence_record_manifest_closed);
    assert!(skeleton.eem003_finite_reproduction_protocol_descriptor_closed);
    assert!(skeleton.eem004_paper9_comparison_compatibility_closed);
    assert!(!skeleton.eem005_evidence_stability_coarse_graining_closed);
    assert!(!skeleton.paper10_theorem_closed);
    assert!(!skeleton.closes_paper10_theorem());
}

#[test]
fn eem004_paper9_compatibility_fails_closed_on_bypass_or_hidden_imports() {
    let compatibility = Paper9ComparisonCompatibility::canonical_eem004();

    let missing_eem003 = Paper9ComparisonCompatibility {
        eem003_finite_reproduction_protocol_descriptor_closed: false,
        ..compatibility
    };
    assert!(!missing_eem003.closes_eem004());

    let missing_paper9_certificate = Paper9ComparisonCompatibility {
        paper9_final_certificate_consumed: false,
        ..compatibility
    };
    assert!(!missing_paper9_certificate.closes_eem004());

    let missing_no_signaling = Paper9ComparisonCompatibility {
        causal_cone_no_signaling_preserved: false,
        ..compatibility
    };
    assert!(!missing_no_signaling.closes_eem004());

    let bypass = Paper9ComparisonCompatibility {
        paper9_bypass_attempt: true,
        ..compatibility
    };
    assert!(!bypass.closes_eem004());

    let observed_catalog_recovery = Paper9ComparisonCompatibility {
        observed_catalog_recovery_import: true,
        ..compatibility
    };
    assert!(!observed_catalog_recovery.closes_eem004());

    let physical_standard_model_import = Paper9ComparisonCompatibility {
        physical_standard_model_content_import: true,
        ..compatibility
    };
    assert!(!physical_standard_model_import.closes_eem004());

    let fit_only = Paper9ComparisonCompatibility {
        fit_only_calibration: true,
        ..compatibility
    };
    assert!(!fit_only.closes_eem004());
}

#[test]
fn eem005_evidence_stability_closes_only_stability_rows() {
    let stability = EvidenceStabilityCoarseGraining::canonical_eem005();
    assert!(stability.closes_eem005());
    assert!(stability.eem004_paper9_comparison_compatibility_closed);
    assert!(stability.coarse_evidence_id_bound <= stability.evidence_id_bound);
    assert!(stability.coarse_provenance_descriptor_bound <= stability.provenance_descriptor_bound);
    assert!(stability.coarse_paper9_link_bound <= stability.paper9_link_bound);
    assert!(stability.coarse_transfer_bound <= stability.transfer_bound);
    assert!(stability.causal_cone_no_signaling_stability_preserved);
    assert_eq!(
        eem005_evidence_stability_coarse_graining_marker(),
        "eem005-evidence-stability-coarse-graining-closed"
    );

    let skeleton =
        Paper10SkeletonCertificate::with_eem005_evidence_stability_coarse_graining_closed();
    assert!(skeleton.eem001_upstream_binding_closed);
    assert!(skeleton.eem002_finite_external_evidence_record_manifest_closed);
    assert!(skeleton.eem003_finite_reproduction_protocol_descriptor_closed);
    assert!(skeleton.eem004_paper9_comparison_compatibility_closed);
    assert!(skeleton.eem005_evidence_stability_coarse_graining_closed);
    assert!(!skeleton.eem006_paper9_regime_consistency_closed);
    assert!(!skeleton.paper10_theorem_closed);
    assert!(!skeleton.closes_paper10_theorem());
}

#[test]
fn eem005_stability_fails_closed_on_growth_or_continuum_imports() {
    let stability = EvidenceStabilityCoarseGraining::canonical_eem005();

    let missing_eem004 = EvidenceStabilityCoarseGraining {
        eem004_paper9_comparison_compatibility_closed: false,
        ..stability
    };
    assert!(!missing_eem004.closes_eem005());

    let evidence_growth = EvidenceStabilityCoarseGraining {
        coarse_evidence_id_bound: stability.evidence_id_bound + 1,
        ..stability
    };
    assert!(!evidence_growth.closes_eem005());

    let transfer_growth = EvidenceStabilityCoarseGraining {
        coarse_transfer_bound: stability.transfer_bound + 1,
        ..stability
    };
    assert!(!transfer_growth.closes_eem005());

    let external_conservation = EvidenceStabilityCoarseGraining {
        external_conservation_law_import: true,
        ..stability
    };
    assert!(!external_conservation.closes_eem005());

    let continuum_current = EvidenceStabilityCoarseGraining {
        continuum_current_import: true,
        ..stability
    };
    assert!(!continuum_current.closes_eem005());

    let continuum_oracle = EvidenceStabilityCoarseGraining {
        continuum_limit_oracle_import: true,
        ..stability
    };
    assert!(!continuum_oracle.closes_eem005());
}

#[test]
fn eem006_paper9_regime_consistency_closes_only_recorded_regime_rows() {
    let regime = Paper9RegimeConsistency::canonical_eem006();
    assert!(regime.closes_eem006());
    assert!(regime.eem005_evidence_stability_coarse_graining_closed);
    assert_eq!(regime.paper1_commit, PAPER1_FROZEN_COMMIT);
    assert_eq!(regime.paper2_commit, PAPER2_FROZEN_COMMIT);
    assert_eq!(regime.paper3_commit, PAPER3_FROZEN_COMMIT);
    assert_eq!(regime.paper4_commit, PAPER4_FROZEN_COMMIT);
    assert_eq!(regime.paper5_commit, PAPER5_FROZEN_COMMIT);
    assert_eq!(regime.paper6_commit, PAPER6_FROZEN_COMMIT);
    assert_eq!(regime.paper7_commit, PAPER7_FROZEN_COMMIT);
    assert_eq!(regime.paper8_commit, PAPER8_FROZEN_COMMIT);
    assert_eq!(regime.paper9_commit, PAPER9_FROZEN_COMMIT);
    assert_eq!(regime.paper9_final_certificate, PAPER9_FINAL_CERTIFICATE);
    assert_eq!(
        eem006_paper9_regime_consistency_marker(),
        "eem006-paper9-regime-consistency-no-upstream-bypass-closed"
    );

    let skeleton = Paper10SkeletonCertificate::with_eem006_paper9_regime_consistency_closed();
    assert!(skeleton.eem001_upstream_binding_closed);
    assert!(skeleton.eem002_finite_external_evidence_record_manifest_closed);
    assert!(skeleton.eem003_finite_reproduction_protocol_descriptor_closed);
    assert!(skeleton.eem004_paper9_comparison_compatibility_closed);
    assert!(skeleton.eem005_evidence_stability_coarse_graining_closed);
    assert!(skeleton.eem006_paper9_regime_consistency_closed);
    assert!(!skeleton.eem007_no_hidden_physical_promotion_audit_closed);
    assert!(!skeleton.paper10_theorem_closed);
    assert!(!skeleton.closes_paper10_theorem());
}

#[test]
fn eem006_regime_consistency_fails_closed_on_bypass_or_unrecorded_revision() {
    let regime = Paper9RegimeConsistency::canonical_eem006();

    let wrong_paper9_commit = Paper9RegimeConsistency {
        paper9_commit: "unapproved-paper9-revision",
        ..regime
    };
    assert!(!wrong_paper9_commit.closes_eem006());

    let missing_eem005 = Paper9RegimeConsistency {
        eem005_evidence_stability_coarse_graining_closed: false,
        ..regime
    };
    assert!(!missing_eem005.closes_eem006());

    let upstream_mutation = Paper9RegimeConsistency {
        upstream_mutation_attempt: true,
        ..regime
    };
    assert!(!upstream_mutation.closes_eem006());

    let paper9_bypass = Paper9RegimeConsistency {
        paper9_bypass_attempt: true,
        ..regime
    };
    assert!(!paper9_bypass.closes_eem006());

    let upstream_bypass = Paper9RegimeConsistency {
        upstream_chain_bypass_attempt: true,
        ..regime
    };
    assert!(!upstream_bypass.closes_eem006());

    let unapproved_revision = Paper9RegimeConsistency {
        unapproved_paper9_revision: true,
        ..regime
    };
    assert!(!unapproved_revision.closes_eem006());

    let unrecorded_revision = Paper9RegimeConsistency {
        unrecorded_upstream_revision: true,
        ..regime
    };
    assert!(!unrecorded_revision.closes_eem006());
}

#[test]
fn eem007_no_hidden_import_audit_closes_only_audit_rows() {
    let audit = NoHiddenPhysicalPromotionAudit::canonical_eem007();
    assert!(audit.closes_eem007());
    assert!(audit.eem006_paper9_regime_consistency_closed);
    assert!(audit.audited_eem_rung_count >= audit.required_eem_rung_count);
    assert!(audit.theorem_docs_audited);
    assert!(audit.proof_log_audited);
    assert!(audit.state_files_audited);
    assert!(audit.upstream_manifest_audited);
    assert!(audit.lean_gate_audited);
    assert!(audit.rust_gate_audited);
    assert!(audit.publication_skeleton_audited);
    assert!(audit.rust_only_runtime_verified);
    assert!(audit.fail_closed_audit_certificate_emitted);
    assert_eq!(
        eem007_no_hidden_physical_promotion_audit_marker(),
        "eem007-no-hidden-physical-promotion-import-audit-closed"
    );

    let skeleton =
        Paper10SkeletonCertificate::with_eem007_no_hidden_physical_promotion_audit_closed();
    assert!(skeleton.eem001_upstream_binding_closed);
    assert!(skeleton.eem002_finite_external_evidence_record_manifest_closed);
    assert!(skeleton.eem003_finite_reproduction_protocol_descriptor_closed);
    assert!(skeleton.eem004_paper9_comparison_compatibility_closed);
    assert!(skeleton.eem005_evidence_stability_coarse_graining_closed);
    assert!(skeleton.eem006_paper9_regime_consistency_closed);
    assert!(skeleton.eem007_no_hidden_physical_promotion_audit_closed);
    assert!(!skeleton.eem008_final_conditional_certificate_closed);
    assert!(!skeleton.paper10_theorem_closed);
    assert!(!skeleton.closes_paper10_theorem());
}

#[test]
fn eem007_no_hidden_import_audit_fails_closed_on_missing_coverage_or_imports() {
    let audit = NoHiddenPhysicalPromotionAudit::canonical_eem007();

    let missing_eem006 = NoHiddenPhysicalPromotionAudit {
        eem006_paper9_regime_consistency_closed: false,
        ..audit
    };
    assert!(!missing_eem006.closes_eem007());

    let insufficient_coverage = NoHiddenPhysicalPromotionAudit {
        audited_eem_rung_count: audit.required_eem_rung_count - 1,
        ..audit
    };
    assert!(!insufficient_coverage.closes_eem007());

    let missing_rust_only = NoHiddenPhysicalPromotionAudit {
        rust_only_runtime_verified: false,
        ..audit
    };
    assert!(!missing_rust_only.closes_eem007());

    let simulation_only = NoHiddenPhysicalPromotionAudit {
        simulation_only_promotion: true,
        ..audit
    };
    assert!(!simulation_only.closes_eem007());

    let fit_only = NoHiddenPhysicalPromotionAudit {
        fit_only_calibration: true,
        ..audit
    };
    assert!(!fit_only.closes_eem007());

    let generated_prose = NoHiddenPhysicalPromotionAudit {
        generated_prose_proof_import: true,
        ..audit
    };
    assert!(!generated_prose.closes_eem007());

    let external_catalog_as_proof = NoHiddenPhysicalPromotionAudit {
        external_catalog_as_proof_import: true,
        ..audit
    };
    assert!(!external_catalog_as_proof.closes_eem007());

    let physical_promotion = NoHiddenPhysicalPromotionAudit {
        physical_promotion: true,
        ..audit
    };
    assert!(!physical_promotion.closes_eem007());
}

#[test]
fn upstream_json_records_paper9_certificate_and_nonpromotion() {
    let root = project_root();
    let upstream = read(&root, "UPSTREAM-PAPERS.json");

    assert_contains(&upstream, PAPER1_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER2_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER3_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER4_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER5_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER6_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER7_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER8_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER9_FROZEN_COMMIT, "UPSTREAM-PAPERS.json");
    assert_contains(&upstream, PAPER9_FINAL_CERTIFICATE, "UPSTREAM-PAPERS.json");
    assert_contains(
        &upstream,
        "\"eem002_finite_external_evidence_record_manifest_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"eem003_finite_reproduction_protocol_descriptor_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"eem004_paper9_comparison_compatibility_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"eem005_evidence_stability_coarse_graining_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"eem006_paper9_regime_consistency_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"eem007_no_hidden_physical_promotion_audit_closed\": true",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"external_evidence_manifest_theorem_closed\": false",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"physical_nature_claim\": false",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"simulation_only_promotion\": false",
        "UPSTREAM-PAPERS.json",
    );
    assert_contains(
        &upstream,
        "\"fit_only_calibration_claim\": false",
        "UPSTREAM-PAPERS.json",
    );
}

#[test]
fn docs_keep_eem008_active_and_physical_claims_false() {
    let root = project_root();
    let theorem = read(&root, "docs/external_evidence_manifest_theorem.md");
    let state = read(&root, "GPD/STATE.md");
    let readme = read(&root, "README.md");

    for artifact in [
        ("theorem doc", theorem.as_str()),
        ("state", state.as_str()),
        ("readme", readme.as_str()),
    ] {
        assert_contains(artifact.1, "EEM-001", artifact.0);
        assert_contains(artifact.1, "EEM-002", artifact.0);
        assert_contains(artifact.1, "EEM-003", artifact.0);
        assert_contains(artifact.1, "EEM-004", artifact.0);
        assert_contains(artifact.1, "EEM-005", artifact.0);
        assert_contains(artifact.1, "EEM-006", artifact.0);
        assert_contains(artifact.1, "EEM-007", artifact.0);
        assert_contains(artifact.1, "EEM-008", artifact.0);
        assert_contains(artifact.1, "observed particle catalog recovery", artifact.0);
        assert_contains(artifact.1, "physical Standard Model", artifact.0);
        assert_contains(artifact.1, "simulation-only promotion", artifact.0);
        assert_contains(artifact.1, "fit-only calibration", artifact.0);
        assert_contains(artifact.1, "physical nature", artifact.0);
        assert_contains(artifact.1, "unified", artifact.0);
    }
}

#[test]
fn repository_contains_no_python_artifacts() {
    let root = project_root();
    let mut python_artifacts = Vec::new();
    collect_python_artifacts(&root, &mut python_artifacts);

    assert!(
        python_artifacts.is_empty(),
        "Paper 10 scaffold must remain Rust-only; found Python artifacts: {python_artifacts:?}"
    );
}
