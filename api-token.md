# Referensi API & Token — mrust-macro (`view!{}`) + mrust-fw

Panduan lengkap mulai dari unit terkecil (leksikal, tag dasar) sampai token
perilaku `hx_*`/`mix_*`, tanpa terkecuali. Seluruh token dijelaskan beserta
contoh penggunaan dan (bila ada) batasan/error-nya.

**Ringkasan lapisan API:**

| Lapisan | Isi | Status |
|---|---|---|
| 1. Leksikal | `<` `>` `</` `/>` `=` `,` `{ }`, ident, literal | Stable |
| 2. Element & attribute | tag, `name=value`, `name` (boolean), node konten | Stable |
| 3. Tag dasar v0.1 | `row`, `column`, `stack`, `container`, `scrollable`, `text`, `button`, `rule` | Stable |
| 4. Ekstensi HTML v0.2 | `<div>`, `<input>`, `<checkbox>`, `<slider>`, event alias `onclick` dsb., attribute boolean | Stable |
| 5. Iterasi & editor v0.3 | `{expr}` → `Element`/`Vec<Element>`, `<editor>`/`<text_editor>`, `Spread` | Stable |
| 6. Kosakata HTML v0.4 | `ul`/`ol`/`menu`/`dl`, container semantik, `h1–h6`, role teks, `<a>`, stateful, asset `src` | Stable |
| 7. Styling `style="…css…"` | parse CSS compile-time per jenis widget | Stable (v0.5-ish) |
| 8. Token perilaku `hx_*`/`mix_*` | event alias, `hx_if`/`hx_visible`, `hx_disabled`, `hx_value`/`hx_bind`, `hx_hoist`/`hx_disinherit` (di-generate); sisanya deferred | Campuran |
| 9. Runtime (di `mrust-fw`) | `Spread`, `if_elem`, `interval` | Stable |

---

## 1. Leksikal & Struktur Element

### Token leksikal

| Token | Contoh | Keterangan |
|---|---|---|
| `<` `>` | `<text>` | buka/tutup tag |
| `</` `/>` | `</text>`, `<space/>` | tag penutup / tag self-close |
| `=` | `spacing=10` | pemisah attribute |
| `,` | `spacing=10, padding=4` | pemisah attribute (opsional; spasi juga sah) |
| `ident` | `column`, `on_press` | nama tag / nama attribute |
| literal string | `"Hello"` | teks/konten |
| `{ ekspresi }` | `{self.value}` | ekspresi Rust (`proc_macro2::Group` Brace) |

Token dibaca langsung dari token-stream Rust (buatan compiler), **bukan** dari
string. Karena itu whitespace tak relevan: `spacing=10`, `spacing = 10`, dan
`spacing=10,padding=2` semuanya sah.

### Grammar (EBNF)

```
markup   := element+
element  := open_tag  ( "/>" |  ">" node* close_tag )
open_tag := "<" ident attr*
close_tag:= "</" ident ">"
attr     := ident ("=" value)?
value    := TokenStream   (* sampai `>`, `/>`, `,`, atau awal attr berikutnya *)
node     := element | literal | "{" expr "}"
```

### Element

Penguraian dibuat oleh `parser.rs` (recursive-descent manual, tanpa `syn`).
Hasil AST: `Element { name, attrs, children, span, cond, disabled }`.

- Tag **self-close** `<tag/>` → tanpa anak.
- Tag **buka–tutup** `<tag>…</tag>` → anak dari isi.
- Tag penutup harus cocok nama (`</column>` menutup `<column>`); tak cocok =
  error kompilasi.
- Tag tanpa penutup sampai akhir = error `tag "x" tidak ditutup`.

### Attribute

- `name=value` → method chain `.name(value)` pada widget (urutan penulisan).
- `name` saja (tanpa `=`) → **attribute boolean** → method tanpa argumen:
  `<container center_x>` → `.center_x()`.
