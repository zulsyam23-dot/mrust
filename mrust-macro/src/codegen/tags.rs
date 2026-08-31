//! Tabel tag/alias HTML → widget/method iced, plus metadata token `hx_*`.
//! Murni data; tidak ada logika generasi di sini.

/// Layout multi-anak -> macro `iced::widget::{tag}![]`.
/// `ul`/`ol`/`menu` (daftar HTML) dipetakan ke `Column`.
pub(crate) const LAYOUT_TAGS: &[&str] = &["row", "column", "stack", "ul", "ol", "menu", "dl", "tab_bar"];

pub(crate) const LAYOUT_ALIAS: &[(&str, &str)] = &[
    ("ul", "column"),
    ("ol", "column"),
    ("menu", "column"),
    ("dl", "column"),
    ("tab_bar", "row"),
];

/// Kontainer satu-anak -> `iced::widget::container(x)` / `scrollable(x)`.
pub(crate) const CONTAINER_TAGS: &[&str] = &[
    "container", "scrollable", "div", "section", "article", "main", "header", "footer",
    "aside", "nav", "navbar", "form", "dialog", "fieldset", "figure", "blockquote", "hgroup", "search",
    "card", "badge",
];

pub(crate) const RULE_TAGS: &[&str] = &["rule", "hr", "horizontal_rule", "vertical_rule", "divider", "separator"];

/// Alias bahasa alami -> text.
pub(crate) const TEXT_TAGS: &[&str] =
    &["p", "label", "li", "dd", "legend", "caption", "figcaption", "title", "time", "address"];

/// Tag yang attribute-nya menjadi argumen positional constructor.
/// Format: `(tag, constructor_iced, daftar_attribute_argumen_urutan)`
pub(crate) const POS_ARGS: &[(&str, &str, &[&str])] = &[
    ("input", "text_input", &["placeholder", "value"]),
    ("checkbox", "checkbox", &["label", "is_checked"]),
    ("toggler", "toggler", &["is_checked"]),
    ("slider", "slider", &["range", "value", "on_change"]),
    ("progress", "progress_bar", &["range", "value"]),
    ("image", "image", &["src"]),
    ("img", "image", &["src"]),
    ("svg", "svg", &["src"]),
    ("icon", "svg", &["src"]),
    ("editor", "text_editor", &["state"]),
    ("text_editor", "text_editor", &["state"]),
    ("textarea", "text_editor", &["state"]),
    ("select", "pick_list", &["options", "selected", "on_selected"]),
    ("meter", "progress_bar", &["range", "value"]),
    ("radio", "radio", &["label", "value", "selected", "on_selected"]),
    ("password", "text_input", &["placeholder", "value"]),
    ("email", "text_input", &["placeholder", "value"]),
    ("url", "text_input", &["placeholder", "value"]),
    ("tel", "text_input", &["placeholder", "value"]),
    ("range", "slider", &["range", "value", "on_change"]),
    ("switch", "toggler", &["is_checked"]),
    ("field", "text_input", &["placeholder", "value"]),
    ("textbox", "text_input", &["placeholder", "value"]),
    ("number", "text_input", &["placeholder", "value"]),
    ("search", "text_input", &["placeholder", "value"]),
    ("date", "text_input", &["placeholder", "value"]),
    ("month", "text_input", &["placeholder", "value"]),
    ("week", "text_input", &["placeholder", "value"]),
];

/// Tag kontruksi diikuti method chain tambahan (dipanggil tanpa argumen)
/// setelah attribute pengguna — mis. `<password .../>` -> `.password()`.
pub(crate) const EXTRA_METHODS: &[(&str, &str)] = &[("password", "password")];

/// Alias attribute ala-HTML -> method iced.
pub(crate) const ATTR_ALIASES: &[(&str, &str)] = &[
    ("onclick", "on_press"),
    ("oninput", "on_input"),
    ("onchange", "on_change"),
    ("ontoggle", "on_toggle"),
    ("onsubmit", "on_submit"),
    ("onselect", "on_select"),
    ("oncancel", "on_cancel"),
    // sinonim interaktif ala-htmx: `hx-*` -> method iced
    ("hx_press", "on_press"),
    ("hx_click", "on_press"),
    ("hx_enter", "on_press"),
    ("hx_close", "on_press"),
    ("hx_exit", "on_press"),
    ("hx_input", "on_input"),
    ("hx_type", "on_input"),
    ("hx_submit", "on_submit"),
    ("hx_change", "on_change"),
    ("hx_toggle", "on_toggle"),
    ("hx_switch", "on_toggle"),
    ("hx_select", "on_select"),
    ("hx_cancel", "on_cancel"),
    ("hx_edit", "on_edit"),
    ("hx_action", "on_action"),
    ("hx_drag", "on_drag"),
    ("hx_release", "on_release"),
    ("hx_focus", "on_focus"),
    ("hx_blur", "on_blur"),
    ("hx_scroll", "on_scroll"),
    // sinonim `mix_*` -> method iced (identik dengan `hx_*`)
    ("mix_press", "on_press"),
    ("mix_click", "on_press"),
    ("mix_enter", "on_press"),
    ("mix_close", "on_press"),
    ("mix_exit", "on_press"),
    ("mix_input", "on_input"),
    ("mix_type", "on_input"),
    ("mix_submit", "on_submit"),
    ("mix_change", "on_change"),
    ("mix_toggle", "on_toggle"),
    ("mix_switch", "on_toggle"),
    ("mix_select", "on_select"),
    ("mix_cancel", "on_cancel"),
    ("mix_edit", "on_edit"),
    ("mix_action", "on_action"),
    ("mix_drag", "on_drag"),
    ("mix_release", "on_release"),
    ("mix_focus", "on_focus"),
    ("mix_blur", "on_blur"),
    ("mix_scroll", "on_scroll"),
];

