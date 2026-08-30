use iced::Element;
use mrust_macro::view;

#[derive(Debug, Clone)]
enum Message {
    Tekan,
    Saklar(bool),
    Simpan,
}

#[derive(Default)]
struct App {
    live: bool,
    simpanan: u32,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        view! {
            <column spacing=14 padding=24>
                <text size=22>"Demo6 - event alias & hx_if/hx_visible/hx_disabled"</text>

                <row spacing=8>
                    <button hx_click=Message::Tekan>"hx_click -> on_press"</button>
                    <button mix_press=Message::Tekan>"mix_press -> on_press"</button>
                    <button onclick=Message::Tekan>"onclick -> on_press"</button>
                </row>

                <checkbox label="mode live" is_checked={self.live} ontoggle=Message::Saklar/>

                <text hx_visible={self.live}>"laporan live: aktif"</text>
                <text hx_if={!self.live}>"laporan tertutup saat live"</text>

                <button hx_click={Message::Simpan}
                    hx_disabled={self.simpanan > 3}>
                    {"simpan (nonaktif setelah 3x)"}</button>
                <text size=12>"tersimpan {self.simpanan}x"</text>

                <text size=12 color={iced::Color::from_rgb(0.6, 0.6, 0.65)}>
                    "hx_if / hx_visible identik; hx_disabled hanya utk button (on_press_maybe)"
                </text>
            </column>
        }
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(
        "mrust demo6 - event & kondisi",
        |app: &mut App, msg: Message| match msg {
            Message::Tekan => {}
            Message::Saklar(on) => app.live = on,
            Message::Simpan => app.simpanan += 1,
        },
        App::view,
    )
}
