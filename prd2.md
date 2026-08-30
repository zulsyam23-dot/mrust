# PRD 2 — `mrust-macro`: Token Deklaratif "ala-htmx" untuk GUI Iced (tetap `.rs`)

> **Referensi API & token lengkap ada di [`api-token.md`](api-token.md)** — mulai
> leksikal, tag dasar, kosakata HTML, styling CSS, sampai seluruh token `hx_*`/
> `mix_*` (di-generate & deferred) dengan contoh penggunaan, tanpa terkecuali.
> Dokumen ini (PRD) menetapkan *mengapa* & *kontrak*; api-token.md menetapkan
> *cara pakai* tiap token.

| | |
|---|---|
| Nama | mrust-macro — aksi/behavior deklaratif (`hx_*`) |
| Status | Draf — v0.1..v0.3 (roadmap) |
| Versi PRD | 2.0 |
| Tanggal | 2026-08-30 |
| Stack | Rust, Iced 0.13, proc-macro (`proc-macro2` + `quote`), `mrust-runtime` (opsional) |
| Kategori | Library deklaratif — wiring event/state/behavior tanpa boilerplate |

---

## 1. Ringkasan Produk

[htmx](https://htmx.org) membuat HTML bisa memicu behavior (request, swap, confirm,
polling) langsung dari atribut, tanpa menulis JavaScript. **mrust-macro versi "htmx"**
melakukan hal yang sama untuk GUI Iced: atribut `hx_*` pada tag `view!{}` diubah pada
waktu kompilasi menjadi *wiring* event → `Message`/closure, data-binding, dan behavior
(polling, konfirmasi, indicator) — tanpa menulis `on_*`/closure/`Subscription` manual.

Karena Iced **sudah reaktif** (ubah state → `view` di-render ulang), dua konsep inti
htmx — *target* & *swap* — **otomatis gratis**: mengubah state membawa UI ikut berubah.
Token `hx_*` di sini adalah **selubung kenyamanan** di atas event/state/`Command`/
`Subscription` yang sudah ada, bukan mesin AJAX/DOM.

```rust
// sebelum  ->  wiring manual: closure, message, state busy, dsb.
view! {
    <column>
        <text_input value={self.q}
            on_input={move |v| Msg::QueryChanged(v)}/>
        <button on_press=Msg::Run>cari</button>
        {if self.busy { some_loading_widget().into() } else { iced::widget::Space::new(0,0).into() }}
    </column>
}

// sesudah ->  deklaratif ala-htmx
view! {
    <column>
        <text_input hx_value={self.q}/>
        <button hx_press=Msg::Run>cari</button>
        <progress hx_visible={self.busy}/>
    </column>
}
```

## 2. Latar Belakang & Masalah

- `validate` v0.4/`v0.5` membuat *penyusunan* widget deklaratif (`<column>`, `<text>`,
  `<button style="...">`). Tapi *wiring behavior* (event + pengambilan nilai input +
  polling + konfirmasi + indicator) masih manual: closure `move |..|`, message enum,
  state `busy`, `Subscription::run` — menumpuk dan rawan error di aplikasi nyata
  (demo2).
- htmx memecahkan masalah analog di web dengan atribut `hx-*` deklaratif. Kita
  mengadopsi idiom ini, **dimap ulang ke model Iced** (message/render, bukan HTTP/DOM).
- Bonus: sebagian fondasi sudah ada — `codegen.rs` sudah mengenal alias event `hx_*`
  (`hx_click`→`on_press`, `hx_input`→`on_input`). PRD ini menaikkannya menjadi sistem
  penuh (trigger modifier, confirm, binding, polling, inheritance) yang konsisten dan
  terdokumentasi.

## 3. Tujuan & Metrik Sukses

1. Atribut `hx_*` menghasilkan wiring Iced idiomatis murni — nol `unsafe`, error
   compile-time berpesan jelas berbahasa Indonesia.
2. **Zero-runtime** untuk fitur v0.1/v0.2 (hanya perluasan alias + codegen); runtime
   helper `mrust-runtime` hanya untuk fitur yang memang butuh (`every` polling).
