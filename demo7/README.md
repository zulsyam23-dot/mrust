# demo7 — Binding & hoist (lap 8b)

Token binding dua arah + penurunan properti (`hx_hoist`) dan pembatalannya (`hx_disinherit`).

## Jalankan

```
cargo run --package demo7
```

## Isi

Menggunakan `iced::run`:

- `<input placeholder="Kata kunci" hx_value={self.q} hx_value_to={Message::QueryChanged}/>` — binding dua arah via `value` + `on_input`
- `<input placeholder="Email" hx_bind={self.email} hx_bind_to={Message::EmailChanged}/>` — alias `hx_bind`
- `hx_value` TANPA `hx_value_to` tidak menambah `on_input` (baca-saja)
- **Hoist**: `<column hx_hoist="align_x" align_x={iced::Alignment::Center}>` menurunkan `align_x=Center` ke semua `p` anak
- **Disinherit**: `<text hx_disinherit="align_x">` membatalkan warisan `align_x` pada node itu

## Konstruk yang ditonjolkan

- `hx_value(_to)` / `hx_bind(_to)`
- `hx_hoist="property"` menurunkan property ke seluruh anak
- `hx_disinherit="property"` menghentikan pewarisan per node
