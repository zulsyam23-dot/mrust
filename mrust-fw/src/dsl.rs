//! DSL aksi string ala-htmx (runtime mrust-fw).
//!
//! PRD 1-3 mengharuskan tetap `.rs` & `view!` tak berubah, dan melarang DSL baru
//! di *luar* view!. Solusinya: DSL ini hidup DI DALAM framework (bukan makro baru).
//! User tetap menulis `view!`; atribut event memakai NAMA aksi (string) yang sudah
//! diregistrasi sekali lewat `actions!`, bukan closure. Mutasi didefinisikan
//! sebagai metode `App` — itu satu-satunya Rust yang ditulis (sekali, bukan tiap
//! event). Di `view!`: `on_press={mrust_fw::on("inc")}`.
//!
//! ```no_run
//! #[derive(Default)]
//! struct App { count: i32 }
//! impl App {
//!     fn inc(&mut self) { self.count += 1 }
//!     fn dec(&mut self) { self.count -= 1 }
//! }
//!
//! fn view(app: &App) -> mrust_fw::Element<'_, App> {
//!     mrust_fw::view! {
//!         <column spacing=16 padding=32>
//!             <text size=30>"Counter: " {app.count}</text>
//!             <row spacing=8>
//!                 <button on_press={mrust_fw::on("dec")}>"-"</button>
//!                 <button on_press={mrust_fw::on("inc")}>"+"</button>
//!             </row>
//!         </column>
//!     }.into()
//! }
//!
//! fn main() -> mrust_fw::Result {
//!     let a = mrust_fw::actions![ App : "inc" => App::inc, "dec" => App::dec ];
//!     mrust_fw::run_with_actions::<App>("Counter", view, a, None)
//! }
//! ```

use crate::action::{act, Action, AppSubscription, Element};
use crate::run::{run, run_with, Result};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc as StdArc, Mutex, OnceLock};

/// Setter input bernama: `fn(&mut App, String)` yang dijemput `on_val`.
type Setter<App> = StdArc<dyn Fn(&mut App, String) + Send + Sync>;

/// Registri aksi bernama per-tipe-`App`: mutator `fn(&mut App)` yang dipanggil
/// dari string. Dibangun sekali lewat `actions!` / `actions_val!`.
#[derive(Default)]
pub struct Actions<App> {
    map: HashMap<&'static str, Action<App>>,
    setmap: HashMap<&'static str, Setter<App>>,
}

impl<App> Actions<App> {
    /// Dipakai oleh makro `actions!`/`actions_val!` di crate pemakai.
    pub fn push(&mut self, name: &'static str, a: Action<App>) {
        self.map.insert(name, a);
    }
    /// Dipakai oleh makro `actions_val!` di crate pemakai.
    pub fn push_set(&mut self, name: &'static str, s: impl Fn(&mut App, String) + Send + Sync + 'static) {
        self.setmap.insert(name, StdArc::new(s));
    }
    fn get(&self, name: &str) -> Option<Action<App>> {
        self.map.get(name).cloned()
    }
    fn set(&self, name: &str) -> Option<Setter<App>> {
        self.setmap.get(name).cloned()
    }
}

/// Registri global per-`TypeId` agar `on(name)` di `view!` membaca `Actions<App>`
/// tanpa mengubah signature `view(&App)`. Karena satu `App` berjalan per proses,
/// kunci `TypeId` cukup (`ponytail:` satu-app/multi-layar; multi-app serentak butuh
/// per-window — di luar cakupan).
fn global_registry() -> &'static Mutex<HashMap<TypeId, StdArc<dyn Any + Send + Sync>>> {
    static M: OnceLock<Mutex<HashMap<TypeId, StdArc<dyn Any + Send + Sync>>>> = OnceLock::new();
    M.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_actions<App: Send + Sync + 'static>(a: Actions<App>) {
    global_registry()
        .lock()
        .unwrap()
        .insert(TypeId::of::<App>(), StdArc::new(a));
}

/// Ambil mutator `App` (mutasi state) oleh nama string, utk nilai event di
/// `view!`: `on_press={mrust_fw::on("inc")}`. Nama tak terdaftar = aksi no-op.
pub fn on<App: Send + Sync + 'static>(name: &str) -> Action<App> {
    let reg = global_registry().lock().unwrap();
    reg.get(&TypeId::of::<App>())
        .and_then(|a| a.downcast_ref::<Actions<App>>())
        .and_then(|a| a.get(name))
        .unwrap_or_else(act_noop)
}

/// Ambil setter `App` (mutasi dari String) utk `on_input={mrust_fw::on_val("nama")}`.
pub fn on_val<App: Send + Sync + 'static>(name: &str) -> impl Fn(String) -> Action<App> {
    let reg = global_registry().lock().unwrap();
    let setter = reg
        .get(&TypeId::of::<App>())
        .and_then(|a| a.downcast_ref::<Actions<App>>())
        .and_then(|a| a.set(name));
    move |v: String| match setter.clone() {
        Some(s) => act(move |app: &mut App| s(app, v.clone())),
        None => act_noop(),
    }
}

