# Architecture

## Purpose

Distributed Forensic Recovery is designed as a read-only forensic analysis and deleted-file recovery platform.

The architecture separates low-level evidence handling from higher-level distributed analysis. Raw evidence access, filesystem parsing, and file reconstruction require strict control over byte ranges, offsets, validation, and provenance.

## High-Level Architecture

```text
Evidence Source
      |
      v
Evidence Access Layer
      |
      v
Integrity and Hashing
      |
      v
Filesystem Detection
      |
      +----------------------+
      |                      |
      v                      v
Filesystem Metadata      Raw Scanner
Parsing                      |
      |                      v
      |                  File Signatures
      |                      |
      +----------+-----------+
                 |
                 v
        Recovery Candidates
                 |
                 v
       Recovery and Reconstruction
                 |
                 v
          Validation Layer
                 |
                 v
       Hashes and Audit Records
                 |
                 v
       JSON / Parquet Outputs
                 |
                 v
       Optional Spark Analysis
```

## Core Components

### 1. Evidence Access Layer

The evidence access layer provides controlled, read-only access to:

* Raw disk images
* dd images
* Partition images
* Other supported evidence containers

Responsibilities include:

* Opening evidence sources safely
* Reading bounded byte ranges
* Reporting evidence size
* Rejecting write operations
* Handling truncated or inaccessible sources
* Preserving source identity and provenance

This layer must not silently modify evidence.

### 2. Integrity and Hashing

The integrity layer calculates and records:

* Evidence size
* SHA-256 evidence hash
* Range-based hashes where required
* Recovered-file hashes
* Tool and configuration metadata

Hashes must be calculated from the actual bytes read from the evidence source.

### 3. Filesystem Detection

The filesystem detection layer identifies supported partition and filesystem structures.

Responsibilities include:

* Partition-layout detection
* Filesystem-type detection
* Filesystem offset detection
* Filesystem geometry extraction
* Unsupported-layout reporting

Detection results must include confidence, offsets, and provenance.

### 4. Filesystem Metadata Parsing

Filesystem parsers interpret metadata without changing the source.

The first filesystem target is NTFS.

The NTFS parser will eventually support:

* Boot-sector information
* Master File Table location
* File-record parsing
* File names
* File attributes
* Allocation status
* Data runs
* Deleted-file candidates

Additional filesystems will be implemented incrementally.

### 5. Raw Scanner

The raw scanner searches evidence for known byte patterns and file signatures.

Responsibilities include:

* Bounded block scanning
* Parallel read-only processing
* Signature detection
* Candidate offset reporting
* Duplicate and overlap handling
* Deterministic results

The scanner must not assume that every signature represents a valid file.

### 6. Recovery and Reconstruction

The recovery layer reconstructs files using available evidence.

Possible sources include:

* Filesystem metadata
* Data runs
* File signatures
* File trailers
* Fragmentation evidence
* Validation results

Recovery results must distinguish between:

* Verified recovery
* Partial recovery
* Uncertain candidate
* Failed recovery

Uncertain results must not be presented as verified files.

### 7. Validation Layer

The validation layer checks recovered output before it is reported as usable.

Validation may include:

* Expected file size
* File-format structure
* Byte-range validity
* Hash calculation
* Metadata consistency
* Completeness checks

Validation failures must be recorded rather than hidden.

### 8. Audit and Reporting

Every meaningful operation should produce structured audit information.

Audit records may include:

* Evidence identifier
* Evidence hash
* Tool version
* Configuration
* Operation name
* Source offset
* Byte length
* Candidate identifier
* Recovery status
* Validation result
* Error details
* Output hash

The project should support deterministic JSON manifests and structured datasets.

### 9. Spark Analysis Layer

Apache Spark is intended for large-scale analysis of structured outputs.

Spark may be used for:

* Filtering findings
* Grouping candidates
* Ranking results
* Aggregating evidence
* Comparing datasets
* Processing multiple evidence reports

Spark should not directly perform:

* Physical-disk access
* Raw device acquisition
* Low-level filesystem parsing
* Unbounded byte reads
* File reconstruction
* Evidence modification

The low-level recovery engine should produce structured records that Spark can consume.

## Language and Technology Boundaries

### Rust

Rust is intended for:

* Evidence access
* Byte-range reads
* Filesystem parsing
* File recovery
* File carving
* Hashing
* Validation
* Audit generation
* Command-line interfaces

Rust is selected for memory safety, performance, portability, and control over binary data.

### Apache Spark

Apache Spark is intended for:

* Structured evidence analysis
* Large-scale filtering
* Aggregation
* Ranking
* Dataset comparison
* Distributed processing of generated outputs

Spark is not the primary recovery engine.

## Read-Only and Forensic Integrity Requirements

The architecture must enforce the following requirements:

1. Original evidence must never be modified.
2. Evidence access must be read-only.
3. All important offsets must be recorded.
4. All recovered outputs must be hashed.
5. Recovery decisions must be reproducible.
6. Errors and limitations must be visible.
7. Unsupported structures must be reported explicitly.
8. The system must not claim recovery where validation is incomplete.
9. Resource limits must prevent uncontrolled memory or disk usage.
10. Test evidence must be separated from real investigative evidence.

## Cross-Platform Goals

The project is intended to support:

* Linux
* Windows
* macOS

The implementation must account for differences in:

* File paths
* Permissions
* Device access
* Line endings
* Character encoding
* Filesystem availability
* Native dependencies

The first implementation should prioritize portable disk-image analysis rather than direct physical-device access.

## Initial Non-Goals

The initial architecture does not promise:

* Guaranteed recovery from SSDs
* Full support for every filesystem
* Full APFS recovery
* Perfect fragmented-file reconstruction
* Physical-disk acquisition
* A graphical interface
* Cloud orchestration
* Distributed raw-device access
* Forensic certification

## Architectural Rule

Correctness, evidence integrity, and reproducibility take priority over speed and feature count.