3. Mengurangi boilerplate event/state secara terukur di demo3 (banding demo2).
4. **Metrik:** `cargo build --workspace` hijau; `cargo test --workspace` lulus dengan
   unit-test baru (trigger parse, confirm, binding, polling codegen); `cargo run
   --package demo3` menampilkan app yang memakai `hx_*` end-to-end.

## 4. Ruang Lingkup

### 4.1 In-scope

- **v0.1 — Event deklaratif + trigger modifier** (macro murni)
  - Lengkapi/terapkan alias `hx_*` → method event iced untuk semua event umum.
  - `hx_trigger="<event> [modifier]*"` untuk kontrol kapan event memicu.
  - `hx_confirm="<teks literal>"` — konfirmasi sebelum event (overlay stateful).
- **v0.2 — Data binding & nilai** (macro murni)
  - `hx_value` / `hx_bind` — binding widget input ↔ state.
  - `hx_vals` / `hx_include` — kumpulkan nilai input sibling ke satu message.
  - `hx_visible` / `hx_disabled` / `hx_if` — render/status deklaratif oleh state.
- **v0.3 — Behavior + runtime helper** (`mrust-runtime` diperluas)
  - `every:<dur>` polling (timer `Subscription`).
  - `hx_indicator` / `hx_busy` — tampilan "sedang memproses".
  - inheritance `hx_hoist` / `hx_disinherit` — penyebaran atribut ke anak.
  - `hx_on` — jalankan efek samping sampingan pada event.
- **v0.4+ (roadmap)** — lihat bab 11.

> **Status implementasi (v0.1–v0.3).** Yang benar-benar di-generate makro:
> alias event `hx_*`/`mix_*`, `hx_value`/`hx_bind`, `hx_visible`/`hx_if`,
> `hx_disabled` (button), inheritance `hx_hoist`/`hx_disinherit`, dan helper
> polling `mrust_runtime::interval` (dihubungkan developer di `subscription()`).
> Token lain yang butuh *state/wiring level aplikasi* — `hx_trigger` (delay/
> throttle/once), `hx_confirm`, `hx_vals`/`hx_include`, `hx_busy`/`hx_indicator`,
> `hx_on` — **tetap deferred**: makro mengeluarkan error jelas ber-version plus
> panduan singkat (mis. `hx_poll` mencontohkan baris `Subscription` siap-salin).
> Alasannya: `view!{}` hanya menghasilkan satu fragment widget; ia tak melihat
> struct state maupun `fn subscription()`, sehingga tak bisa menyuntikkan state,
> timer, atau routing message yang token-token itu butuhkan.
>
> **Catatan struktur kode:** `codegen.rs` (1123 baris) telah dipecah menjadi
> `src/codegen/` (tags.rs, css.rs, attrs.rs, widget.rs, mod.rs) agar tiap
> domain berdiri sendiri; `apply_attrs_kind`/`chain_attr` kini di `attrs.rs`,
> `WKind`/`apply_css` di `css.rs`, kosakata & metadata `hx_*` di `tags.rs`.

### 4.2 Out-of-scope

- Meniru HTTP/AJAX/headers htmx (`hx-get/post`, `HX-*` request/response header),
  *history*, *out-of-band swap*, *morphing*, *extension JS*. Tidak relevan untuk
  desktop-native; Iced reaktif menangani rendering.
- File `.html` eksternal / interpretasi runtime — tetap harus `.rs` + compile-time.
- Mesin styling baru (sudah ada `style="..."` di v0.5-ish).
- Hot-reload.

## 5. Terminologi

| Istilah | Arti |
|---|---|
| event token | alias `hx_*` yang memetakan ke method event iced (`hx_press`→`on_press`) |
| trigger modifier | `once`, `changed`, `delay:…`, `throttle:…` pada `hx_trigger` |
| bind | pasangan input ↔ state (`value` + `on_input`) yang dihasilkan `hx_value` |
| payload | nilai input yang dikumpulkan `hx_vals`/`hx_include` dan dikirim ke message |
| busy | flag state yang di-toggle oleh `hx_busy`/`hx_indicator` |
| hoist | mewariskan atribut dari tag induk ke anak (`hx_hoist`) |

## 6. Pemetaan Konsep htmx → Iced

