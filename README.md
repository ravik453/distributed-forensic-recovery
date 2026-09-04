# Distributed Forensic Recovery

A cross-platform forensic deleted-file recovery platform designed for safe evidence handling, filesystem-aware recovery, file carving, integrity verification, and large-scale evidence analysis.

## Project Status

This project is in the initial planning and foundation stage.

The first release will focus on:

- Read-only disk-image access
- Evidence hashing and integrity verification
- Raw evidence scanning
- Partition and filesystem detection
- NTFS metadata parsing
- Metadata-based deleted-file recovery
- Selected file-carving capabilities
- Recovered-file hashing
- JSON recovery manifests
- Audit logging
- Cross-platform builds for Linux, Windows, and macOS

## Design Principles

- Preserve forensic integrity.
- Never modify the original evidence.
- Make all recovery operations reproducible.
- Record hashes, offsets, decisions, and errors.
- Separate low-level evidence handling from distributed analytics.
- Prefer explicit validation over assumptions.
- Fail safely when evidence is incomplete or ambiguous.

## Planned Architecture

The project will use a hybrid architecture:

- Rust for evidence access, filesystem parsing, recovery, validation, hashing, and audit generation.
- Apache Spark for large-scale structured evidence analysis, filtering, grouping, ranking, and aggregation.

Spark will not directly control physical-disk access or low-level file reconstruction.

## Repository Documentation

- [Roadmap](ROADMAP.md)
- [Architecture](ARCHITECTURE.md)
- [Contributing](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)

## Important Notice

This project is intended for lawful forensic, investigative, recovery, research, and educational use.

Always obtain appropriate authorization before examining or recovering data from a device, disk image, or other evidence source.
