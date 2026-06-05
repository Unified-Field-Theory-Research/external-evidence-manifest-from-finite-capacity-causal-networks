# Agent Instructions

## Paper 10 Target

This repository is organized around the tenth theorem-paper target:

> External evidence manifests can be stated and conditionally recovered as
> finite, reproducible, non-promoting evidence-interface structure from the
> closed Paper 9 observed-catalog comparison observable package while
> preserving finite capacity, locality, bounded transfer,
> causal-cone/no-signaling constraints, comparison stability,
> conservation/coarse-graining stability, and compatibility with the closed
> Papers 1 through 9 theorem chain, without importing observed particle
> catalog recovery, physical Standard Model content, physical particle
> excitations, continuum quantum field theory, background Hilbert bundles,
> external matter fields, external gauge fields, physical quantum dynamics,
> simulation-only promotion, fit-only calibration, physical nature promotion,
> or a unified field theory as microscopic premises.

Treat this repo as a downstream Paper 10 workspace. Do not edit the upstream
Paper 1 through Paper 9 repositories from this repo.

## Upstream Boundary

The upstream repositories are:

```text
https://github.com/Unified-Field-Theory-Research/finite-capacity-causal-geometry
https://github.com/Unified-Field-Theory-Research/higher-dimensional-geometry-recovery
https://github.com/Unified-Field-Theory-Research/curvature-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/gravitational-dynamics-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/quantum-compatible-local-dynamics-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/matter-gauge-observables-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/particle-excitation-observables-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/standard-model-candidate-observables-from-finite-capacity-causal-networks
https://github.com/Unified-Field-Theory-Research/observed-catalog-comparison-observables-from-finite-capacity-causal-networks
```

The expected local sibling checkouts are `../finite-capacity-causal-geometry`,
`../higher-dimensional-geometry-recovery`,
`../curvature-from-finite-capacity-causal-networks`,
`../gravitational-dynamics-from-finite-capacity-causal-networks`,
`../quantum-compatible-local-dynamics-from-finite-capacity-causal-networks`,
`../matter-gauge-observables-from-finite-capacity-causal-networks`,
`../particle-excitation-observables-from-finite-capacity-causal-networks`,
`../standard-model-candidate-observables-from-finite-capacity-causal-networks`,
and
`../observed-catalog-comparison-observables-from-finite-capacity-causal-networks`.
Frozen upstream commits are recorded in `UPSTREAM-PAPERS.json`. Any Paper 10
artifact that depends on Papers 1 through 9 must cite the recorded commit or a
later explicitly approved upstream revision. If Paper 10 reveals a defect or
missing definition upstream, record an upstream-revision request instead of
silently modifying the upstream repo.

## Claim Boundary

This repo does not currently prove:

- external evidence manifest recovery;
- observed particle catalog recovery;
- physical Standard Model content;
- physical particle excitations;
- physical matter fields or physical gauge fields;
- physical quantum dynamics;
- continuum quantum field theory;
- physical/nature-level realization of the finite-capacity substrate;
- a unified field theory.

Closed rungs currently include only `EEM-001`: upstream binding through the
closed Paper 9 conditional observed-catalog comparison observables theorem and
a claim-boundary scaffold. External evidence recovery, observed-catalog
recovery, physical Standard Model, physical-particle, physical
quantum-dynamics, continuum-QFT, simulation-only, fit-only, physical-nature,
and unified-field promotion flags remain false. Do not promote this scaffold
to observed particle catalog recovery, physical Standard Model recovery,
physical particle physics, physical quantum dynamics, continuum QFT,
nature-level realization, or unified-field promotion.

## Rust-Only Policy

Runtime, validators, tests, simulations, migration tooling, and proof-gate
helpers must be Rust-native. Do not add Python files, Python notebooks,
Python packaging, Python helper scripts, or Python dependency manifests.

Lean is permitted only as an opt-in proof-certificate lane under `GPD/formal`.
Do not make Lean mandatory for normal Rust tests unless explicitly scoped.

## Rigor Rule

Do not state that Paper 10 is proved until finite external evidence record
manifests, finite reproduction protocol descriptors, Paper 9 comparison
compatibility, evidence-stability/coarse-graining checks, Paper 9 regime
consistency, a no-hidden-physical-promotion/simulation-only/fit-only import
audit, and a final fail-closed certificate close in Lean or an equivalent
fail-closed audit, and the local Rust gates pass. Do not state that nature
uses this substrate unless a separate external physical-promotion track
closes.
