#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
#[allow(unused_extern_crates)]
extern crate rustc_driver;

use rustc_ast::{
    MacCall,
    token::TokenKind,
    tokenstream::{TokenStreamIter, TokenTree},
};

/// Given a macro call, return if is a `view!` macro
pub fn is_leptos_view_macro_call(macro_call: &MacCall) -> bool {
    macro_call
        .path
        .segments
        .iter()
        .last()
        .map_or(false, |segment| segment.ident.name.as_str() == "view")
}

/// Iterator for id attribute values in macro calls
pub struct ViewMacroCallIdAttributeValueIter<'a> {
    iter: TokenStreamIter<'a>,
    // 1: Initial
    // 2: Inside id attribute
    // 4: Inside id attribute value
    parser_state: u8,
}

impl<'a> ViewMacroCallIdAttributeValueIter<'a> {
    pub fn new(macro_call: &'a MacCall) -> Self {
        Self {
            iter: macro_call.args.tokens.iter(),
            parser_state: 1,
        }
    }
}

impl<'a> Iterator for ViewMacroCallIdAttributeValueIter<'a> {
    type Item = &'a TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.iter.next()?;
        if self.parser_state == 1 {
            if let TokenTree::Token(token, _) = token {
                if let TokenKind::Ident(symbol, _) = token.kind {
                    if symbol.as_str() == "id" {
                        self.parser_state <<= 1;
                        return self.next();
                    }
                }
            }
            self.next()
        } else if self.parser_state == 2 {
            if let TokenTree::Token(token, _) = token {
                if token.kind == TokenKind::Eq {
                    self.parser_state <<= 1;
                    return self.next();
                }
            }
            self.parser_state >>= 1;
            self.next()
        } else {
            // Here always the parser state is 4
            self.parser_state >>= 2;
            return Some(token);
        }
    }
}
