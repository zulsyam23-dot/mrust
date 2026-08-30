//! CSS `style="…"` -> method/`style(closure)` iced yang type-safe.
//! Tiap `WKind` memetakan properti ke method/field yang sah (bukan string bebas).

use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::ToTokens;

use super::attr_text;
use crate::error::Error;

/// Kategori widget — menentukan pemetaan properti CSS `style="…"` ke
/// method/`style(closure)` yang sah (agar tetap type-safe, bukan string bebas).
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum WKind {
    Text,
    Layout,
    Container,
    Button,
    Rule,
    Other,
}

/// Nilai CSS terurai untuk widget ber-`.style(closure)`.
#[derive(Default)]
struct WStyle {
    bg: Option<TokenStream>,
    text: Option<TokenStream>,
    border: Option<BorderSpec>,
    shadow: Option<(f32, f32, TokenStream)>,
}

/// Lebar, warna, radius border — tiap bagian diisi terpisah saat parse CSS.
type BorderSpec = (Option<f32>, Option<TokenStream>, Option<f32>);

impl WStyle {
    fn is_empty(&self) -> bool {
        self.bg.is_none() && self.text.is_none() && self.border.is_none() && self.shadow.is_none()
    }
}

fn call(acc: TokenStream, method: &str, args: TokenStream) -> TokenStream {
    let m = Ident::new(method, Span::call_site());
    quote::quote! { #acc . #m ( #args ) }
}

fn f32_lit(n: f32) -> TokenStream {
    Literal::f32_unsuffixed(n).to_token_stream()
}

/// `style="color: red"` / `style={"color: red"}` -> isi literal string CSS
/// (bukan ekspresi). Error bila nilainya bukan tepat satu literal string.
pub(crate) fn css_text(v: &TokenStream) -> Result<String, ()> {
    attr_text(v)
}

pub(crate) fn apply_css(
    acc: TokenStream,
    css: &str,
    kind: WKind,
    span: Span,
) -> Result<TokenStream, Error> {
    let rules = parse_css(css);
    let mut ts = acc;
    let mut ws = WStyle::default();
    for (p, v) in &rules {
        match kind {
            WKind::Text => match p.as_str() {
                "color" => ts = call(ts, "color", color_ts(v, span)?),
                "font-size" => ts = call(ts, "size", num_ts(v, span)?),
                "font-weight" => ts = call(ts, "font", weight_ts(v, span)?),
                "font-family" => ts = call(ts, "font", family_ts(v, span)?),
                "text-align" | "align-x" => ts = call(ts, "align_x", horiz_ts(v, span)?),
                "align-y" => ts = call(ts, "align_y", vert_ts(v, span)?),
                "width" => ts = call(ts, "width", length_ts(v, span)?),
                "height" => ts = call(ts, "height", length_ts(v, span)?),
                _ => css_ignore(p, kind, span)?,
            },
            WKind::Layout => match p.as_str() {
                "gap" | "spacing" => ts = call(ts, "spacing", num_ts(v, span)?),
                "width" => ts = call(ts, "width", length_ts(v, span)?),
                "height" => ts = call(ts, "height", length_ts(v, span)?),
                _ => css_ignore(p, kind, span)?,
            },
            WKind::Container => match p.as_str() {
                "background" => ws.bg = Some(color_ts(v, span)?),
                "color" => ws.text = Some(color_ts(v, span)?),
                "border" => ws.border = Some(border_ts(v, span)?),
                "border-width" => {
                    let mut b = ws.border.take().unwrap_or((None, None, None));
                    b.0 = Some(num(v, span)?);
                    ws.border = Some(b);
                }
                "border-color" => {
                    let mut b = ws.border.take().unwrap_or((None, None, None));
                    b.1 = Some(color_ts(v, span)?);
                    ws.border = Some(b);
                }
                "border-radius" => {
                    let mut b = ws.border.take().unwrap_or((None, None, None));
                    b.2 = Some(num(v, span)?);
                    ws.border = Some(b);
                }
                "shadow" => ws.shadow = Some(shadow_ts(v, span)?),
                "padding" => ts = call(ts, "padding", padding_ts(v, span)?),
                "width" => ts = call(ts, "width", length_ts(v, span)?),
                "height" => ts = call(ts, "height", length_ts(v, span)?),
                _ => css_ignore(p, kind, span)?,
            },
            WKind::Button => match p.as_str() {
                "background" => ws.bg = Some(color_ts(v, span)?),
                "color" => ws.text = Some(color_ts(v, span)?),
                "border" => ws.border = Some(border_ts(v, span)?),
                "border-width" => {
                    let mut b = ws.border.take().unwrap_or((None, None, None));
                    b.0 = Some(num(v, span)?);
                    ws.border = Some(b);
                }
                "border-color" => {
                    let mut b = ws.border.take().unwrap_or((None, None, None));
                    b.1 = Some(color_ts(v, span)?);
                    ws.border = Some(b);
                }
                "border-radius" => {
                    let mut b = ws.border.take().unwrap_or((None, None, None));
                    b.2 = Some(num(v, span)?);
                    ws.border = Some(b);
                }
                "shadow" => ws.shadow = Some(shadow_ts(v, span)?),
                "width" => ts = call(ts, "width", length_ts(v, span)?),
                "height" => ts = call(ts, "height", length_ts(v, span)?),
                _ => css_ignore(p, kind, span)?,
            },
            WKind::Rule => match p.as_str() {
                "color" => ws.text = Some(color_ts(v, span)?),
                _ => css_ignore(p, kind, span)?,
            },
            WKind::Other => match p.as_str() {
                "width" => ts = call(ts, "width", length_ts(v, span)?),
                "height" => ts = call(ts, "height", length_ts(v, span)?),
                _ => css_ignore(p, kind, span)?,
            },
        }
    }
    if !ws.is_empty() {
        ts = call(ts, "style", style_closure(&ws, kind)?);
    }
    Ok(ts)
}

fn style_closure(ws: &WStyle, kind: WKind) -> Result<TokenStream, Error> {
    // Button `.style` ber-param (Theme, Status); Container/Rule hanya Theme.
    let (params, sty, mut set) = match kind {
        WKind::Button => (
            quote::quote! { |_t, _s| },
            quote::quote! { ::iced::widget::button::Style },
            Vec::new(),
        ),
        WKind::Container => (
            quote::quote! { |_t| },
            quote::quote! { ::iced::widget::container::Style },
            Vec::new(),
        ),
        WKind::Rule => (quote::quote! { |_t| }, quote::quote! {}, Vec::new()),
        _ => return Err(Error::new(Span::call_site(), "internal: style utk jenis tak-ber-style")),
    };
    if let Some(c) = &ws.bg {
        set.push(quote::quote! { s.background = Some((#c).into()); });
    }
    if let Some(c) = &ws.text {
        let assign = match kind {
            // container text_color bertipe Option; button & rule adalah Color
            WKind::Button => quote::quote! { s.text_color = #c; },
            WKind::Rule => quote::quote! { s.color = #c; },
            _ => quote::quote! { s.text_color = Some(#c); },
        };
        set.push(assign);
    }
    if let Some((w, c, r)) = &ws.border {
        let w = w.map(f32_lit).unwrap_or(quote::quote! { 1.0 });
        let c = c
            .clone()
            .unwrap_or(quote::quote! { ::iced::Color::from_rgba8(128, 128, 128, 1.0) });
        let r = r.map(f32_lit).unwrap_or(quote::quote! { 0.0 });
        set.push(quote::quote! {
            s.border = ::iced::Border {
                width: #w,
                color: #c,
                radius: ::iced::border::Radius::from(#r),
            };
        });
    }
    if let Some((x, y, c)) = &ws.shadow {
        set.push(quote::quote! {
            s.shadow = ::iced::Shadow {
                color: #c,
                offset: ::iced::Vector::new(#x, #y),
                blur_radius: #y,
            };
        });
    }
    // rule::Style tak punya Default -> isi field eksplisit.
    if kind == WKind::Rule {
        let color = ws.text.clone().unwrap_or(quote::quote! { ::iced::Color::from_rgba8(128, 128, 128, 1.0) });
        return Ok(quote::quote! {
            #params ::iced::widget::rule::Style {
                color: #color,
                width: 1,
                radius: ::iced::border::Radius::from(0.0),
                fill_mode: ::iced::widget::rule::FillMode::Full,
            }
        });
    }
    Ok(quote::quote! {
        #params {
            let mut s = #sty ::default();
            #(#set)*
            s
        }
    })
}

fn parse_css(s: &str) -> Vec<(String, String)> {
    s.split(';')
        .filter_map(|r| {
            let (k, v) = r.trim().split_once(':')?;
            let k = k.trim().to_ascii_lowercase();
            if k.is_empty() {
                return None;
            }
            Some((k, v.trim().to_string()))
        })
        .collect()
}

fn css_known(p: &str) -> bool {
    matches!(
        p,
        "background" | "color" | "border" | "border-width" | "border-color" | "border-radius"
            | "shadow" | "padding" | "gap" | "spacing" | "width" | "height" | "align-x" | "align-y"
            | "justify-content" | "align-items" | "font-size" | "font-weight" | "font-family"
            | "text-align"
    )
}

/// Properti dikenal-umum tapi tak berlaku untuk jenis widget ini -> diabaikan
/// (seperti CSS); nama properti tak dikenal -> error agar typo terdeteksi.
fn css_ignore(p: &str, kind: WKind, span: Span) -> Result<(), Error> {
    if css_known(p) {
        let _ = kind;
        Ok(())
    } else {
        Err(Error::new(span, format!("style: properti `{p}` tak dikenal")))
    }
}

/// panjang: `12` / `12px` / `fill` / `50%` -> `Length::Fixed/Fill/FillPortion`.
fn length_ts(v: &str, span: Span) -> Result<TokenStream, Error> {
    let s = v.trim();
    if s.eq_ignore_ascii_case("fill") {
        return Ok(quote::quote! { ::iced::Length::Fill });
    }
    if let Some(pct) = s.strip_suffix('%') {
        let pct: usize = pct
            .trim()
            .parse()
            .map_err(|_| Error::new(span, format!("style: panjang `{v}` tak sah")))?;
        return Ok(quote::quote! { ::iced::Length::FillPortion(#pct) });
    }
    let n = num(v, span)?;
    Ok(quote::quote! { ::iced::Length::Fixed(#n) })
}

fn num(v: &str, span: Span) -> Result<f32, Error> {
    let s = v.trim().trim_end_matches("px").trim();
    s.parse()
        .map_err(|_| Error::new(span, format!("style: angka `{v}` tak sah")))
}

fn num_ts(v: &str, span: Span) -> Result<TokenStream, Error> {
    num(v, span).map(f32_lit)
}

/// padding: `8px` (satu nilai) atau `8px 12px` (vertikal horizontal, ala CSS).
fn padding_ts(v: &str, span: Span) -> Result<TokenStream, Error> {
    let toks: Vec<&str> = v.split_whitespace().collect();
    match toks.as_slice() {
        [one] => Ok(f32_lit(num(one, span)?)),
        [a, b] => {
            let a = f32_lit(num(a, span)?);
            let b = f32_lit(num(b, span)?);
            Ok(quote::quote! { [#a, #b] })
        }
        _ => Err(Error::new(span, format!("style: padding `{v}` — 1 atau 2 nilai"))),
    }
}

/// warna: `#rgb`, `#rrggbb`, `#rrggbbaa`, `rgb(r,g,b)`, `rgba(r,g,b,a)` (a: 0..1), nama.
fn color_ts(v: &str, span: Span) -> Result<TokenStream, Error> {
    let s = v.trim().to_ascii_lowercase();
    if s == "transparent" {
        return Ok(quote::quote! { ::iced::Color::TRANSPARENT });
    }
    let bad = |v: &str| Error::new(span, format!("style: warna `{v}` tak sah"));
    if let Some(hex) = s.strip_prefix('#') {
        let hex = match hex.len() {
            3 => hex.chars().map(|c| format!("{c}{c}")).collect::<String>(),
            n if n == 6 || n == 8 => hex.to_string(),
            _ => return Err(bad(v)),
        };
        let rd = |c: &str| u8::from_str_radix(c, 16).map_err(|_| bad(v));
        let (r, g, b) = (rd(&hex[0..2])?, rd(&hex[2..4])?, rd(&hex[4..6])?);
        let a = match hex.len() {
            8 => rd(&hex[6..8])? as f32 / 255.0,
            _ => 1.0,
        };
        let a = f32_lit(a);
        return Ok(quote::quote! { ::iced::Color::from_rgba8(#r, #g, #b, #a) });
    }
    // rgb() -> opaque; rgba() -> channel alpha 0..1.
    if s.starts_with("rgb") {
        let Some(body) = s.split('(').nth(1).and_then(|x| x.strip_suffix(')')) else {
            return Err(bad(v));
        };
        let ch: Vec<&str> = body.split(',').map(str::trim).collect();
        let rd = |c: &str| c.parse::<u8>().map_err(|_| bad(v));
        match ch.as_slice() {
            [r, g, b] => {
                let (r, g, b) = (rd(r)?, rd(g)?, rd(b)?);
                return Ok(quote::quote! { ::iced::Color::from_rgb8(#r, #g, #b) });
            }
            [r, g, b, a] => {
                let (r, g, b) = (rd(r)?, rd(g)?, rd(b)?);
                let a = a
                    .trim_end_matches('%')
                    .parse::<f32>()
                    .map(f32_lit)
                    .map_err(|_| bad(v))?;
                return Ok(quote::quote! { ::iced::Color::from_rgba8(#r, #g, #b, #a) });
            }
            _ => return Err(bad(v)),
        }
    }
    let (r, g, b) = match s.as_str() {
        "black" => (0u8, 0u8, 0u8),
        "white" => (255u8, 255u8, 255u8),
        "red" => (255u8, 0u8, 0u8),
        "green" => (0u8, 128u8, 0u8),
        "lime" => (0u8, 255u8, 0u8),
        "blue" => (0u8, 0u8, 255u8),
        "yellow" => (255u8, 255u8, 0u8),
        "orange" => (255u8, 165u8, 0u8),
        "purple" => (128u8, 0u8, 128u8),
        "gray" | "grey" => (128u8, 128u8, 128u8),
        "silver" => (192u8, 192u8, 192u8),
        "cyan" | "aqua" => (0u8, 255u8, 255u8),
        "magenta" | "fuchsia" => (255u8, 0u8, 255u8),
        "teal" => (0u8, 128u8, 128u8),
        "navy" => (0u8, 0u8, 128u8),
        "maroon" => (128u8, 0u8, 0u8),
        "olive" => (128u8, 128u8, 0u8),
        "brown" => (165u8, 42u8, 42u8),
        _ => return Err(Error::new(span, format!("style: warna `{v}` tak dikenal"))),
    };
    Ok(quote::quote! { ::iced::Color::from_rgb8(#r, #g, #b) })
}

fn horiz_ts(v: &str, span: Span) -> Result<TokenStream, Error> {
    let side = match v.trim() {
        "left" | "start" => "Left",
        "right" | "end" => "Right",
        "center" | "center-x" => "Center",
        _ => return Err(Error::new(span, format!("style: arah `{v}` tak dikenal"))),
    };
    let ident = Ident::new(side, Span::call_site());
    Ok(quote::quote! { ::iced::alignment::Horizontal::#ident })
}

fn vert_ts(v: &str, span: Span) -> Result<TokenStream, Error> {
    let side = match v.trim() {
        "top" | "start" => "Top",
        "bottom" | "end" => "Bottom",
        "center" | "center-y" => "Center",
        _ => return Err(Error::new(span, format!("style: arah `{v}` tak dikenal"))),
    };
    let ident = Ident::new(side, Span::call_site());
    Ok(quote::quote! { ::iced::alignment::Vertical::#ident })
}

fn weight_ts(v: &str, span: Span) -> Result<TokenStream, Error> {
    let weight = match v.trim() {
        "bold" => quote::quote! { ::iced::font::Weight::Bold },
        "normal" => quote::quote! { ::iced::font::Weight::Normal },
        _ => return Err(Error::new(span, "style: font-weight: pakai `bold` atau `normal`")),
    };
    Ok(quote::quote! { ::iced::Font { weight: #weight, ..::iced::Font::DEFAULT } })
}

fn family_ts(v: &str, span: Span) -> Result<TokenStream, Error> {
    match v.trim().to_ascii_lowercase().as_str() {
        "mono" | "monospace" => Ok(quote::quote! { ::iced::Font::MONOSPACE }),
        "default" | "normal" | "regular" => Err(Error::new(span, "style: font-family default tidak perlu ditulis")),
        _ => Err(Error::new(span, format!("style: font-family `{v}` — pakai `mono`"))),
    }
}

/// `border: 1px solid #ccc` / `border: #ccc` / `border: 2px`.
fn border_ts(v: &str, span: Span) -> Result<BorderSpec, Error> {
    let mut w = None;
    let mut c = None;
    for t in v.split_whitespace() {
        if matches!(t, "solid" | "dashed" | "dotted" | "none") {
            continue;
        }
        if t.ends_with("px") || t.chars().next().is_some_and(|ch| ch.is_ascii_digit()) {
            w = Some(num(t, span)?);
        } else {
            c = Some(color_ts(t, span)?);
        }
    }
    if w.is_none() && c.is_none() {
        return Err(Error::new(span, format!("style: border `{v}` tak dikenal")));
    }
    Ok((w, c, None))
}

/// `shadow: <color>` | `shadow: <x> <color>` | `shadow: <x> <y> <color>`.
fn shadow_ts(v: &str, span: Span) -> Result<(f32, f32, TokenStream), Error> {
    let toks: Vec<&str> = v.split_whitespace().collect();
    match toks.as_slice() {
        [c] => Ok((0.0, 0.0, color_ts(c, span)?)),
        [x, c] => {
            let x = num(x, span)?;
            Ok((x, x * 0.5, color_ts(c, span)?))
        }
        [x, y, c] => Ok((num(x, span)?, num(y, span)?, color_ts(c, span)?)),
        _ => Err(Error::new(span, format!("style: shadow `{v}` — 1..3 nilai"))),
    }
}
