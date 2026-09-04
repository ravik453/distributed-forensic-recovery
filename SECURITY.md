# Security Policy

## Scope

Distributed Forensic Recovery processes disk images and other evidence sources. Security and forensic integrity are essential project requirements.

## Supported Versions

During the initial development stage, security fixes will be applied to the latest version on the default development branch.

## Reporting a Vulnerability

Please do not publicly disclose security vulnerabilities before the project has had an opportunity to investigate them.

Report security issues privately through the repository's configured security contact or private vulnerability-reporting mechanism.

If no private reporting mechanism is available, open a minimal issue without including exploit details or sensitive evidence, and request a private communication channel.

## Do Not Share Sensitive Evidence

Never include the following in public issues, pull requests, logs, or discussions:

- Real forensic images

- Personal data

- Confidential documents

- Credentials

- Access tokens

- Private keys

- Identifying information

- Unreleased investigative material

Use synthetic or sanitized test evidence instead.

## Security Expectations

Contributors should consider:

- Malformed disk images

- Truncated files

- Integer overflow

- Out-of-bounds reads

- Unbounded memory allocation

- Path traversal

- Unsafe output paths

- Malicious filenames

- Decompression or parser abuse

- Denial-of-service inputs

- Accidental evidence modification

- Insecure temporary files

- Sensitive data leakage through logs

## Forensic Integrity

The project must preserve the original evidence source. Implementations must not silently write to, alter, or repair the source being analyzed.

All recovered outputs and important processing decisions should be traceable through hashes, offsets, validation results, and audit records.

## Security Fixes

Security fixes should include:

- A clear explanation of the issue

- A reproducible test where possible

- The smallest practical change

- Documentation when user behavior or configuration is affected

- Review of related code paths
