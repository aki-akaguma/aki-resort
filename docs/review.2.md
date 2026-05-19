# Code Review 2: aki-resort

## Overview
This is a follow-up code review for `aki-resort` after the implementation of several optimizations and bug fixes. The project continues to show strong engineering practices, with recent changes successfully addressing previous concerns regarding numeric sorting, memory management, and code consistency.

## Improvements Verified
- **Numeric Sort (f64):** The transition from `i64` to `f64` using `total_cmp` correctly implements the requirements for floating-point sorting, including handling of decimals and special values.
- **Memory Optimization:** Removal of aggressive `shrink_to_fit()` in the main input loop improves throughput by reducing unnecessary reallocations.
- **Code Consistency:** The refactoring of `src/sort/string.rs` to remove unnecessary `Result` wraps has made the code cleaner and more idiomatic.

## New Observations & Recommendations

### 1. Potential Efficiency in `unique` filter (Reverted)
- **Observation:** In `src/run.rs`, the logic for `--unique` currently clones the line when writing it to output.
- **Action:** Attempted to use references, but reverted because the `runnel` library's `write_line` API requires ownership of a `String`.

### 2. String Construction with Capacity (Resolved)
- **Observation:** `make_out_s` in `src/run.rs` constructed strings for colored output without pre-allocating capacity.
- **Action:** Updated to use `String::with_capacity()` to minimize reallocations.

### 3. Regex Key Matching Logic
- **Observation:** In `lines_loop`, if a regex match fails, the entire line is used as the key.
  ```rust
  } else {
      KeyColumns::new(0, line_len)
  }
  ```
- **Discussion:** This behavior is consistent with the requirements, but it's worth noting that "no match" results in the same sorting priority as "match everything". This is generally acceptable for a CLI tool of this type.

### 4. Stability Implementation
- **Strength:** The use of an original index (`num`) in all `SortLine` implementations to ensure stable sorting while benefiting from the performance of `par_sort_unstable_by` is an excellent pattern.

## Conclusion
The project has improved significantly with the recent updates. The code is more efficient and strictly follows the requirements. The remaining recommendations are minor optimizations that would further enhance the tool's performance in high-throughput scenarios.

---
Review Date: 2026-05-19
Reviewer: Gemini CLI Agent
