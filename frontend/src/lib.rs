mod model;
mod store;
mod components;

use leptos::prelude::*;
use store::{HabitStore, TrackerStore};
use components::{Dashboard, HabitManager, TrackerCreator};
use shared::tracker::TrackerState;
use crate::model::{Tracker, TrackerType};
use uuid::Uuid;
use chrono::{DateTime, Utc, TimeZone};

#[derive(Clone, PartialEq)]
enum AppView {
    Dashboard,
    Habits,
    NewTracker,
}

#[component]
pub fn App() -> impl IntoView {
    let h_store = HabitStore::new();
    let t_store = TrackerStore::new();
    provide_context(h_store);
    provide_context(t_store);
    
    let (current_view, set_current_view) = signal(AppView::Dashboard);

    // Handle URL hash for shared trackers
    #[cfg(target_arch = "wasm32")]
    {
        let hash = leptos::web_sys::window().unwrap().location().hash().unwrap();
        if hash.len() > 1 {
            let encoded = &hash[1..];
            if let Ok(state) = TrackerState::decode_from_url(encoded) {
                if let Ok(habit_id) = Uuid::parse_str(&state.habit_id) {
                    let dt = DateTime::<Utc>::from_naive_utc_and_offset(
                        chrono::NaiveDateTime::from_timestamp_opt(state.start_timestamp, 0).unwrap(),
                        Utc
                    );
                    let tracker = Tracker::new_abstinence(habit_id, dt);
                    // Add it only if it doesn't exist
                    if !t_store.trackers.get().iter().any(|t| t.habit_id == habit_id) {
                        t_store.add_tracker(tracker);
                    }
                }
            }
        }
    }

    view! {
        <div class="app-container">
            <header>
                <h1>"ADDICT TRACKER // WINAMP EDITION"</h1>
                <nav class="main-nav">
                    <button 
                        class=move || if current_view.get() == AppView::Dashboard { "winamp-btn active" } else { "winamp-btn" }
                        on:click=move |_| set_current_view.set(AppView::Dashboard)
                    >
                        "DASHBOARD"
                    </button>
                    <button 
                        class=move || if current_view.get() == AppView::Habits { "winamp-btn active" } else { "winamp-btn" }
                        on:click=move |_| set_current_view.set(AppView::Habits)
                    >
                        "HABITS"
                    </button>
                    <button 
                         class=move || if current_view.get() == AppView::NewTracker { "winamp-btn active" } else { "winamp-btn" }
                        on:click=move |_| set_current_view.set(AppView::NewTracker)
                    >
                        "NEW TRACKER"
                    </button>
                </nav>
            </header>

            <main>
                {move || match current_view.get() {
                    AppView::Dashboard => view! { <Dashboard /> }.into_any(),
                    AppView::Habits => view! { <HabitManager /> }.into_any(),
                    AppView::NewTracker => view! { 
                        <TrackerCreator on_close=move || set_current_view.set(AppView::Dashboard) /> 
                    }.into_any(),
                }}
            </main>
            
            <footer>
                <p>"Built with Rust & Leptos. Stay clean (or don't)."</p>
            </footer>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    mount_to_body(App);
}
