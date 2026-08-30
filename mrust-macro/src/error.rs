use proc_macro2::{Span, TokenStream, TokenTree};

pub(crate) fn span_of(t: Option<&TokenTree>) -> Span {
    t.map(TokenTree::span).unwrap_or_else(Span::call_site)
}

#[derive(Debug)]
pub(crate) struct Error {
    pub(crate) span: Span,
    pub(crate) msg: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl Error {
    pub(crate) fn new(span: Span, msg: impl Into<String>) -> Self {
        Self { span, msg: msg.into() }
    }

    pub(crate) fn to_compile_error(&self) -> TokenStream {
        let msg = self.msg.clone();
        let mut first = true;
        quote::quote! { compile_error!(#msg) }
            .into_iter()
            .map(|mut t| {
                if first {
                    t.set_span(self.span);
                    first = false;
                }
                t
            })
            .collect()
    }
}