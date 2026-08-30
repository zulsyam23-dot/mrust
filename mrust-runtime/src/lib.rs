//! `mrust-runtime` — helper runtime opsional untuk fitur `hx_*` yang
//! membutuhkan perulangan/efek (v0.3 prd2.md): interval polling.
//!
//! Dependensi crate ini hanya ditarik bila fitur tersebut dipakai — sama seperti
//! `Spread` untuk anak `{ ekspresi }` pada layout/container.

use iced::{Element, Subscription};

pub trait Spread<'a, M, Theme, Renderer> {
    fn spread(self) -> Vec<Element<'a, M, Theme, Renderer>>;
}

impl<'a, M, Theme, Renderer> Spread<'a, M, Theme, Renderer>
    for Element<'a, M, Theme, Renderer>
{
    fn spread(self) -> Vec<Element<'a, M, Theme, Renderer>> {
        vec![self]
    }
}

impl<'a, M, Theme, Renderer> Spread<'a, M, Theme, Renderer>
    for Vec<Element<'a, M, Theme, Renderer>>
{
    fn spread(self) -> Vec<Element<'a, M, Theme, Renderer>> {
        self
    }
}

/// Reaktivitas ala-htmx `hx_if`/`hx_visible`: hasilkan widget atau kosongkan
/// (Space tinggi-0) sesuai kondisi, bertipe `Element` yang sama (menambatkan
/// generik Message/Theme/Renderer sehingga `if/else` di hasil makro ter-infer).
pub fn if_elem<'a, Message: 'a>(
    cond: bool,
    yes: Element<'a, Message, iced::Theme, iced::Renderer>,
) -> Element<'a, Message, iced::Theme, iced::Renderer> {
    if cond {
        yes
    } else {
        iced::widget::Space::with_height(iced::Length::Fixed(0.0)).into()
    }
}

/// Polling ala-htmx `hx_poll="every:2s"`: `Subscription` yang memancarkan
/// `Message` (klon) setiap `secs` detik. Dipakai di `fn subscription()` app:
///
/// ```ignore
/// fn subscription(&self) -> Subscription<Message> {
///     Subscription::batch(vec![
///         mrust_runtime::interval(2.0, Message::Tick),
///     ])
/// }
/// ```
///
/// id interval diturunkan dari durasi → dua interval berbeda durasi tidak
/// bentrok. (`ponytail:` id per-durasi cukup utk kasus umum; gunakan id unik
/// per-elemen bila banyak polling identik-durasi di app besar.)
pub fn interval<Message: Clone + Send + 'static>(secs: f64, msg: Message) -> Subscription<Message> {
    use iced::futures::stream;

    let every = std::time::Duration::from_secs_f64(secs.max(0.0));
    // Bangun stream sendiri (ticker tokio) dan map `Instant→Message` DI DALAM
    // stream — bukan via `Subscription::map`, yang di iced 0.13 panic saat
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    enum Msg {
        Tick,
    }

    #[test]
    fn interval_returns_subscription_of_message() {
        // kompilasi-verifikasi: `interval` menghasilkan Subscription<Msg> yang
        // sah untuk `iced::Subscription`. Nilai runtime tak relevan.
        let _: iced::Subscription<Msg> = interval(2.0, Msg::Tick);
    }
}
