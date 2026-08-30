# PRD — `mrust-macro`: Markup HTML-like untuk GUI Iced (tetap `.rs`)

> **Referensi API & token lengkap ada di [`api-token.md`](api-token.md)** — mulai
> leksikal, tag dasar, ekstensi HTML, kosakata v0.4, styling CSS, sampai seluruh
> token `hx_*`/`mix_*` dengan contoh penggunaan, tanpa terkecuali. Dokumen ini
> (PRD) menetapkan tujuan & desain; api-token.md menetapkan cara pakai tiap token.

| | |
|---|---|
| Nama proyek | mrust-macro (`view!{}`) |
| Status | v0.1.0 — berjalan (dipakai demo counter) |
| Versi PRD | 1.0 |
| Tanggal | 2026-08-29 |
| Stack | Rust, Iced 0.13, proc-macro (`proc-macro2` + `quote`) |
| Kategori | Library deklaratif — produktivitas penulisan UI |

---

## 1. Ringkasan Produk

**mrust-macro** adalah proc-macro bernama `view!{}` yang membuat penulisan GUI
[Iced](https://iced.rs/) terasa seperti menulis HTML — tetapi tetap ditulis
sebagai kode Rust di dalam file `.rs`. Token markup dibaca pada waktu kompilasi
dan diubah menjadi kode `iced::widget::...` idiomatik biasa. Tidak ada runtime,
tidak ada file template eksternal, tidak ada interpretasi. Keluaran macro sama
persis dengan kode Iced yang ditulis manual:

```rust
// sebelum  ->  builder-chain bertingkat, sulit dibaca
let col = iced::widget::column![text("A"), button("B").on_press(Msg::Go)]
    .spacing(20)
    .padding(40);

// sesudah ->  deklaratif, mirip HTML
view! {
    <column spacing=20 padding=40>
        <text>A</text>
        <button on_press=Msg::Go>B</button>
    </column>
}
```

## 2. Latar Belakang & Masalah

- Iced menghasilkan UI lewat builder-chain: `row![...].push(...).spacing(...).padding(...)`.
  Semakin dalam layout bersarang, kode makin sulit dibaca dan rawan error kurung.
- Solusi umum lain (file `.html`/`.slint` eksternal) memecah basis kode dan
  butuh parser/codegen terpisah.
- Cara "murahan tapi baik": jadikan markup sebagai **macro Rust** sehingga:
  - tetap satu bahasa, satu file `.rs` (semua fitur IDE/`cargo` jalan);
  - dikompilasi, bukan diinterpretasi → error terdeteksi sejak compile-time;
  - keluaran = kode Iced normal, bisa dicampur bebas dengan kode Iced biasa;
  - nol overhead runtime.

## 3. Tujuan & Metrik Sukses

1. Satu macro `view!{}` yang menerima markup HTML-ish di dalam `.rs`.
2. Ekspansi menghasilkan kode Iced murni — **bebas kode `unsafe`, nol runtime**.
3. Kesalahan markup (tag tak ditutup, penutup tak cocok) = error kompilasi
   dengan pesan jelas berbahasa Indonesia.
4. Layout yang sama dengan kode manual: hasil render identik.
5. **Metrik:** demo counter dikompilasi dengan `cargo build` tanpa error, dan
   unit-test parser lulus (`cargo test`).

## 4. Ruang Lingkup

### 4.1 In-scope (v0.1)

- Macro `view!{}` dengan token standar awal (bab 6).
- Tag layout: `row`, `column`, `stack`.
- Tag kontainer satu-anak: `container`, `scrollable`.
- Tag leaf: `text`, `button`, dan konstruktor leaf generik
  `iced::widget::{nama}` untuk widget lain.
- Tag `rule` → `horizontal_rule`, `vertical_rule`.
- Attribute `name=value` → method chain `.name(value)`.
- Ekspresi Rust di dalam `{ ... }` (kondisional, variabel, elemen hasil).
- Error kompilasi untuk markup yang salah.

### 4.2 Out-of-scope (v0.1)

- Mesin styling/`class=`/CSS-like.
- Parsing file `.html` eksternal (tetap harus `.rs`).
- Rendering/interpretasi runtime.
- Event shorthand (`+click`, `+input`, dst).
- Widget stateful multi-argumen (`text_input`, `checkbox`, `slider`,
  `pick_list`, `toggler`, `radio`) — masuk **v0.2** (bab 11).
- Hot-reload.

## 5. Terminologi

| Istilah | Arti |
|---|---|
| `tag` | Nama widget, `<text>`, `<column>`, dst. |
| `attribute` | Pasangan `nama=nilai` di dalam tag pembuka. |
| `text node` | Teks literal di dalam tag, `>Hello<`. |
| `expr node` | `{ ekspresi }` — robot Rust mentah disisipkan. |
| `element` | Satu unit tag lengkap ` <tag a=...> ... </tag>`. |
| `leaf` | Widget tanpa anak (isi = teks). |
| `container` | Widget yang membungkus elemen lain. |
| `layout` | Widget multi-anak (`row`, `column`, `stack`). |

## 6. Spesifikasi Token Standar Awal (v0.1)

### 6.1 Token & Leksikal

Token dibaca langsung dari token stream Rust (buatan compiler), bukan dari
string. Karena itu **spesifikasi whitespace tidak relevan** — `spacing=10`,
`spacing = 10`, atau `spacing=10,padding=2` semuanya sah.

| Token | Contoh | Keterangan |
|---|---|---|
| `<` `>` | `<text>` | pembuka / penutup tag |
| `</` `/>` | `</text>` `<space/>` | tag penutup / tag self-close |
| `=` | `spacing=10` | pemisah attribute |
| `,` | `spacing=10, padding=4` | pemisah attribute opsional |
| `ident` | `column`, `on_press` | nama tag / nama attribute |
| literal | `"Hello"` | teks/konten berupa string |
| `{ }` | `{self.value}` | ekspresi Rust |

### 6.2 Grammar (EBNF)

```
markup   := element+
element  := open_tag  ( "/>" |  ">" node* close_tag )
open_tag := "<" ident attr*
close_tag:= "</" ident ">"
attr     := ident "=" value
value    := TokenStream            (* sampai awal attribute berikutnya,
                                      tanda `>`, `/>`, atau `,` *)
node     := element | literal | "{" expr "}"
```

### 6.3 Tabel Tag Standar Awal

| Tag | Ekspansi `iced` | Jenis | Anak? |
|---|---|---|---|
| `<row>` | `::iced::widget::row![ ... ]` | layout | ≥1 elemen |
| `<column>` | `::iced::widget::column![ ... ]` | layout | ≥1 elemen |
| `<stack>` | `::iced::widget::stack![ ... ]` | layout | ≥1 elemen |
| `<container>` | `::iced::widget::container(x)` | kontainer | 1 anak (banyak → dibungkus `column!` otomatis) |
| `<scrollable>` | `::iced::widget::scrollable(x)` | kontainer | 1 anak (banyak → dibungkus `column!` otomatis) |
| `<text>` | `::iced::widget::text(konten)` | leaf | teks/`{expr}` |
| `<button>` | `::iced::widget::button(konten)` | leaf | teks/`{expr}` |
| `<rule>` | `::iced::widget::horizontal_rule(1.0)` | leaf | — |
| `<horizontal_rule>` | `::iced::widget::horizontal_rule(1.0)` | leaf | — |
| `<vertical_rule>` | `::iced::widget::vertical_rule(1.0)` | leaf | — |
| *tag lain apa pun* | `::iced::widget::{tag}(konten)` | leaf | teks/`{expr}` |

Aturan **fallback generik**: tag di luar tabel dipanggang menjadi
`::iced::widget::{nama}(konten)` + attribute. Jika nama tag bukan konstruktor
Iced yang sah, compiler membalas dengan error tipe biasa — tetap dihitung
sebagai error compile-time, bukan runtime.

### 6.4 Attribute

- Setiap `nama=nilai` menjadi `.nama(nilai)` pada widget (urutan sesuai
  penulisan).
- `nilai` boleh literal, path (`Message::Inc`), ekspresi grup `{...}`, atau
  ekspresi apa pun yang bisa ditulis sebagai token Rust.
- **Peta namanya langsung ke method Iced** → kalau API salah, compiler
  menunjuk ke lokasi attribute tersebut.
- Mengganti pemisah koma: `spacing=10, padding=40` dan `spacing=10 padding=40`
  sama-sama sah.

### 6.5 Node Konten

- `literal` tunggal → jadi argumen constructor: `<text>Hello</text>` →
  `text("Hello")`.
- `{expr}` tunggal → argumen langsung: `<text>{nama}</text>` →
  `text(nama)`.
- Campuran literal + `{expr}` → digabung `::std::format!(...)`:
  `<text>Halo, {nama}!</text>` → `text(format!("{}{}{}", "Halo, ", nama, "!"))`.
- Teks bebas di dalam tag layout (`<column>teks...</column>`) = **error**
  kompilasi.
- **Batas teks polos** (`ponytail:`): teks di luar string literal hanyalah
  token Rust — karakter non-token (emoji, `—`, `…`) gagal di-lekser dan spasi
  antar-token hilang. Untuk teks bebas/spasi, selalu tulis string literal:
  `<text>"teks bebas, aman"</text>`.

### 6.6 Ekstensi Full-Mirip-HTML (v0.2)

**Alias tag** (tetap menuju widget Iced asli):

| Tag alias | -> iced |
|---|---|
| `<div>`, `<section>`, `<article>`, `<main>`, `<header>`, `<footer>` | `container(...)` |
| `<p>`, `<label>` | `text(...)` |
| `<hr/>` | `horizontal_rule(1.0)` |
| `<br/>` | `Space::with_height(Length::Fixed(12.0))` |
| `<input .../>` | `text_input(placeholder, value)` + `.on_input` |
| `<checkbox .../>` | `checkbox(label, is_checked)` + `.on_toggle` |
| `<toggler .../>` | `toggler(is_checked)` + `.on_toggle` |
| `<slider .../>` | `slider(range, value, on_change)` |
| `<progress .../>` | `progress_bar(range, value)` |
| `<image/>` / `<img/>` / `<svg/>` | `image(src)` / `svg(src)` |

**Aturan stateful widget:** attribute yang terdaftar menjadi argumen
constructor sesuai urutan tabel; attribute lain menjadi method chain.
Nilai `text_input` selain literal string di-stringify otomatis
(`&std::format!("{}", expr)`).

**Alias event:** `onclick`→`on_press`, `oninput`→`on_input`,
`onchange`→`on_change`, `ontoggle`→`on_toggle`, `onsubmit`→`on_submit`,
`onselect`→`on_select`, `oncancel`→`on_cancel`.

**Attribute boolean** tanpa `=nilai` → method tanpa argumen:
`<container center_x>` → `.center_x()`.

### 6.7 Iterasi Dinamis & Editor (v0.3)

**Anak `{ ekspresi }`** pada layout/container sekarang menerima **satu
`Element` ATAU `Vec<Element>`** (hasil `.map().collect()` untuk list dinamis),
di-spread lewat trait `Spread` di crate `mrust-runtime`:

```rust
<row>
    {files.iter().enumerate().map(|(i, f)| /* ...Element... */).collect::<Vec<_>>()}
</row>
```

`view!{}` memerlukan dependensi `mrust-runtime` hanya bila ada anak `{ }` pada
layout/container.

**Tag `editor` / `text_editor`** → `iced::widget::text_editor(state)` +
method (`on_action`, `font`, `size`, dst). State adalah `text_editor::Content`
yang dimutasi lewat `content.perform(action)`.

**Layout/container internal**: dibangun dengan `Row/Column/Stack::new().extend(...)`
— child tag di-`once(value).into()`, child `{ }` via `Spread` (bukan lagi
token makro `row!/column!`).

### 6.8 Kosakata Tag HTML (v0.4)

Tabel pemetaan lengkap tag ke konstruksi iced:

**Layout (multi-anak):**

| Tag | Keluaran | |
|---|---|---|
| `<row>` | `Row::new()` | |
| `<column>` | `Column::new()` | |
| `<stack>` | `Stack::new()` (overlay) | |
| `<ul>` / `<ol>` / `<menu>` / `<dl>` | `Column::new()` (daftar HTML `<dl>` = istilah + definisi) | nama HTML, stack iced |

**Container (satu/multi-anak → `container(...)`):**

`<div>` `<main>` `<section>` `<article>` `<header>` `<footer>` `<aside>`
`<nav>` `<navbar>`* `<form>` `<dialog>` `<fieldset>` `<figure>` `<blockquote>`
`<hgroup>` `<search>` `<scrollable>` (→ `scrollable(...)`).

*`navbar` alias ergonomis; nama HTML aslinya `<nav>`.

**Teks (leaf, `text(konten)`+preset):**

| Tag | Preset default |
|---|---|
| `<h1>`–`<h6>` | ukuran 32/26/22/18/15/13 + **bold** |
| `<strong>` `<b>` `<dt>` | bold (`dt` = istilah daftar) |
| `<code>` `<kbd>` `<samp>` `<pre>` | `Font::MONOSPACE` |
| `<small>` | ukuran 12 |
| `<big>` | ukuran 18 |
| `<mark>` | warna sorot kuning |
| `<span>` `<em>` `<i>` `<var>` `<cite>` `<dfn>` `<abbr>` | teks biasa (`em`/`i` tanpa italic, iced belum punya) |
| `<p>` `<label>` `<li>` `<dd>` `<legend>` `<caption>` `<figcaption>` | teks biasa |

Attribute pengguna selalu menggantikan preset (diterapkan setelahnya).

**Widget stateful (positional / self-close):**

| Tag | Constructor iced | Argumen |
|---|---|---|
| `<textarea>` | `text_editor(state)` | `state` (+ `on_action`) |
| `<select>` | `pick_list(options, selected, on_selected)` | ketiganya wajib |
| `<meter>` | `progress_bar(range, value)` | |
| `<radio>` | `radio(label, value, selected, on_selected)` | via `on_selected` (HTML `onchange`) |
| `<password>` | `text_input(placeholder, value)` + `.password()` | teropaque |
| `<email>` `<url>` `<tel>` | `text_input(placeholder, value)` | tipe input HTML |
| `<range>` | `slider(range, value, on_change)` | alias `<input type=range>` |
| `<switch>` | `toggler(is_checked)` | alias visual toggler |
| `<input>` `<checkbox>` `<toggler>` `<slider>` `<progress>` `<image>` `<svg>` `<editor>` | seperti v0.2/v0.3 | |

**Klik:** `<a>` `<link>` → `button(...)` (isi teks/ekspresi; event `onclick`/`on_press`).
`<button>` juga boleh berisi **satu widget** (mis. `<button><icon …/></button>`).
**Pengisi:** `<spacer/>` → `Space::with_width(Fill)` (dorong isi ke ujung baris); `<br/>` → `Space::with_height(12)`.

**Asset (`src` literal = di-embed):**

`<img src="x.png">` → `image::Handle::from_bytes(include_bytes!(CARGO_MANIFEST_DIR/x.png))`;
`<svg src="x.svg">` dan `<icon src="x.svg">` → `svg::Handle::from_memory(include_bytes!(...))`.
Literal string `src` dibungkus `include_bytes!` relatif ke `CARGO_MANIFEST_DIR` pemanggil (asset ikut program, ala HTML membawa assetnya); `src={ekspresi}` diteruskan apa adanya (`Handle` pas). Ikon butuh fitur iced `svg`, raster butuh `image`.

## 7. Mekanisme

`view!{}` adalah proc-macro dengan pipeline 3 tahap, berjalan 100% di
compile-time:

```
 token stream Rust        TokenStream AST          TokenStream iced
 ──────────────►  Parser  ────────────►  Codegen  ──────────►  Rust compiler
<tag a=b>…</tag>   recursive-descent     quote! → ::iced::widget::…
                       manual, tanpa syn     + .method(value)
```

1. **Parser** (`Parser` di `src/lib.rs`): membaca token `< > </ /> = { }`,
   ident, dan literal dari `TokenStream`, lalu membangun AST `Node`/`Element`.
   Tanda kurung `{...}` datang sebagai satu token grup compiler, sehingga
   ekspresi Rust bersarang aman tanpa parsing manual.
2. **AST**: `Element { name, attrs, children, span }`.
3. **Codegen** (`gen_element` + `gen_children` + `gen_content_args` +
   `apply_attrs`): menyusun token output `iced::widget::...` via `quote!`.
   Anak layout dikeluarkan apa adanya — `Element` menyediakan `From` untuk
   semua widget, jadi tidak perlu `.into()` di setiap anak (malah bikin
   ambiguitas tipe).

Keluaran adalah **kode Iced normal** — tidak ada runtime/library baru, dapat
dicampur dalam fungsi `view()` yang sama dengan elemen yang ditulis manual.

## 8. Panduan Penggunaan

### 8.1 Setup

```toml
# Cargo.toml aplikasi
[dependencies]
iced = "0.13"
mrust-macro = { path = "../mrust-macro" }
```

```rust
use mrust_macro::view;
```

### 8.2 Contoh lengkap (counter)

```rust
use iced::widget::text;
use iced::{Element, Sandbox, Settings};
use mrust_macro::view;

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
                {/*
                    `{...}` menyisipkan ekspresi Rust apa pun.
                    Ini kasus berguna: menyisipkan Element hasil kondisi.
                */}
                {if self.value > 0 {
                    text("nilai positif").into()
                } else {
                    text("nol atau negatif").into()
                }}
            </column>
        }
        .into()
    }
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}
```

### 8.3 Pola penggunaan

| Kebutuhan | Cara |
|---|---|
| Layout tumpuk | `<column>` (vertikal), `<row>` (horizontal), `<stack>` (overlay) |
| Bungkus satu anak | `<container>`, `<scrollable>` |
| Teks/tekanan | `<text>`, `<button>` |
| Event | `<button on_press=Message::Foo>` → `.on_press(Message::Foo)` |
| Nilai dinamis | `<text>Halo, {nama}!</text>` → `format!` |
| Kondisional/list | isi `{ ... }` dengan ekspresi/loop Rust yang mengembalikan `Element` |
| Styling | pakai attribute apa pun yang dikenali Iced, mis. `<container style=...>` |
| Campur kode manual | `view!{ <column>...</column> }` dan `iced::widget::...` bebas berada dalam satu fungsi |

### 8.4 Aturan mapping (ringkas)

| Masukan | Keluaran |
|---|---|
| `<tag>konten</tag>` (leaf) | `::iced::widget::tag(konten)` |
| `<row>` / `<column>` / `<stack>` | `::iced::widget::row![anak, ...]` dan kembalinya |
| `<container>` / `<scrollable>` | `::iced::widget::container(anak)` (+bungkus kolom bila banyak) |
| `<rule>` | `::iced::widget::horizontal_rule(1.0)` |
| attribute `nama=v` | `.nama(v)` |
| `{expr}` di konten | argumen / di-`format!` |
| `{expr}` di layout | disisipkan apa adanya sebagai anak |

## 9. Penanganan Error

Semua error adalah **error kompilasi** (span menunjuk lokasi asal token).

| Kasus | Pesan |
|---|---|
| Tag tidak ditutup | `tag \`column\` tidak ditutup` |
| Tag penutup tak cocok | `` tag penutup `</row>` tak cocok dengan pembuka `<column>` `` |
| Layout tanpa anak | `tag \`row\` butuh minimal satu anak` |
| Container tanpa anak | `tag \`container\` butuh minimal satu anak` |
| Teks bebas di layout | `teks bebas hanya boleh dipakai sebagai isi widget leaf` |
| Leaf diberi anak elemen | `` tag `text` bukan kontainer; tidak boleh punya anak `` |
| Attribute nilai kosong | `nilai attribute di tag \`x\` tidak boleh kosong` |
| Atom tak dikenal | `node tidak dikenal: harapkan <tag>, literal, atau { ekspresi }` |
| `view!{}` kosong | `view!{} tidak boleh kosong` |
| API Iced salah (mis. atribut tak ada) | error tipe Rust biasa dari compiler |

## 10. Dependensi & Kompatibilitas

- Rust (edition 2021 atau 2024), proc-macro stable.
- `proc-macro2` 1.x, `quote` 1.x.
- Iced **0.13** (uji).
- Macro meng-hardcode path `::iced::` — pemakai tidak boleh me-rename nama
  crate iced. (`ponytail:` kalau perlu rename crate, tambahkan config nama
  crate ke pemanggilan macro.)
- `view!{}` mengembalikan widget mentah; ubah ke `Element` dengan `.into()`
  di pemanggil.

## 11. Versi & Roadmap

| Versi | Isi |
|---|---|
| ~~v0.1~~ | Token standar awal, layout dasar, leaf generik, ekspresi `{}`, self-close, error kompilasi |
| ~~v0.2~~ (kini) | **Full mirip HTML**: alias tag (`div`, `p`, `hr`, `br`), alias event (`onclick`, `oninput`, `onchange`, `ontoggle`, dst), attribute boolean tanpa nilai (`center_x`), widget stateful (`input`, `checkbox`, `toggler`, `slider`, `progress`, `image`, `svg`), nilai `&str` di-stringify otomatis, literal angka→teks |
| ~~v0.3~~ (kini) | `editor`/`text_editor` (state + `on_action`), iterasi dinamis anak — `{ ekspresi }` menerima `Element` ATAU `Vec<Element>` via crate `mrust-runtime`, layout/container dibangun dengan `.extend()` |
| ~~v0.4~~ (kini) | **Kosakata tag HTML**: `ul`/`ol`/`menu`, container semantik (`nav`, `aside`, `form`, `dialog`, `fieldset`, `figure`, `blockquote`, `hgroup`, `search`, `navbar`), judul `h1`–`h6`, role teks (`strong`, `code`, `kbd`, `samp`, `pre`, `small`, `big`, `mark`, `span`, `em`, `i`, `var`, `cite`, `dfn`, `abbr`, `dt`), `<a>`/`<link>`→button, `<textarea>`→editor, `<select>`→pick_list, `<meter>`→progress_bar, `<radio>`, `<password>`, `<email>`/`<url>`/`<tel>`, `<range>`, `<switch>`, `<spacer/>`, **asset: `src` literal di-embed `include_bytes!` pada `<img>`/`<svg>`/`<icon>`, `<button>` berisi-widget** |
| v0.5-ish | **Mesin styling `style="…css…"` di mrust-macro**: compile-time parse CSS per jenis widget (text/container/button/layout/rule), warna hex/nama/rgb(a), `padding`, `border`, `shadow`, `gap`, ukuran/font, alignment — dipakai di demo2 (status bar & explorer ganti closure manual) |
| v0.5 | `class=`/map style untuk tema reusable; `<ul>` daftar ber-titik/bernomor |
| v0.6 | Namespace widget custom; iterasi tingkat atas; `for`-`in` bawaan |

Prioritas drive by kebutuhan nyata, bukan spekulasi (`ponytail:`).

## 12. Kriteria Sukses & Verifikasi

1. `cargo test --workspace` → unit-test parser & codegen lulus (warning kebanyakan).
2. `cargo build --workspace` → seluruh workspace terkompilasi tanpa error.
3. `cargo run --package demo` → jendela counter Iced tampil; `+1`/`-1`/`reset`
   bekerja; baris status berubah sesuai nilai.
4. Ekspansi `view!{}` = kode Iced murni (tidak ada makhluk runtime baru).

## 13. Lampiran — Contoh Ekspansi

```rust
// masukan
view! {
    <column spacing=20>
        <text>Halo, {nama}!</text>
        <button on_press=Message::Submit>kirim</button>
    </column>
}

// keluaran (disederhanakan)
::iced::widget::column![
    ::iced::widget::text(::std::format!("{}{}{}", "Halo, ", nama, "!")),
    ::iced::widget::button("kirim").on_press(Message::Submit),
]
.spacing(20)
```