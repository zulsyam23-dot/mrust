use iced::Element;
use mrust_macro::view;

#[derive(Debug, Clone)]
enum Message {
    QueryChanged(String),
    EmailChanged(String),
}

#[derive(Default)]
struct App {
    q: String,
    email: String,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        view! {
            <column spacing=14 padding=24>
                <text size=22>"Demo7 - hx_value / hx_bind & hx_hoist"</text>

                <input placeholder="Kata kunci" hx_value={self.q}
                    hx_value_to={Message::QueryChanged}/>
                <input placeholder="Email" hx_bind={self.email}
                    hx_bind_to={Message::EmailChanged}/>

                <text size=13>"q: " {self.q.clone()}</text>
                <text size=13>"email: " {self.email.clone()}</text>

                <column spacing=6 hx_hoist="align_x" align_x={iced::Alignment::Center}>
                    <p>"teks ini di-hoist align_x=Center"</p>
                    <p>"begitu juga ini"</p>
                    <text hx_disinherit="align_x">"tapi ini tidak (hx_disinherit)"</text>
                </column>

                <text size=12 color={iced::Color::from_rgb(0.6, 0.6, 0.65)}>
                    "hx_value/tanpa hx_value_to tak menambah on_input"
                </text>
            </column>
        }
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(
        "mrust demo7 - binding & hoist",
        |app: &mut App, msg: Message| match msg {
            Message::QueryChanged(q) => app.q = q,
            Message::EmailChanged(e) => app.email = e,
        },
        App::view,
    )
}
