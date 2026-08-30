# demo4 — Kosakata HTML v0.4

Keragaman tag semantik + form control (select/radio/meter) yang diperkenalkan di v0.4.

## Jalankan

```
cargo run --package demo4
```

## Isi

- **Tag semantik container**: `<section>` (bisa ber-`style`), `<nav>`, `<dl>`/`<dt>`/`<dd>`, `<ul>`/`<li>`
- **Heading**: `<h1>` `<h2>` `<h6>`
- **Role teks**: `<text>` `<strong>` `<code>` `<small>` `<mark>`
- **Tautan**: `<a onclick=Message::Pilih("...")>`
- **Form control**:
  - `<select options={vec![...]} selected={...} on_selected={move |s: &str| ...}/>` → `PickList`
  - `<radio label value selected on_selected/>` → `Radio` dengan enum `Kategori { Ringan, Sedang }`
  - `<meter range value/>` → progress statis
- State: `App { pilihan: Option<String>, gent: Option<Kategori> }` di-update lewat `Message::Pilih` / `Message::Kat`.

## Konstruk yang ditonjolkan

- Tags v0.4: `section nav dl dt dd ul li select radio meter a` dan heading/role teks
- `style="background:...; color:...; padding:..."` pada container semantik
- `Kategori: Copy + Eq` agar bisa dipakai langsung `selected={self.gent}`
