# Code Review Report for aki-resort

## Overview
The `aki-resort` project is a CLI tool for sorting text lines with regex-based key extraction and various sorting modes (month, numeric, string, time, version). It leverages `rayon` for parallel sorting and `regex` for flexible key matching. The architecture is well-structured with a clear separation between configuration, execution, and sorting logic.

## Review Findings

### 1. Bug in Time Parsing (Fractional Seconds)
**File:** `src/sort/time.rs`
**Observation:** The `make_time` function incorrectly parses fractional seconds. For example, `.5` is parsed as 5 milliseconds instead of 500 milliseconds. This is because it directly parses the fractional part as a `u64` and passes it to `Duration::from_millis()`.
**Impact:** Incorrect sorting of time values with varying precision in the fractional part (e.g., `.1` vs `.01` vs `.001` all being treated as 1ms).
**Recommendation:** Adjust the fractional part based on the number of digits or convert to nanoseconds (e.g., `.5` -> 5 * 10^(9-1) nanoseconds).

### 2. Unexpected Time Parsing Behavior (Minutes vs. Seconds)
**File:** `src/sort/time.rs`
**Observation:** In `make_time`, if no colons are present in the time string (e.g., "20.5"), the logic skips the "seconds" parsing and treats the value as "minutes". Most users would expect a single numeric value to represent seconds.
**Impact:** Counter-intuitive sorting for simple numeric time strings.
**Recommendation:** Re-evaluate the parsing logic to prioritize seconds when fewer components are provided.

### 3. Potential Panic in Tail Processing
**File:** `src/run.rs`
**Observation:** In `lines_loop`, the calculation `let at = buf_lines.len() - n;` for the `--tail` option does not check if `buf_lines.len()` is greater than or equal to `n`.
**Impact:** If the number of lines in the input is less than the value specified by `--tail`, the program will panic due to `usize` underflow.
**Recommendation:** Use `buf_lines.len().saturating_sub(n)` or add an explicit check.

### 4. Inconsistent Sorting Behavior with Regex
**File:** `src/run.rs`
**Observation:** If a regex is provided via `-e` but no matches are found in any lines, sorting is skipped entirely (`match_count == 0`). However, if at least one line matches, ALL lines (including non-matching ones) are sorted using the whole line as a fallback key.
**Impact:** Inconsistent behavior depending on whether a match occurs.
**Recommendation:** Consider always sorting if a sort is requested, or clearly define the behavior when no matches occur.

### 5. Unique Flag Skips First Empty Line
**File:** `src/run.rs`
**Observation:** The `unique` flag logic initializes `pre_line` as an empty string (`String::new()`). If the first line in the sorted output is also an empty string, it will be skipped because `pre_line != key_line.line` will be false.
**Impact:** Loss of an empty line at the beginning of the output when `--unique` is used.
**Recommendation:** Use `Option<String>` for `pre_line` and initialize it as `None`.

### 6. Memory Efficiency
**File:** `src/run.rs`, `src/sort/*.rs`
**Observation:** The tool reads all input lines into memory as `String` objects before sorting. While `opt_max_buffer` provides a safeguard, this approach limits the tool's ability to handle very large files that exceed available RAM.
**Impact:** High memory usage for large inputs.
**Recommendation:** For a tool of this nature, this is often acceptable, but if performance for multi-gigabyte files is required, consider a multi-pass or external sort approach.

### 7. Stable Parallel Sort Implementation
**File:** `src/sort/*.rs`
**Observation:** The use of `rayon`'s `par_sort_unstable_by` combined with an explicit original index `num` to ensure stability is an excellent way to achieve high-performance stable sorting.
**Recommendation:** None. This is a good practice.

### 8. Strict Error Handling on Invalid Keys
**File:** `src/sort/*.rs`
**Observation:** The sorting buffers for month, numeric, version, and time return an error if a key cannot be parsed. This causes the entire program to exit. Standard Unix `sort` usually handles invalid values by placing them at the beginning or end.
**Impact:** Brittle behavior when processing files with inconsistent data.
**Recommendation:** Consider a "lenient" mode or default behavior that treats unparseable keys as a specific category rather than a fatal error.

## Conclusion
The codebase is well-written and follows idiomatic Rust patterns in many areas. The use of traits for sorting strategies is particularly commendable. Addressing the identified bugs in time parsing and the potential panic in tail processing will significantly improve the tool's reliability.

---
Review Date: 2026-06-01
Reviewer: Gemini CLI Agent