- Nilai boleh literal, path (`Message::Inc`), ekspresi `{...}`, atau token Rust
  apa pun. Nilai berupa satu grup `{ ... }` dibuka kurungnya (isi diteruskan
  apa adanya); selain itu diteruskan mentah.
- Pemetaan nama **langsung ke method Iced** → API salah = error tipe Rust yang
  menunjuk lokasi attribute.
- Attribute nilai kosong (`name=`) = error `nilai attribute di tag "x" tidak boleh kosong`.

### Node konten

- `literal` tunggal → argumen constructor: `<text>Hello</text>` → `text("Hello")`.
- `{expr}` tunggal → argumen langsung: `<text>{nama}</text>` → `text(nama)`.
- Campuran literal + `{expr}` → digabung `::std::format!(...)`:
  `<text>Halo, {nama}!</text>` → `text(format!("…", …))`.
- Teks bebas di dalam tag **layout** = error kompilasi
  (`teks bebas hanya boleh dipakai sebagai isi widget leaf`).
- Leaf diberi anak elemen (<text>berisi <tag>) = error.
- `view!{}` kosong = error `view!{} tidak boleh kosong`. Lebih dari satu
  element tingkat atas dibungkus `column![...]` otomatis.

> **Batas teks polos (`ponytail:`):** teks di luar string literal hanyalah
> token Rust. Karakter non-token (emoji, `—`, `…`) gagal di-lekser dan spasi
> antar-token hilang. Untuk teks bebas/spasi, selalu tulis string literal:
> `<text>"teks bebas, aman"</text>`.

---

## 2. Tag Dasar (v0.1)

| Tag | Jenis | Ekspansi | Anak? |
|---|---|---|---|
| `<row>` | layout | `Row::new().extend(…)` | ≥1 |
| `<column>` | layout | `Column::new().extend(…)` | ≥1 |
| `<stack>` | layout | `Stack::new().extend(…)` (overlay) | ≥1 |
| `<container>` | kontainer | `container(x)` | 1 (banyak → dibungkus `column!`) |
| `<scrollable>` | kontainer | `scrollable(x)` | 1 (banyak → dibungkus `column!`) |
| `<text>…</text>` | leaf | `text(konten)` | teks/`{expr}` |
| `<button>…</button>` | leaf | `button(konten)` | teks/`{expr}` / 1 widget |
| `<rule/>` | leaf | `horizontal_rule(1.0)` | — |
| `<horizontal_rule/>` | leaf | `horizontal_rule(1.0)` | — |
| `<vertical_rule/>` | leaf | `vertical_rule(1.0)` | — |
| *tag lain* | leaf generik | `widget::{tag}(konten)` | teks/`{expr}` |

Internal layout/container dibangun dengan `Row/Column/Stack::new().extend(...)`:
- child tag → `extend(once((widget).into()))`
- child `{ }` → `extend(Spread::spread(expr))`

**Contoh:**

```rust
view! {
    <column spacing=20 padding=40>
        <text size=30>Counter: {self.value}</text>
        <row spacing=10>
            <button on_press=Message::Inc>+1</button>
            <button on_press=Message::Dec>-1</button>
        </row>
    </column>
}
```

Hasil multi-tag tingkat atas dibungkus `column![]`. Hasil `view!{}` adalah
widget mentah — ubah ke `Element` dengan `.into()` di pemanggil.

---

## 3. Ekstensi Alias HTML (v0.2)

### Alias tag → widget iced asli