| Konsep htmx | Padanan mrust-macro / Iced | Token |
|---|---|---|
| `hx-get`/`hx-post` + event | event → `Message`/closure | `hx_press=…`, `hx_*` |
| response + `hx-swap`/`hx-target` | render ulang otomatis saat state berubah | (gratis) |
| `hx-trigger` + modifier | kapan event membawa efek: `once`/`changed`/`delay`/`throttle` | `hx_trigger` |
| `hx-confirm` | gate sebelum event → overlay konfirmasi stateful | `hx_confirm` |
| `hx-vals`/`hx-include` | kumpulkan nilai input → satu message payload | `hx_vals`/`hx_include` |
| `every` polling | timer `Subscription::run(iced::time::every)` | `hx_poll="every:2s"` |
| `hx-indicator` / disabled | widget/gaya kondisional atas state busy | `hx_indicator`/`hx_busy`/`hx_visible` |
| inheritance | promulgasi atribut turunan di codegen | `hx_hoist`/`hx_disinherit` |
| `hx-on`/efek | jalankan side-effect deklaratif pada event | `hx_on` |

## 7. Spesifikasi Token (roadmap)

> Semua token adalah **atribut** normal pada tag `view!{}`; nama ditulis `hx_<x>`
> agar satu kata di Rust (sinonim `mix_<x>` disediakan untuk yang merasa lebih
> natural; keduanya identik). Nilai berupa **literal string** (dileks jadi kontrol,
> pola `style="…"` yang sudah ada) atau **ekspresi `{expr}`** (diteruskan apa adanya).

### 7.1 v0.1 — Event & trigger modifier

**Tabel event token → method iced** (menggantikan/melengkapi `ATTR_ALIASES`):

| Token | Method iced | Token | Method iced |
|---|---|---|---|
| `hx_press` / `hx_click` / `hx_enter` | `on_press` | `hx_focus` | `on_focus` |
| `hx_close` / `hx_exit` | `on_press` | `hx_blur` | `on_blur` |
| `hx_input` / `hx_type` | `on_input` | `hx_drag` | `on_drag` |
| `hx_change` | `on_change` | `hx_release` | `on_release` |
| `hx_toggle` / `hx_switch` | `on_toggle` | `hx_submit` | `on_submit` |
| `hx_select` | `on_select` | `hx_cancel` | `on_cancel` |
| `hx_edit` / `hx_action` | `on_action` / `on_edit` | `hx_scroll` | `on_scroll` |

- Nilai event = `Message` atau closure, sama seperti `on_press=…` sekarang.
- **`hx_trigger="<event-spec>[, <event-spec>…]"`** — event-spec:
  `click` / `input` / `change` / `focus` / `blur` … ditambah modifier:
  - `once` — efek hanya sekali.
  - `changed` — hanya bila nilai elemen berubah.
  - `delay:500ms` (atau `delay:1s`) — tunda; reset bila event datang lagi.
  - `throttle:1s` — batasi laju; buang event selama batas.
  - Contoh: `hx_trigger="input delay:300ms, click once"`.
- **`hx_confirm="<teks literal>"`** — sebelum efek dijalankan, tampil overlay
  konfirmasi stateful (pola overlay demo2: tombol "Batal"/"Lanjut" memilih jalannya
  `Message`). Event yang dipicu menunggu, tidak langsung dieksekusi.

**Codegen:** tambah faktor `chain_attr_hx` yang menangkap token `hx_*` (di
`codegen/attrs.rs`) dan menguraikan `hx_trigger`/`hx_confirm` setelah event
biasa. Output tetap kode Iced normal. Helper `attr_text(&TokenStream) -> Option<String>`
(sekarang di `codegen/mod.rs`) untuk meleks literal string.

### 7.2 v0.2 — Data binding & nilai

- **`hx_value={expr}`** — binding dua arah utk widget stateful:
  - `<text_input hx_value={self.q}/>` → `text_input(&self.q).on_input(move |v| Msg::…(v))`
    (nama message diambil dari `hx_on_change`/`hx_bind` bila ada; default memakai
    `hx_value_to={Msg::Variant}`).
  - `hx_bind={self.q} hx_value_to=Msg::QueryChanged` — bentuk eksplisit.