/// Token behavior `hx_*`/`mix_*` yang DIKETAHUI tapi belum di-generate pada
/// iterasi implementasi ini (membutuhkan mekanisme/runtime — lihat prd2.md).
/// Saat dikenali, memberi error jelas ber-version alih-alih diam-diam jadi
/// method `hx_*` yang tidak pernah ada (error tipe membingungkan).
pub(crate) const HX_DEFERRED: &[&str] = &[
    "hx_trigger", "mix_trigger",
    "hx_confirm", "mix_confirm",
    "hx_value", "mix_value",
    "hx_value_to", "mix_value_to",
    "hx_bind", "mix_bind",
    "hx_vals", "mix_vals",
    "hx_include", "mix_include",
    "hx_poll", "mix_poll",
    "hx_busy", "mix_busy",
    "hx_indicator", "mix_indicator",
    "hx_on", "mix_on",
    "hx_hoist", "mix_hoist",
    "hx_disinherit", "mix_disinherit",
];

pub(crate) fn hx_version(token: &str) -> &'static str {
    let base = token.strip_prefix("mix_").unwrap_or_else(|| token.strip_prefix("hx_").unwrap_or(&token[3..]));
    match base {
        "trigger" | "confirm" => "v0.1",
        "value" | "value_to" | "bind" | "vals" | "include" => "v0.2",
        "poll" | "busy" | "indicator" | "on" | "hoist" | "disinherit" => "v0.3",
        _ => "v0.x",
    }
}

/// Panduan singkat untuk token yang masih deferred: apa yang harus developer
/// tulis manual karena makro view tak bisa menjangkaunya (butuh state/wiring
/// di level aplikasi). `hx_poll` punya helper runtime siap-salin.
pub(crate) fn hx_hint(token: &str) -> &'static str {
    let base = token
        .strip_prefix("mix_")
        .or_else(|| token.strip_prefix("hx_"))
        .unwrap_or(token);
    match base {
        "poll" => "pasang `Subscription::batch(vec![mrust_fw::interval(dtk, Msg::X)])` di `fn subscription()` app Anda",
        "trigger" => "butuh timer/state antar-event — tulis manual dgn `Subscription` (Debounce/throttle tak ada di iced)",
        "confirm" => "butuh state mesin konfirmasi di app (gate + overlay Batal/Lanjut) — tulis manual",
        "vals" | "include" => "butuh akses field state app utk mengumpulkan payload — tulis manual",
        "busy" | "indicator" => "butuh state busy bool lintas-widget di app — tulis manual",
        "on" => "`on_press`/`on_input` hanya menerima satu Message; buat varian multi-efek di update() Anda",
        _ => "lihat prd2.md",
    }
}

/// Tag teks ala-HTML -> preset ukuran Text (heading).
pub(crate) const HEADINGS: &[(&str, f32)] = &[
    ("h1", 32.0), ("h2", 26.0), ("h3", 22.0), ("h4", 18.0), ("h5", 15.0), ("h6", 13.0),
];

#[derive(Clone, Copy)]
pub(crate) enum TextRole {
    Bold,
    Mono,
    Small,
    Mark,
    Big,
    Plain,
}

pub(crate) const TEXT_ROLES: &[(&str, TextRole)] = &[
    ("strong", TextRole::Bold),
    ("b", TextRole::Bold),
    ("dt", TextRole::Bold),
    ("code", TextRole::Mono),
    ("kbd", TextRole::Mono),
    ("samp", TextRole::Mono),
    ("pre", TextRole::Mono),
    ("small", TextRole::Small),
    ("mark", TextRole::Mark),
    ("big", TextRole::Big),
    ("var", TextRole::Plain),
    ("cite", TextRole::Plain),
    ("dfn", TextRole::Plain),
    ("abbr", TextRole::Plain),
    ("span", TextRole::Plain),
    ("em", TextRole::Plain),
    ("i", TextRole::Plain),
];

/// Tag klik HTML -> button iced (leaf: isi teks/ekspresi).
pub(crate) const CLICKABLE_TAGS: &[&str] = &["a", "link", "tab"];
