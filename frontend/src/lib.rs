use leptos::prelude::*;

mod components;
mod model;
mod store;

use store::{HabitStore, TrackerStore};
use components::{TrackersView, HabitManager, TrackerCreator, TrackerDetailView, HabitDetailView, ThemeManager};
#[cfg(target_arch = "wasm32")]
use shared::tracker::TrackerState;
#[cfg(target_arch = "wasm32")]
use crate::model::{Tracker, TrackerType};
use uuid::Uuid;
#[cfg(target_arch = "wasm32")]
use chrono::{DateTime};

#[derive(Clone, PartialEq)]
pub enum AppView {
    Trackers,
    Habits,
    NewTracker,
    Themes,
    TrackerDetail(Uuid),
    HabitDetail(Uuid),
}

#[component]
pub fn App() -> impl IntoView {
    let h_store = HabitStore::new();
    let t_store = TrackerStore::new();
    let theme_store = store::ThemeStore::new();
    provide_context(h_store);
    provide_context(t_store);
    provide_context(theme_store);
    
    let (current_view, set_current_view) = signal(AppView::Trackers);
    provide_context(set_current_view); // Provide the setter to components so they can navigate

    let habits = h_store.habits;
    #[cfg(target_arch = "wasm32")]
    let trackers = t_store.trackers;
    let themes = theme_store.themes;

    let default_theme = Memo::new(move |_| {
        themes.get().iter().find(|t| t.id.is_nil()).cloned()
    });

    // Routing Logic
    let route = move || {
        // Only route if habits are loaded
        if habits.get().is_empty() { return; }

        #[cfg(target_arch = "wasm32")]
        {
            let window = leptos::web_sys::window().unwrap();
            let location = window.location();
            let pathname = location.pathname().unwrap_or_default();
            let hash = location.hash().unwrap_or_default();
            
            leptos::logging::log!("Routing check: Path='{}', Hash='{}'", pathname, hash);

            // Check if we are at root or just /
            if (pathname == "/" || pathname.is_empty()) && hash.is_empty() {
                set_current_view.set(AppView::Trackers);
                return;
            }

            if pathname == "/habits" {
                set_current_view.set(AppView::Habits);
                return;
            }
            if pathname == "/themes" {
                set_current_view.set(AppView::Themes);
                return;
            }
            if pathname == "/new-tracker" {
                set_current_view.set(AppView::NewTracker);
                return;
            }

            if hash.len() > 1 {
                let encoded = &hash[1..];
                
                match TrackerState::decode_from_url(encoded) {
                    Ok(state) => {
                        let clean_path = pathname.trim_matches('/');
                        if !clean_path.is_empty() {
                            let slug = clean_path.to_string();
                            
                            if let Some(h) = habits.get().iter().find(|h| h.slug == slug).cloned() {
                                let dt = DateTime::from_timestamp(state.start_timestamp, 0).unwrap();
                                
                                // Check for exact existing tracker for this habit and timestamp
                                // We need to check against the CURRENT trackers in store
                                let current_trackers = trackers.get();
                                let existing_tracker = current_trackers.iter().find(|t| {
                                    if t.habit_id != h.id { return false; }
                                    match t.tracker_type {
                                        TrackerType::Abstinence { start_date, .. } => start_date.timestamp() == state.start_timestamp
                                    }
                                });
                                
                                let tracker_id = if let Some(existing) = existing_tracker {
                                    // Check if properties match URL state, if not update
                                    let mut needs_update = false;
                                    let mut updated_tracker = existing.clone();
                                    
                                    if let TrackerType::Abstinence { units_per_day, theme_id, user_name, .. } = &mut updated_tracker.tracker_type {
                                        let url_theme_id = state.theme_id.as_ref().and_then(|id_str| uuid::Uuid::parse_str(id_str).ok());
                                        
                                        if (*units_per_day - state.units_per_day).abs() > f64::EPSILON {
                                            *units_per_day = state.units_per_day;
                                            needs_update = true;
                                        }
                                        if *theme_id != url_theme_id {
                                            *theme_id = url_theme_id;
                                            needs_update = true;
                                        }
                                        if *user_name != state.user_name {
                                            *user_name = state.user_name.clone();
                                            needs_update = true;
                                        }
                                    }
                                    
                                    if needs_update {
                                        leptos::logging::log!("Routing: Updating existing tracker {} from URL state", existing.id);
                                        t_store.update_tracker(updated_tracker);
                                    }

                                    existing.id
                                } else {
                                    leptos::logging::log!("Routing: Creating new tracker for habit {}", h.id);
                                    let t_id = state.theme_id.and_then(|id_str| uuid::Uuid::parse_str(&id_str).ok());
                                    let tracker = Tracker::new_abstinence(h.id, dt, state.units_per_day, t_id, state.user_name);
                                    t_store.add_tracker(tracker.clone());
                                    tracker.id
                                };
                                
                                set_current_view.set(AppView::TrackerDetail(tracker_id));
                            } else {
                                leptos::logging::warn!("Routing: Habit not found for slug '{}'", slug);
                                set_current_view.set(AppView::Trackers);
                            }
                        }
                    },
                    Err(e) => {
                        leptos::logging::error!("Routing: Failed to decode hash: {}", e);
                        set_current_view.set(AppView::Trackers);
                    }
                }
            } else if pathname != "/" {
                 // Handle case where we have slug but no hash (maybe show habit details? or redirect?)
                 // For now, if just slug is present, maybe show Dashboard or HabitDetail if we supported it via slug
                 // But sticking to Dashboard is safer to avoid "not found"
                 set_current_view.set(AppView::Trackers);
            }
        }
    };

    // Run routing when habits change (initial load)
    Effect::new(move |_| {
        route();
    });

    // Listen for popstate (Browser Back/Forward)
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::ev::popstate;
        let _ = window_event_listener(popstate, move |_| {
             leptos::logging::log!("Popstate detected");
             route();
        });
    }

    view! {
        <div class="app-container">
            {move || default_theme.get().map(|t| view! { <style>{t.css}</style> })}
            <header>
                <h1>"YOU, QUITTER!"</h1>
                <nav class="main-nav">
                    <button 
                        class=move || if current_view.get() == AppView::Trackers { "winamp-btn active" } else { "winamp-btn" }
                        on:click=move |_| {
                            set_current_view.set(AppView::Trackers);
                            if let Ok(history) = web_sys::window().unwrap().history() {
                                let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/"));
                            }
                        }
                    >
                        "TRACKERS"
                    </button>
                    <button 
                        class=move || if current_view.get() == AppView::Habits { "winamp-btn active" } else { "winamp-btn" }
                        on:click=move |_| {
                            set_current_view.set(AppView::Habits);
                            if let Ok(history) = web_sys::window().unwrap().history() {
                                let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/habits"));
                            }
                        }
                    >
                        "HABITS"
                    </button>
                    <button 
                        class=move || if current_view.get() == AppView::Themes { "winamp-btn active" } else { "winamp-btn" }
                        on:click=move |_| {
                            set_current_view.set(AppView::Themes);
                            if let Ok(history) = web_sys::window().unwrap().history() {
                                let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/themes"));
                            }
                        }
                    >
                        "THEMES"
                    </button>
                </nav>
            </header>

            <main>
                {move || match current_view.get() {
                    AppView::Trackers => view! { <TrackersView /> }.into_any(),
                    AppView::Habits => view! { <HabitManager /> }.into_any(),
                    AppView::NewTracker => view! { 
                        <TrackerCreator on_close=move || set_current_view.set(AppView::Trackers) /> 
                    }.into_any(),
                    AppView::Themes => view! { <ThemeManager /> }.into_any(),
                    AppView::TrackerDetail(id) => view! { <TrackerDetailView tracker_id=id /> }.into_any(),
                    AppView::HabitDetail(id) => view! { <HabitDetailView habit_id=id /> }.into_any(),
                }}
            </main>
            
            <footer>
                <p>"©Alcoholics Audacious"</p>
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