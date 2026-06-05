use cclab_accel::{
    paper10_skeleton_marker, Paper10SkeletonCertificate, Paper10UpstreamBinding,
    PAPER1_FROZEN_COMMIT, PAPER2_FROZEN_COMMIT, PAPER3_FROZEN_COMMIT, PAPER4_FROZEN_COMMIT,
    PAPER5_FROZEN_COMMIT, PAPER6_FROZEN_COMMIT, PAPER7_FROZEN_COMMIT, PAPER8_FROZEN_COMMIT,
    PAPER9_FINAL_CERTIFICATE, PAPER9_FROZEN_COMMIT,
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
fn docs_keep_eem002_active_and_physical_claims_false() {
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
