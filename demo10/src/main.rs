use iced::{Element, Length, Subscription};
use mrust_fw::view;
#[derive(Debug, Clone)]
enum Message {
    Cari,
    QueryChanged(String),
    Saklar(bool),
    TambahItem,
    HapusItem(usize),
    Tick,
}

#[derive(Default)]
struct App {
    q: String,
    live: bool,
    items: Vec<String>,
    detik: u32,
}

fn main() -> iced::Result {
    iced::application("mrust demo10 - semua fitur", update, view)
        .subscription(|_| {
            Subscription::batch(vec![mrust_fw::interval(2.0, Message::Tick)])
        })
        .run()
}

fn update(app: &mut App, msg: Message) {
    match msg {
        Message::Cari => {}
        Message::QueryChanged(q) => app.q = q,
        Message::Saklar(on) => app.live = on,
        Message::TambahItem => {
            let n = app.items.len() + 1;
            app.items.push(format!("item ke-{n}"));
        }
        Message::HapusItem(i) => {
            if i < app.items.len() {
                app.items.remove(i);
            }
        }
        Message::Tick => app.detik += 1,
    }
}

fn view(app: &App) -> Element<'_, Message> {
    view! {
        <column spacing=12 padding=24>
            <header padding=8 style="background:#26292e; padding:8px; border-radius:4px">
                <row spacing=10>
                    <icon src="assets/icons/new.svg" width={Length::Fixed(18.0)}/>
                    <h3>"Demo10 - gabungan semua fitur (v0.1..v0.5 + hx_* + runtime)"</h3>
                    <spacer/>
                    <button hx_click=Message::Cari style="background:#007acc; color:white; border-radius:4">
                        "cari"
                    </button>
                </row>
            </header>

            <row spacing=10>
                <input placeholder="Kata kunci" hx_value={app.q}
                    hx_value_to={Message::QueryChanged}/>
                <button mix_press=Message::TambahItem>"+ item"</button>
            </row>

            <checkbox label="tampilkan daftar" is_checked={app.live}
                ontoggle={Message::Saklar}/>

            <text hx_visible={app.live}>
                "Responsif tiap {app.detik}s (polling via interval)"
            </text>

            <column spacing=4 hx_hoist="align_x" align_x={iced::Alignment::Center}>
                <div style="background:#1a1c21; padding:4px; border-radius:3px">
                    <column spacing=2>
                        {app.items.iter().enumerate().map(|(i, it)| {
                            view! {
                                <row spacing=6 style="padding:2px">
                                    <text size=13>{it}</text>
                                    <spacer/>
                                    <button on_press=Message::HapusItem(i) style="background:#e5484d; color:white; border-radius:3px">"hapus"</button>
                                </row>
                            }
                            .into()
                        }).collect::<Vec<_>>()}
                    </column>
                </div>
            </column>

            <div style="background:#16181d; padding:8px; border-radius:4px">
                <row spacing=4>
                    <text size=13 color={iced::Color::from_rgb(0.6, 0.6, 0.65)}>
                        "q: " {app.q.clone()} {" | "}
                    </text>
                    <code>"ex tab mix hx css editor list"</code>
                    <text size=13 color={iced::Color::from_rgb(0.6, 0.6, 0.65)}>" | "</text>
                    <mark>"mark"</mark>
                </row>
            </div>
            <row>
                <text size=13>"Dicetak via &lt;code&gt; &amp; &lt;mark&gt;"</text>
            </row>
        </column>
    }
    .into()
}