| Tag | → iced |
|---|---|
| `<div>` `<section>` `<article>` `<main>` `<header>` `<footer>` `<aside>` `<nav>` `<navbar>` `<form>` `<dialog>` `<fieldset>` `<figure>` `<blockquote>` `<hgroup>` `<search>` `<card>` `<badge>` | `container(...)` |
| `<p>` `<label>` | `text(...)` |
| `<hr/>` | `horizontal_rule(1.0)` |
| `<br/>` | `Space::with_height(Fixed(12.0))` (tanpa anak) |
| `<spacer/>` | `Space::with_width(Fill)` (tanpa anak) |
| `<a>` `<link>` `<tab>` | `button(...)` |
| `<input …/>` | `text_input(placeholder, value)` |
| `<checkbox …/>` | `checkbox(label, is_checked)` |
| `<toggler …/>` | `toggler(is_checked)` |
| `<slider …/>` | `slider(range, value, on_change)` |
| `<progress …/>` | `progress_bar(range, value)` |
| `<image/>` `<img/>` | `image(src)` |
| `<svg/>` `<icon/>` | `svg(src)` |

### Alias event (ATTR_ALIASES)

| Alias HTML | → method iced |
|---|---|
| `onclick` | `on_press` |
| `oninput` | `on_input` |
| `onchange` | `on_change` |
| `ontoggle` | `on_toggle` |
| `onsubmit` | `on_submit` |
| `onselect` | `on_select` |
| `oncancel` | `on_cancel` |

---

## 4. Widget Stateful (Positional / Self-Close)

Attribute yang terdaftar menjadi **argumen constructor** sesuai urutan; attribute
lain (termasuk event) menjadi **method chain**. Tag stateful **harus** self-close
(`<tag/>`); bermuatan anak = error `tag "x" butuh self-close "/>"`.

| Tag | Constructor | Argumen wajib (urutan) |
|---|---|---|
| `<input>` `<field>` `<textbox>` | `text_input(placeholder, value)` | placeholder, value |
| `<password>` | `text_input(placeholder, value)` **+ `.password()`** | placeholder, value |
| `<email>` `<url>` `<tel>` `<number>` `<search>` `<date>` `<month>` `<week>` | `text_input(placeholder, value)` | placeholder, value |
| `<checkbox>` | `checkbox(label, is_checked)` | label, is_checked |
| `<toggler>` `<switch>` | `toggler(is_checked)` | is_checked |
| `<slider>` `<range>` | `slider(range, value, on_change)` | range, value, on_change |
| `<progress>` `<meter>` | `progress_bar(range, value)` | range, value |
| `<image>` `<img>` | `image(src)` | src |
| `<svg>` `<icon>` | `svg(src)` | src |
| `<editor>` `<text_editor>` `<textarea>` | `text_editor(state)` | state |
| `<select>` | `pick_list(options, selected, on_selected)` | options, selected, on_selected |
| `<radio>` | `radio(label, value, selected, on_selected)` | label, value, selected, on_selected |

Catatan:
- Nilai `text_input` selain literal string di-stringify otomatis:
  `&std::format!("{}", expr)`.
- `.password()` (via `EXTRA_METHODS`) dijalankan **setelah** attribute pengguna,
  sehingga pengguna masih bisa menimpa gaya.
- `options`/`selected`/`on_selected`/`is_checked`/`range`/etika: umumnya
  ekspresi `{...}`.

**Contoh:**

```rust
view! {
    <column spacing=8>
        <input placeholder="cari" value={self.q}
               on_input={move |v| Msg::Query(v)}/>
        <slider range={0.0_f32..=100.0} value={self.volume}
                on_change={Msg::Volume}/>
        <radio label={"Ringan"} value={0} selected={Some(0)}
               on_selected={Msg::Gent}/>
        <editor state={&self.doc} on_action={move |a| Msg::Edit(a)}
                font={iced::Font::MONOSPACE}/>
    </column>
}
```

---

## 5. Asset `src` (v0.4)

Literal string `src="path"` di-embed **compile-time** via `include_bytes!` relatif
`CARGO_MANIFEST_DIR`; ekspresi `src={handle}` diteruskan apa adanya.

| Tag + src literal | Keluaran |
|---|---|
| `<img src="x.png">` | `image::Handle::from_bytes(include_bytes!(...))` |
| `<svg src="x.svg">` `<icon src="x.svg">` | `svg::Handle::from_memory(Cow::Borrowed(&include_bytes!(...)[..]))` |

