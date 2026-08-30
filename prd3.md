# PRD 3 — `mrust-fw`: Framework untuk Membangun Aplikasi Iced dengan MRust ("tanpa pusing")

> **Referensi API & token lengkap `mrust-macro` ada di [`api-token.md`](api-token.md)**.
> **Roadmap dasar → lanjutan & desain sebelumnya: [`prd.md`](prd.md) & [`prd2.md`](prd2.md)**.
> Dokumen ini (PRD 3) menetapkan *mengapa* & *kontrak* framework `mrust-fw`; README.md
> & bill chapter berpola lain menetapkan *cara pakai*-nya.

| | |
|---|---|
| Nama | `mrust-fw` — framework aplikasi di atas mrust-macro + mrust-runtime |
| Status | Draf — v0.1 (roadmap) |
| Versi PRD | 3.0 |
| Tanggal | 2026-08-30 |
| Stack | Rust, Iced 0.13, `mrust-macro` (mesin `view!`), `mrust-runtime` (opsional) |
| Kategori | **Framework tipis** — menyembunyikan setup/loop Iced & boilerplate; user hanya bangun UI dengan `view!` |

---

## 1. Ringkasan Produk

`mrust-macro` sudah membuat **penyusunan UI** deklaratif (`view!` + CSS + `hx_*`).
`mrust-runtime` sudah menyediakan **behavior** opsional (interval/polling, Spread).

**Masalah yang tersisa**: untuk menjalankan satu layar, user masih harus menulis
boilerplate Iced tiap kali — `iced::application(...)`, `enum Message`, `fn update`,
`Subscription::batch`, deps `iced` (+ fitur `svg`/`tokio`), dst. Ini berulang,
memusingkan, dan bikin user berhenti "membangun aplikasi" dan malah "bermain dengan
ekosistem Iced".

**`mrust-fw`** adalah **framework minimal** yang menyembunyikan seluruh setup itu.
Kontraknya sederhana dan *default-first*:

> **User cukup membangun isi layar dengan `view!` dan memberitahu framework apa
> yang tampil / apa yang terjadi. Selebihnya (loop, state, async, deps) diurus
> framework di belakang.**

Prinsip desain:

1. **Tanpa deps di otak user** — framework mengemas iced + fitur yang dipakai.
2. **Tanpa `Message`/`update` eksplisit untuk kasus umum** — state disimpan di
   `App`, dan framework menautkan otomatis.
3. **Kompatibel penuh dengan kemampuan mrust saat ini** — semua tag/`hx_*`/CSS
   tetap dipakai apa adanya di dalam `view!`.
4. **Tetap `.rs` murni** — tidak ada DSL waktu-runtime baru di luar `view!`.

> Bukan kloning Laravel. `mrust-fw` adalah **selubung konvensi** di atas library
> mrust yang sudah ada agar pembuatan aplikasi nyata menjadi "isi konten saja".

---

## 2. Latar Belakang & Masalah

- Demo1–demo10 membuktikan `view!` + `hx_*` + CSS + runtime berfungsi untuk banyak
  layar. Namun **tiap demo menulis ulang boilerplate yang sama**:
  `iced::application(...).subscription(...)`, `enum Message`, `fn update`,
  deps `iced = { features = [...] }`.
- Fragmen itu kecil tapi **berulang & rentan salah** (mis. lupa fitur `svg` untuk
  `<icon>`, lupa backend `tokio` untuk `interval`).
- Pengguna pemula yang "hanya mau bikin app" tidak ingin paham ekosistem Iced
  (executor, subscription recipe, fitur crate) sebelum bisa menampilkan satu tombol.

**Dampak**: hambatan masuk (onboarding) tinggi; fokus pindah dari "produk" ke
"infrastruktur".

---

## 3. Tujuan & Non-Tujuan

### 3.1 Tujuan (v0.1)

- [ ] API framework yang memungkinkan menampilkan layar dengan `view!` **tanpa**
      menulis `iced::application`, `Message`, `update`, atau `Subscription` manual.
- [ ] State aplikasi disimpan di struct `App`; perubahan state otomatis me-refresh
      `view` (reaktivitas bawaan Iced tetap dipakai, disembunyikan).
- [ ] Tersedia `run`/awalan yang menerima judul, `App`, dan `view` — satu titik
      masuk.
