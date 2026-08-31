//! `dfw2` — verifikasi penuh mrust via `mrust-fw` + DSL aksi STRING (PRD 3 Fase 3
//! & pengembangan DSL). Layar: CSS `style`, `hx_*`, iterasi dinamis, `<icon>`.
//! Event TIDAK memakai closure/value-Rust: tombol & input diikat lewat NAMA string
//! (`on("add")`, `on_val("q")`) yang diregistrasi sekali lewat `actions_val!`.
//! Satu-satunya Rust yang ditulis: definisi mutasi `App` (metode), bukan tiap event.
use mrust_fw::{self, view};
use iced::Length;

#[derive(Default)]
struct App {
    q: String,
    live: bool,
    items: Vec<String>,
    detik: u32,
}

impl App {
    fn clear(&mut self) {
        self.q.clear();
    }
    fn add_item(&mut self) {
        let n = self.items.len() + 1;
        self.items.push(format!("item ke-{n}"));
    }
    fn toggle_live(&mut self) {
        self.live = !self.live;
    }
    fn set_q(&mut self, v: String) {
        self.q = v;
    }
    fn tick(&mut self) {
        self.detik += 1;
    }
}

fn main() -> mrust_fw::Result {
    let a = mrust_fw::actions_val![
        App : "clear" => App::clear,
        "add" => App::add_item,
        "toggle" => App::toggle_live,
        "tick" => App::tick ;
        "q" => App::set_q
    ];
    mrust_fw::run_with_actions::<App>(
        "dfw2 - PRD3 Fase 3 + DSL string",
        view,
        a,
        Some(|_| mrust_fw::AppSubscription::batch(vec![
            mrust_fw::interval(2.0, mrust_fw::on::<App>("tick")),
        ])),
    )
}

fn view(app: &App) -> mrust_fw::Element<'_, App> {
    view! {
        <column spacing=12 padding=24>
            <header padding=8 style="background:#26292e; padding:8px; border-radius:4px">
                <row spacing=10 align_y={iced::Alignment::Center}>
                    <icon src="assets/icons/new.svg" width={Length::Fixed(18.0)}/>
                    <h3>"dfw2 - CSS + hx_* + iterasi + icon, event lewat string"</h3>
                    <spacer/>
                    <button on_press={mrust_fw::on("clear")}
                        style="background:#007acc; color:white; border-radius:4">"bersihkan"</button>
                </row>
            </header>

            <row spacing=10>
                <input placeholder="Kata kunci" hx_value={app.q}
                    on_input={mrust_fw::on_val("q")}/>
                <button on_press={mrust_fw::on("add")}>"+ item"</button>
            </row>

            <checkbox label="tampilkan daftar" is_checked={app.live}
                on_toggle={mrust_fw::act_v(|a: &mut App, _: bool| a.live = !a.live)}/>

            <row spacing=8>
                <icon src="assets/icons/dot.svg" width={Length::Fixed(14.0)}/>
                <text hx_visible={app.live}>
                    "Responsif tiap " {app.detik} "s (polling via interval)"
                </text>
            </row>

            <column spacing=4 hx_hoist="align_x" align_x={iced::Alignment::Center}>
                <div style="background:#1a1c21; padding:4px; border-radius:3px">
                    <column spacing=2>
                        {app.items.iter().enumerate().map(|(i, it)| {
                            view! {
                                <row spacing=6 style="padding:2px">
                                    <text size=13>{it}</text>
                                    <spacer/>
                                    <button on_press={mrust_fw::act(move |a: &mut App| {
                                        if i < a.items.len() { a.items.remove(i); }
                                    })} style="background:#e5484d; color:white; border-radius:3px">"hapus"</button>
                                </row>
                            }
                            .into()
                        }).collect::<Vec<_>>()}
                    </column>
                </div>
            </column>

            <hr style="color:#ff8800"/>
            <button on_press={mrust_fw::on("toggle")}>"toggle live (via metode App)"</button>
            <text hx_if={app.live && !app.q.is_empty()}>
                "q: " {app.q} " (hx_if = live & q terisi)"
            </text>
        </column>
    }
    .into()
}