`<button>` boleh berisi **satu widget**: `<button padding=4><icon src="assets/i.svg" width={iced::Length::Fixed(20.0)}/></button>`.

---

## 6. Kosakata Konten & Preset (v0.4)

### Heading `h1`–`h6` (text + preset ukuran + bold)

| Tag | Ukuran |
|---|---|
| `<h1>` | 32 |
| `<h2>` | 26 |
| `<h3>` | 22 |
| `<h4>` | 18 |
| `<h5>` | 15 |
| `<h6>` | 13 |

### Role teks (preset font/warna/ukuran)

| Tag | Preset |
|---|---|
| `<strong>` `<b>` `<dt>` | **bold** (Font `Weight::Bold`) |
| `<code>` `<kbd>` `<samp>` `<pre>` | `Font::MONOSPACE` |
| `<small>` | ukuran 12 |
| `<big>` | ukuran 18 |
| `<mark>` | warna sorot `from_rgba8(255,190,60,1.0)` |
| `<span>` `<em>` `<i>` `<var>` `<cite>` `<dfn>` `<abbr>` | teks biasa (em/i tanpa italic; iced 0.13 belum punya) |
| `<p>` `<label>` `<li>` `<dd>` `<legend>` `<caption>` `<figcaption>` `<title>` `<time>` `<address>` | text biasa |

Attribute pengguna selalu **menimpa** preset (diterapkan setelahnya).

### Daftar HTML

| Tag | Keluaran |
|---|---|
| `<ul>` `<ol>` `<menu>` | `Column::new()` (daftar) |
| `<dl>` | `Column::new()` (istilah + definisi; `<dt>` bold, `<dd>` text) |
| `<li>` | `text` |
| `<dt>` | `text` bold |
| `<dd>` | `text` |

**Contoh:**

```rust
view! {
    <dl spacing=4>
        <dt>"Rust"</dt>
        <dd>"Bahasa sistem modern"</dd>
        <dt>"Iced"</dt>
        <dd>"GUI native"</dd>
    </dl>
}
```

---

## 7. Styling `style="…"` (v0.5-ish)

`style="css1; css2; …"` di-parse **compile-time** menjadi method / closure
`.style(...)` yang type-safe per jenis widget. Nilai `{expr}` diteruskan sebagai
method `.style(expr)` (bukan CSS).

### Format nilai

| Bentuk | Contoh | Keterangan |
|---|---|---|
| warna hex | `#333`, `#ff8800`, `#ff8800aa`, `#f80` | `rrggbb` / `rrggbbaa` / `rgb` |
| warna rgb(a) | `rgb(10,20,30)`, `rgba(10,20,30,0.5)` | alpha 0..1 (atau `50%`) |
| warna nama | `red`, `green`, `blue`, `black`, `white`, `yellow`, `orange`, `purple`, `gray`/`grey`, `silver`, `cyan`/`aqua`, `magenta`/`fuchsia`, `teal`, `navy`, `maroon`, `olive`, `brown`, `lime`, `transparent` | 19 nama |
| panjang | `12`, `12px`, `fill`, `50%` | → `Length::Fixed` / `Fill` / `FillPortion` |
| padding | `8px` | satu nilai → `f32`; dua nilai `8px 12px` → `[8.0, 12.0]` |
| border | `1px solid #ccc`, `#ccc`, `2px` | kata `solid/dashed/dotted/none` diabaikan |
| shadow | `<color>`, `<x> <color>`, `<x> <y> <color>` | |

### Properti per jenis widget (`WKind`)

**Text** (`<text>`, role teks):
`color`, `font-size`, `font-weight` (bold/normal), `font-family` (mono),
`text-align`/`align-x` (`left|start|center|right|end`), `align-y`
(`top|start|center|bottom|end`), `width`, `height`.

**Layout** (`row/column/stack`): `gap`/`spacing`, `width`, `height`.

