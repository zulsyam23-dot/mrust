use iced::Element;
use mrust_macro::view;

#[derive(Default)]
struct App;

fn main() -> iced::Result {
    iced::application("mrust demo5 - styling CSS", update, view).run()
}

fn update(_app: &mut App, _msg: ()) {}

fn view(_app: &App) -> Element<'_, ()> {
    view! {
        <column spacing=16 padding=24>

            <text size={22.0}>"Demo5 - styling CSS"</text>

            <container style="background:#111; color:#eee; padding:12px;
                              border:1px solid #444; border-radius:6px; shadow:0 2 #00000060">
                <text style="color:#66d9ef; font-weight:bold">"container + style lengkap"</text>
                <text style="color:#a6e22e">"teks: warna + tebal"</text>
            </container>

            <row spacing=10>
                <button style="background:#007acc; color:white; border-radius:4;
                               border:1px solid #0099ff">"biru"</button>
                <button style="background:#e5484d; color:white; border-radius:4">"merah"</button>
            </row>

            <hr style="color:#ff8800"/>

            <rule style="color:#22aaaa"/>

            <container style="background:#1a1c21; padding:8px; width:fill">
                <row style="gap:8">
                    <text style="color:#f8f8f2">"kiri"</text>
                    <text style="color:#f8f8f2; text-align:center">"tengah"</text>
                    <text style="color:#f8f8f2">"kanan"</text>
                </row>
            </container>

            <text size={12.0} color={iced::Color::from_rgb(0.6, 0.6, 0.65)}>
                "style={expr} (bukan string) diteruskan sebagai .style(expr)"
            </text>
        </column>
    }
    .into()
}
