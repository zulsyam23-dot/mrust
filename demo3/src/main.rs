use iced::{Element, Subscription};
use mrust_fw::view;

#[derive(Debug, Clone)]
enum Message {
    QueryChanged(String),
    SaklarChanged(bool),
    NamaChanged(String),
    Cari,
    Tick,
}

#[derive(Default)]
struct App {
    q: String,
    nama: String,
    live: bool,
    ticks: u32,
}

fn main() -> iced::Result {
    iced::application("mrust demo3 - token hx_*", update, view)
        .subscription(|_| Subscription::batch(vec![mrust_fw::interval(2.0, Message::Tick)]))
        .run()
}

fn update(app: &mut App, msg: Message) {
    match msg {
        Message::QueryChanged(q) => app.q = q,
        Message::NamaChanged(n) => app.nama = n,
        Message::SaklarChanged(on) => app.live = on,
        Message::Cari => {}
        Message::Tick => app.ticks += 1,
    }
}

fn view(app: &App) -> Element<'_, Message> {
    view! {
        <column spacing=16 padding=32>
            <text size=26>"Demo3 - hx_value / hx_hoist / polling"</text>

            <input placeholder="Kata kunci" hx_value={app.q}
                hx_value_to=Message::QueryChanged/>

            <input placeholder="Nama" hx_bind={app.nama}
                hx_bind_to=Message::NamaChanged/>

            <checkbox label="Live" is_checked={app.live}
                ontoggle=Message::SaklarChanged/>

            <button hx_click=Message::Cari>"cari"</button>
            <button mix_press=Message::Cari>"cari (mix)"</button>
            <button hx_click=Message::Cari hx_disabled={app.ticks > 3}>"cari (nonaktif saat ticks>3)"</button>
            <text hx_visible={app.live}>"laporan live: ON"</text>
            <text hx_if={app.live && app.nama.len() > 2}>"live + nama terisi"</text>

            <column spacing=8 hx_hoist="align_x" align_x={iced::Alignment::Center}>
                <p>"Kata kunci: " {app.q}</p>
                <p>"Nama: " {app.nama}</p>
                <p>"Ticks (polling 2s): " {app.ticks}</p>
                <text>"(semua text di sini di-hoist align_x=Center)"</text>
            </column>
        </column>
    }
    .into()
}
