//! `WdlFunctionProcessor` trait — per-function dispatch for `WdlFunctionCallOperation`.
//!
//! Mirrors Java's `WdlFunctionProcessor` interface and `WdlFunctionProcessorBase` abstract class.
//!
//! ## Usage
//!
//! Implement `WdlFunctionProcessor` and override only the per-function hooks you care about.
//! Call `self.process_function_call(call)` to start dispatch; it delegates to
//! `dispatch_by_function` which switches on `call.function`.
//!
//! The Java `WdlFunctionProcessorBase` class (which merely called `dispatchByFunction`) maps
//! directly to the provided `process_function_call` default here — no separate struct needed.

use crate::expressions::{WdlFunction, WdlFunctionCallOperation};

/// Per-function dispatch trait for function-call expressions.
///
/// Mirrors Java's `WdlFunctionProcessor` interface + `WdlFunctionProcessorBase`.
pub trait WdlFunctionProcessor {
    // -----------------------------------------------------------------------
    // Entry point
    // -----------------------------------------------------------------------

    /// Process a function-call expression by dispatching to the per-function hook.
    ///
    /// Default: calls `dispatch_by_function(call)`.
    fn process_function_call(&mut self, call: &WdlFunctionCallOperation) {
        self.dispatch_by_function(call);
    }

    // -----------------------------------------------------------------------
    // Dispatch helper
    // -----------------------------------------------------------------------

    /// Dispatch to the appropriate per-function hook based on `call.function`.
    fn dispatch_by_function(&mut self, call: &WdlFunctionCallOperation) {
        match call.function {
            WdlFunction::Floor => self.process_floor(call),
            WdlFunction::Ceil => self.process_ceil(call),
            WdlFunction::Round => self.process_round(call),
            WdlFunction::Min => self.process_min(call),
            WdlFunction::Max => self.process_max(call),
            WdlFunction::Sub => self.process_sub(call),
            WdlFunction::Stdout => self.process_stdout(call),
            WdlFunction::Stderr => self.process_stderr(call),
            WdlFunction::ReadLines => self.process_read_lines(call),
            WdlFunction::ReadTsv => self.process_read_tsv(call),
            WdlFunction::ReadMap => self.process_read_map(call),
            WdlFunction::ReadObject => self.process_read_object(call),
            WdlFunction::ReadObjects => self.process_read_objects(call),
            WdlFunction::ReadJson => self.process_read_json(call),
            WdlFunction::ReadInt => self.process_read_int(call),
            WdlFunction::ReadFloat => self.process_read_float(call),
            WdlFunction::ReadString => self.process_read_string(call),
            WdlFunction::ReadBoolean => self.process_read_boolean(call),
            WdlFunction::WriteLines => self.process_write_lines(call),
            WdlFunction::WriteTsv => self.process_write_tsv(call),
            WdlFunction::WriteMap => self.process_write_map(call),
            WdlFunction::WriteObject => self.process_write_object(call),
            WdlFunction::WriteObjects => self.process_write_objects(call),
            WdlFunction::WriteJson => self.process_write_json(call),
            WdlFunction::Glob => self.process_glob(call),
            WdlFunction::Size => self.process_size(call),
            WdlFunction::Basename => self.process_basename(call),
            WdlFunction::Prefix => self.process_prefix(call),
            WdlFunction::Suffix => self.process_suffix(call),
            WdlFunction::Quote => self.process_quote(call),
            WdlFunction::Squote => self.process_squote(call),
            WdlFunction::Sep => self.process_sep(call),
            WdlFunction::Length => self.process_length(call),
            WdlFunction::Range => self.process_range(call),
            WdlFunction::Chunk => self.process_chunk(call),
            WdlFunction::Cross => self.process_cross(call),
            WdlFunction::Zip => self.process_zip(call),
            WdlFunction::Unzip => self.process_unzip(call),
            WdlFunction::Transpose => self.process_transpose(call),
            WdlFunction::Flatten => self.process_flatten(call),
            WdlFunction::SelectFirst => self.process_select_first(call),
            WdlFunction::SelectAll => self.process_select_all(call),
            WdlFunction::Contains => self.process_contains(call),
            WdlFunction::ContainsKey => self.process_contains_key(call),
            WdlFunction::Keys => self.process_keys(call),
            WdlFunction::Values => self.process_values(call),
            WdlFunction::AsPairs => self.process_as_pairs(call),
            WdlFunction::AsMap => self.process_as_map(call),
            WdlFunction::CollectByKey => self.process_collect_by_key(call),
            WdlFunction::Matches => self.process_matches(call),
            WdlFunction::Find => self.process_find(call),
            WdlFunction::Defined => self.process_defined(call),
            WdlFunction::JoinPaths => self.process_join_paths(call),
            WdlFunction::Value => self.process_value(call),
            WdlFunction::Nonstandard => self.process_nonstandard(call),
        }
    }

    // -----------------------------------------------------------------------
    // Per-function hooks — all default to no-op
    // -----------------------------------------------------------------------

    fn process_floor(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_ceil(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_round(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_min(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_max(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_sub(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_stdout(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_stderr(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_read_lines(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_read_tsv(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_read_map(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_read_object(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_read_objects(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_read_json(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_read_int(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_read_float(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_read_string(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_read_boolean(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_write_lines(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_write_tsv(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_write_map(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_write_object(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_write_objects(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_write_json(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_glob(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_size(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_basename(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_prefix(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_suffix(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_quote(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_squote(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_sep(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_length(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_range(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_chunk(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_cross(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_zip(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_unzip(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_transpose(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_flatten(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_select_first(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_select_all(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_contains(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_contains_key(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_keys(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_values(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_as_pairs(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_as_map(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_collect_by_key(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_matches(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_find(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_defined(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_join_paths(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_value(&mut self, _call: &WdlFunctionCallOperation) {}
    fn process_nonstandard(&mut self, _call: &WdlFunctionCallOperation) {}
}
