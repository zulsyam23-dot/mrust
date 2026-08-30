//! Penerapan attribute pengguna ke widget: method chain type-safe per
//! kategori widget (meniru cara iced memisahkan API Text/Button/etc.).

use proc_macro2::{Ident, Span, TokenStream};

use super::css::{apply_css, css_text, WKind};
use super::tags;

use crate::error::Error;

/// `apply_attrs` dengan span default titik pemanggilan makro.
pub(crate) fn apply_attrs(
    ts: TokenStream,
    attrs: &[(String, Option<TokenStream>)],
    kind: WKind,
) -> Result<TokenStream, Error> {
    apply_attrs_kind(ts, attrs, kind, Span::call_site())
}

/// Attribute `name=value` -> `.name(value)`; `name` saja -> `.name()`;
/// nama alias HTML diterjemahkan via ATTR_ALIASES. Attribute `style="…"`
/// (literal string) diperlakukan sebagai CSS: diterjemahkan per `WKind`.
///
/// Namespace `hx_*`/`mix_*`: alias event diteruskan ke chain; token behavior
/// yang belum di-generate (per prd2.md) diberi error jelas ber-version;
/// salah ketik -> error "tak dikenal" (bukan method `hx_x` membingungkan).
pub(crate) fn apply_attrs_kind(
    ts: TokenStream,
    attrs: &[(String, Option<TokenStream>)],
    kind: WKind,
    span: Span,
) -> Result<TokenStream, Error> {
    let mut acc = ts;
    for (name, value) in attrs {
        if name == "style" {
            if let Some(v) = value {
                if let Ok(css) = css_text(v) {
                    acc = apply_css(acc, &css, kind, span)?;
                    continue;
                }
            }
        }
        if (name.starts_with("hx_") || name.starts_with("mix_"))
            && !tags::ATTR_ALIASES.iter().any(|(a, _)| a == name)
        {
            if tags::HX_DEFERRED.contains(&name.as_str()) {
                return Err(Error::new(
                    span,
                    format!(
                        "token `{name}` ({} per prd2.md) belum di-generate di iterasi ini; {}",
                        tags::hx_version(name),
                        tags::hx_hint(name)
                    ),
                ));
            }
            return Err(Error::new(
                span,
                format!("token `{name}` tak dikenal (lihat prd2.md untuk daftar hx_*)"),
            ));
        }
        acc = chain_attr(acc, name, value);
    }
    Ok(acc)
}

pub(crate) fn chain_attr(acc: TokenStream, name: &str, value: &Option<TokenStream>) -> TokenStream {
    let method =
        tags::ATTR_ALIASES.iter().find(|(a, _)| *a == name).map(|(_, m)| *m).unwrap_or(name);
    let m = Ident::new(method, Span::call_site());
    match value {
        Some(v) => quote::quote! { #acc . #m ( #v ) },
        None => quote::quote! { #acc . #m () },
    }
}

/// Button ber-`hx_disabled`: event `on_press` pertama (via alias `hx_click`/
/// `mix_press`/`onclick`/dll) diubah jadi `on_press_maybe(if cond {None} else
/// {Some(ev)})`; attribute lain di-apply biasa.
pub(crate) fn apply_disabled(
    e: &crate::ast::Element,
    base: TokenStream,
    kind: WKind,
) -> Result<TokenStream, Error> {
    let dis = e.disabled.as_ref().expect("apply_disabled: hanya dipanggil dgn hx_disabled");
    let mut ev = None;
    let rest: Vec<(String, Option<TokenStream>)> = e
        .attrs
        .iter()
        .filter_map(|(n, v)| {
            let is_press = tags::ATTR_ALIASES.iter().any(|(a, m)| a == n && *m == "on_press");
            if is_press && ev.is_none() {
                ev = Some(v.clone());
                None
            } else {
                Some((n.clone(), v.clone()))
            }
        })
        .collect();
    let acc = apply_attrs(base, &rest, kind)?;
    match ev {
        Some(Some(ev)) => Ok(quote::quote! {
            #acc .on_press_maybe(if #dis { None } else { Some(#ev) })
        }),
        Some(None) => Err(Error::new(e.span, "event klik (mis. `hx_click`) di button butuh nilai")),
        None => Err(Error::new(
            e.span,
            "button dengan `hx_disabled` butuh event klik (mis. `hx_click=Message::X`)",
        )),
    }
}
