use iced::{Element, widget::text_editor};
use mrust_fw::view;

#[derive(Debug, Clone)]
enum Message {
    Tambah,
    Hapus(usize),
    Edit(text_editor::Action),
    Simpan,
}

#[derive(Default)]
struct App {
    daftar: Vec<String>,
    editor: text_editor::Content,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        view! {
            <column spacing=12 padding=24>
                <text size=22>"Demo2 - iterasi dinamis & editor"</text>

                <row spacing=8>
                    <button on_press=Message::Tambah>"Tambah"</button>
                    <button on_press=Message::Simpan>"Simpan isi editor"</button>
                </row>

                <text size=14>"Daftar (di-render dari Vec lewat { } + Spread):"</text>
                <column spacing=4>
                    {self.daftar.iter().enumerate().map(|(i, item)| {
                        view! {
                            <row spacing=8>
                                <text size=13>{item}</text>
                                <button on_press=Message::Hapus(i)>"x"</button>
                            </row>
                        }
                        .into()
                    }).collect::<Vec<_>>()}
                </column>

                <text size=14>"Editor (text_editor + on_action):"</text>
                <editor state={&self.editor} on_action={Message::Edit}
                    height={iced::Length::Fixed(180.0)}
                    style="padding:8px"/>

                <text size=12 color={iced::Color::from_rgb(0.6, 0.6, 0.65)}>
                    "(anak { ... } menerima satu Element atau Vec<Element>)"
                </text>
            </column>
        }
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(
        "mrust demo2 - iterasi & editor",
        |app: &mut App, msg: Message| match msg {
            Message::Tambah => {
                let n = app.daftar.len() + 1;
                app.daftar.push(format!("item ke-{n}"));
            }
            Message::Hapus(i) => {
                if i < app.daftar.len() {
                    app.daftar.remove(i);
                }
            }
            Message::Edit(a) => app.editor.perform(a),
            Message::Simpan => {
                let teks = app.editor.text().to_string();
                if !teks.is_empty() {
                    app.daftar.push(teks);
                    app.editor = text_editor::Content::new();
                }
            }
        },
        App::view,
    )
}
