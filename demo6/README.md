# demo6 — Event alias & hx_if / hx_visible / hx_disabled (lap 8a)

Fokus: alias event tombol + token kondisi tampil/nonaktif.

## Jalankan

```
cargo run --package demo6
```

## Isi

Menggunakan `iced::run`:

- **Alias event tombol** (ketiganya → `on_press`): `hx_click=Message::Tekan`, `mix_press=Message::Tekan`, `onclick=Message::Tekan`
- `<checkbox ontoggle=Message::Saklar>` → `on_toggle`
- `hx_visible={self.live}` / `hx_if={!self.live}` — tampilan kondisional (identik)
- `hx_click={Message::Simpan}` + `hx_disabled={self.simpanan > 3}` — tombol nonaktif setelah 3 klik (memakai `on_press_maybe`)
- State `App { live: bool, simpanan: u32 }`; `Message::Simpan` → `simpanan += 1`

## Konstruk yang ditonjolkan

- Alias event `hx_click`/`mix_press`/`onclick` → `on_press`
- `hx_disabled` hanya untuk button; kondisi bisa ekspresi `{...}`
- `hx_visible`/`hx_if` identik di level tampil
