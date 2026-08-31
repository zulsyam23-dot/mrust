//! `dfw3` — bukti `app!`: boilerplate struktural (`struct App`, `Default`,
//! registri aksi, `fn main`, wiring) disembunyikan makro. User menulis: state,
//! mutasi (sekali), dan `fn view` (layout). Event tombol/input lewat NAMA string
//! (`on`/`on_val`). `ponytail:` satu `app!` per file (`App`/`main` di scope).
use mrust_fw::{app, view};

fn view(app: &App) -> mrust_fw::Element<'_, App> {
    let row_delete = |i: usize| {
        mrust_fw::act(move |a: &mut App| {
            if i < a.items.len() {
                a.items.remove(i);
            }
        })
    };
    view! {
        <column spacing=12 padding=24>
            <header padding=8 style="background:#26292e; padding:8px; border-radius:4px">
                <row spacing=10 align_y={iced::Alignment::Center}>
                    <icon src="assets/icons/new.svg" width={iced::Length::Fixed(18.0)}/>
                    <h3>"dfw3 - app!: satu makro, tak ada struct/impl/fn main manual"</h3>
                    <spacer/>
                    <button on_press={mrust_fw::on("clear")}
                        style="background:#007acc; color:white; border-radius:4">"bersihkan"</button>
                </row>
            </header>

            <row spacing=10>
                <input placeholder="Kata kunci" value={app.q}
                    on_input={mrust_fw::on_val("q")}/>
                <button on_press={mrust_fw::on("add")}>"+ item"</button>
            </row>

            <checkbox label="tampilkan" is_checked={app.live}
                on_toggle={mrust_fw::act_v(|a: &mut App, _: bool| a.live = !a.live)}/>

            <row spacing=8>
                <icon src="assets/icons/dot.svg" width={iced::Length::Fixed(14.0)}/>
                <text hx_visible={app.live}>
                    "Responsif tiap " {app.detik} "s"
                </text>
            </row>

            <hr style="color:#ff8800"/>
            <button on_press={mrust_fw::on("toggle")}>"toggle live (via nama)"</button>
            <text hx_if={app.live && !app.q.is_empty()}>
                "q: " {app.q} " (hx_if)"
            </text>

            <div style="background:#1a1c21; padding:4px; border-radius:3px">
                <column spacing=2>
                    {app.items.iter().enumerate().map(|(i, it)| {
                        view! {
                            <row spacing=6 style="padding:2px">
                                <text size=13>{it}</text>
                                <spacer/>
                                <button on_press={row_delete(i)}
                                    style="background:#e5484d; color:white; border-radius:3px">"hapus"</button>
                            </row>
                        }.into()
                    }).collect::<Vec<_>>()}
                </column>
            </div>
        </column>
    }
    .into()
}

app! {
    title = "dfw3 - Seluruh app via satu makro";

    state {
        q: String,
        live: bool,
        items: Vec<String>,
        detik: u32,
    }

    tap {
        "clear"  => |a: &mut App| a.q.clear(),
        "add"    => |a: &mut App| {
            let n = a.items.len() + 1;
            a.items.push(format!("item ke-{n}"));
        },
        "toggle" => |a: &mut App| a.live = !a.live,
        "tick"   => |a: &mut App| a.detik += 1,
    }

    bind {
        "q" => |a: &mut App, v: String| a.q = v,
    }

    view: view
}
