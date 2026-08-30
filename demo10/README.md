# demo10 — Gabungan semua fitur

Demo lengkap yang menggabungkan hampir seluruh kemampuan `mrust-macro` + runtime: tag dasar/HTML v0.2–v0.4, styling CSS, token `hx_*`/`mix_*`, iterasi dinamis, asset SVG, dan polling interval.

**Butuh fitur iced `svg`** (sudah di `demo10/Cargo.toml`).

## Jalankan

```
cargo run --package demo10
```

## Isi

Menggunakan `iced::application(...).subscription(...).run()`:

- **Header**: `<header style=...>` berisi `<icon>`, `<h3>`, `<spacer/>`, tombol `hx_click` ber-style
- **Search + tambah**: `<input hx_value ...>` dan tombol `mix_press=Message::TambahItem`
- **Kontrol**: `<checkbox ontoggle>` + `<text hx_visible>`
- **Iterasi dinamis**: daftar item dari `Vec<String>` via `{ ... .map(...).collect::<Vec<_>>() }`, tiap item `<row><text/><spacer/><button hapus/></row>`
- **Hoist**: `<column hx_hoist="align_x" align_x={Alignment::Center}>`
- **Role teks**: `<code>` & `<mark>` dalam `<div><row>`
- **Runtime**: `mrust_runtime::interval(2.0, Message::Tick)` → `app.detik += 1`, tampil `"Responsif tiap {app.detik}s"`

## Konstruk yang ditonjolkan

- Semua fitur mrust v0.1–v0.5 + `hx_*` + runtime dalam satu layar
- Tag `<header>`, `<spacer/>`, `<h3>`, `<code>`, `<mark>`, `<div>`
- Perpaduan `style=` CSS dan token event/kondisi
