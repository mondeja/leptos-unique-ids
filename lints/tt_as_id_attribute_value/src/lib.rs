#![feature(rustc_private)]
#![feature(let_chains)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;

use clippy_utils::diagnostics::span_lint_and_help;
use lints_helpers::{ViewMacroCallIdAttributeValueIter, is_leptos_view_macro_call};
use rustc_ast::{
    token::{LitKind, TokenKind},
    tokenstream::TokenTree,
};
use rustc_lint::{EarlyContext, EarlyLintPass};

const HELP: &str = concat!(
    "for further information visit ",
    "https://github.com/mondeja/leptos-unique-ids/tree/main/lints/tt_as_id_attribute_value#readme"
);
const MESSAGE: &str = "token tree that is not `Ids` enum passed as id attribute value";

dylint_linting::declare_pre_expansion_lint! {
    /// ### What it does
    ///
    /// Check for token trees passed as id attribute values (except for `Ids` enum variants).
    ///
    /// ### Why is this bad?
    ///
    /// Passing `Ids` enum to an id attribute value is the only way to ensure that
    /// `leptos-unique-ids` has to ensure that the id is unique in the DOM. Other token
    /// trees must be avoided to prevent potential issues with duplicate ids in the DOM.
    ///
    /// ### Known problems
    ///
    /// Only checks for tokens in the id attribute values of the `view!` macro.
    /// Currently, it does not check it in Leptos builder syntax.
    ///
    /// ### Example
    ///
    /// ```rust,ignore
    /// let foo = "my-identifier";
    ///
    /// view! {
    ///     <div id=foo>Hello, world!</div>
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
    pub TT_AS_ID_ATTRIBUTE_VALUE,
    Warn,
    "Check for token trees passed as id attribute values (except for `Ids` enum variants)."
}

impl EarlyLintPass for TtAsIdAttributeValue {
    fn check_mac(&mut self, cx: &EarlyContext, macro_call: &rustc_ast::MacCall) {
        if !is_leptos_view_macro_call(macro_call) {
            return;
        }
        for tt in ViewMacroCallIdAttributeValueIter::new(macro_call) {
            if let TokenTree::Token(token, _) = tt {
                if let TokenKind::Ident(symbol, _) = token.kind {
                    if symbol.as_str() == "Ids" {
                        continue;
                    }
                } else if let TokenKind::Literal(lit) = token.kind
                    && lit.kind == LitKind::Str
                {
                    // this case is catched by `literal_as_id_attribute_value` lint
                    continue;
                }
                span_lint_and_help(
                    cx,
                    TT_AS_ID_ATTRIBUTE_VALUE,
                    token.span,
                    MESSAGE,
                    None,
                    HELP,
                );
            } else if let TokenTree::Delimited(delim_span, ..) = tt {
                span_lint_and_help(
                    cx,
                    TT_AS_ID_ATTRIBUTE_VALUE,
                    delim_span.entire(),
                    MESSAGE,
                    None,
                    HELP,
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
