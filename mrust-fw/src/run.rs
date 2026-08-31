//! Entrypoint aplikasi: `run` / `run_with` membungkus `iced::application` dan
//! menyembunyikan loop + update internal.

use crate::action::{self, AppSubscription, Element};

/// `Result` kembalian `run` (alias `iced::Result`).
pub type Result = iced::Result;

/// Bangun aplikasi Iced dengan satu `view`, tanpa `Message`/`update`/`Subscription`.
///
/// Memetakan ke `iced::application(...)` dan menyembunyikan loop + `update`
/// internal: tiap `Action<App>` yang datang memuat closure untuk memutasi `App`.
pub fn run<App>(title: &'static str, view: for<'a> fn(&'a App) -> Element<'a, App>) -> Result
where
    App: 'static + Send + Sync + Default,
{
    iced::application(title, action::update_internal::<App>, view)
        .subscription(action::subscription_internal::<App>)
        .run()
}

/// Sama dengan `run`, plus `subscription` (polling `interval` dll):
///
/// ```no_run
/// use mrust_fw::{view, Subscription};
/// # #[derive(Default)] struct App;
/// # fn view(_: &App) -> mrust_fw::Element<'_, App> { mrust_fw::view!{<text>"x"</text>}.into() }
/// # fn main() -> mrust_fw::Result {
/// mrust_fw::run_with::<App>(
///     "app",
///     view,
///     |_| Subscription::batch(vec![
///         mrust_fw::interval(2.0, mrust_fw::act(|a: &mut App| { let _ = a; })),
///     ]),
/// )
/// # }
/// ```
pub fn run_with<App>(
    title: &'static str,
    view: for<'a> fn(&'a App) -> Element<'a, App>,
    subscription: fn(&App) -> AppSubscription<App>,
) -> Result
where
    App: 'static + Send + Sync + Default,
{
    iced::application(title, action::update_internal::<App>, view)
        .subscription(subscription)
        .run()
}
