//! `dfw` — demo PRD 3 (`mrust-fw`): satu layar + aksi, TANPA `iced::`
//! sama sekali (tanpa Message, update, subscription). Bandingkan demo3 yang
//! menulis boilerplate `iced::application` + `enum Message` + `fn update`.
use mrust_fw::{self, view};

#[derive(Default)]
struct App {
    count: i32,
    nama: String,
    live: bool,
    ticks: u32,
}

fn view(app: &App) -> mrust_fw::Element<'_, App> {
    view! {
        <column spacing=16 padding=32>
            <text size=26>"dfw - mrust-fw tanpa Message/update/iced"</text>
            <text size=40>"Counter: " {app.count}</text>

            <row spacing=8>
                <button on_press={mrust_fw::act(|a: &mut App| a.count -= 1)}>"-"</button>
                <button on_press={mrust_fw::act(|a: &mut App| a.count += 1)}>"+"</button>
                <button on_press={mrust_fw::act(|a: &mut App| { a.count = 0; a.ticks += 1 })}>"reset"</button>
            </row>

            <input placeholder="Nama" value={app.nama}
                on_input={mrust_fw::act_v(|a: &mut App, v: String| a.nama = v)}/>

            <checkbox label="Live" is_checked={app.live}
                on_toggle={mrust_fw::act_v(|a: &mut App, on: bool| a.live = on)}/>

            <text hx_visible={app.live}>"Laporan live: ON"</text>
            <text>"Salam, " {app.nama}</text>
            <text>"Ticks: " {app.ticks}</text>
        </column>
    }
    .into()
}

fn main() -> mrust_fw::Result {
    mrust_fw::run::<App>("dfw - PRD3", view)
}
