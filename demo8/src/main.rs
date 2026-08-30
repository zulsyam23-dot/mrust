use iced::{Element, Length};
use mrust_macro::view;

#[derive(Debug, Clone)]
enum Message {
    Buka,
}

#[derive(Default)]
struct App;

impl App {
    fn view(&self) -> Element<'_, Message> {
        // `handle` disiapkan manual (diteruskan lewat src={expr}).
        let handle = iced::widget::svg::Handle::from_memory(
            std::borrow::Cow::Borrowed(
                &include_bytes!("../assets/icons/menu.svg")[..],
            ),
        );

        view! {
            <column spacing=14 padding=24>
                <text size=22>"Demo8 - asset src (literal di-embed & ekspresi)"</text>

                <row spacing=16>
                    <icon src="assets/icons/dot.svg" width={Length::Fixed(24.0)}/>
                    <icon src="assets/icons/menu.svg" width={Length::Fixed(24.0)}/>
                    <svg src="assets/icons/dot.svg" width={Length::Fixed(24.0)}/>
                </row>

                <button on_press=Message::Buka padding=6>
                    <icon src="assets/icons/menu.svg" width={Length::Fixed(20.0)}/>
                </button>

                <row spacing=16>
                    <icon src={handle.clone()} width={Length::Fixed(24.0)}/>
                    <icon src={handle} width={Length::Fixed(24.0)}/>
                    <text size=13>"(src={handle} diteruskan apa adanya)"</text>
                </row>
            </column>
        }
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(
        "mrust demo8 - asset src",
        |_app: &mut App, msg: Message| match msg {
            Message::Buka => {}
        },
        App::view,
    )
}
