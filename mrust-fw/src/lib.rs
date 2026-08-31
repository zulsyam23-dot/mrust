//! `mrust-fw` — framework tipis (PRD 3) di atas `mrust-macro` + runtime.
//!
//! User cukup membangun isi layar dengan `view!` dan memberi tahu framework apa
//! yang tampil / apa yang terjadi. Selebihnya (loop, state, async, deps Iced)
//! diurus framework di belakang — tanpa menulis `iced::application`,
//! `enum Message`, `fn update`, atau `Subscription` manual.
//!
//! ## Mode 1 — DSL aksi string (paling sedikit Rust; "Rust tak terlihat")
//!
//! Mutasi didefinisikan SEKALI sebagai metode `App`, event di `view!` hanya
//! memakai NAMA string lewat `on("nama")` / `on_val("nama")`. Tidak ada closure
//! di view, tidak ada `Message`.
//!
//! ```no_run
//! use mrust_fw::view;
//!
//! #[derive(Default)]
//! struct App { count: i32 }
//! impl App {
//!     fn inc(&mut self) { self.count += 1 }
//!     fn dec(&mut self) { self.count -= 1 }
//! }
//!
//! fn view(app: &App) -> mrust_fw::Element<'_, App> {
//!     view! {
//!         <column spacing=16 padding=32>
//!             <text size=30>"Counter: " {app.count}</text>
//!             <row spacing=8>
//!                 <button on_press={mrust_fw::on("dec")}>"-"</button>
//!                 <button on_press={mrust_fw::on("inc")}>"+"</button>
//!             </row>
//!         </column>
//!     }
//!     .into()
//! }
//!
//! fn main() -> mrust_fw::Result {
//!     let a = mrust_fw::actions![ App : "inc" => App::inc, "dec" => App::dec ];
//!     mrust_fw::run_with_actions::<App>("Counter", view, a, None)
//! }
//! ```
//!
//! ## Mode 2 — closure langsung (kasus dinamis / ad-hoc)
//!
//! Untuk event yang butuh logika inline atau payload dinamis (mis. hapus item
//! per-baris), closure `act`/`act_v` tetap dipakai di `view!`:
//!
//! ```no_run
//! use mrust_fw::view;
//! # #[derive(Default)] struct App { count: i32 }
//! # fn view(app: &App) -> mrust_fw::Element<'_, App> {
//! view! {
//!     <row spacing=8>
//!         <button on_press={mrust_fw::act(|a: &mut App| a.count -= 1)}>"-"</button>
//!         <button on_press={mrust_fw::act(|a: &mut App| a.count += 1)}>"+"</button>
//!     </row>
//! }
//! .into()
//! # }
//! # fn main() -> mrust_fw::Result { mrust_fw::run::<App>("Counter", view) }
//! ```
//!
//! ## Mode 3 — `app!`: seluruh boilerplate struktural disembunyikan
//!
//! Makro [`app!`](crate::app!) menghapus `struct App`, `#[derive(Default)]`,
//! registri `Actions`, `fn main`, dan wiring. User hanya menulis state + mutasi +
//! `fn view`. Lihat dokumen makro untuk contoh. `ponytail:` satu `app!` per file.

mod action;
mod dsl;
mod run;
mod runtime;

pub use action::{act, act_v, Action, AppSubscription, Element};
pub use dsl::{on, on_val, register_actions, run_with_actions, Actions};
pub use run::{run, run_with, Result};
pub use runtime::{if_elem, interval, Spread, Subscription};

pub use mrust_macro::view;
pub use mrust_macro::view as mrust;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{act, actions, actions_val, on, on_val, view};

    #[derive(Default, PartialEq)]
    struct TApp {
        n: i32,
    }

    impl TApp {
        fn inc(&mut self) {
            self.n += 1;
        }
        fn set_n(&mut self, v: String) {
            self.n = v.parse().unwrap_or(0);
        }
    }

    #[test]
    fn action_mutates_app() {
        let mut app = TApp { n: 1 };
        let a = act(|a: &mut TApp| a.n += 10);
        a.run(&mut app);
        assert_eq!(app.n, 11);
    }

    #[test]
    fn action_is_send_debug_clone() {
        fn assert_send_clone<T: Send + Clone>() {}
        assert_send_clone::<Action<TApp>>();
        let a = act(|a: &mut TApp| a.n += 1);
        let b = a.clone();
        let mut app = TApp::default();
        b.run(&mut app);
        assert_eq!(app.n, 1);
        let _ = format!("{:?}", a);
    }

    #[test]
    fn view_macro_produces_element_without_iced() {
        fn v(app: &TApp) -> Element<'_, TApp> {
            view! { <text>"halo " {app.n}</text> }.into()
        }
        let e: Element<'_, TApp> = v(&TApp { n: 3 });
        // kompilasi-verifikasi: view! + Element tanpa menulis iced::.
        let _ = e;
    }

    #[test]
    fn actions_registry_by_name() {
        let a = actions![TApp : "inc" => TApp::inc];
        register_actions(a);
        let act = on::<TApp>("inc");
        let mut app = TApp { n: 0 };
        act.run(&mut app);
        assert_eq!(app.n, 1);
        // nama tak dikenal = no-op
        let noop = on::<TApp>("tidak-ada");
        noop.run(&mut app);
        assert_eq!(app.n, 1);
    }

    #[test]
    fn actions_val_setter_by_name() {
        let a = actions_val![ TApp : ; "n" => TApp::set_n ];
        register_actions(a);
        let h = on_val::<TApp>("n");
        let act = h("42".to_string());
        let mut app = TApp { n: 0 };
        act.run(&mut app);
        assert_eq!(app.n, 42);
    }
}
