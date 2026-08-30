//! Generasi widget tunggal: leaf (text/button/stateful), layout, container,
//! rule, dan tooltip. Dispatcher `gen_element` ada di `mod.rs`.

use proc_macro2::{Ident, Literal, Span, TokenStream};

use super::attrs::apply_attrs;
use super::css::WKind;
use super::tags::{EXTRA_METHODS, HEADINGS, LAYOUT_ALIAS, TEXT_ROLES, TextRole};
use super::{apply_disabled, extend_all, gen_child, gen_element, Child};
use crate::ast::{Element, Node, Part};
use crate::error::Error;

pub(crate) fn gen_layout(e: &Element) -> Result<TokenStream, Error> {
    let children = e.children.iter().map(gen_child).collect::<Result<Vec<_>, Error>>()?;
    if children.is_empty() {
        return Err(Error::new(e.span, format!("tag `{}` butuh minimal satu anak", e.name)));
    }
    let ident = layout_ident(&e.name, e.span);
    let base = quote::quote! { ::iced::widget::#ident::new() };
    apply_attrs(extend_all(base, &children), &e.attrs, WKind::Layout)
}

/// `row` -> `Row`, `column` -> `Column`, dst; `ul`/`ol`/`menu` -> `Column`.
pub(crate) fn layout_ident(name: &str, span: Span) -> Ident {
    let base = LAYOUT_ALIAS.iter().find(|(t, _)| *t == name).map(|(_, c)| *c).unwrap_or(name);
    struct_ident(base, span)
}

/// `row` -> `Row`, `column` -> `Column`, `stack` -> `Stack`.
pub(crate) fn struct_ident(name: &str, span: Span) -> Ident {
    let mut chars = name.chars();
    let capitalized = match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    };
    Ident::new(&capitalized, span)
}