- [ ] Mendukung semua kemampuan `mrust-macro` saat ini di dalam `view!` (CSS,
      `hx_*`, iterasi, dsb.) **tanpa perubahan** pada mrust-macro.
- [ ] Konstruksi contoh aplikasi nyata (satu layar + satu aksi) dengan kode terukur
      jauh lebih sedikit daripada demo saat ini.

### 3.2 Non-Tujuan (belum di v0.1)

- **Tidak** meniru semua fitur Laravel (routing lengkap, ORM, auth, blade component).
- **Tidak** membuat DSL baru di luar `view!`.
- **Tidak** mengganti `mrust-macro`/`mrust-runtime` — framework **membungkus**-nya.
- **Tidak** mendukung multi-window / daemon di v0.1 (bisa jadi v0.2+).
- **Tidak** ada CLI scaffold (`mrust new`) — dipisah ke proyek/versi lain.

---

## 4. Pengguna & Skenario (Persona)

| Persona | Kebutuhan |
|---------|-----------|
| **Pemula** (bikin app pertama) | Tampilkan UI + bereaksi ke klik tanpa tahu detail iced. |
| **Praktisi** (produk nyata) | Bangun banyak layar cepat, state tersentral, gaya konsisten. |
| **Pembaca kode mrust** | Contoh framework yang menunjukkan **semua** kemampuan mrust dipakai. |

---

## 5. Kontrak & Desain API (v0.1)

### 5.1 Crate baru

Crate baru di workspace: **`mrust-fw`** (library, bukan proc-macro).

```
mrust-fw/
└── src/
    ├── lib.rs        # re-export: App, view, iced-primitif yang dibutuhkan
    └── (modul)       # loop, state, action
```

`mrust-fw` **depend** pada `mrust-macro` dan `mrust-runtime`, dan mengemas `iced`
(dengan fitur yang dibutuhkan) sebagai ketergantungan internal — user tidak perlu
menambahkan `iced` sendiri.

### 5.2 Konsep inti

- **`App`** — struct milik user yang menyimpan seluruh state aplikasi.
- **`view(&App) -> Element`** — dibangun dengan `view!`, memakai state.
- **`on(…)`** — aksi/kondisi yang menautkan event ke perubahan state, bersifat
  opsional; untuk kasus umum (klik → ubah satu field) framework menyediakan
  penolong deklaratif.
- **`run`** — titik masuk: menerima judul + `App` + `view`, menjalankan loop Iced.

### 5.3 Bentuk eksternal (ilustrasi awal — bisa berubah saat implementasi)

```rust
use mrust_fw::mrust;

#[derive(Default)]
struct App {
    count: i32,
    nama: String,
}

fn view(app: &App) -> mrust_fw::Element {
    mrust! {
        <column spacing=16 padding=24>
            <text size=40>"Counter: " {app.count}</text>
            <input placeholder="Nama" value={&app.nama}
                on_input={mrust_fw::set(app, |a, v: String| a.nama = v)}/>
            <button on_press={mrust_fw::tap(app, |a| a.count += 1)}>"+"</button>
        </column>
    }
}

fn main() -> mrust_fw::Result {
    mrust_fw::run::<App>("App Saya", view)
}
```

Catatan: pada baris `<text size=40>"Counter: " {app.count}</text>` — ekspresi
`{app.count}`/`{&app.nama}` disisipkan persis seperti `mrust-macro` saat ini.
Ini **kontrak** yang harus tetap berlaku: `view!` framework = `view!` mrust-macro.

> Bentuk tepat penolong (`set`/`tap`, atau trait `on`) **belum dikunci** — PRD ini
> menetapkan *kontrak*, bukan implementasi final. Iterasi pada skenario orang parkir.

### 5.4 Kontrak minimum (acceptance dasar)

1. Kode dibawah di bawah ini **harus compile** tanpa menulis `iced::` sama sekali:
   ```rust
   fn view(app: &App) -> mrust_fw::Element {
       mrust! { <text>"Halo " {&app.nama}</text> }
   }
   fn main() -> mrust_fw::Result { mrust_fw::run::<App>("Halo", view) }
   ```