**Container** (`<container>` dst): `background`, `color` (→ text_color),
`border`/`border-width`/`border-color`/`border-radius`, `shadow`,
`padding`, `width`, `height`. Closure `|_t|`.

**Button**: `background`, `color`, border set, `shadow`, `width`, `height`.
Closure `|_t,_s|` (Status).

**Rule**: `color`. (rule::Style tak ber-Default → struct literal.)

**Other** (stateful/leaf lain): `width`, `height`.

Properti dikenal umum yang tak berlaku untuk jenis widget tersebut diabaikan
(seperti CSS); nama properti tak dikenal = **error compile** (typo terdeteksi).

**Contoh:**

```rust
<container style="background:#111; color:#eee; padding:8px 12px;
                  border:1px solid #ccc; shadow:0 1 #00000080">
    <text>isi</text>
</container>
```

---

## 8. Token Perilaku `hx_*` / `mix_*`

Dua namespace identik: `hx_*` dan `mix_*` — keduanya sama-sama sah dan memetakan
hal yang sama. Semua token adalah **attribute** biasa pada tag `view!{}`.

Nilai berupa **literal string** (dileks jadi kontrol, pola `style="…"`) atau
**ekspresi `{expr}`** (diteruskan apa adanya) — tergantung token (lihat tabel tiap
token di bawah).

### 8.1 Event alias (`hx_*` / `mix_*` → method iced)

Nilai = `Message` atau closure, sama seperti `on_press=…`/`on_input=…`.

| Token (`hx_` / `mix_`) | → method iced |
|---|---|
| `hx_press` `hx_click` `hx_enter` `hx_close` `hx_exit` (`mix_*`) | `on_press` |
| `hx_input` `hx_type` (`mix_*`) | `on_input` |
| `hx_change` (`mix_*`) | `on_change` |
| `hx_toggle` `hx_switch` (`mix_*`) | `on_toggle` |
| `hx_select` (`mix_*`) | `on_select` |
| `hx_cancel` (`mix_*`) | `on_cancel` |
| `hx_submit` (`mix_*`) | `on_submit` |
| `hx_edit` (`mix_*`) | `on_edit` |
| `hx_action` (`mix_*`) | `on_action` |
| `hx_drag` (`mix_*`) | `on_drag` |
| `hx_release` (`mix_*`) | `on_release` |
| `hx_focus` (`mix_*`) | `on_focus` |
| `hx_blur` (`mix_*`) | `on_blur` |
| `hx_scroll` (`mix_*`) | `on_scroll` |

**Contoh:**

```rust
<button hx_click=Message::Go>"ok"</button>
<text_input value={self.q} mix_input={move |s| Msg::T(s)}/>
```

### 8.2 Dibangkitkan makro (bekerja penuh)

#### `hx_if={cond}` / `hx_visible={cond}` / `mix_if` / `mix_visible`

Render kondisional: widget ditampilkan bila `cond` benar, dikosongkan menjadi
`Space` tinggi-0 bila salah (via `mrust_fw::if_elem`). `hx_if` dan
`hx_visible` identik (iced 0.13 tak punya "visible" yang menjaga keberadaan
widget). Nilai **wajib ekspresi**.

```rust
<button hx_if={show} hx_click=Message::Go>"ok"</button>
<text hx_visible={is_on}>"selalu"</text>
```

#### `hx_disabled={cond}` / `mix_disabled`

Nonaktifkan interaksi selama `cond` benar. **Hanya untuk `<button>`**; pada
widget lain = error. Butuh event klik (mis. `hx_click`/`mix_press`/`onclick`).
Diterapkan via `on_press_maybe(if cond {None} else {Some(ev)})`.

```rust
<button hx_disabled={busy} hx_click=Message::Go>"kirim"</button>
```

#### Binding input: `hx_value` / `hx_bind` / `hx_value_to` / `hx_bind_to`

