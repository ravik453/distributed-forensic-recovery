# Project Roadmap

## Phase 0 — Project Definition and Governance

### M0 — Project Definition and Governance

- Define the project purpose and scope.
- Establish the Apache-2.0 license.
- Document lawful-use and authorization requirements.
- Define contribution and security-reporting procedures.
- Document the initial release scope.

### M1 — Repository and Build Foundation

- Establish the Rust workspace.
- Add the initial command-line application.
- Add formatting, linting, and test commands.
- Add continuous integration for Linux, Windows, and macOS.
- Define repository-wide coding conventions.

### M2 — Core Interfaces and Data Model

- Define evidence-source interfaces.
- Define read-only byte-range access.
- Define evidence metadata and provenance structures.
- Define recovery-candidate and recovery-result models.
- Define structured audit events.

## Phase 1 — Evidence Layer and Scanning

### M3 — Evidence Source and Read-Only Access

- Support raw and dd disk images.
- Implement bounded byte-range reads.
- Reject writes to evidence sources.
- Handle truncated and inaccessible evidence safely.

### M4 — Evidence Hashing and Integrity

- Calculate SHA-256 hashes for evidence sources.
- Record evidence size and acquisition metadata.
- Support whole-image and range-based hashing.
- Verify that evidence remains unchanged during analysis.

### M5 — Parallel Raw Scanner

- Scan evidence in bounded blocks.
- Support parallel read-only scanning.
- Record offsets and matching signatures.
- Avoid duplicate or overlapping results.
- Provide deterministic scan output.

## Phase 2 — Filesystem-Aware Recovery

### M6 — Filesystem Detection

- Detect supported partition layouts.
- Detect filesystem types.
- Record filesystem offsets and geometry.
- Reject unsupported or ambiguous layouts safely.

### M7 — NTFS Metadata Parser

- Parse NTFS boot-sector information.
- Locate and interpret the Master File Table.
- Parse file-record metadata.
- Identify deleted-file candidates.
- Preserve record offsets and source provenance.

### M8 — Metadata-Based Recovery

- Recover files using NTFS metadata and data runs.
- Support resident and non-resident data.
- Validate recovered ranges.
- Handle fragmented files where metadata is available.
- Record recovery decisions and errors.

## Phase 3 — Carving and Reconstruction

### M9 — File Signature Detection

- Define supported file signatures.
- Detect file headers and relevant trailers.
- Record candidate offsets and confidence.
- Avoid unsafe unbounded extraction.

### M10 — File Carving

- Implement bounded carving for selected formats.
- Validate carved output.
- Calculate recovered-file hashes.
- Record carving limitations.

### M11 — Fragmented File Reconstruction

- Define reconstruction strategies.
- Use filesystem metadata where available.
- Score reconstruction candidates.
- Reject uncertain results instead of presenting them as verified.

## Phase 4 — Validation, Audit, and Reporting

### M12 — Recovery Validation

- Validate file structure and size.
- Verify hashes and byte ranges.
- Distinguish verified, partial, and uncertain results.
- Add negative and corruption tests.

### M13 — Audit Logging

- Record evidence identity.
- Record tool version and configuration.
- Record scan and recovery operations.
- Record errors, offsets, and decisions.
- Produce deterministic audit output.

### M14 — Reporting

- Generate JSON recovery manifests.
- Generate human-readable summaries.
- Include evidence and recovered-file hashes.
- Include limitations and unresolved candidates.

## Phase 5 — Structured Analysis and Spark

### M15 — Structured Evidence Dataset

- Define a stable schema for evidence findings.
- Export findings to JSON and Parquet.
- Preserve provenance and deterministic identifiers.

### M16 — Spark Analysis

- Read structured findings with Apache Spark.
- Support filtering, grouping, ranking, and aggregation.
- Keep Spark separate from low-level evidence access.

### M17 — Distributed Workflows

- Support large-scale analysis of multiple evidence datasets.
- Define partitioning and reproducibility requirements.
- Document local and cluster execution.

## Phase 6 — Cross-Platform Filesystem Expansion

### M18 — Linux Filesystems

- Add ext4 support.
- Add FAT and exFAT support where practical.
- Add filesystem-specific tests.

### M19 — macOS Filesystems

- Research and define APFS support boundaries.
- Implement read-only parsing incrementally.
- Clearly document unsupported cases.

### M20 — Cross-Platform Compatibility

- Validate builds on Linux, Windows, and macOS.
- Test path, encoding, and filesystem differences.
- Document platform-specific limitations.

## Phase 7 — Performance, Security, and Release

### M21 — Performance

- Benchmark sequential and parallel scanning.
- Measure memory usage.
- Optimize only after correctness is established.

### M22 — Security and Robustness

- Add fuzz testing for parsers.
- Test malformed and adversarial evidence.
- Add resource and size limits.
- Review unsafe code carefully.

### M23 — First Public Release

- Complete documentation.
- Publish supported-platform binaries.
- Publish known limitations.
- Provide reproducible test instructions.
- Avoid unsupported forensic-certification claims.

## Initial Release Scope

The first release should prioritize:

- Raw disk-image reading
- Read-only evidence handling
- SHA-256 hashing
- Evidence scanning
- Partition and filesystem detection
- NTFS metadata parsing
- NTFS metadata-based recovery
- Selected file carving
- Recovered-file hashing
- JSON manifests
- Audit logging
- Linux, Windows, and macOS builds

The following are outside the initial release scope:

- Full APFS recovery
- Advanced fragmented reconstruction
- Spark execution
- A graphical user interface
- Physical-disk acquisition
- Cloud orchestration
- Guaranteed recovery from SSDs
- Forensic certification claims
