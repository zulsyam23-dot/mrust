use iced::Element;
use mrust_fw::view;

#[derive(Debug, Clone)]
enum Message {
    Inc,
    Dec,
    Reset,
    NameChanged(String),
    Toggle(bool),
    VolumeChanged(f32),
}

#[derive(Default)]
struct App {
    count: i32,
    name: String,
    ready: bool,
    volume: f32,
}

fn main() -> iced::Result {
    iced::run(
        "mrust counter",
        |app: &mut App, msg: Message| match msg {
            Message::Inc => app.count += 1,
            Message::Dec => app.count -= 1,
            Message::Reset => app.count = 0,
            Message::NameChanged(name) => app.name = name,
            Message::Toggle(ready) => app.ready = ready,
            Message::VolumeChanged(volume) => app.volume = volume,
        },
        App::view,
    )
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        view! {
            <column spacing=16 padding=32>
                <text size=26 color={iced::Color::from_rgb(0.8, 0.85, 1.0)}>MRust counter</text>

                <div padding=12>
                    <row spacing=8 align_y={iced::Alignment::Center}>
                        <button onclick=Message::Dec>-1</button>
                        <button on_press=Message::Reset>0</button>
                        <button onclick=Message::Inc>+1</button>
                    </row>
                </div>

                <text size=40 color={iced::Color::from_rgb(0.9, 0.95, 1.0)}>
                    Counter: {self.count}
                </text>

                <hr/>

                <input placeholder="Nama kamu" value={self.name} oninput=Message::NameChanged/>

                <p>"Salam, " {self.name}</p>

                <checkbox label="Sudah siap" is_checked={self.ready} ontoggle=Message::Toggle/>
                <toggler is_checked={self.ready} on_toggle=Message::Toggle/>

                <slider range={0.0_f32..=100.0} value={self.volume}
                    on_change=Message::VolumeChanged/>
                <progress range={0.0_f32..=100.0} value={self.volume}/>

                <br/>

                <text size=13 color={iced::Color::from_rgb(0.6, 0.6, 0.65)}>
                    "div hr p input checkbox slider progress br onclick - hanya fn view + view!{}"
                </text>
            </column>
        }
        .into()
    }
}