Keluarga **`text_input`** (input/field/textbox/password/email/dst):
- `hx_value={expr}` / `hx_bind={expr}` → mengisi argumen `value`.
- `hx_value_to=Msg::Variant` / `hx_bind_to=Msg::Variant` → menambah
  `.on_input(move |v| Msg::Variant(v))`.
- `hx_value` tanpa `hx_value_to` → hanya mengisi value (tanpa `on_input`).

```rust
<input placeholder="cari" hx_value={self.q} hx_value_to=Msg::QueryChanged/>
<input placeholder="email" hx_bind={self.email} hx_bind_to=Msg::EmailChanged/>
```

> Pada tag **selain** `text_input`, `hx_value`/`hx_bind` dsb. **deferred** →
> error ber-version (lihat §8.4).

#### Inheritance: `hx_hoist` / `hx_disinherit`

Atribut yang disebut `hx_hoist="a b"` pada **induk** disebar (dalam) ke seluruh
tag turunan (kecuali anak yang sudah menulis atribut itu sendiri); `hx_disinherit="a b"`
pada anak membatalkan warisan. Diproses pre-pass; token meta tak jadi method.

```rust
<column hx_hoist="align_x" align_x={iced::Alignment::Center}>
    <text>"kiri"</text>
    <text>"kanan"</text>
</column>
```

> **Batasan:** atribut yang di-hoist harus menjadi method yang **sah pada induk
> DAN anak**. `<column hx_hoist="size" size=14>` salah (Column tak punya
> `.size()`). Gunakan atribut valid bersama, mis. `align_x`. Properti yang hanya
> relevan di anak (mis. `size` teks) tulis langsung di anak.

### 8.3 Sinonim ringkas `hx_*` vs `mix_*`

Prefix `hx_*` → "htmx-like"; `mix_*` → "mrust" rasa lebih natural. **Identik**,
boleh dipakai bergantian di seluruh token di atas (event, if/visible, disabled,
value/bind, hoist/disinherit).

### 8.4 Token deferred (belum di-generate makro)

Token berikut **belum** di-generate makro karena butuh *state/wiring level
aplikasi* (`view!{}` hanya menghasilkan satu fragment widget; ia tak melihat
struct state maupun `fn subscription()`). Memakai salah satunya → **error
kompilasi ber-version** plus panduan singkat.

| Token (`hx_`/`mix_`) | Versi | Apa yang dibutuhkan manual |
|---|---|---|
| `hx_trigger` | v0.1 | trigger modifier (`once`/`changed`/`delay`/`throttle`) butuh timer/state antar-event → tulis manual dgn `Subscription` |
| `hx_confirm` | v0.1 | overlay konfirmasi stateful + gate di app |
| `hx_value` (non-input) | v0.2 | akses field state app utk payload |
| `hx_value_to` (non-input) | v0.2 | idem |
| `hx_bind` (non-input) | v0.2 | idem |
| `hx_vals` | v0.2 | kumpulkan nilai input → message; akses state app |
| `hx_include` | v0.2 | idem |
| `hx_poll` | v0.3 | polling timer — **ada helper** `mrust_fw::interval` siap-salin (lihat §9) |
| `hx_busy` | v0.3 | state busy bool lintas-widget di app |
| `hx_indicator` | v0.3 | idem |
| `hx_on` | v0.3 | `on_press`/`on_input` hanya satu Message; buat varian multi-efek di `update()` |
| `hx_hoist` | v0.3 | **bekerja** (lihat §8.2) — tercantum utk kelengkapan |
| `hx_disinherit` | v0.3 | **bekerja** (lihat §8.2) — tercantum utk kelengkapan |

Token `hx_*`/`mix_*` yang **tidak dikenali** (salah ketik) → error
`token "hx_x" tak dikenal (lihat PRD.md)` — bukan method `hx_x` yang
membingungkan.

---

## 9. Runtime (tergabung di `mrust-fw`)

`mrust-fw` (bukan crate terpisah) menyediakan helper `Spread`/`if_elem`/`interval`
serta makro `app!` dan re-export `view!`. Dependensi hanya ditarik bila fitur
yang memakainya dipakai.

