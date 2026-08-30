# MRust — Tulis GUI Iced dengan Markup HTML

**MRust** adalah DSL (domain-specific language) untuk [Iced](https://iced.rs/), GUI toolkit native Rust. Alih-alih merangkai widget Iced satu per satu secara manual:

```rust
// Tanpa MRust — verbose
iced::widget::Column::new()
    .push(iced::widget::Text::new("Halo"))
    .push(
        iced::widget::Button::new(Text::new("Klik"))
            .on_press(Message::Klik),
    )
```

kamu menulis markup yang mirip HTML, dan sebuah **macro** (`view!`) mengubahnya menjadi kode Iced biasa saat kompilasi:

```rust
view! {
    <column spacing=12 padding=24>
        <text size=26>"Halo"</text>
        <button on_press=Message::Klik>"Klik"</button>
    </column>
}
```

Tidak ada runtime JavaScript, tidak ada webview — hasilnya aplikasi desktop native murni.

## Repositori Ini

Workspace Cargo berisi **library DSL** (macro + runtime) dan **10 demo** berjenjang yang menunjukkan fitur dari dasar hingga paling lengkap.

> Roadmap & desain detail ada di [`prd.md`](prd.md) dan [`prd2.md`](prd2.md).
> Referensi lengkap setiap tag & token (leksikal → `hx_*` → runtime) ada di [`api-token.md`](api-token.md).

---

## Struktur Proyek

```
mrust/
├── Cargo.toml            # workspace: mrust-macro, mrust-runtime, demo1–demo10
├── mrust-macro/          # proc-macro `view!` (inti DSL)
│   └── src/codegen/      # codegen per-domain: tags, css, attrs, widget
├── mrust-runtime/        # helper opsional (interval polling / Spread)
├── demo/                 # demo1 — counter dasar (v0.1–v0.2)
├── demo2/ … demo10/      # demo lanjutan, dari iterasi hingga gabungan semua fitur
├── prd.md, prd2.md       # PRD & kontrak
└── api-token.md          # referensi API & token lengkap
```

| Katalog | Peran |
|---------|-------|
| `mrust-macro` | Proc-macro `view!` — mengurai markup HTML & men-generate kode Iced. |
| `mrust-runtime` | Crate opsional: `interval` (polling), `Spread`, `if_elem`. Ditarik hanya bila dipakai. |
| `demo`.1–`.10` | Contoh berjenjang, dari yang paling dasar sampai fitur lengkap. |

---

## Cara Mulai

### Prasyarat

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- Iced 0.13 (di-resolve otomatis via `Cargo.lock`)

### Build semua

```bash
cargo build --workspace
```

### Jalankan salah satu demo

```bash
cargo run --package demo3
```

### Uji & lint

```bash
cargo test --workspace
cargo clippy --workspace --all-targets
```

---

## Daftar Demo (urutan dasar → lanjutan)

Setiap demo punya `README.md` sendiri. Salah satunya, demo10, menggabungkan hampir semua fitur.

| Demo | Fokus | `cargo run --package …` |
|------|-------|--------------------------|
| `demo` | Tag dasar v0.1–v0.2: counter, `text`/`button`/`input`/`checkbox`/`slider`/`progress` | `demo` |
| `demo2` | Iterasi dinamis dari `Vec` + widget `<editor>` (v0.3) | `demo2` |
| `demo3` | Token `hx_*` end-to-end + polling (v0.3) | `demo3` |
| `demo4` | Kosakata HTML v0.4: `section`/`nav`/`dl`/`select`/`radio`/`meter` | `demo4` |
| `demo5` | Mesin styling `style="…css…"` (v0.5) | `demo5` |
| `demo6` | Event alias + `hx_if`/`hx_visible`/`hx_disabled` | `demo6` |
| `demo7` | Binding & `hx_hoist`/`hx_disinherit` | `demo7` |
| `demo8` | Aset `src` (ikon/SVG, butuh fitur iced `svg`) | `demo8` |
| `demo9` | `mrust_runtime::interval` (polling 2 detik) | `demo9` |
| `demo10` | Gabungan semua fitur + runtime | `demo10` |

Contoh:

```bash
cargo run --package demo10   # melihat hampir semua fitur sekaligus
```

---

## Contoh Lengkap: `view!` + State

```rust
use iced::Element;
use mrust_macro::view;

#[derive(Debug, Clone)]
enum Message {
    Inc,
    Dec,
}

#[derive(Default)]
struct App {
    count: i32,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        view! {
            <column spacing=16 padding=24>
                <text size=40>Counter: {self.count}</text>
                <row spacing=8>
                    <button on_press=Message::Dec>"-1"</button>
                    <button on_press=Message::Inc>"+1"</button>
                </row>
            </column>
        }
        .into()
    }
}

fn main() -> iced::Result {
    iced::run("Contoh MRust", update, App::view)
}

fn update(app: &mut App, msg: Message) {
    match msg {
        Message::Inc => app.count += 1,
        Message::Dec => app.count -= 1,
    }
}
```

---

## Fitur Utama

- **Markup seperti HTML** → langsung jadi widget Iced saat compile, tanpa biaya runtime.
- **Binding dua arah** (`hx_value`/`hx_bind`) dan **penurunan properti** (`hx_hoist`).
- **Kontrol** via token `hx_*` / `mix_*` (event alias, kondisi `hx_if`/`hx_visible`, `hx_disabled`).
- **Styling CSS** inline `style="background:#111; color:#eee; padding:12px; border-radius:6px"`.
- **Iterasi dinamis**: render `Vec<T>` lewat `{ … .map(...).collect::<Vec<_>>() }`.
- **Runtime ringan opsional**: `interval` untuk polling ala-htmx `hx_poll`.

Referensi lengkap semua tag, attribute, dan token: **[`api-token.md`](api-token.md)**.

---

## Lisensi

(Isi sesuai lisensi proyek bila ada; kosongkan bila belum ditetapkan.)
