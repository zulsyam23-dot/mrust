//! Helper runtime: `interval` (polling ala-htmx), `if_elem` (kondisional),
//! `Spread` (anak layout), dan re-export `Subscription`.

use iced::{Renderer, Theme};

/// Re-export `iced::Subscription` agar pengguna framework tak perlu depend `iced`.
pub use iced::Subscription;

/// Polling ala-htmx `hx_poll="every:2s"`: `Subscription` yang memancarkan
/// `Message` (klon) setiap `secs` detik, dipakai di `fn subscription()` app.
/// Generic atas `Message`, jadi cocok untuk `Action<App>` (langsung) maupun
/// `enum Message` buatan user.
///
/// id interval diturunkan dari durasi — dua interval beda durasi tak bentrok.
/// (`ponytail:` id per-durasi cukup utk kasus umum; gunakan id unik per-elemen
/// bila banyak polling identik-durasi di app besar.)
pub fn interval<Message: Clone + Send + 'static>(secs: f64, msg: Message) -> Subscription<Message> {
    use iced::futures::stream;

    let every = std::time::Duration::from_secs_f64(secs.max(0.0));
    // Bangun stream sendiri (ticker tokio) dan map `Instant` -> `Message` DI
    // DALAM stream — bukan via `Subscription::map`, yang di iced 0.13 panic saat
    // closure-nya menangkap `msg` (wajib non-capturing).
    let stream = stream::unfold(every, move |d| {
        let msg = msg.clone();
        async move {
            tokio::time::sleep(d).await;
            Some((msg, d))
        }
    });
    iced::Subscription::run_with_id(every, stream)
}

/// `if_elem`: hasilkan widget atau kosongkan (Space tinggi-0) sesuai kondisi —
/// reaktivitas ala-htmx `hx_if`/`hx_visible`, bertipe `Element` sama (menambatkan
/// generik Message/Theme/Renderer agar `if/else` hasil `view!` ter-infer).
pub fn if_elem<'a, Message: 'a>(
    cond: bool,
    yes: iced::Element<'a, Message, Theme, Renderer>,
) -> iced::Element<'a, Message, Theme, Renderer> {
    if cond {
        yes
    } else {
        iced::widget::Space::with_height(iced::Length::Fixed(0.0)).into()
    }
}

/// Spread anak `{ ekspresi }` container/layout: terima `Element` tunggal atau
/// `Vec<Element>` dan hasilkan `Vec<Element>` seragam untuk `.extend(...)`.
pub trait Spread<'a, Message, Theme, Renderer> {
    fn spread(self) -> Vec<iced::Element<'a, Message, Theme, Renderer>>;
}

impl<'a, Message, Theme, Renderer> Spread<'a, Message, Theme, Renderer>
    for iced::Element<'a, Message, Theme, Renderer>
{
    fn spread(self) -> Vec<iced::Element<'a, Message, Theme, Renderer>> {
        vec![self]
    }
}

impl<'a, Message, Theme, Renderer> Spread<'a, Message, Theme, Renderer>
    for Vec<iced::Element<'a, Message, Theme, Renderer>>
{
    fn spread(self) -> Vec<iced::Element<'a, Message, Theme, Renderer>> {
        self
    }
}
