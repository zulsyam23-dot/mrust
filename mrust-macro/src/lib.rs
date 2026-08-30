//! mrust-macro — markup HTML-like untuk gui iced, tetap di dalam file `.rs`.

mod ast;
mod codegen;
mod error;
mod parser;

use proc_macro2::{Span, TokenStream};

#[proc_macro]
pub fn view(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match expand(input.into()) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Titik masuk yang bisa diuji langsung (tanpa runtime proc-macro).
pub(crate) fn expand(input: TokenStream) -> Result<TokenStream, error::Error> {
    use ast::Node;

    let mut p = parser::Parser::new(input);
    let mut nodes = p.parse()?;
    // token reaktivitas (v0.2/v0.3): `hx_if`/`hx_visible` dan `hx_disabled`
    // ditarik keluar dari attrs menjadi field elemen (pre-pass) supaya codegen
    // tidak menganggapnya method dan bisa membungkus hasil sesuai behavior.
    for n in &mut nodes {
        if let Node::Element(e) = n {
            extract_behavior(e);
        }
    }
    // inheritance ala-htmx (v0.3): `hx_hoist`/`hx_disinherit` diproses dulu
    // (pre-pass), menghapus attr meta itu sehingga codegen tak melihatnya.
    for n in &mut nodes {
        if let Node::Element(e) = n {
            apply_inheritance(e, &[]);
        }
    }
    let mut items = Vec::new();
    for n in &nodes {
        match n {
            Node::Element(e) => items.push(codegen::gen_element(e)?),
            Node::Expr(ts) => items.push(ts.clone()),
            Node::Text(_) => {
                return Err(error::Error::new(
                    Span::call_site(),
                    "node tingkat atas harus berupa <tag>, literal tidak sah di sini",
                ));
            }
        }
    }
    match items.len() {
        0 => Err(error::Error::new(Span::call_site(), "view!{} tidak boleh kosong")),
        1 => Ok(items.into_iter().next().unwrap()),
        _ => Ok(quote::quote! { ::iced::widget::column![ #(#items),* ] }),
    }
}

/// Token reaktivitas `hx_if`/`hx_visible` (sinonim) dan `hx_disabled`
/// (termasuk varian `mix_*`) ditarik keluar dari `attrs` menjadi field elemen,
/// lalu dihapus dari attrs (pre-pass). `hx_if` dan `hx_visible` dianggap identik
/// karena iced 0.13 tak punya "visible" yang menjaga keberadaan widget.
fn extract_behavior(e: &mut ast::Element) {
    e.cond = take_behavior_attr(e, &["hx_if", "hx_visible", "mix_if", "mix_visible"]);
    e.disabled = take_behavior_attr(e, &["hx_disabled", "mix_disabled"]);
    for child in &mut e.children {
        if let ast::Node::Element(ce) = child {
            extract_behavior(ce);
        }
    }
}

/// Ambil atribut nama pertama yang cocok, hapus dari attrs, kembalikan nilainya.
fn take_behavior_attr(e: &mut ast::Element, names: &[&str]) -> Option<TokenStream> {
    let idx = e.attrs.iter().position(|(n, _)| names.contains(&n.as_str()))?;
    e.attrs.remove(idx).1
}

/// Atribut yang di-`hx_hoist` pada induk disebar (dalam) ke seluruh tag turunan,
/// kecuali yang sudah menulis attr itu sendiri atau di-`hx_disinherit`. Pre-pass
/// dijalankan sebelum codegen; menghapus `hx_hoist`/`hx_disinherit` setelah dipakai.
fn apply_inheritance(e: &mut ast::Element, inherited: &[(String, Option<TokenStream>)]) {
    let disinherit = attr_names(e, "hx_disinherit");
    let mut active: Vec<(String, Option<TokenStream>)> = inherited
        .iter()
        .filter(|(n, _)| !disinherit.iter().any(|d| d == n))
        .cloned()
        .collect();
    let hoist = attr_names(e, "hx_hoist");
    for (n, v) in &e.attrs {
        if hoist.iter().any(|h| h == n) {
            active.push((n.clone(), v.clone()));
        }
    }
    e.attrs.retain(|(n, _)| n != "hx_hoist" && n != "hx_disinherit");
    for child in &mut e.children {
        if let ast::Node::Element(ce) = child {
            let cd = attr_names(ce, "hx_disinherit");
            for (n, v) in &active {
                let already = ce.attrs.iter().any(|(an, _)| an == n);
                if !already && !cd.iter().any(|d| d == n) {
                    ce.attrs.push((n.clone(), v.clone()));
                }
            }
            apply_inheritance(ce, &active);
        }
    }
}

/// Daftar nama attr dari nilai literal string `name` (`hx_hoist="a b"` /
/// `hx_disinherit="a b"`); selain literal string -> kosong (abai).
fn attr_names(e: &ast::Element, key: &str) -> Vec<String> {
    e.attrs
        .iter()
        .find(|(n, _)| n == key)
        .and_then(|(_, v)| v.as_ref())
        .and_then(|v| codegen::attr_text(v).ok())
        .map(|s| s.split_whitespace().map(|w| w.to_string()).collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strip(ts: TokenStream) -> String {
        ts.to_string().chars().filter(|c| !c.is_whitespace()).collect()
    }

    #[test]
    fn expands_basic_tree() {
        let out = expand(quote::quote! {
            <column spacing=10 padding=20>
                <text>Hello</text>
                <button on_press=Message::Inc>+</button>
            </column>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("Column::new()"), "{s}");
        assert!(s.contains("extend"), "{s}");
        assert!(s.contains("spacing(10)"), "{s}");
        assert!(s.contains("padding(20)"), "{s}");
        assert!(s.contains("text(\"Hello\")"), "{s}");
        assert!(s.contains("on_press(Message::Inc)"), "{s}");
        assert!(!s.contains("column!["), "{s}");
    }

    #[test]
    fn mixed_text_becomes_format() {
        let out = expand(quote::quote! { <text>Counter: {value}</text> }).unwrap();
        let s = strip(out);
        assert!(s.contains("format!("), "{s}");
        assert!(s.contains("value"), "{s}");
    }

    #[test]
    fn self_close_rule() {
        let out = expand(quote::quote! { <rule/> }).unwrap();
        assert!(strip(out).contains("horizontal_rule(1.0)"));
    }

    #[test]
    fn unmatched_close_is_error() {
        let out = expand(quote::quote! { <column><row></column></row> });
        assert!(out.is_err());
    }

    #[test]
    fn layout_with_expr_child() {
        let out = expand(quote::quote! { <column>{some_element}</column> }).unwrap();
        let s = strip(out);
        assert!(s.contains("spread(some_element)"), "{s}");
        assert!(!s.contains("error"), "{s}");
    }

    #[test]
    fn vec_of_elements_spreads() {
        let out = expand(quote::quote! {
            <column>{vec_of_elements}</column>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("Spread::spread(vec_of_elements)"), "{s}");
        assert!(s.contains("extend"), "{s}");
    }

    #[test]
    fn editor_widget_positional() {
        let out = expand(quote::quote! {
            <editor state={&self.doc} on_action={move |a| Msg::Edit(a)}
                font={iced::Font::MONOSPACE}/>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("text_editor(&self.doc)"), "{s}");
        assert!(s.contains("on_action(move|a|Msg::Edit(a))"), "{s}");
        assert!(s.contains("font(iced::Font::MONOSPACE)"), "{s}");
    }

    #[test]
    fn bare_attr_is_noarg_method() {
        let out = expand(quote::quote! {
            <container center_x><text>a</text></container>
        })
        .unwrap();
        assert!(strip(out).contains("center_x()"));
    }

    #[test]
    fn html_aliases() {
        let out = expand(quote::quote! {
            <div>
                <button onclick=Message::Go>ok</button>
                <hr/>
            </div>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("container("), "{s}");
        assert!(s.contains("on_press(Message::Go)"), "{s}");
        assert!(s.contains("horizontal_rule(1.0)"), "{s}");
        assert!(!s.contains("div"), "{s}");
    }

    #[test]
    fn stateful_widget_positional_args() {
        let out = expand(quote::quote! {
            <slider range={0.0_f32..=100.0} value={self.volume}
                on_change=Message::VolumeChanged/>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("slider(0.0_f32..=100.0,self.volume,Message::VolumeChanged)"), "{s}");
    }

    #[test]
    fn stateful_widget_missing_arg_is_error() {
        let out = expand(quote::quote! { <input value={self.name}/> });
        assert!(out.is_err());
    }

    #[test]
    fn list_tags_map_to_column() {
        for tag in ["ul", "ol", "menu"] {
            let src: TokenStream =
                format!("<{tag} spacing=4><text>a</text><text>b</text></{tag}>").parse().unwrap();
            let s = strip(expand(src).unwrap());
            assert!(s.contains("Column::new()"), "{tag}: {s}");
        }
    }

    #[test]
    fn semantic_container_tags() {
        for tag in ["nav", "navbar", "aside", "form", "dialog", "fieldset", "figure", "blockquote", "hgroup", "search"] {
            let src: TokenStream = format!("<{tag}><text>isi</text></{tag}>").parse().unwrap();
            let s = strip(expand(src).unwrap());
            assert!(s.contains("container("), "{tag}: {s}");
            assert!(!s.contains("vertical_rule"), "{tag}: {s}");
        }
    }

    #[test]
    fn headings_have_size_and_bold() {
        let out = expand(quote::quote! { <h1>"judul"</h1> }).unwrap();
        let s = strip(out);
        assert!(s.contains("size(32f32)"), "{s}");
        assert!(s.contains("Weight::Bold"), "{s}");
    }

    #[test]
    fn text_roles_presets() {
        let strong = strip(expand(quote::quote! { <strong>"penting"</strong> }).unwrap());
        assert!(strong.contains("Weight::Bold"), "{strong}");

        let code = strip(expand(quote::quote! { <code>"let x = 1"</code> }).unwrap());
        assert!(code.contains("Font::MONOSPACE"), "{code}");

        let small = strip(expand(quote::quote! { <small>"catatan"</small> }).unwrap());
        assert!(small.contains("size(12f32)"), "{small}");

        let mark = strip(expand(quote::quote! { <mark>"disorot"</mark> }).unwrap());
        assert!(mark.contains("from_rgba8"), "{mark}");
    }

    #[test]
    fn anchor_is_button() {
        let out = expand(quote::quote! {
            <a onclick=Message::Go>"buka"</a>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("button(\"buka\")"), "{s}");
        assert!(s.contains("on_press(Message::Go)"), "{s}");
    }

    #[test]
    fn textarea_is_editor() {
        let out = expand(quote::quote! {
            <textarea state={&self.doc} on_action={move |a| Msg::Edit(a)}/>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("text_editor(&self.doc)"), "{s}");
        assert!(s.contains("on_action(move|a|Msg::Edit(a))"), "{s}");
    }

    #[test]
    fn select_is_picklist() {
        let out = expand(quote::quote! {
            <select options={vec!["Ringan", "Sedang"]} selected={Some("Ringan")}
                on_selected={move |t: &str| Msg::Gent(t.to_string())}/>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("pick_list(vec![\"Ringan\",\"Sedang\"],Some(\"Ringan\"),move|t:&str|Msg::Gent(t.to_string()))"), "{s}");
    }

    #[test]
    fn meter_is_progress() {
let out = expand(quote::quote! {
            <meter range={0.0_f32..=100.0} value={self.p}/>
        })
        .unwrap();
        assert!(strip(out).contains("progress_bar(0.0_f32..=100.0,self.p)"));
    }

    #[test]
    fn radio_is_radio_widget() {
        let out = expand(quote::quote! {
            <radio label={"Ringan"} value={0} selected={Some(0)} on_selected={Msg::Gent}/>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("radio(\"Ringan\",0,Some(0),Msg::Gent)"), "{s}");
    }

    #[test]
    fn password_masks_input() {
        let out = expand(quote::quote! {
            <password placeholder="sandi" value={sandi}/>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("text_input("), "{s}");
        assert!(s.contains(".password()"), "{s}");
    }

    #[test]
    fn range_is_slider_and_switch_is_toggler() {
        let range = strip(expand(quote::quote! {
            <range range={0.0_f32..=1.0} value={0.5} on_change={Msg::Upd}/>
        }).unwrap());
        assert!(range.contains("slider(0.0_f32..=1.0,0.5"), "{range}");
        let switch = strip(expand(quote::quote! {
            <switch is_checked={on}/>
        }).unwrap());
        assert!(switch.contains("toggler(on)"), "{switch}");
    }

    #[test]
    fn input_type_aliases() {
        for tag in ["email", "url", "tel"] {
            let src: TokenStream =
                format!("<{tag} placeholder=\"x\" value={{self.v}}/>").parse().unwrap();
            let s = strip(expand(src).unwrap());
            assert!(s.contains("text_input("), "{tag}: {s}");
        }
    }

    #[test]
    fn dl_list_and_dt_dd() {
        let out = expand(quote::quote! {
            <dl spacing=4><dt>"Istilah"</dt><dd>"Definisi"</dd></dl>
        }).unwrap();
        let s = strip(out);
        assert!(s.contains("Column::new()"), "{s}");
        assert!(s.contains("Weight::Bold"), "{s}");
        assert!(s.contains("text(\"Definisi\")"), "{s}");
    }

    #[test]
    fn spacer_is_fill_space() {
        let out = expand(quote::quote! { <spacer/> }).unwrap();
        let s = strip(out);
        assert!(s.contains("Space::with_width(::iced::Length::Fill)"), "{s}");
    }
#[test]
    fn big_text_is_larger() {
        let out = expand(quote::quote! { <big>"besar"</big> }).unwrap();
        let s = strip(out);
        assert!(s.contains("size(18f32)"), "{s}");
    }

    #[test]
    fn src_literal_embeds_asset() {
        for (tag, ctor) in [("img", "image"), ("svg", "svg"), ("icon", "svg")] {
            let src: TokenStream =
                format!("<{tag} src=\"assets/x.svg\"/>").parse().unwrap();
            let s = strip(expand(src).unwrap());
            assert!(s.contains("include_bytes!"), "{tag}: {s}");
            let needle = if ctor == "svg" {
                "svg::Handle::from_memory"
            } else {
                "image::Handle::from_bytes"
            };
            assert!(s.contains(needle), "{tag}: {s}");
            assert!(s.contains("CARGO_MANIFEST_DIR"), "{tag}: {s}");
        }
    }

    #[test]
    fn src_expression_passthrough() {
        let out = expand(quote::quote! { <img src={handle}/> }).unwrap();
        let s = strip(out);
        assert!(s.contains("image(handle)"), "{s}");
        assert!(!s.contains("include_bytes!"), "{s}");
    }

    #[test]
    fn button_with_icon_child() {
        let out = expand(quote::quote! {
            <button on_press={Msg::Go} padding=4><icon src="assets/i.svg" width={iced::Length::Fixed(20.0)}/></button>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("button(::iced::widget::svg(::iced::widget::svg::Handle::from_memory"), "{s}");
        assert!(s.contains("on_press(Msg::Go)"), "{s}");
        assert!(s.contains("width(iced::Length::Fixed(20.0))"), "{s}");
    }

    #[test]
    fn container_style_css() {
        let out = expand(quote::quote! {
            <container style="background:#111; color:#eee; padding:8px 12px; border:1px solid #ccc; shadow:0 1 #00000080">
                <text>a</text>
            </container>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("padding([8.0,12.0])"), "{s}");
        assert!(s.contains("background=Some((::iced::Color::from_rgba8(17u8,17u8,17u8,1.0)).into())"), "{s}");
        assert!(s.contains("text_color=Some(::iced::Color::from_rgba8(238u8,238u8,238u8,1.0))"), "{s}");
        assert!(s.contains("container::Style"), "{s}");
        assert!(s.contains("s.border=::iced::Border"), "{s}");
        assert!(s.contains("s.shadow=::iced::Shadow"), "{s}");
        assert!(s.contains("|_t|"), "{s}");
    }

    #[test]
    fn text_style_css() {
        let out = expand(quote::quote! {
            <text style="color:#eee; font-size:13; text-align:center">judul</text>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("color(::iced::Color::from_rgba8(238u8,238u8,238u8,1.0))"), "{s}");
        assert!(s.contains("size(13.0)"), "{s}");
        assert!(s.contains("align_x(::iced::alignment::Horizontal::Center)"), "{s}");
        assert!(!s.contains("Style"), "{s}");
    }

    #[test]
    fn rule_style_css() {
        let out = expand(quote::quote! { <rule style="color:#444"/> }).unwrap();
        let s = strip(out);
        assert!(s.contains("rule::Style"), "{s}");
        assert!(s.contains("color:::iced::Color::from_rgba8(68u8,68u8,68u8,1.0)"), "{s}");
        assert!(s.contains("fill_mode:::iced::widget::rule::FillMode::Full"), "{s}");
    }

    #[test]
    fn button_style_css() {
        let out = expand(quote::quote! {
            <button style="background:#007acc; border-radius:4">ok</button>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("button::Style"), "{s}");
        assert!(s.contains("|_t,_s|"), "{s}");
        assert!(s.contains("radius:::iced::border::Radius::from(4.0)"), "{s}");
    }

    #[test]
    fn unknown_style_property_is_error() {
        let out = expand(quote::quote! {
            <container style="bakground:#111"><text>a</text></container>
        });
        assert!(out.unwrap_err().to_string().contains("tak dikenal"));
    }

    #[test]
    fn style_expression_not_parsed_as_css() {
        // `style={expr}` diteruskan apa adanya sebagai method `.style(expr)`.
        let out = expand(quote::quote! {
            <container style={my_style_fn}><text>a</text></container>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains(".style(my_style_fn)"), "{s}");
        assert!(!s.contains("style=|"), "{s}");
    }

    #[test]
    fn hx_event_alias_maps_to_on_press() {
        let out = expand(quote::quote! {
            <button hx_click=Message::Go>ok</button>
        })
        .unwrap();
        assert!(strip(out).contains("on_press(Message::Go)"));
    }

    #[test]
    fn mix_event_alias_maps_identik() {
        let out = expand(quote::quote! {
            <text_input placeholder="x" value="y" mix_input={move |s| Msg::T(s)}/>
        })
        .unwrap();
        assert!(strip(out).contains("on_input(move|s|Msg::T(s))"));
    }

    #[test]
    fn hx_deferred_token_gives_clear_error() {
        let out = expand(quote::quote! {
            <button hx_trigger="click delay:300ms">ok</button>
        });
        let e = out.unwrap_err().to_string();
        assert!(e.contains("hx_trigger"), "{e}");
        assert!(e.contains("v0.1"), "{e}");
    }

    #[test]
    fn hx_poll_error_hints_subscription_hook() {
        let out = expand(quote::quote! {
            <div hx_poll="every:2s"><text>a</text></div>
        });
        let e = out.unwrap_err().to_string();
        assert!(e.contains("hx_poll"), "{e}");
        assert!(e.contains("mrust_runtime::interval"), "{e}");
        assert!(e.contains("subscription()"), "{e}");
    }

    #[test]
    fn unknown_hx_token_is_error() {
        let out = expand(quote::quote! {
            <button hx_buton=Message::Go>ok</button>
        });
        let e = out.unwrap_err().to_string();
        assert!(e.contains("hx_buton"), "{e}");
        assert!(e.contains("tak dikenal"), "{e}");
    }

    #[test]
    fn hx_value_binds_text_input() {
        match expand(quote::quote! {
            <input placeholder="cari" hx_value={self.q} hx_value_to=Msg::QueryChanged/>
        }) {
            Ok(out) => {
                let s = strip(out);
                assert!(s.contains("text_input("), "{s}");
                assert!(s.contains("&(self.q)"), "{s}");
                assert!(s.contains("on_input(move|v|Msg::QueryChanged(v))"), "{s}");
            }
            Err(e) => panic!("EXPAND_ERR: {e}"),
        }
    }

    #[test]
    fn hx_bind_synonym_works() {
        let out = expand(quote::quote! {
            <input placeholder="email" hx_bind={self.email} hx_bind_to=Msg::EmailChanged/>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("&(self.email)"), "{s}");
        assert!(s.contains("on_input(move|v|Msg::EmailChanged(v))"), "{s}");
    }

    #[test]
    fn hx_value_without_to_has_no_on_input() {
        let out = expand(quote::quote! {
            <input placeholder="x" hx_value={self.q}/>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("&(self.q)"), "{s}");
        assert!(!s.contains("on_input"), "{s}");
    }

    #[test]
    fn hx_value_on_non_input_still_deferred() {
        let out = expand(quote::quote! {
            <container hx_value={self.q}><text>a</text></container>
        });
        let e = out.unwrap_err().to_string();
        assert!(e.contains("hx_value"), "{e}");
        assert!(e.contains("v0.2"), "{e}");
    }

    #[test]
    fn hx_hoist_propagates_to_descendants() {
        let out = expand(quote::quote! {
            <column hx_hoist="size" size=14>
                <text>a</text>
                <text>c</text>
            </column>
        })
        .unwrap();
        let s = strip(out);
        // dua text sama-sama dapat .size(14)
        assert!(s.contains("size(14)"), "{s}");
        // meta attr hx_hoist tidak ikut jadi method
        assert!(!s.contains("hx_hoist"), "{s}");
        assert!(!s.contains("hx_hoist(14f32)"), "{s}");
    }

    #[test]
    fn hx_hoist_child_override_wins() {
        let out = expand(quote::quote! {
            <column hx_hoist="size" size=14>
                <text size=30>a</text>
                <text>c</text>
            </column>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("size(30)"), "{s}");
        assert!(s.contains("size(14)"), "{s}");
    }

    #[test]
    fn hx_disinherit_blocks_hoist() {
        let out = expand(quote::quote! {
            <column hx_hoist="size" size=14>
                <text hx_disinherit="size">a</text>
                <text>c</text>
            </column>
        })
        .unwrap();
        let s = strip(out);
        assert!(!s.contains("hx_disinherit"), "{s}");
        assert!(s.contains("size(14)"), "{s}");
    }

    #[test]
    fn hx_if_wraps_widget_in_condition() {
        let out = expand(quote::quote! {
            <button hx_if={show} hx_click=Message::Go>"ok"</button>
        })
        .unwrap();
        let s = strip(out);
        // meta hx_if tidak jadi method
        assert!(!s.contains("hx_if"), "{s}");
        // dibungkus if_elem (reaktivitas)
        assert!(s.contains("if_elem(show,("), "{s}");
        assert!(s.contains("button(\"ok\")"), "{s}");
    }

    #[test]
    fn hx_visible_sinonim_dari_if() {
        let out = expand(quote::quote! {
            <text hx_visible={is_on}>"selalu"</text>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("if_elem(is_on,("), "{s}");
        assert!(!s.contains("hx_visible"), "{s}");
    }

    #[test]
    fn hx_nested_cond_wraps_child() {
        let out = expand(quote::quote! {
            <column>
                <text hx_visible={a}>"x"</text>
                <text>y</text>
            </column>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("if_elem(a,("), "{s}");
        assert!(s.contains("text(\"y\")"), "{s}");
    }

    #[test]
    fn hx_disabled_button_uses_on_press_maybe() {
        let out = expand(quote::quote! {
            <button hx_disabled={busy} hx_click=Message::Go>"kirim"</button>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("on_press_maybe(ifbusy{None}else{Some(Message::Go)})"), "{s}");
    }

    #[test]
    fn hx_disabled_accepts_mix_alias_event() {
        let out = expand(quote::quote! {
            <button hx_disabled={busy} mix_press=Message::Go>"kirim"</button>
        })
        .unwrap();
        let s = strip(out);
        assert!(s.contains("on_press_maybe(ifbusy{None}else{Some(Message::Go)})"), "{s}");
    }

    #[test]
    fn hx_disabled_non_button_is_error() {
        let out = expand(quote::quote! {
            <text hx_disabled={busy}>"x"</text>
        });
        let e = out.unwrap_err().to_string();
        assert!(e.contains("hx_disabled"), "{e}");
        assert!(e.contains("button"), "{e}");
    }

    #[test]
    fn hx_disabled_button_without_event_is_error() {
        let out = expand(quote::quote! {
            <button hx_disabled={busy}>"kirim"</button>
        });
        assert!(out.is_err());
    }
}
