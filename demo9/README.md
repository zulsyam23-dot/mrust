# demo9 — Runtime interval (polling)

Memakai helper runtime `mrust_fw::interval` sebagai polling ala-htmx `hx_poll`, lewat `iced::application(...).subscription(...)`.

## Jalankan

```
cargo run --package demo9
```

## Isi

- `Subscription::batch(vec![mrust_fw::interval(2.0, Message::Tick)])` dipasang di `subscription` → tiap 2 detik kirim `Message::Tick`
- `Message::Tick` → `app.detik += 1`; bila `nama` kosong dan `detik > 2`, isi otomatis `"otomatis @ N.0s"` (efek polling)
- `<input hx_value={app.nama} hx_value_to={Message::NamaChanged}/>` — binding
- `<text hx_visible={!app.nama.is_empty()}>` — tampil setelah nama terisi
- Catatan di UI: interval butuh backend timer (tokio) dan dipasang di `fn subscription()`

## Konstruk yang ditonjolkan

- `mrust_fw::interval(detik, Message)` → `Subscription`
- Perbedaan `iced::application` vs `iced::run` (subscription)