```toml
[dependencies]
mrust-fw = { path = "../mrust-fw" }
```

### `Spread`

Anak `{ ekspresi }` pada layout/container menerima **satu `Element`** ATAU
**`Vec<Element>`** (hasil `.map().collect::<Vec<_>>()` untuk list dinamis).
Di-call via `mrust_fw::Spread::spread(expr)`.

```rust
<row>
    {files.iter().enumerate().map(|(i, f)| /* …Element… */).collect::<Vec<_>>()}
</row>
```

### `if_elem(cond, widget) -> Element`

Hasilkan widget atau kosongkan (Space tinggi-0) sesuai kondisi, bertipe
`Element` yang sama (menambatkan generik). Dipanggil makro oleh
`hx_if`/`hx_visible`. Biasanya tak perlu dipanggil manual.

### `interval(secs: f64, msg: Message) -> Subscription<Message>`

Polling ala-htmx `hx_poll="every:2s"`: `Subscription` memancarkan `Message`
(klon) tiap `secs` detik. Dipakai di `fn subscription()` app:

```rust
fn subscription(&self) -> Subscription<Message> {
    Subscription::batch(vec![
        mrust_fw::interval(2.0, Message::Tick),
    ])
}
```

Butuh fitur `tokio` dgn backend timer (lihat mrust-fw/Cargo.toml).
(`ponytail:` id interval = durasi → cukup utk kasus umum; gunakan id unik
per-elemen bila banyak polling identik-durasi.)

---

## 10. Setup & Pemakaian

```toml
[dependencies]
iced = "0.13"
mrust-fw = { path = "../mrust-fw" }   # memuat view!, Spread, if_elem, interval, app!
```

```rust
use mrust_fw::view;
use iced::{Element, Sandbox, Settings};

#[derive(Debug, Clone, Copy)]
enum Message { Inc, Dec, Reset }

#[derive(Default)]
struct Counter { value: i32 }

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self { Self::default() }
    fn title(&self) -> String { String::from("mrust counter") }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::Inc => self.value += 1,
            Message::Dec => self.value -= 1,
            Message::Reset => self.value = 0,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        view! {
            <column spacing=20 padding=40>
                <text size=30>Counter: {self.value}</text>
                <row spacing=10>
                    <button on_press=Message::Inc>+1</button>
                    <button on_press=Message::Dec>-1</button>
                </row>
            </column>
        }
        .into()
    }
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}
```

---

## 11. Aturan Mapping Ringkas

| Masukan | Keluaran |
|---|---|
| `<tag>konten</tag>` (leaf) | `::iced::widget::tag(konten)` |
| `<row>`/`<column>`/`<stack>` | `::iced::widget::row![anak, …]` / `Column::new().extend(…)` |
| `<container>`/`<scrollable>` | `container(anak)` (+bungkus column bila banyak) |
| `<rule/>` | `horizontal_rule(1.0)` |
| attribute `name=v` | `.name(v)` |
| attribute `name` (tanpa =) | `.name()` |
| `{expr}` di konten | argumen / di-`format!` |
| `{expr}` di layout | disisipkan apa adanya sebagai anak (Spread) |
| `style="…"` | parse CSS → method/closure `.style(...)` |
| `hx_*`/`mix_*` event | method event iced |
| `hx_if`/`hx_visible` | `if_elem(cond, widget)` |
| `hx_disabled` (button) | `on_press_maybe(if cond {None} else {Some(ev)})` |
| `hx_value`/`hx_bind` (text_input) | argumen `value` + opsional `.on_input` |
| `hx_hoist`/`hx_disinherit` | penyebaran atribut decendant (pre-pass) |

---

## 12. Verifikasi

- `cargo build --workspace` → seluruh workspace kompilasi tanpa error.
- `cargo test --workspace` → 54 tes macro + tes runtime lulus.
- `cargo clippy --workspace --all-targets` → bersih.