fn act_noop<App: Send + Sync + 'static>() -> Action<App> {
    act(|_: &mut App| {})
}

/// Jalankan dengan `Actions<App>` (DSL aksi string ala-htmx): daftarkan ke
/// registri, lalu `on(name)`/`on_val(name)` di `view!` membacanya. `subscription`
/// opsional (`None` = tanpa polling).
///
/// ```no_run
/// # #[derive(Default)] struct App { count: i32 }
/// # impl App { fn inc(&mut self) { self.count += 1 } }
/// # fn view(_: &App) -> mrust_fw::Element<'_, App> { mrust_fw::view!{<text>"x"</text>}.into() }
/// # fn main() -> mrust_fw::Result {
/// let a = mrust_fw::actions![ App : "inc" => App::inc ];
/// mrust_fw::run_with_actions::<App>("app", view, a, None)
/// # }
/// ```
pub fn run_with_actions<App>(
    title: &'static str,
    view: for<'a> fn(&'a App) -> Element<'a, App>,
    actions: Actions<App>,
    subscription: Option<fn(&App) -> AppSubscription<App>>,
) -> Result
where
    App: 'static + Send + Sync + Default,
{
    register_actions(actions);
    match subscription {
        Some(sub) => run_with(title, view, sub),
        None => run(title, view),
    }
}

/// Bangun `Actions<App>` dari daftar `"nama" => App::metode` (nama = string yang
/// dipakai di `on(...)`). Helper builder biasa (makro `macro_rules!`, bukan
/// proc-macro); jenis `App` disebut eksplisit di depan.
#[macro_export]
macro_rules! actions {
    ( $app:ty : $( $name:literal => $m:path ),* $(,)? ) => {{
        let mut __a = <$crate::Actions<$app>>::default();
        $( __a.push($name, $crate::act($m)); )*
        __a
    }};
}

/// Bangun `Actions<App>` + setter input: list mutator `"nama" => App::metode`,
/// lalu `;`, lalu setter `"bindNama" => |a, v| a.nama = v` untuk `on_input`.
#[macro_export]
macro_rules! actions_val {
    ( $app:ty : $( $name:literal => $m:path ),* $(,)? ; $( $vname:literal => $setter:expr ),* $(,)? ) => {{
        let mut __a = <$crate::Actions<$app>>::default();
        $( __a.push($name, $crate::act($m)); )*
        $( __a.push_set($vname, $setter); )*
        __a
    }};
}

/// Makro aplikasi "seluruh-boilerplate" — menyembunyikan `struct App`, `fn main`,
/// registri `Actions`, dan wiring. User menulis: state, mutasi, dan `fn view`
/// (layout) — `app!` menggabungkannya jadi satu program berjalan.
///
/// `view` ditulis sebagai `fn view(app: &App) -> Element<'_, App>` biasa (agar
/// lifetime/HRTB Iced benar), lalu diteruskan ke makro lewat path `view: view`.
/// Item Rust tak bergantung urutan, jadi `fn view` boleh merujuk `App` yang
/// dibangkitkan makro setelahnya. `macro_rules!` tak bisa menyembunyikan logika
/// bisnis arbitrer (butuh interpreter) — ia menyembunyikan *struktur*.
/// `ponytail:` SATU `app!` per file (`App`/`main` dibangkitkan di scope pemakai).
///
/// ```no_run
/// use mrust_fw::{app, view};
///
/// fn view(app: &App) -> mrust_fw::Element<'_, App> {
///     view! {
///         <column spacing=16 padding=32>
///             <input placeholder="q" value={app.q} on_input={mrust_fw::on_val("q")}/>
///             <text>"count=" {app.count}</text>
///             <row spacing=8>
///                 <button on_press={mrust_fw::on("dec")}>"-"</button>
///                 <button on_press={mrust_fw::on("inc")}>"+"</button>
///             </row>
///         </column>
///     }
///     .into()
/// }
///
/// app! {
///     title = "Counter App";
///     state { count: i32, q: String }
///     tap {
///         "inc" => |a: &mut App| a.count += 1,
///         "dec" => |a: &mut App| a.count -= 1,
///     }
///     bind {
///         "q" => |a: &mut App, v: String| a.q = v,
///     }
///     view: view
/// }
/// ```
#[macro_export]
macro_rules! app {
    (
        title = $title:literal;
        state { $( $f:ident : $ty:ty ),+ $(,)? }
        tap { $( $tn:literal => $tb:expr ),* $(,)? }
        bind { $( $bn:literal => $bb:expr ),* $(,)? }
        view: $view:path
    ) => {
        #[derive(Default)]
        struct App {
            $( $f: $ty, )+
        }

        fn main() -> $crate::Result {
            let mut __a = <$crate::Actions<App>>::default();
            $( __a.push($tn, $crate::act($tb)); )*
            $( __a.push_set($bn, $bb); )*
            $crate::register_actions(__a);
            $crate::run::<App>($title, $view)
        }
    };
}
