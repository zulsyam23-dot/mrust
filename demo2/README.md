# demo2 — Iterasi dinamis & editor (lap 5)

Ekstensi v0.3: merender daftar dinamis dari `Vec<String>` lewat anak `{ ... }` (Spread) dan `<editor>` (widget `text_editor::TextEditor`).

## Jalankan

```
cargo run --package demo2
```

## Isi

Menggunakan `iced::run`:

- Tombol **Tambah** → `daftar.push(...)`; **x** per baris → hapus; **Simpan isi editor** → pindahkan teks editor ke daftar.
- Daftar di-render dari `self.daftar.iter().enumerate().map(...).collect::<Vec<_>>()` — tiap item jadi `<row><text>{item}</text><button>x</button></row>`. Ini memakai fitur **anak `{ expr }` menerima `Element` atau `Vec<Element>`** (Spread).
- `<editor state={&self.editor} on_action={Message::Edit}>` — wrapper `text_editor::TextEditor`; `height` memakai `iced::Length::Fixed(180.0)`; `style="padding:8px"`.

## Konstruk yang ditonjolkan

- Iterasi dinamis: `{ ... .map(...).collect::<Vec<_>>() }` di dalam kontainer
- Tag `<editor>` → `text_editor::TextEditor` dengan `state` + `on_action`
- `Message::Edit(text_editor::Action)` → `app.editor.perform(action)`
