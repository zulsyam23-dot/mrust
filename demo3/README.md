# demo3 — Token `hx_*` end-to-end (lap token)

Demo yang memakai token `hx_*`/`mix_*` + runtime polling. Berpindah dari `iced::run` ke `iced::application(...).subscription().run()`.

## Jalankan

```
cargo run --package demo3
```

## Isi

- `hx_value={app.q}` + `hx_value_to=Message::QueryChanged` — binding dua arah input
- `hx_bind={app.nama}` + `hx_bind_to=Message::NamaChanged` — alias hx untuk bind
- `hx_click=Message::Cari`, `mix_press=Message::Cari`, `onclick` — tiga alias → `on_press`
- `hx_disabled={app.ticks > 3}` — nonaktifkan tombol sesuai kondisi
- `hx_visible={app.live}` / `hx_if={...}` — tampilkan/sembunyikan teks
- `hx_hoist="align_x"` + `align_x={iced::Alignment::Center}` — turunkan alignment ke semua anak kolom
- Subscription: `mrust_runtime::interval(2.0, Message::Tick)` di `Subscription::batch` → `Message::Tick` → `app.ticks += 1`

## Konstruk yang ditonjolkan

- `iced::application` + `.subscription(...)` (bukan `iced::run`)
- Token `hx_*` + `mix_*` di-generate
- `hx_hoist` penurun properti ke anak
- `mrust_runtime::interval` (butuh Cargo.toml ber-deps `mrust-runtime`)
