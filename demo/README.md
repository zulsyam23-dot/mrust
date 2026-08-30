# demo — MRust counter (lap 3–4)

Demo paling dasar `mrust-macro`: kontrol counter `-1 / 0 / +1`, input nama, checkbox/toggler, slider + progress.

## Jalankan

```
cargo run --package demo
```

## Isi

Menggunakan `iced::run` (pattern lama) + `view!{}` dengan tag dasar v0.1:

- `column` / `row` / `div` — layout
- `text` + ekspresi `{self.count}` (format otomatis)
- `button` — `onclick` / `on_press` / `onclick` menyala `Message::Dec | Reset | Inc`
- `input` — `value` + `oninput=Message::NameChanged`
- `p` — teks dengan interpolasi `{self.name}`
- `checkbox` + `toggler` — `is_checked` + `ontoggle` / `on_toggle`
- `slider` + `progress` — `range={f32::MIN..=MAX}` + `value` / `on_change`
- `hr` / `br` — pemisah & baris

Tanpa subscription; state `App { count, name, ready, volume }` di-update lewat closure `!update` di `iced::run`.

## Konstruk yang ditonjolkan

- Tag dasar v0.1: `div hr p input checkbox slider progress br button text`
- Alias event: `onclick` → `on_press`, `ontoggle` → `on_toggle`, `oninput` → `on_input`
- Anak `{ ... }` berupa ekspresi di dalam `text`/`p`
