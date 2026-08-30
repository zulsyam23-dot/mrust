use iced::Element;
use mrust_macro::view;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kategori {
    Ringan,
    Sedang,
}

#[derive(Debug, Clone)]
enum Message {
    Pilih(String),
    Kat(Kategori),
}

#[derive(Default)]
struct App {
    pilihan: Option<String>,
    gent: Option<Kategori>,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        view! {
            <column spacing=12 padding=24>
                <text size=26>"Demo4 - kosakata HTML v0.4"</text>

                <section padding=8 style="background:#1a1c21; color:#e6e6e6; padding:8px">
                    <h1>"Judul h1"</h1>
                    <h2>"Judul h2"</h2>
                    <h6>"Judul h6"</h6>
                </section>

                <section padding=8>
                    <text>"Role teks: "</text>
                    <strong>"ini kuat"</strong>
                    <code>"let x = 1"</code>
                    <small>"kecil"</small>
                    <mark>"disorot"</mark>
                </section>

                <nav padding=8 style="background:#26292e; padding:8px">
                    <text>"Nav (container semantik): "</text>
                    <a onclick=Message::Pilih("beranda".into())>"Beranda"</a>
                    <a onclick=Message::Pilih("tentang".into())>"Tentang"</a>
                </nav>

                <dl spacing=4>
                    <dt>"Rust"</dt>
                    <dd>"Bahasa sistem modern"</dd>
                    <dt>"Iced"</dt>
                    <dd>"GUI native"</dd>
                </dl>

                <ul spacing=4>
                    <li>"satu"</li>
                    <li>"dua"</li>
                </ul>

                <select options={vec!["Ringan", "Sedang"]}
                    selected={self.pilihan.as_deref()}
                    on_selected={move |s: &str| Message::Pilih(s.to_string())}/>

                <row spacing=12>
                    <radio label={"Ringan"} value={Kategori::Ringan}
                        selected={self.gent} on_selected={Message::Kat}/>
                    <radio label={"Sedang"} value={Kategori::Sedang}
                        selected={self.gent} on_selected={Message::Kat}/>
                </row>

                <meter range={0.0_f32..=100.0} value={40.0}/>

                <text size=13 color={iced::Color::from_rgb(0.6, 0.6, 0.65)}>
                    "pilihan: " {self.pilihan.clone().unwrap_or_default()}
                    {"  |  kategori: "} {format!("{:?}", self.gent)}
                </text>
            </column>
        }
        .into()
    }
}

fn main() -> iced::Result {
    iced::run(
        "mrust demo4 - kosakata HTML v0.4",
        |app: &mut App, msg: Message| match msg {
            Message::Pilih(x) => app.pilihan = Some(x),
            Message::Kat(k) => app.gent = Some(k),
        },
        App::view,
    )
}
