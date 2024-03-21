use proc_macro::TokenStream;
use quote::quote;
use syn::{
    spanned::Spanned as _,
    parse_macro_input, parse_quote_spanned,
    visit_mut::{self, VisitMut},
    Expr, Item, ItemFn,
    ImplItemFn,
    ItemImpl
};

struct AddContextVisitor {
    scope_stack: Vec<String>,
}

impl AddContextVisitor {
    fn new() -> Self {
        Self {
            scope_stack: vec![],
        }
    }
    fn generate_context(&self) -> String {
        self.scope_stack.join("::")
    }
}

impl VisitMut for AddContextVisitor {
    fn visit_item_fn_mut(&mut self, node: &mut ItemFn) {
        self.scope_stack.push(node.sig.ident.to_string());
        visit_mut::visit_item_fn_mut(self, node);
        self.scope_stack.pop();
    }

    fn visit_impl_item_fn_mut(&mut self, node: &mut ImplItemFn) {
        self.scope_stack.push(node.sig.ident.to_string());
        visit_mut::visit_impl_item_fn_mut(self, node);
        self.scope_stack.pop();
    }

    fn visit_item_impl_mut(&mut self, node: &mut ItemImpl) {
        let self_ty = &node.self_ty;
        self.scope_stack.push(quote!(#self_ty).to_string());
        visit_mut::visit_item_impl_mut(self, node);
        self.scope_stack.pop();
    }

    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Try(expr_try) = node {
            let inner_expr = &expr_try.expr;
            let ctx = self.generate_context();
            let span = expr_try.span();
            expr_try.expr = Box::new(parse_quote_spanned! {
                span=> ::anyhow::Context::with_context(#inner_expr, || {
                    #[track_caller]
                    fn format_location(msg: &str) -> ::std::string::String {
                        ::std::format!("{msg} at {}", ::std::panic::Location::caller())
                    }
                    format_location(#ctx)
                })
            });
        }

        visit_mut::visit_expr_mut(self, node);
    }
}

#[proc_macro_attribute]
pub fn anyhow_trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as Item);
    let mut visitor = AddContextVisitor::new();
    visitor.visit_item_mut(&mut item);

    quote!(#item).into()
}
