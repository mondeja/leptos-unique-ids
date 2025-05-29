#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;

use clippy_utils::diagnostics::span_lint_and_help;
use lints_helpers::{ViewMacroCallIdAttributeValueIter, is_leptos_view_macro_call};
use rustc_ast::token::{LitKind, TokenKind};
use rustc_lint::{EarlyContext, EarlyLintPass};

dylint_linting::declare_pre_expansion_lint! {
    /// ### What it does
    ///
    /// Check for literals passed to id attribute values.
    ///
    /// ### Why is this bad?
    ///
    /// Passing a literal to an id attribute value can lead to issues with duplicate ids
    /// in the DOM, which can cause unexpected behavior in the application. It is
    /// recommended to use leptos-uniques-ids crate to generate unique ids instead.
    ///
    /// ### Known problems
    ///
    /// Only checks for literals in the id attribute values of the `view!` macro.
    /// Currently, it does not check it in Leptos builder syntax.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// view! {
    ///     <div id="my-identifier">Hello, world!</div>
    /// }
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust,ignore
    /// use ids::Ids;
    ///
    /// view! {
    ///     <div id=Ids::MyIdentifier>Hello, world!</div>
    /// }
    /// ```
    pub LITERAL_AS_ID_ATTRIBUTE_VALUE,
    Warn,
    "Check for literals passed to id attribute values."
}

impl EarlyLintPass for LiteralAsIdAttributeValue {
    fn check_mac(&mut self, cx: &EarlyContext, macro_call: &rustc_ast::MacCall) {
        if !is_leptos_view_macro_call(macro_call) {
            return;
        }
        for token in ViewMacroCallIdAttributeValueIter::new(macro_call) {
            if let TokenKind::Literal(lit) = token.kind
                && lit.kind == LitKind::Str
            {
                span_lint_and_help(
                    cx,
                    LITERAL_AS_ID_ATTRIBUTE_VALUE,
                    token.span,
                    "literal string passed as id attribute value",
                    None,
                    "for further information visit https://github.com/mondeja/leptos-unique-ids/tree/main/lints/literal_as_id_attribute_value#readme",
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
    }
}
