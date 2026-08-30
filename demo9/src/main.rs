use iced::{Element, Subscription};
use mrust_macro::view;

#[derive(Debug, Clone)]
enum Message {
    Tick,
    NamaChanged(String),
}

#[derive(Default)]
struct App {
    detik: u32,
    nama: String,
}

fn main() -> iced::Result {
    iced::application("mrust demo9 - runtime polling", update, view)
        .subscription(|_| {
            Subscription::batch(vec![
                // polling ala-htmx `hx_poll="every:2s"` — helper runtime.
                mrust_runtime::interval(2.0, Message::Tick),
            ])
        })
        .run()
}

fn update(app: &mut App, msg: Message) {
    match msg {
        Message::Tick => {
            app.detik += 1;
            // jika nama kosong, isi otomatis tiap tick (efek polling)
            if app.nama.is_empty() && app.detik > 2 {
                app.nama = format!("otomatis @ {}.0s", app.detik);
            }
        }
        Message::NamaChanged(n) => app.nama = n,
    }
}

fn view(app: &App) -> Element<'_, Message> {
    view! {
        <column spacing=14 padding=24>
            <text size=22>"Demo9 - mrust_runtime::interval (polling 2s)"</text>

            <text size=30 color={iced::Color::from_rgb(0.6, 1.0, 0.7)}>
                "detik: {app.detik}"
            </text>

            <input placeholder="Nama (dari tick bila kosong)" hx_value={app.nama}
                hx_value_to={Message::NamaChanged}/>

            <text hx_visible={!app.nama.is_empty()}>
                "nama terisi setelah polling: " {app.nama.clone()}
            </text>

            <text size=12 color={iced::Color::from_rgb(0.6, 0.6, 0.65)}>
                "interval butuh backend timer (tokio) & dipasang di fn subscription()"
            </text>
        </column>
    }
    .into()
}
