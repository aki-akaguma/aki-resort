# Code Review: aki-resort

## Overview
`aki-resort` is a well-engineered Rust CLI tool for sorting text lines using regex-based keys. The project demonstrates high technical standards, particularly in performance optimization and error handling.

## Strengths
- **Parallel Processing:** Effective use of `rayon` for parallel sorting significantly improves performance on multi-core systems.
- **Stable Sorting:** The implementation correctly achieves stable sorting while using high-performance unstable sort by augmenting data with original indices.
- **Architectural Clarity:** The use of the `SortLinesBuffer` trait allows for clean extension of sorting algorithms (e.g., month, version, time).
- **Robust Error Handling:** Informative error messages with context and graceful handling of broken pipes (common in CLI filters) enhance user experience.
- **Memory Safety:** The inclusion of `--max-buffer` prevents potential OOM (Out Of Memory) issues when processing very large files.

## Identified Issues & Recommendations

### 1. Discrepancy in Numeric Sort Requirements
- **Issue:** `specs/0.requirements.md` specifies that the `numeric` sort should interpret keys as floating-point numbers. However, `src/sort/numeric.rs` parses keys as `i64`.
- **Recommendation:** Update `src/sort/numeric.rs` to use `f64` for parsing and comparison (considering NaN/Infinity cases) to align with requirements.

### 2. Aggressive Memory Management (Resolved)
- **Observation:** `src/run.rs` previously called `shrink_to_fit()` on every line read.
- **Action:** Removed `shrink_to_fit()` to reduce reallocation overhead and improve processing speed, especially for large numbers of small lines.

### 3. Niche Argument Parser
- **Observation:** The project uses `flood-tide` and `xtask` for argument parsing.
- **Impact:** This approach involves code generation and `include!` macros, which can be less intuitive for new contributors compared to standard libraries like `clap`.
- **Recommendation:** Ensure the `xtask` process is well-documented in a developer guide if one exists.

### 4. Code Consistency (Resolved)
- **Observation:** Some modules (like `src/sort/string.rs`) previously used `#[allow(clippy::unnecessary_wraps)]`.
- **Action:** Refactored `src/sort/string.rs` to return `Self` directly instead of `Result<Self>`, following Clippy's recommendation.

## Conclusion
The codebase is of high quality and follows modern Rust idioms. Addressing the numeric sort discrepancy and reviewing the memory management strategy would further polish the tool.

---
Review Date: 2026-05-19
Reviewer: Gemini CLI Agent
