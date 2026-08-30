use proc_macro2::{Literal, Span, TokenStream};
use quote::ToTokens;

#[derive(Debug)]
pub(crate) enum Node {
    /// `<tag>...</tag>` atau `<tag/>`
    Element(Element),
    /// teks literal / rangkaian teks
    Text(Literal),
    /// `{ ekspresi }`
    Expr(TokenStream),
}

#[derive(Debug)]
pub(crate) struct Element {
    pub(crate) name: String,
    /// `(nama, nilai)` — `None` untuk attribute boolean tanpa nilai (`center_x`)
    pub(crate) attrs: Vec<(String, Option<TokenStream>)>,
    pub(crate) children: Vec<Node>,
    pub(crate) span: Span,
    /// `hx_if`/`hx_visible` (sinonim) — ditarik dari attrs saat pre-pass;
    /// bila `Some(cond)`, hasil widget dibungkus kondisi tampil/disembunyikan.
    pub(crate) cond: Option<TokenStream>,
    /// `hx_disabled` — ditarik dari attrs saat pre-pass; `Some(cond)` berarti
    /// nonaktifkan interaksi selama kondisi benar.
    pub(crate) disabled: Option<TokenStream>,
}

/// Satu potongan konten leaf: literal atau ekspresi (untuk digabung `format!`).
#[derive(Debug)]
pub(crate) enum Part {
    Lit(Literal),
    Expr(TokenStream),
}

impl Part {
    pub(crate) fn tokens(&self) -> TokenStream {
        match self {
            Part::Lit(l) => l.to_token_stream(),
            Part::Expr(ts) => ts.clone(),
        }
    }
}