pub(crate) fn gen_container(e: &Element) -> Result<TokenStream, Error> {
    let children = e.children.iter().map(gen_child).collect::<Result<Vec<_>, Error>>()?;
    let inner = match children.as_slice() {
        [] => return Err(Error::new(e.span, format!("tag `{}` butuh minimal satu anak", e.name))),
        // satu widget pas langsung; spread (expr-`{}`) butuh pembungkus column[]
        [Child::Widget(one)] => one.clone(),
        _ => extend_all(quote::quote! { ::iced::widget::Column::new() }, &children),
    };
    let iced = if e.name == "scrollable" { "scrollable" } else { "container" };
    let ident = Ident::new(iced, e.span);
    let base = quote::quote! { ::iced::widget::#ident(#inner) };
    apply_attrs(base, &e.attrs, WKind::Container)
}

pub(crate) fn gen_rule(e: &Element) -> Result<TokenStream, Error> {
    if e.children.iter().any(|n| matches!(n, Node::Element(_))) {
        return Err(Error::new(e.span, "rule tidak boleh punya anak"));
    }
    let fname = if e.name == "vertical_rule" { "vertical_rule" } else { "horizontal_rule" };
    let ident = Ident::new(fname, e.span);
    let base = quote::quote! { ::iced::widget::#ident(1.0) };
    apply_attrs(base, &e.attrs, WKind::Rule)
}

/// Tag stateful: attribute yang didaftar menjadi argumen constructor sesuai
/// urutan, attribute lainnya (termasuk event) jadi method chain.
pub(crate) fn gen_positional(e: &Element, ctor: &str, pos: &[&str]) -> Result<TokenStream, Error> {
    if !e.children.is_empty() {
        return Err(Error::new(e.span, format!("tag `{}` butuh self-close `/>`", e.name)));
    }
    let mut used = vec![false; e.attrs.len()];
    let mut args = Vec::new();
    // Binding ala-htmx (v0.2) utk keluarga text_input: `hx_value`/`hx_bind`
    // menyuplai argumen `value` + `hx_value_to`/`hx_bind_to` memberi message
    // penulisan-balik (`.on_input(move |v| Msg::Var(v))`).
    let bind_val = e.attrs.iter().position(|(n, _)| n == "hx_value" || n == "hx_bind");
    let bind_to = e.attrs.iter().position(|(n, _)| n == "hx_value_to" || n == "hx_bind_to");
    let mut bound = false;
    for want in pos {
        match e.attrs.iter().position(|(n, _)| n == want) {
            // bila `value` tak ditulis tapi ada binding, pakai nilai binding-nya
            None if *want == "value" && ctor == "text_input" && bind_val.is_some() => {
                let bi = bind_val.unwrap();
                used[bi] = true;
                bound = true;
                let value = e.attrs[bi]
                    .1
                    .clone()
                    .ok_or_else(|| Error::new(e.span, format!("attribute `hx_value` di tag `{}` butuh nilai", e.name)))?;
                args.push(quote::quote! { &(#value) });
            }
            Some(i) => {
                used[i] = true;
                let value = e.attrs[i]
                    .1
                    .clone()
                    .ok_or_else(|| Error::new(e.span, format!("attribute `{want}` di tag `{}` butuh nilai", e.name)))?;
                // text_input menerima &str (placeholder/value); nilai selain
                // literal string di-stringify dulu biar ergonomis macam HTML
                let value = if ctor == "text_input" && !value.to_string().starts_with('"') {
                    quote::quote! { &::std::format!("{}", #value) }
                } else {
                    value
                };
                // src literal string (mis. `src="assets/x.svg"`) -> asset
                // di-embed compile-time via include_bytes!, seperti HTML membawa
                // assetnya; ekspresi (mis. `src={handle}`) diteruskan apa adanya.
                let value = if *want == "src" && value.to_string().starts_with('"') {
                    if ctor == "svg" {
                        // svg::Handle tidak punya from_bytes (hanya from_memory).
                        quote::quote! {
                            ::iced::widget::svg::Handle::from_memory(
                                ::std::borrow::Cow::Borrowed(
                                    &include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #value))[..]
                                )
                            )
                        }
                    } else {
                        quote::quote! {
                            ::iced::widget::image::Handle::from_bytes(
                                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", #value))
                            )
                        }
                    }
                } else {
                    value
                };
                args.push(value);
            }
            None => return Err(Error::new(e.span, format!("tag `{}` butuh attribute `{want}`", e.name))),
        }
    }
    let rest: Vec<(String, Option<TokenStream>)> = e
        .attrs
        .iter()
        .zip(&used)
        .enumerate()
        .filter(|(i, (_, u))| !**u && Some(*i) != bind_to && Some(*i) != bind_val)
        .map(|(_, (a, _))| a.clone())
        .collect();
    let ident = Ident::new(ctor, e.span);
    let base = quote::quote! { ::iced::widget::#ident( #(#args),* ) };
    let base = apply_attrs(base, &rest, WKind::Other)?;
    // penulisan-balik binding (hanya bila binding benar-benar dipakai utk `value`)
    let base = match bind_to.and_then(|i| e.attrs.get(i).and_then(|(_, v)| v.clone())) {
        Some(variant) if bound => quote::quote! { #base .on_input(move |v| #variant (v)) },
        _ => base,
    };
    // method tambahan wajib (mis. password): dipanggil setelah attribute user
    // supaya pengguna tetap bisa menimpa gaya dengan attribute berikutnya.
    let base = EXTRA_METHODS
        .iter()
        .filter(|(t, _)| *t == e.name)
        .fold(base, |acc, (_, m)| {
            let m = Ident::new(m, e.span);
            quote::quote! { #acc . #m () }
        });
    Ok(base)
}

pub(crate) fn gen_leaf(e: &Element, ctor: &str) -> Result<TokenStream, Error> {
    // `hx_disabled` hanya bermakna utk widget interaktif (button): matikan
    // interaksi saat kondisi benar via `on_press_maybe(if cond {None} else {Some(_)})`.
    let disabled = match (ctor == "button", &e.disabled) {
        (true, Some(_)) => Some(true),
        (true, None) => None,
        (false, Some(_)) => {
            return Err(Error::new(
                e.span,
                format!("token `hx_disabled` di tag `{}` hanya didukung untuk `button`", e.name),
            ));
        }
        (false, None) => None,
    };

    // button boleh berisi SATU widget (mis. `<button><icon src=.../></button>`),
    // persis seperti kata-kunci HTML; selain itu konten leaf = teks/ekspresi.
    if ctor == "button" {
        if let [Node::Element(one)] = e.children.as_slice() {
            let inner = gen_element(one)?;
            let ident = Ident::new(ctor, e.span);
            let base = quote::quote! { ::iced::widget::#ident(#inner) };
            return if disabled.is_some() {
                apply_disabled(e, base, WKind::Button)
            } else {
                apply_attrs(base, &e.attrs, WKind::Button)
            };
        }
    }
    let args = gen_content_args(&e.children)?;
    let ident = Ident::new(ctor, e.span);
    let kind = match ctor {
        "button" => WKind::Button,
        "text" => WKind::Text,
        _ => WKind::Other,
    };
    let base = match args {
        Some(a) => quote::quote! { ::iced::widget::#ident(#a) },
        None => quote::quote! { ::iced::widget::#ident() },
    };
    if disabled.is_some() {
        apply_disabled(e, base, kind)
    } else {
        apply_attrs(base, &e.attrs, kind)
    }
}

pub(crate) fn gen_text_like(
    e: &Element,
    size: Option<f32>,
    font: TokenStream,
    color: Option<TokenStream>,
) -> Result<TokenStream, Error> {
    let args = gen_content_args(&e.children)?;
    let base = quote::quote! { ::iced::widget::text(#args) };
    let base = match size {
        Some(s) => quote::quote! { #base .size(#s) },
        None => base,
    };
    let base = quote::quote! { #base .font(#font) };
    let base = match color {
        Some(c) => quote::quote! { #base .color(#c) },
        None => base,
    };
    apply_attrs(base, &e.attrs, WKind::Text)
}

pub(crate) fn font_default() -> TokenStream {
    quote::quote! { ::iced::Font::DEFAULT }
}

pub(crate) fn font_mono() -> TokenStream {
    quote::quote! { ::iced::Font::MONOSPACE }
}

pub(crate) fn font_bold() -> TokenStream {
    quote::quote! { ::iced::Font { weight: ::iced::font::Weight::Bold, ..::iced::Font::DEFAULT } }
}

pub(crate) fn mark_color() -> TokenStream {
    quote::quote! { ::iced::Color::from_rgba8(255, 190, 60, 1.0) }
}

pub(crate) fn heading_size(name: &str) -> Option<f32> {
    HEADINGS.iter().find(|(t, _)| *t == name).map(|(_, s)| *s)
}

/// Preset teks penekanan (bold/mono/small/mark/big) -> (size, font, color).
pub(crate) fn text_role(name: &str) -> Option<(Option<f32>, TokenStream, Option<TokenStream>)> {
    let role = TEXT_ROLES.iter().find(|(t, _)| *t == name).map(|(_, r)| *r)?;
    let (size, font, color) = match role {
        TextRole::Bold => (None, font_bold(), None),
        TextRole::Mono => (None, font_mono(), None),
        TextRole::Small => (Some(12.0), font_default(), None),
        TextRole::Mark => (None, font_default(), Some(mark_color())),
        TextRole::Big => (Some(18.0), font_default(), None),
        TextRole::Plain => (None, font_default(), None),
    };
    Some((size, font, color))
}

/// Konten leaf -> argumen constructor. Literal tunggal jadi argumen langsung,
/// campuran literal + `{expr}` digabung lewat `format!`.
pub(crate) fn gen_content_args(children: &[Node]) -> Result<Option<TokenStream>, Error> {
    if children.is_empty() {
        return Ok(None);
    }
    let parts = children
        .iter()
        .map(|n| match n {
            Node::Element(e) => Err(Error::new(
                e.span,
                format!("tag `{}` bukan kontainer; tidak boleh punya anak", e.name),
            )),
            Node::Text(l) => Ok(Part::Lit(l.clone())),
            Node::Expr(ts) => Ok(Part::Expr(ts.clone())),
        })
        .collect::<Result<Vec<_>, Error>>()?;
    match parts.as_slice() {
        [Part::Lit(l)] => Ok(Some(quote::quote! { #l })),
        [Part::Expr(ts)] => Ok(Some(quote::quote! { #ts })),
        _ => {
            let holes = Literal::string(&"{}".repeat(parts.len()));
            let args = parts.iter().map(Part::tokens).collect::<Vec<_>>();
            Ok(Some(quote::quote! { ::std::format!(#holes, #(#args),*) }))
        }
    }
}

/// `tooltip` -> `iced::widget::tooltip(content, tip, pos)`.
/// Wajib: satu anak widget (konten) + attribute `tip`. `pos` opsional (default Bottom).
pub(crate) fn gen_tooltip(e: &Element) -> Result<TokenStream, Error> {
    let children = e.children.iter().map(gen_child).collect::<Result<Vec<_>, Error>>()?;
    let [Child::Widget(content)] = children.as_slice() as &[Child] else {
        return Err(Error::new(e.span, "tag `tooltip` butuh tepat satu anak widget"));
    };
    let tip = e.attrs.iter().find(|(n, _)| n == "tip")
        .and_then(|(_, v)| v.clone())
        .ok_or_else(|| Error::new(e.span, "tag `tooltip` butuh attribute `tip`"))?;
    let tip = if tip.to_string().starts_with('"') {
        quote::quote! { ::iced::widget::text(#tip) }
    } else {
        tip
    };
    let pos = e.attrs.iter().find(|(n, _)| n == "pos")
        .and_then(|(_, v)| v.clone())
        .unwrap_or_else(|| quote::quote! { ::iced::Position::Bottom });
    let rest: Vec<(String, Option<TokenStream>)> = e.attrs.iter()
        .filter(|(n, _)| n != "tip" && n != "pos")
        .cloned().collect();
    let base = quote::quote! { ::iced::widget::tooltip(#content, #tip, #pos) };
    apply_attrs(base, &rest, WKind::Other)
}
