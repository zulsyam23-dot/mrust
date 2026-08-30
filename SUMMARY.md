# Proyek mrust — Anchor Summary

## Goal
Editor Rust (demo2) mirip VSCode 1:1 (title/menu bar, activity bar, Explorer + Search side panel, tab editor, output panel, status bar, palet komando), semua UI dirakit lewat tag HTML mrust-macro v0.4 (`nav`, `h1–h6`, `select`, `textarea`, `ul/menu`, dst), bukan widget iced manual di dalam `view!{}`.

## Constraints & Preferences
- Iced: `iced` 0.13.1, `iced_core` 0.13.2, `iced_widget` 0.13.4 = sumber kebenaran signature (registry: `C:\Users\PC\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\`)
- `Edit` enum 0.13 TIDAK punya Undo/Redo; `alignment::Horizontal` = `{Left, Center, Right}` (bukan `Start`)
- `container::Style` harus closure `impl Fn(&Theme) -> Style`, bukan nilai `Style` langsung (E0277 kalau dipakai mentah)
- `<div>`/container butuh minimal satu anak (spacer `<div/>` kosong = error codegen "tag X butuh minimal satu anak")
- Macro `view!{}` mengembalikan widget mentah (mis. `Column`); assign ke `let x: Element = view!{...}.into()` wajib `.into()`
- demo1 (v0.2) harus tetap compile

## Progress
### Done
- **Fase macro v0.4 selesai** (29/29 test lulus, clippy bersih):
  - Kosakata HTML di `codegen.rs`: `ul/ol/menu`→Column; `dl`→Column (daftar istilah; `dt`→bold, `dd`→text); `nav/navbar/aside/form/dialog/fieldset/figure/blockquote/hgroup/search/div/main/section/article/header/footer`→container; `h1–h6` heading bold (32/26/22/18/15/13); `strong/b/dt`→bold, `code/kbd/samp/pre`→mono, `small`→12px, `big`→18px, `mark`→warna (255,190,60), `span/em/i/var/cite/dfn/abbr`→plain (em/i tanpa italic — iced 0.13 tak punya); `a/link`→button; `textarea`→text_editor, `select`→pick_list (options/selected/on_selected), `meter`→progress_bar, `radio`→radio(label,value,selected,on_selected), `password`→text_input+.password(), `email/url/tel`→text_input, `range`→slider, `switch`→toggler, `br`→Space(12), `spacer`→Space::with_width(Fill)
  - **Asset embed**: `src="path"` literal → `include_bytes!(concat!(env!(CARGO_MANIFEST_DIR),"/",src))`: `<img>`→`image::Handle::from_bytes`, `<svg>`/`<icon>`→`svg::Handle::from_memory(Cow::Borrowed(&..[..]))`; ekspresi `src={..}` passthrough. `<button>` boleh berisi satu widget (`<button><icon …/></button>`)
  - Mekanisme baru: `EXTRA_METHODS` (hook method chain wajib — `.password()`) DIJALANKAN SETELAH attribute user; Jebakan: (a) tag di `LAYOUT_ALIAS` (`dl`) juga wajib di `LAYOUT_TAGS`; (b) `svg::Handle` tak punya `from_bytes` — hanya `from_memory`; (c) `contains(&format!(..))` kena E0277 Pattern — pakai `.as_str()`
  - PRD §6.8 kosakata + roadmap di-sync (v0.4 = sekarang; **v0.5-ish mesin styling `style="…css…"` selesai**; sisa v0.5 = `class=`/`<ul>` list bullets)
- **Fase editor multi-proyek** (awal): `projects.rs` (projects.txt + last.txt persistensi), root proyek diingat, file tree relatif proyek
- **Fase VSCode — playback selesai dan hijau (dipakai token kosakata baru):**
  - `title.rs`: title bar + menu Arsip/Tampilan/Bantuan (`<spacer/>` untuk dorong isi kanan; tombol "Panel" diganti `<switch is_checked={panel_open}>`); data di kiri, Panel + Palet Komando di kanan
  - `activity.rs`: activity bar 42px, ikon `<icon src="assets/icons/explorer.svg">`/`search.svg` + mark `<icon dot.svg>`, bg SIDEBG
  - `tree.rs`: Explorer ala VSCode — caret SVG (caret-right/down.svg), font proporsional, indent 12px, row aktif bg highlight `(0.09,0.14,0.21)` ganti `•`; header "Penjelajah"; bg sidebar lewat `style="background:#1a1c21"`
  - `tabs.rs`: header tab satu blok menyatu ala VSCode — `container` per tab berisi nama + × (`<icon close.svg>`), tombol transparan; aktif = blok terang (PANELC), dirty mark `●`
  - `status.rs`: status bar biru; kiri = proyek/rendering/`N masalah`; kanan = baris:col | Spasi:4 | UTF-8 | Rust | CRLF
  - `output.rs`: panel KELUARAN 150px + log scrollable + × (`<spacer/>`)
  - `title.rs` + logo `<icon logo.svg>`
  - `overlay.rs`: palet komando (filter per `Cat`, tombol ✓), files, projects, about — About kini `<dl>`/`<dt>`/`<dd>` (meta), `<mark>`, `<ul>`/`<li>`, `<small>`
  - `app.rs`: `Cat` (Any/Arsip/Tampilan/Bantuan), `Modal`, `Cmd` (Palette/BukaFile/Simpan/SimpanSemua/BukaProyek/Panel/Explorer/Search/Tentang), `Pat`/`pats()`, `App` (`idx`, `tabs: Vec<usize>`, `active`, `docs: HashMap<usize,Content>`, `dirty`, `open: HashSet<String>`, `s_query`, `panel_open`, `log` cap 400), `run(cmd)`, `save`, `search_matches`, `spacer()`
  - `picker.rs` dihapus; `main.rs` = modul lengkap + judul "mrust-code — editor mirip VSCode (demo2)"
- **Error build fase VSCode yang sudah dikalahkan** (12 error + 4 warning → 0):
  1. `<div/>` kosong ×4 → ganti `spacer()` (`Space::with_width(Length::Fill)`) juga di title/status/output/tree
  2. `cannot find Cat in crate` → referensi `app::Cat` sudah benar
  3. `cannot find function tabs in module editor` → `tabs::bar(self)` (import `App` masih `pub use` di editor.rs)
  4. `E0277 style closure` ×2 → bungkus `|_| Style::default().background(...)`
  5. `E0599 no variant Undo/Redo` → `Cmd::Batal/Ulang` + menu Edit + `Cat::Edit` DIBUANG dari palet
  6. `E0308 mismatched types` → `let body` butuh `.into()` setelah `view!{}`
  7. `E0599 Horizontal::Start` → `Left`
  8. Dead code: `BLUE`/`CHROME` hapus; `Msg::Save`/`SaveAll` hapus (dipakai via `Cmd::Simpan/SimpanSemua`); import `Style/Color/Font/Length` bereskan
- **Verifikasi final**: `cargo build --package demo2` bersih (0 error, 0 warning); `cargo test --workspace` = 37/37 lulus (35 macro + 2 close-tab bounds); `cargo clippy --workspace --all-targets` bersih
- **Demo2 memakai token baru**: `<spacer/>` menggantikan `app::spacer()` (helper dihapus) di title/status/tree/output; `switch` di title bar; About pakai `dl/dt/dd`, `ul/li`, `mark`, `small`
- **Ikon**: `demo2/assets/icons/*.svg` (explorer, search, close, caret-right/down, dot, logo), dipakai lewat `<icon>` di activity bar / title / × buttons; fitur iced `svg` di demo2; veto awal: placeholder text ✓. `radio`/`range`/`password` tak punya slot alami — skip
- **Mesin styling `style="…css…"` (v0.5-ish) di mrust-macro**:
  - `WKind` per jenis widget (Text/Layout/Container/Button/Rule/Other) → pemetaan properti CSS type-safe; properti dikenal-umum yang tak berlaku diabaikan ala CSS, nama tak dikenal = error compile (typo terdeteksi)
  - Colors: `#rgb`/`#rrggbb`/`#rrggbbaa`, `rgb()`/`rgba()` (alpha 0..1), `transparent`, 18 nama; lebar: `12`/`12px`/`fill`/`50%`→`Length`
  - text: `color`,`font-size`,`font-weight`(bold/normal),`font-family`(mono),`text-align`,`align-y`,`width`,`height`; container: `background`,`color`(text_color),`border`/`border-width`/`border-color`/`border-radius`, `shadow`(`[x] [y] color`),`padding`(1|2 nilai),`width`,`height`; button sama minus padding; layout: `gap`/`spacing`,`width`,`height` (Row tak punya align_x/Column tak punya align_y → align di-CSS di-ignore); rule: `color` (rule::Style tak ber-Default → struct literal)
  - Button closure `|_t,_s|` (Status), Container/Rule `|_t|`
  - **Jebakan yang dilawan**: `Color::from_rgba8` alpha = `f32` & tak ada `from_rgb` (hanya `from_rgb8`); `padding_ts` pakai `[#a,#b]` literal (bukan `.replace_tokens`); `rule::Style`/`FillMode` tak ber-Default; Button/Rule tak punya `padding`/`width`-method → arm CSS disesuaikan
  - Dipakai nyata di demo2: status bar `style="background:#2670b8; color:white"`, explorer `style="background:#1a1c21"` (closure skin() dihapus) — bukti compile end-to-end

- **Refactor `codegen.rs` (1123 baris) → modul `src/codegen/`** (selesai, hijau):
  - `tags.rs` (tabel tag/alias + metadata token `hx_*` — murni data), `css.rs` (mesin styling `WKind`/`apply_css`/parser), `attrs.rs` (`apply_attrs`/`apply_attrs_kind`/`chain_attr`/`apply_disabled`), `widget.rs` (gen_leaf/layout/container/rule/positional/text/tooltip + font), `mod.rs` (dispatcher `gen_element` + `Child`/`gen_child`/`extend_all`/`attr_text` + re-export `gen_element`/`attr_text`/`apply_attrs`/`apply_disabled`)
  - lib.rs tak berubah: masih `codegen::gen_element` & `codegen::attr_text`
  - Verifikasi: 54 macro + 1 runtime test lulus, `cargo test --workspace` & `cargo clippy --workspace --all-targets` bersih, `demo3` build OK

- **`api-token.md` dibuat** — referensi API & token lengkap: lapisan 1–9 (leksikal, element/attribute, tag dasar v0.1, ekstensi HTML v0.2, iterasi/editor v0.3, kosakata v0.4, styling CSS, token `hx_*`/`mix_*` di-generate & deferred, runtime `mrust-runtime`) + setup + aturan mapping + verifikasi. prd.md & prd2.md kini menunjuknya utk cara pakai tiap token.

### In Progress
- (tidak ada) — semua hijau

### Blocked
- (tidak ada)

## Key Decisions
- Semua UI VSCode dirakit via `view!{}` + tag HTML mrust-macro — tujuan "1:1 dengan fitur macro"; hanya `Space`/`Element` baku di jalinan luar
- `<div>` tidak bisa kosong → `app::spacer()` (`Space::with_width(0.0)`/`with_height` di jalinan yang butuh tipe lain)
- Undo/Redo dihapus dari palet (iced 0.13 tak punya `Edit::Undo/Redo`) daripada memalsukan aksi
- `Modal::Palette(cat)` — menu bar buka palette dengan filter kategori (`query` awal = `cat_str(cat)`)
- Ikon activity bar karakter teks (`▦`, `◎`, `•`) karena font simbol tak tersedia
- `Input` value harus `String` (clone), tidak bisa `&str`
- `scrollable` tak boleh berisi konten `Height::Fill` (panic); TextEditor tidak dibungkus scrollable
- Warna VSCode di app.rs: `SIDEBG (0.12,0.13,0.15)`, `PANELC (0.11,0.12,0.14)`, `GREY (0.45,0.47,0.52)`, `LIGHT (0.88,0.89,0.92)`

## Next Steps
1. `cargo run --package demo2` — verifikasi visual: status bar biru & explorer sidebar tetap senada dengan CSS baru
2. Sisa roadmap v0.5 bila diminta: `class=`/map style (tema reusable) + `<ul>` daftar ber-titik/bernomor
3. v0.6 = namespace widget custom; `for`/`in` bawaan

## Critical Context
- Source of truth iced: `iced_core-0.13.2\src\text\editor.rs` (enum Edit/Action), `iced_core-0.13.2\src\alignment.rs` (Horizontal = Left/Center/Right)
- `container::Style::default().background(color)` + closure `Fn(&Theme)->Style`; `Style::background` bukan konstruktor statis
- `Space::with_width(0.0)`/`with_height(0.0)` untuk spacer; `Space::new()` butuh 2 arg
- `on_press={Msg::X}` nilai langsung; `on_input`/`on_action` = closure `move`
- `projects::read_last()` fallback `BASE` (= `CARGO_MANIFEST_DIR`); `projects.txt` + `last.txt` di `demo2/`
- Warning PowerShell `NativeCommandError` pada stdout cargo = noise biasa (bukan error build)

## Relevant Files
- `C:\Users\PC\Documents\mrust\api-token.md` — referensi API & token LENGKAP (leksikal → tag dasar → kosakata HTML → styling CSS → seluruh `hx_*`/`mix_*` di-generate & deferred + runtime) dengan contoh pemakaian, tanpa terkecuali
- `C:\Users\PC\Documents\mrust\prd.md` / `prd2.md` — PRD (tujuan & kontrak); menunjuk api-token.md utk cara pakai
- `C:\Users\PC\Documents\mrust\demo2\src\app.rs` — state inti VSCode (Msg/Cat/Modal/Cmd/App/run/save/search_matches/spacer)
- `C:\Users\PC\Documents\mrust\demo2\src\title.rs` — title bar + menu Arsip/Tampilan/Bantuan → `Modal::Palette(cat)`
- `C:\Users\PC\Documents\mrust\demo2\src\activity.rs` — activity bar ▦/◎ + mark • (42px, SIDEBG)
- `C:\Users\PC\Documents\mrust\demo2\src\tree.rs` — Explorer caret SVG, indent 12px, row aktif highlight; bg sidebar CSS
- `C:\Users\PC\Documents\mrust\demo2\src\status.rs` — status bar biru (bg via `style="background:#2670b8; color:white"`; kanan: baris/Spaces/UTF-8/Rust/CRLF)
- `C:\Users\PC\Documents\mrust\demo2\src\tabs.rs` — bar tab blok menyatu (nama + ×, aktif terang, test close bounds)
- `C:\Users\PC\Documents\mrust\demo2\src\output.rs` — panel KELUARAN 150px + log
- `C:\Users\PC\Documents\mrust\demo2\src\overlay.rs` — palet komando/files/projects/about (panel)
- `C:\Users\PC\Documents\mrust\demo2\src\editor.rs` — TextEditor per tab (highlight Rust, on_action)
- `C:\Users\PC\Documents\mrust\demo2\src\projects.rs` — persistensi projects.txt + last.txt
- `C:\Users\PC\Documents\mrust\demo2\src\main.rs` — modul lengkap + judul jendela; `picker.rs` dihapus
- `C:\Users\PC\Documents\mrust\mrust-macro\src\codegen\` — modul codegen (per-domain): `mod.rs` (dispatcher + Child/gen_child/extend_all + attr_text), `tags.rs` (tabel tag/alias + token hx_*), `css.rs` (mesin styling WKind), `attrs.rs` (apply_attrs/apply_disabled), `widget.rs` (gen_* + font)
- `C:\Users\PC\Documents\mrust\mrust-macro\src\lib.rs` — 54 unit test macro (6 tes style tambahan)
- `C:\Users\PC\Documents\mrust\mrust-macro\src\error.rs` — Error + impl Display (untuk assert test)
- `C:\Users\PC\Documents\mrust\prd.md` — §6.8 kosakata tag; roadmap v0.4 & styling v0.5-ish selesai

## Demo (urutan dasar → lanjutan manual)
- `demo/src/main.rs` — counter (lap 3–4): header, row, text, input, checkbox, slider, progress, button
- `demo2/src/main.rs` — lap 5: iterasi `.map().collect` + `<editor>` TextEditor
- `demo3/src/main.rs` — lap token `hx_*` end-to-end (stateful)
- `demo4/src/main.rs` — lap 6 kosakata v0.4: `<section>/<nav>/<dl>/<ul>/<select>/<radio>/<meter>`, enum Kategori
- `demo5/src/main.rs` — lap 7 styling CSS: container/button/`<hr>`/`<rule>`, style=..., border/shadow/radius
- `demo6/src/main.rs` — lap 8a event & kondisi: hx_click/mix_press/ontoggle/hx_visible/hx_if/hx_disabled
- `demo7/src/main.rs` — lap 8b binding & hoist: hx_value(_to)/hx_bind(_to)/hx_hoist/hx_disinherit
- `demo8/src/main.rs` — asset src: `<icon>`/`<svg>` (fitur iced `svg`), assets/icons/{dot,menu}.svg, Handle manual
- `demo9/src/main.rs` — runtime interval: `mrust_runtime::interval` + `Subscription::batch(Message::Tick)`
- `demo10/src/main.rs` — gabungan semua fitur + runtime polling (subscription interval)