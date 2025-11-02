# RAM Monitor Constitution

## Core Principles

### I. Performance
The application MUST maintain a performance footprint of less than 0.5% CPU usage, a memory footprint below 5MB, and achieve sub-1ms polling intervals. These constraints ensure the tool remains efficient and does not interfere with system performance.

### II. Lightweight
The binary size MUST be under 2MB, delivered as a single executable with no external dependencies. This guarantees ease of distribution and minimal system impact.

### III. Native Implementation
The application MUST use the Win32 API directly, avoiding any runtimes or frameworks. This ensures compatibility and performance on Windows 10 and later systems.

### IV. Tech Stack
The application MUST be developed using Rust (stable) with the `windows-rs` crate and minimal additional dependencies. This choice balances modern development practices with the need for a lightweight and efficient implementation.

### V. Platform Constraints
The application MUST target Windows 10 and later exclusively, with no cross-platform support or historical data storage. This focus allows for optimized performance and simplicity.

## Additional Constraints

### Technology Stack Requirements
- Use Rust stable version.
- Employ the `windows-rs` crate for Win32 API interactions.
- Avoid any runtime or framework dependencies.

### Compliance Standards
- Ensure compatibility with Windows 10 and later.
- Maintain a binary size under 2MB.

## Development Workflow

### Review Process
- All code changes MUST undergo peer review to ensure compliance with the principles.
- Performance benchmarks MUST be included in pull requests.

### Quality Gates
- Automated tests MUST validate performance metrics (CPU, memory, polling).
- Manual testing MUST confirm binary size and dependency constraints.

## Governance

The constitution supersedes all other practices. Amendments require documentation, approval, and a migration plan. All pull requests and reviews MUST verify compliance with the principles outlined in this document.

**Version**: 1.0.0 | **Ratified**: 2025-11-02 | **Last Amended**: 2025-11-02
