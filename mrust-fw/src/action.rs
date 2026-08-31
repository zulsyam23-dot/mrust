//! Inti event: `Action<App>` menggantikan `enum Message`, plus builder `act`/`act_v`
//! dan fungsional internal loop Iced (`update`/`subscription`).

use iced::{Renderer, Subscription, Theme};
use std::sync::Arc;

/// Message internal tersembunyi: closure yang memutasi `App`.
///
/// Inilah pengganti `enum Message` untuk kasus umum. Framework menahan `App`,
/// dan tiap `Action<App>` adalah satu "event" yang memutasi state; perubahan itu
/// memicu re-render `view` lewat reaktivitas bawaan Iced.
///
/// `App` harus `Send + Sync` agar `Action` bisa `Send` (syarat `Message` Iced).
/// Disimpan di `Arc` agar `Clone` (syarat lain `Message` Iced).
pub struct Action<App>(Arc<dyn Fn(&mut App) + Send + Sync + 'static>);

impl<App> Clone for Action<App> {
    fn clone(&self) -> Self {
        Action(self.0.clone())
    }
}

impl<App> std::fmt::Debug for Action<App> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Action(..)")
    }
}

/// `Element` yang dipakai `view` — Message-nya adalah `Action<App>` (tersembunyi),
/// dan umur borrow mengikuti `&App` (data state dipinjam saat render).
pub type Element<'a, App> = iced::Element<'a, Action<App>, Theme, Renderer>;

impl<App> Action<App> {
    pub(crate) fn run(&self, app: &mut App) {
        (self.0)(app);
    }
}

/// `Subscription` dengan Message `Action<App>` (tersembunyi).
pub type AppSubscription<App> = Subscription<Action<App>>;

/// Bangun `Action<App>` dari sebuah closure yang memutasi `App`.
///
/// `act` mengemas closure sehingga bisa dipakai sebagai nilai event di dalam
/// `view!` (mis. `on_press={mrust_fw::act(|a: &mut App| a.count += 1)}`) tanpa
/// perlu `enum Message`.
pub fn act<App: Send + Sync + 'static>(
    f: impl Fn(&mut App) + Send + Sync + 'static,
) -> Action<App> {
    Action(Arc::new(f))
}

/// Bangun handler untuk event yang membawa nilai (input, toggle, dsb.).
///
/// `view!` memetakan `on_input`/`on_toggle` ke closure `|v| Action(...)`, jadi
/// kembalian `act_v` adalah closure `Fn(V) -> Action<App>` — bukan `Action`
/// langsung. Contoh: `on_input={mrust_fw::act_v(|a: &mut App, v: String| a.nama = v)}`.
pub fn act_v<App, V, F>(f: F) -> impl Fn(V) -> Action<App>
where
    App: Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    F: Fn(&mut App, V) + Send + Sync + 'static,
{
    let f = Arc::new(f);
    move |v: V| {
        let f = Arc::clone(&f);
        Action(Arc::new(move |a: &mut App| f(a, v.clone())))
    }
}

pub(crate) fn update_internal<App>(app: &mut App, action: Action<App>) -> iced::Task<Action<App>> {
    action.run(app);
    iced::Task::none()
}

pub(crate) fn subscription_internal<App>(_: &App) -> Subscription<Action<App>> {
    Subscription::none()
}
