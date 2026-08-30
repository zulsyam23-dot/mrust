# demo8 — Asset `src` (ikon & SVG)

Memuat SVG dari `src` literal (di-embed saat runtime dari path) dan dari `src={expr}` (`Handle` manual).

**Butuh fitur iced `svg`** (sudah ada di `demo8/Cargo.toml`).

## Jalankan

```
cargo run --package demo8
```

## Isi

Menggunakan `iced::run`:

- Aset: `assets/icons/dot.svg` dan `assets/icons/menu.svg`
- `<icon src="assets/icons/dot.svg" width={Length::Fixed(24.0)}/>` — `src` literal string (iced memuat dari path)
- `<svg src="assets/icons/dot.svg" .../>` — alias `icon`
- `<button on_press=Message::Buka padding=6><icon .../></button>` — ikon sebagai anak tombol
- `src={expr}`: `Handle` disiapkan manual via `iced::widget::svg::Handle::from_memory(include_bytes!(...))`, lalu `<icon src={handle.clone()}/>` dan `<icon src={handle}/>` diteruskan apa adanya

## Konstruk yang ditonjolkan

- `<icon>`/`<svg>` → `iced::widget::svg` (Svg widget)
- `src` literal vs `src={expr}` (Handle)
- `width={iced::Length::Fixed(n)}`
- Fitur `svg` non-default di iced 0.13 → diaktifkan di Cargo.toml