2. Perubahan state dari interaksi (klik/input) **harus** memicu re-render `view`.
3. Semua fitur `mrust-macro` (CSS, `hx_*`, iterasi `{ ... .collect() }`, `<icon>`)
   **harus tetap** bisa dipakai di dalam `mrust!`.

---

## 6. Arsitektur & Komponen

```
┌─────────────────────────────────────────────────────────────┐
│                         USER LAPISAN                         │
│   App (state)  +  view(&App) -> Element  →  mrust!{...}      │
└───────────────────────────────┬─────────────────────────────┘
                                │ (hanya ini yang dilihat user)
┌───────────────────────────────▼─────────────────────────────┐
│                      mrust-fw (framework)                    │
│   • re-export view! (mrust-macro)                            │
│   • trait App + state container                              │
│   • loop: iced::application + subscription (hidden)          │
│   • penolong aksi (set/tap/…) → update state + re-render     │
└───────────────────────────────┬─────────────────────────────┘
┌───────────────────────────────▼─────────────────────────────┐
│   mrust-macro (view!/hx_*/CSS)   +   mrust-runtime (ops.)    │
│                          iced 0.13                           │
└─────────────────────────────────────────────────────────────┘
```

- **mrust-fw** = lapisan tipis: menerjemahkan kontrak user → `iced::application`.
- **mrust-macro/runtime** tidak berubah; dipakai apa adanya.
- Semua detail iced (executor, subscription recipe, fitur crate) berada **di bawah**
  lapisan framework, tidak bocor ke user.

---

## 7. Strategi Implementasi (fase)

### Fase 0 — Fondasi (PRD ini)
- Tambah crate `mrust-fw` ke `members` workspace `Cargo.toml`.
- Siapkan `mrust-fw/Cargo.toml`: depend `mrust-macro`, `mrust-runtime`, dan `iced`
  (fitur yang dibutuhkan, internal).
- Contoh `mrust-fw` **minimal** yang menampilkan `view!` + `run` tanpa `iced::`.

### Fase 1 — Loop + state
- Sediakan `mrust_fw::run::<App>(judul, view)` yang memetakan ke
  `iced::application(...)` dan menahan `&mut App` (hidden).
- Status/hidden update internal agar perubahan state memicu re-render.

### Fase 2 — Penolong aksi
- `tap` (klik/tekan) dan `set` (binding input) versi pertama.
- Verifikasi di contoh nyata (satu layar, satu aksi).

### Fase 3 — Verifikasi penuh kemampuan mrust
- Contoh `mrust-fw` yang memakai CSS, `hx_*`, iterasi, `<icon>` sekaligus.
- Pastikan tak ada perubahan yang diperlukan pada `mrust-macro`.

---

## 8. Metrik & Kriteria Selesai

- [ ] `cargo test --workspace` lulus (termasuk test framework baru).
- [ ] `cargo clippy --workspace --all-targets` bersih.
- [ ] Contoh di §5.4 bisa di-compile **tanpa** menulis `iced::` di kode user.
- [ ] Kode untuk satu layar + satu aksi dengan framework **jauh lebih sedikit**
      daripada `demo`/`demo3` saat ini (target: ≤ setengahnya).
- [ ] Semua fitur `mrust-macro` yang ada tetap bekerja (regresi nol).

---

## 9. Risiko & Mitigasi

| Risiko | Mitigasi |
|--------|----------|
| Framework terlalu "magic" (hard to debug) | Kontrak kecil, dokumentasi, contoh; penolong berbentuk closure/state eksplisit. |
| Bound pada satu versi iced | Framework mengemas fitur; update iced terpusat di `mrust-fw`. |
| User butuh kontrol rendah-level | Framework expose "escape hatch" minimal ke iced bila perlu (v0.2). |
| Over-engineering (tambahan fitur tak diminta) | v0.1 **hanya** loop + state + penolong; sisanya PRD berikutnya. |

---

## 10. Referensi & Roadmap

- [`prd.md`](prd.md) — roadmap dasar & tag v0.1–v0.5
- [`prd2.md`](prd2.md) — token deklaratif `hx_*`/`mix_*` + runtime
- [`api-token.md`](api-token.md) — referensi lengkap API & token `mrust-macro`
- `demo/.1–.10` — bukti keseluruhan kemampuan mrust yang framework ini sarungi