- **`hx_vals={key: value, …}` / `hx_include="<names>"`** — susun nilai input
  bernama (lewat `name=…`) menjadi satu payload saat event:
  `<button hx_press=Msg::Submit hx_include="nama,email">` →
  collect `(&self.nama, &self.email)` → `Msg::Submit(nama, email)` (posisi dari
  urutan `hx_include`; tipe ditarik dari message).
- **`hx_visible={expr}`** — tampil/sembunyi (render kondisional) — untuk widget
  yang punya method `visible`/`opacity`, atau bungkus di `Hidden`-style. **`hx_disabled={expr}`**
  → `.on_press(None)` / `.on_input(None)` dll sesuai jenis. **`hx_if={expr}`** →
  serap node bila false (hilang total dari render).

### 7.3 v0.3 — Behavior + runtime helper (`mrust-runtime` diperluas)

- **`hx_poll="every:2s"`** — polling: helper `mrust_runtime::interval(dur, msg)`
  menghasilkan `Subscription<Msg>`; butuh induk menempatkannya di `subscription()`
  app. Codegen mengekspos blok yang bisa disalin dari macro bila diminta (`hx_poll`
  memberi tahu developer juga lewat error-doc bila dipakai tanpa hook).
- **`hx_busy={flag}` + `hx_indicator=<msg>`** — `hx_busy` menggalang state `bool`
  (true saat event memulai, false saat selesai); `hx_indicator` merujuk widget yang
  tampil ketika `busy` (menggantikan `if busy {..}` manual).
- **`hx_on=<msg>`** — jalankan `Message`/efek sampingan pada event, selain efek
  utama (multi-effect ala `hx-on`).
- **inheritance** — **`hx_hoist="<attr-nama…>"`** pada induk: atribut yang
  disebut disebar ke semua anak langsung (dan turunan bila `deep`); **`hx_disinherit="<attr…>"`**
  pada anak: batalkan warisan. Diolah di codegen rekursif.

  > **Batasan implementasi (v0.3):** sumber `hx_hoist` adalah *nilai atribut induk*
  > itu sendiri — artinya atribut yang di-hoist harus menjadi method yang **sah
  > pada induk DAN anak**. Contoh: `<column hx_hoist="size" size=14>` *salah*
  > karena `Column` tidak punya method `.size()`. Gunakan atribut yang sah pada
  > keduanya, mis. `align_x` (valid pada `Column` dan `text`):
  > `<column hx_hoist="align_x" align_x={iced::Alignment::Center}>`. Untuk
  > properti yang hanya relevan pada anak (mis. `size` pada teks), tulis langsung
  > di anak atau gunakan CSS/preset per-anak.


### 7.4 Grammar (ringkasan)

```
attr      := hx_name "=" value
value     := STRING_LITERAL              (* dileks: trigger/confirm/poll/include *)
           | "{" expr "}"                (* diteruskan apa adanya *)
hx_name   := "hx_" ident | "mix_" ident
hx_trigger:= event-spec ("," event-spec)*
event-spec:= event (modifier)*
modifier  := "once" | "changed" | "delay:" dur | "throttle:" dur
dur       := NUMBER ("ms" | "s")?
```

## 8. Mekanisme

- **Parser (tak berubah).** Atribut dibaca seperti atribut biasa (parser.rs); literal
  string dipertahankan utuh → nilai `hx_trigger`/`hx_confirm`/`hx_poll` terbaca tanpa
  mengubah lekser.
- **Codegen (diperluas).** `apply_attrs_kind` (di `codegen/attrs.rs`) diperlakukan
  token `hx_*` khusus: string dileks via `attr_text`; `{expr}` diteruskan. Event token
  memakai `ATTR_ALIASES` yang diperluas; `hx_trigger`/`hx_confirm`/`hx_vals`/
  `hx_bind`/`hx_poll` menambah generator masing-masing.
- **Runtime (opsional).** `mrust-runtime` menambah `interval(s, msg) -> Subscription`
  dan (bila perlu) bungkus `Hidden`/confirm-state. Dependensi `mrust-runtime` hanya
  ditarik bila fitur v0.3 dipakai — prinsip yang sama seperti `Spread` sekarang.
- Keluaran = kode Iced normal; dapat dicampur bebas dengan kode manual.

## 9. Penanganan Error

Semua error = error kompilasi, span menunjuk asal token.

