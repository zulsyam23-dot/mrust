# demo5 — Styling CSS (lap 7)

Memakai mesin styling `style="...css..."` compile-time di `mrust-macro` untuk berbagai widget. Tanpa Message (pakai `Message = ()`).

## Jalankan

```
cargo run --package demo5
```

## Isi

Menggunakan `iced::application(...).run()` dengan `update` kosong dan `Element<'_, ()>`.

- **Container**: `background:#111; color:#eee; padding:12px; border:1px solid #444; border-radius:6px; shadow:0 2 #00000060`
- **Button**: `background:#007acc; color:white; border-radius:4; border:1px solid #0099ff` (dua contoh biru/merah)
- **Rule**: `<hr style="color:#ff8800"/>` dan `<rule style="color:#22aaaa"/>`
- **Row/container + gap**: `gap:8` + `text-align:center` + `width:fill`
- **Text**: `color`, `font-weight:bold`, `font-size`, `text-align`
- Catatan: `style={expr}` (bukan string) diteruskan apa adanya sebagai `.style(expr)`.

## Konstruk yang ditonjolkan

- Mesin styling `style=` per jenis widget: text / container / button / layout / rule
- Warna hex nama `rgb(a)`; `padding`, `border`, `shadow`, `gap`, ukuran/font, alignment
- `<hr>`/`<rule>` memakai `rule::Style` (WKind::Rule)
