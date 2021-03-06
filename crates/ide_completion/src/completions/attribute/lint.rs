//! Completion for lints
use syntax::ast;

use crate::{
    context::CompletionContext,
    item::{CompletionItem, CompletionItemKind, CompletionKind},
    Completions,
};

pub(super) fn complete_lint(
    acc: &mut Completions,
    ctx: &CompletionContext,
    derive_input: ast::TokenTree,
    lints_completions: &[LintCompletion],
) {
    if let Some(existing_lints) = super::parse_comma_sep_input(derive_input) {
        for lint_completion in lints_completions
            .into_iter()
            .filter(|completion| !existing_lints.contains(completion.label))
        {
            let mut item = CompletionItem::new(
                CompletionKind::Attribute,
                ctx.source_range(),
                lint_completion.label,
            );
            item.kind(CompletionItemKind::Attribute).detail(lint_completion.description);
            item.add_to(acc)
        }
    }
}

pub(crate) struct LintCompletion {
    pub(crate) label: &'static str,
    pub(crate) description: &'static str,
}

#[rustfmt::skip]
pub(super) const DEFAULT_LINT_COMPLETIONS: &[LintCompletion] = &[
    LintCompletion { label: "absolute_paths_not_starting_with_crate", description: r#"fully qualified paths that start with a module name instead of `crate`, `self`, or an extern crate name"# },
    LintCompletion { label: "anonymous_parameters", description: r#"detects anonymous parameters"# },
    LintCompletion { label: "box_pointers", description: r#"use of owned (Box type) heap memory"# },
    LintCompletion { label: "deprecated_in_future", description: r#"detects use of items that will be deprecated in a future version"# },
    LintCompletion { label: "elided_lifetimes_in_paths", description: r#"hidden lifetime parameters in types are deprecated"# },
    LintCompletion { label: "explicit_outlives_requirements", description: r#"outlives requirements can be inferred"# },
    LintCompletion { label: "indirect_structural_match", description: r#"pattern with const indirectly referencing non-structural-match type"# },
    LintCompletion { label: "keyword_idents", description: r#"detects edition keywords being used as an identifier"# },
    LintCompletion { label: "macro_use_extern_crate", description: r#"the `#[macro_use]` attribute is now deprecated in favor of using macros via the module system"# },
    LintCompletion { label: "meta_variable_misuse", description: r#"possible meta-variable misuse at macro definition"# },
    LintCompletion { label: "missing_copy_implementations", description: r#"detects potentially-forgotten implementations of `Copy`"# },
    LintCompletion { label: "missing_crate_level_docs", description: r#"detects crates with no crate-level documentation"# },
    LintCompletion { label: "missing_debug_implementations", description: r#"detects missing implementations of Debug"# },
    LintCompletion { label: "missing_docs", description: r#"detects missing documentation for public members"# },
    LintCompletion { label: "missing_doc_code_examples", description: r#"detects publicly-exported items without code samples in their documentation"# },
    LintCompletion { label: "non_ascii_idents", description: r#"detects non-ASCII identifiers"# },
    LintCompletion { label: "private_doc_tests", description: r#"detects code samples in docs of private items not documented by rustdoc"# },
    LintCompletion { label: "single_use_lifetimes", description: r#"detects lifetime parameters that are only used once"# },
    LintCompletion { label: "trivial_casts", description: r#"detects trivial casts which could be removed"# },
    LintCompletion { label: "trivial_numeric_casts", description: r#"detects trivial casts of numeric types which could be removed"# },
    LintCompletion { label: "unaligned_references", description: r#"detects unaligned references to fields of packed structs"# },
    LintCompletion { label: "unreachable_pub", description: r#"`pub` items not reachable from crate root"# },
    LintCompletion { label: "unsafe_code", description: r#"usage of `unsafe` code"# },
    LintCompletion { label: "unsafe_op_in_unsafe_fn", description: r#"unsafe operations in unsafe functions without an explicit unsafe block are deprecated"# },
    LintCompletion { label: "unstable_features", description: r#"enabling unstable features (deprecated. do not use)"# },
    LintCompletion { label: "unused_crate_dependencies", description: r#"crate dependencies that are never used"# },
    LintCompletion { label: "unused_extern_crates", description: r#"extern crates that are never used"# },
    LintCompletion { label: "unused_import_braces", description: r#"unnecessary braces around an imported item"# },
    LintCompletion { label: "unused_lifetimes", description: r#"detects lifetime parameters that are never used"# },
    LintCompletion { label: "unused_qualifications", description: r#"detects unnecessarily qualified names"# },
    LintCompletion { label: "unused_results", description: r#"unused result of an expression in a statement"# },
    LintCompletion { label: "variant_size_differences", description: r#"detects enums with widely varying variant sizes"# },
    LintCompletion { label: "array_into_iter", description: r#"detects calling `into_iter` on arrays"# },
    LintCompletion { label: "asm_sub_register", description: r#"using only a subset of a register for inline asm inputs"# },
    LintCompletion { label: "bare_trait_objects", description: r#"suggest using `dyn Trait` for trait objects"# },
    LintCompletion { label: "bindings_with_variant_name", description: r#"detects pattern bindings with the same name as one of the matched variants"# },
    LintCompletion { label: "cenum_impl_drop_cast", description: r#"a C-like enum implementing Drop is cast"# },
    LintCompletion { label: "clashing_extern_declarations", description: r#"detects when an extern fn has been declared with the same name but different types"# },
    LintCompletion { label: "coherence_leak_check", description: r#"distinct impls distinguished only by the leak-check code"# },
    LintCompletion { label: "confusable_idents", description: r#"detects visually confusable pairs between identifiers"# },
    LintCompletion { label: "dead_code", description: r#"detect unused, unexported items"# },
    LintCompletion { label: "deprecated", description: r#"detects use of deprecated items"# },
    LintCompletion { label: "ellipsis_inclusive_range_patterns", description: r#"`...` range patterns are deprecated"# },
    LintCompletion { label: "exported_private_dependencies", description: r#"public interface leaks type from a private dependency"# },
    LintCompletion { label: "illegal_floating_point_literal_pattern", description: r#"floating-point literals cannot be used in patterns"# },
    LintCompletion { label: "improper_ctypes", description: r#"proper use of libc types in foreign modules"# },
    LintCompletion { label: "improper_ctypes_definitions", description: r#"proper use of libc types in foreign item definitions"# },
    LintCompletion { label: "incomplete_features", description: r#"incomplete features that may function improperly in some or all cases"# },
    LintCompletion { label: "inline_no_sanitize", description: r#"detects incompatible use of `#[inline(always)]` and `#[no_sanitize(...)]`"# },
    LintCompletion { label: "intra_doc_link_resolution_failure", description: r#"failures in resolving intra-doc link targets"# },
    LintCompletion { label: "invalid_codeblock_attributes", description: r#"codeblock attribute looks a lot like a known one"# },
    LintCompletion { label: "invalid_value", description: r#"an invalid value is being created (such as a NULL reference)"# },
    LintCompletion { label: "irrefutable_let_patterns", description: r#"detects irrefutable patterns in if-let and while-let statements"# },
    LintCompletion { label: "late_bound_lifetime_arguments", description: r#"detects generic lifetime arguments in path segments with late bound lifetime parameters"# },
    LintCompletion { label: "mixed_script_confusables", description: r#"detects Unicode scripts whose mixed script confusables codepoints are solely used"# },
    LintCompletion { label: "mutable_borrow_reservation_conflict", description: r#"reservation of a two-phased borrow conflicts with other shared borrows"# },
    LintCompletion { label: "non_camel_case_types", description: r#"types, variants, traits and type parameters should have camel case names"# },
    LintCompletion { label: "non_shorthand_field_patterns", description: r#"using `Struct { x: x }` instead of `Struct { x }` in a pattern"# },
    LintCompletion { label: "non_snake_case", description: r#"variables, methods, functions, lifetime parameters and modules should have snake case names"# },
    LintCompletion { label: "non_upper_case_globals", description: r#"static constants should have uppercase identifiers"# },
    LintCompletion { label: "no_mangle_generic_items", description: r#"generic items must be mangled"# },
    LintCompletion { label: "overlapping_patterns", description: r#"detects overlapping patterns"# },
    LintCompletion { label: "path_statements", description: r#"path statements with no effect"# },
    LintCompletion { label: "private_in_public", description: r#"detect private items in public interfaces not caught by the old implementation"# },
    LintCompletion { label: "proc_macro_derive_resolution_fallback", description: r#"detects proc macro derives using inaccessible names from parent modules"# },
    LintCompletion { label: "redundant_semicolons", description: r#"detects unnecessary trailing semicolons"# },
    LintCompletion { label: "renamed_and_removed_lints", description: r#"lints that have been renamed or removed"# },
    LintCompletion { label: "safe_packed_borrows", description: r#"safe borrows of fields of packed structs were erroneously allowed"# },
    LintCompletion { label: "stable_features", description: r#"stable features found in `#[feature]` directive"# },
    LintCompletion { label: "trivial_bounds", description: r#"these bounds don't depend on an type parameters"# },
    LintCompletion { label: "type_alias_bounds", description: r#"bounds in type aliases are not enforced"# },
    LintCompletion { label: "tyvar_behind_raw_pointer", description: r#"raw pointer to an inference variable"# },
    LintCompletion { label: "uncommon_codepoints", description: r#"detects uncommon Unicode codepoints in identifiers"# },
    LintCompletion { label: "unconditional_recursion", description: r#"functions that cannot return without calling themselves"# },
    LintCompletion { label: "unknown_lints", description: r#"unrecognized lint attribute"# },
    LintCompletion { label: "unnameable_test_items", description: r#"detects an item that cannot be named being marked as `#[test_case]`"# },
    LintCompletion { label: "unreachable_code", description: r#"detects unreachable code paths"# },
    LintCompletion { label: "unreachable_patterns", description: r#"detects unreachable patterns"# },
    LintCompletion { label: "unstable_name_collisions", description: r#"detects name collision with an existing but unstable method"# },
    LintCompletion { label: "unused_allocation", description: r#"detects unnecessary allocations that can be eliminated"# },
    LintCompletion { label: "unused_assignments", description: r#"detect assignments that will never be read"# },
    LintCompletion { label: "unused_attributes", description: r#"detects attributes that were not used by the compiler"# },
    LintCompletion { label: "unused_braces", description: r#"unnecessary braces around an expression"# },
    LintCompletion { label: "unused_comparisons", description: r#"comparisons made useless by limits of the types involved"# },
    LintCompletion { label: "unused_doc_comments", description: r#"detects doc comments that aren't used by rustdoc"# },
    LintCompletion { label: "unused_features", description: r#"unused features found in crate-level `#[feature]` directives"# },
    LintCompletion { label: "unused_imports", description: r#"imports that are never used"# },
    LintCompletion { label: "unused_labels", description: r#"detects labels that are never used"# },
    LintCompletion { label: "unused_macros", description: r#"detects macros that were not used"# },
    LintCompletion { label: "unused_must_use", description: r#"unused result of a type flagged as `#[must_use]`"# },
    LintCompletion { label: "unused_mut", description: r#"detect mut variables which don't need to be mutable"# },
    LintCompletion { label: "unused_parens", description: r#"`if`, `match`, `while` and `return` do not need parentheses"# },
    LintCompletion { label: "unused_unsafe", description: r#"unnecessary use of an `unsafe` block"# },
    LintCompletion { label: "unused_variables", description: r#"detect variables which are not used in any way"# },
    LintCompletion { label: "warnings", description: r#"mass-change the level for lints which produce warnings"# },
    LintCompletion { label: "where_clauses_object_safety", description: r#"checks the object safety of where clauses"# },
    LintCompletion { label: "while_true", description: r#"suggest using `loop { }` instead of `while true { }`"# },
    LintCompletion { label: "ambiguous_associated_items", description: r#"ambiguous associated items"# },
    LintCompletion { label: "arithmetic_overflow", description: r#"arithmetic operation overflows"# },
    LintCompletion { label: "conflicting_repr_hints", description: r#"conflicts between `#[repr(..)]` hints that were previously accepted and used in practice"# },
    LintCompletion { label: "const_err", description: r#"constant evaluation detected erroneous expression"# },
    LintCompletion { label: "ill_formed_attribute_input", description: r#"ill-formed attribute inputs that were previously accepted and used in practice"# },
    LintCompletion { label: "incomplete_include", description: r#"trailing content in included file"# },
    LintCompletion { label: "invalid_type_param_default", description: r#"type parameter default erroneously allowed in invalid location"# },
    LintCompletion { label: "macro_expanded_macro_exports_accessed_by_absolute_paths", description: r#"macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths"# },
    LintCompletion { label: "missing_fragment_specifier", description: r#"detects missing fragment specifiers in unused `macro_rules!` patterns"# },
    LintCompletion { label: "mutable_transmutes", description: r#"mutating transmuted &mut T from &T may cause undefined behavior"# },
    LintCompletion { label: "no_mangle_const_items", description: r#"const items will not have their symbols exported"# },
    LintCompletion { label: "order_dependent_trait_objects", description: r#"trait-object types were treated as different depending on marker-trait order"# },
    LintCompletion { label: "overflowing_literals", description: r#"literal out of range for its type"# },
    LintCompletion { label: "patterns_in_fns_without_body", description: r#"patterns in functions without body were erroneously allowed"# },
    LintCompletion { label: "pub_use_of_private_extern_crate", description: r#"detect public re-exports of private extern crates"# },
    LintCompletion { label: "soft_unstable", description: r#"a feature gate that doesn't break dependent crates"# },
    LintCompletion { label: "unconditional_panic", description: r#"operation will cause a panic at runtime"# },
    LintCompletion { label: "unknown_crate_types", description: r#"unknown crate type found in `#[crate_type]` directive"# },
];

#[cfg(test)]
mod tests {

    use crate::test_utils::check_edit;

    #[test]
    fn check_empty() {
        check_edit(
            "deprecated",
            r#"#[allow($0)] struct Test;"#,
            r#"#[allow(deprecated)] struct Test;"#,
        )
    }

    #[test]
    fn check_with_existing() {
        check_edit(
            "deprecated",
            r#"#[allow(keyword_idents, $0)] struct Test;"#,
            r#"#[allow(keyword_idents, deprecated)] struct Test;"#,
        )
    }

    #[test]
    fn check_qualified() {
        check_edit(
            "deprecated",
            r#"#[allow(keyword_idents, $0)] struct Test;"#,
            r#"#[allow(keyword_idents, deprecated)] struct Test;"#,
        )
    }
}