| Kasus | Pesan |
|---|---|
| modifier trigger tak dikenal | `hx_trigger: modifier \`oncee\` tak dikenal (pakai once/changed/delay/throttle)` |
| format durasi buruk | `hx_trigger: durasi \`xx\` tak sah (contoh: 500ms atau 1s)` |
| `hx_confirm` bukan literal string | `hx_confirm butuh literal string, contoh: hx_confirm="Yakin hapus?"` |
| `hx_value` tanpa `hx_value_to` | `hx_value butuh hx_value_to=<Msg::Variant>` |
| `hx_poll` durasi buruk | `hx_poll: durasi \`xx\` tak sah (contoh: every:2s)` |
| `hx_include` menyebut nama tanpa input | `hx_include: tidak ada input bernama \`nama\` di sibling` |
| `hx_if` tanpa ekspresi | `hx_if butuh ekspresi: hx_if={expr}` |
| `hx_hoist` atribut tak dikenal | `hx_hoist: atribut \`xx\` tak dikenal` |
| API iced salah | error tipe Rust biasa dari compiler |

## 10. Dependensi & Kompatibilitas

- Rust edition 2021/2024, proc-macro stable; `proc-macro2`, `quote` (sudah ada).
- Iced 0.13 (uji); macro hardcode `::iced::` (sama seperti sekarang).
- `mrust-runtime` opsional: hanya dipakai fitur v0.3 (polling/busy).
- v0.1/v0.2 **zero-runtime** — murni perluasan alias + codegen macro.
- Semua keluaran tetap kode Iced normal; `prd.md`/demo1/demo2 tetap kompatibel.

## 11. Versi & Roadmap

| Versi | Isi |
|---|---|
| v0.1 | Event deklaratif penuh (`hx_*` panjang), `hx_trigger` (+`once`/`changed`/`delay`/`throttle`), `hx_confirm` overlay |
| v0.2 | `hx_value`/`hx_bind`, `hx_vals`/`hx_include`, `hx_visible`/`hx_disabled`/`hx_if` |
| v0.3 | `hx_poll="every:…"`, `hx_busy`/`hx_indicator`, `hx_on`, inheritance `hx_hoist`/`hx_disinherit` (depend `mrust-runtime`) |
| v0.4+ | `for`/`in` bawaan, namespace widget custom, effect scripting `hx_on` diperdalam |

Prioritas didorong kebutuhan nyata, bukan spekulasi (`ponytail:`).

## 12. Kriteria Sukses & Verifikasi

1. `cargo build --workspace` → seluruh workspace terkompilasi tanpa error.
2. `cargo test --workspace` → unit test baru lulus (trigger parse, `attr_text`,
   confirm, bind, poll codegen) + semua test lama tetap hijau.
3. `cargo run --package demo3` → app baru memakai `hx_*` end-to-end: text input
   `hx_value` + polling `hx_poll` + `hx_confirm` + `hx_visible` bekerja.
4. `cargo clippy --workspace --all-targets` → bersih.

## 13. Lampiran — Contoh Ekspansi

```rust
// masukan (v0.1 + v0.2)
view! {
    <column>
        <text_input name="q" hx_value={self.q} hx_value_to=Msg::QueryChanged/>
        <button hx_press=Msg::Run hx_confirm="Mulai pencarian?">cari</button>
        <button hx_press=Msg::Hapus hx_include="q">hapus hasil</button>
        <progress hx_visible={self.busy}/>
    </column>
}

// keluaran (disederhanakan; v0.1/v0.2 komponen macro murni)
::iced::widget::column![
    ::iced::widget::text_input("", &self.q).on_input(move |v| Msg::QueryChanged(v)),
    ::iced::widget::button("cari").on_press(Msg::Run).on_press(confirm_gate("Mulai pencarian?", Msg::Run)),
    ::iced::widget::button("hapus hasil").on_press(Msg::Hapus(self.q_clone)),
    if self.busy { ::iced::widget::progress_bar(0.0_f32..=100.0, 0.0).into() } else { ::iced::widget::Space::new(0,0).into() },
]
```

> Catatan implementasi: `hx_confirm` memakai overlay stateful (pola demo2) alih-alih
> dialog native (iced 0.13 tak punya); detail generator final ditentukan saat
> implementasi codegen, PRD ini menetapkan perilaku & kontrak tokennya.
