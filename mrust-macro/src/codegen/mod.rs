//! Modul pemecahan `codegen.rs` (1123 baris) menjadi per-domain:
//! - `tags.rs`  : tabel tag/alias + metadata token `hx_*` (murni data)
//! - `css.rs`   : kustom `style="…"` -> method/closure iced type-safe
//! - `attrs.rs` : attribute user -> method chain per jenis widget
//! - `widget.rs`: generasi widget tunggal (leaf/layout/container/rule/tooltip)
//! - `mod.rs`   : dispatcher `gen_element` + `Child`/`gen_child`/`extend_all`
//!
//! lib.rs hanya memakai `codegen::gen_element` & `codegen::attr_text`.

mod attrs;
mod css;
mod tags;
mod widget;

pub(crate) use attrs::{apply_attrs, apply_disabled};
pub(crate) use widget::{
    font_bold, gen_container, gen_layout, gen_leaf, gen_positional, gen_rule, gen_text_like,
    gen_tooltip, heading_size, text_role,
};

use proc_macro2::{Span, TokenStream};

use crate::ast::{Element, Node};
use crate::error::Error;

pub(crate) fn gen_element(e: &Element) -> Result<TokenStream, Error> {
    let inner = gen_element_inner(e)?;
    wrap_behavior(e, inner)
}

/// Generasi widget tanpa membungkus behavior reaktivitas (`cond`/`disabled`).
fn gen_element_inner(e: &Element) -> Result<TokenStream, Error> {
    let name = e.name.as_str();

    if tags::LAYOUT_TAGS.contains(&name) {
        return gen_layout(e);
    }
    if tags::CONTAINER_TAGS.contains(&name) {
        return gen_container(e);
    }
    if tags::RULE_TAGS.contains(&name) {
        return gen_rule(e);
    }
    // widget stateful: attribute -> argumen constructor
    if let Some((_, ctor, pos)) = tags::POS_ARGS.iter().find(|(t, _, _)| *t == name) {
        return gen_positional(e, ctor, pos);
    }
    if name == "tooltip" {
        return gen_tooltip(e);
    }
    if name == "br" {
        if !e.children.is_empty() {
            return Err(Error::new(e.span, "tag `br` tidak boleh punya anak"));
        }
        let base = quote::quote! {
            ::iced::widget::Space::with_height(::iced::Length::Fixed(12.0))
        };
        return apply_attrs(base, &e.attrs, css::WKind::Other);
    }
    // pengisi fleksibel di baris (toolbar/status bar): Space lebar penuh
    if name == "spacer" {
        if !e.children.is_empty() {
            return Err(Error::new(e.span, "tag `spacer` tidak boleh punya anak"));
        }
        let base = quote::quote! {
            ::iced::widget::Space::with_width(::iced::Length::Fill)
        };
        return apply_attrs(base, &e.attrs, css::WKind::Other);
    }
    // teks ala-HTML (judul h1-h6 & penekanan) -> `text` + preset ukuran/warna/font
    if let Some(size) = heading_size(name) {
        return gen_text_like(e, Some(size), font_bold(), None);
    }
    if let Some((size, font, color)) = text_role(name) {
        return gen_text_like(e, size, font, color);
    }
    if tags::CLICKABLE_TAGS.contains(&name) {
        return gen_leaf(e, "button");
    }
    // fallback leaf: ctor mengikuti nama tag; `p`/`label` -> text
    let ctor = if tags::TEXT_TAGS.contains(&name) { "text" } else { name };
    gen_leaf(e, ctor)
}

/// Bungkus hasil widget dengan behavior reaktivitas yang sudah di-extract
/// (lihat `extract_behavior` di lib.rs): `cond` -> tampil/sembunyi via if/else.
pub(crate) fn wrap_behavior(e: &Element, inner: TokenStream) -> Result<TokenStream, Error> {
    let cond = match &e.cond {
        Some(c) => quote::quote! { ::mrust_runtime::if_elem(#c, (#inner).into()) },
        None => inner,
    };
    // `disabled` ditangani khusus per-jenis widget di gen_leaf (button).
    Ok(cond)
}

/// Anak layout/container: akan tag -> widget (di-`extend` via `once(..).into()`);
/// anak `{ ekspresi }` -> `Element` atau `Vec<Element>` via `Spread` runtime.
pub(crate) enum Child {
    Widget(TokenStream),
    Spread(TokenStream),
}

pub(crate) fn gen_child(n: &Node) -> Result<Child, Error> {
    match n {
        Node::Element(e) => Ok(Child::Widget(gen_element(e)?)),
        Node::Expr(ts) => Ok(Child::Spread(ts.clone())),
        Node::Text(_) => Err(Error::new(
            Span::call_site(),
            "teks bebas hanya boleh dipakai sebagai isi widget leaf (text, button, dst)",
        )),
    }
}

pub(crate) fn extend_all(base: TokenStream, children: &[Child]) -> TokenStream {
    children.iter().fold(base, |acc, c| match c {
        Child::Widget(w) => quote::quote! { #acc.extend(::std::iter::once((#w).into())) },
        Child::Spread(x) => quote::quote! { #acc.extend(::mrust_runtime::Spread::spread(#x)) },
    })
}

/// Ekstrak isi bila nilai tepat satu literal string `"..."` (dipakai gaya htmx
/// `hx_trigger="…"`, `hx_confirm="…"`, dan CSS `style="…"`). Selain literal
/// string (ekspresi `{…}` dst) -> `Err`.
pub(crate) fn attr_text(v: &TokenStream) -> Result<String, ()> {
    let mut it = v.clone().into_iter();
    let (Some(t), None) = (it.next(), it.next()) else { return Err(()) };
    let s = t.to_string();
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        Ok(s[1..s.len() - 1].to_string())
    } else {
        Err(())
    }
}
