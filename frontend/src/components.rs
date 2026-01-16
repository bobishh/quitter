use leptos::prelude::*;
use crate::model::{Habit, Tracker};
use crate::store::{HabitStore, TrackerStore};
use chrono::{DateTime, Utc};
use leptos::web_sys;
use shared::tracker::TrackerState;
use shared::tracker::Theme as ProtoTheme;
use prost::Message;

// ========================
// DASHBOARD (TRACKERS)
// ========================

#[component]
pub fn TrackersView() -> impl IntoView {
    let t_store = use_context::<TrackerStore>().expect("TrackerStore not found");
    let trackers = t_store.trackers;

    view! {
        <div class="dashboard">
            <div class="dashboard-header" style="margin-bottom: 20px; border-bottom: 1px dashed #333; padding-bottom: 20px;">
                <TrackerCreator on_close=move || {} />
            </div>
            <div class="tracker-list">
                <For
                    each=move || trackers.get()
                    key=|t| t.id
                    children=move |tracker| {
                        view! { <TrackerListItem tracker=tracker /> }
                    }
                />
                {move || if trackers.get().is_empty() {
                     view! { <div class="empty-state">"No active trackers. Use the form above to start."</div> }.into_any()
                } else {
                     view! { <div/> }.into_any()
                }}
            </div>
        </div>
    }
}

#[component]
pub fn TrackerListItem(tracker: Tracker) -> impl IntoView {
    let h_store = use_context::<HabitStore>().expect("HabitStore not found");
    let t_store = use_context::<TrackerStore>().expect("TrackerStore not found");
    let set_view = use_context::<WriteSignal<crate::AppView>>().expect("AppView setter not found");
    
    // Reactive look up
    let habit = Memo::new(move |_| h_store.get_habit(tracker.habit_id));

    let (now, _set_now) = signal(chrono::Utc::now());
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::leptos_dom::helpers::set_interval_with_handle;
        use std::time::Duration;
        let _ = set_interval_with_handle(
            move || _set_now.set(chrono::Utc::now()),
            Duration::from_secs(60), // Update every minute is enough for list
        );
    }
    
    view! {
        {move || {
            let loading = h_store.loading.get();
            
            match habit.get() {
                Some(h) => {
                    let h_clone = h.clone();
                    let t_clone = tracker.clone();
                    now.track();
                    let count = t_clone.get_abstinence_count();
                    
                    let click_handler = move |_| {
                        // Construct URL state
                        let (start_date, units_per_day, theme_id, user_name) = match &t_clone.tracker_type {
                            crate::model::TrackerType::Abstinence { start_date, units_per_day, theme_id, user_name } => {
                                (start_date.timestamp(), *units_per_day, theme_id.map(|id| id.to_string()), user_name.clone())
                            }
                        };
                        let state = TrackerState {
                            start_timestamp: start_date,
                            units_per_day,
                            theme_id,
                            user_name,
                        };
                        let encoded = state.encode_to_url();
                        let url = format!("/{}#{}", h_clone.slug, encoded);
                        
                        // Push to history (silent navigation)
                        if let Ok(history) = web_sys::window().unwrap().history() {
                             let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url));
                        }
                        
                        set_view.set(crate::AppView::Trackers); // This will be overwritten by TrackerDetail below but good for consistency
                        set_view.set(crate::AppView::TrackerDetail(t_clone.id));
                    };
                    
                    let start_date_str = match tracker.tracker_type {
                        crate::model::TrackerType::Abstinence { start_date, .. } => start_date.format("%d/%m/%Y %H:%M").to_string(),
                    };

                    view! {
                        <div 
                            class="habit-item tracker-item" 
                            on:click=click_handler
                        >
                            <span class="icon">{h_clone.icon}</span>
                            <span class="name">"NOT " {h_clone.name}</span>
                            <span class="details">{format!("{:.8} {} since {}", count, h_clone.unit_name, start_date_str)}</span>
                        </div>
                    }.into_any()
                },
                None => {
                    if !loading {
                         let tid = tracker.id;
                         view! {
                            <div class="habit-item error">
                                <span class="name">"Unknown Habit"</span>
                                <button class="winamp-btn-small delete" 
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        t_store.delete_tracker(tid);
                                    }
                                >"DELETE"</button>
                            </div>
                         }.into_any()
                    } else {
                         view! { <div class="habit-item loading">"Loading..."</div> }.into_any()
                    }
                }
            }
        }}
    }
}

#[component]
pub fn TrackerDetailView(tracker_id: uuid::Uuid) -> impl IntoView {
    let t_store = use_context::<TrackerStore>().expect("TrackerStore not found");
    let h_store = use_context::<HabitStore>().expect("HabitStore not found");

    let tracker = Memo::new(move |_| {
        t_store.trackers.get().iter().find(|t| t.id == tracker_id).cloned()
    });

    view! {
        {move || match tracker.get() {
            Some(t) => {
                let h = h_store.get_habit(t.habit_id);
                 view! { <TrackerCardInner tracker=t habit=h /> }.into_any()
            },
            None => view! { <div class="error">"Tracker not found"</div> }.into_any()
        }}
    }
}

#[component]
pub fn TrackerCardInner(tracker: Tracker, habit: Option<Habit>) -> impl IntoView {
    let _t_store = use_context::<TrackerStore>().expect("TrackerStore not found");
    let theme_store = use_context::<crate::store::ThemeStore>().expect("ThemeStore not found");
    
    if habit.is_none() {
        return view! { <div class="loading">"Loading details..."</div> }.into_any();
    }
    let habit = habit.unwrap();

    let (now, _set_now) = signal(chrono::Utc::now());
    
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::leptos_dom::helpers::set_interval_with_handle;
        use std::time::Duration;
        let _ = set_interval_with_handle(
            move || _set_now.set(chrono::Utc::now()),
            Duration::from_secs(1),
        );
    }

    let habit_name = habit.name.clone();
    let habit_icon_viz = habit.icon.clone();
    let habit_unit = habit.unit_name.clone();
    let habit_slug = habit.slug.clone();
    
    let tracker_for_calc = tracker.clone();
    
    let count = Memo::new(move |_| {
        now.track(); 
        tracker_for_calc.get_abstinence_count()
    });
    
    let (user_name, start_date_str) = match &tracker.tracker_type {
        crate::model::TrackerType::Abstinence { user_name, start_date, .. } => {
            (user_name.clone(), start_date.format("%Y-%m-%d %H:%M").to_string())
        }
    };

    let share_tracker = {
        let slug = habit_slug.clone();
        let (start_date, units_per_day, theme_id, u_name) = match &tracker.tracker_type {
            crate::model::TrackerType::Abstinence { start_date, units_per_day, theme_id, user_name } => {
                (start_date.timestamp(), *units_per_day, theme_id.map(|id| id.to_string()), user_name.clone())
            }
        };
        move |_| {
            let state = TrackerState {
                start_timestamp: start_date,
                units_per_day,
                theme_id: theme_id.clone(),
                user_name: u_name.clone(),
            };
            let encoded = state.encode_to_url();
            let origin = web_sys::window().unwrap().location().origin().unwrap();
            let url = format!("{}/{}#{}", origin, slug, encoded);
            let _ = web_sys::window().unwrap().navigator().clipboard().write_text(&url);
            let _ = web_sys::window().unwrap().alert_with_message("Share link copied to clipboard!");
        }
    };

    let theme = match &tracker.tracker_type {
        crate::model::TrackerType::Abstinence { theme_id, .. } => {
            theme_id.and_then(|tid| theme_store.themes.get().iter().find(|t| t.id == tid).cloned())
        }
    };

    let icon_limit = theme.as_ref().and_then(|t| t.icon_limit.map(|l| l as usize));

    view! {
        <div class="habit-card full-view">
            {theme.map(|t| view! { <style>{t.css}</style> })}
            <div class="habit-header">
                <span class="habit-name">{user_name} " is NOT " {move || habit_name.clone()} " since " {start_date_str.clone()}</span>
            </div>
            
            <div class="habit-stats">
                 {move || {
                    let c = count.get();
                    format!("{:.8} {} not consumed since then", c, habit_unit)
                 }}
            </div>

            <Visualizer count=count.get() icon=habit_icon_viz.clone() limit=icon_limit />

            <div class="tracker-card-actions">
                <button class="winamp-btn" on:click=share_tracker>"SHARE"</button>
            </div>
        </div>
    }.into_any()
}

#[component]
pub fn Visualizer(count: f64, icon: String, limit: Option<usize>) -> impl IntoView {
    let int_count = count.floor() as usize;
    // Default to 10,000 if no limit is set, otherwise use the limit
    let display_count = int_count.min(limit.unwrap_or(10000));

    view! {
        <div class="visualizer-container">
            <div class="visualizer-grid">
                {(0..display_count).map(|_| {
                    let i = icon.clone();
                    // Initial random position
                    let top = format!("{}%", (rand_f64() * 100.0));
                    let left = format!("{}%", (rand_f64() * 100.0));
                    
                    // Movement offsets for Brownian-style motion
                    let tx = format!("{}vw", (rand_f64() * 40.0) - 20.0);
                    let ty = format!("{}vh", (rand_f64() * 40.0) - 20.0);
                    let tr = format!("{}deg", (rand_f64() * 360.0));
                    
                    // Randomized timing
                    let dur = format!("{}s", 15.0 + rand_f64() * 15.0);
                    let delay = format!("-{}s", rand_f64() * 30.0);
                    
                    let style = format!(
                        "top: {}; left: {}; --tx: {}; --ty: {}; --tr: {}; animation-duration: {}; animation-delay: {};",
                        top, left, tx, ty, tr, dur, delay
                    );

                    view! { <span class="viz-item" style=style>{move || i.clone()}</span> }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

// Helper for randomness in WASM
fn rand_f64() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Math::random()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0.5
    }
}

// ========================
// HABIT MANAGEMENT
// ========================

#[component]
pub fn HabitManager() -> impl IntoView {
    let store = use_context::<HabitStore>().expect("HabitStore not found");
    let habits = store.habits;
    let (show_form, set_show_form) = signal(false);

    view! {
        <div class="habit-manager">
            {move || if show_form.get() {
                view! { <HabitForm on_close=move || set_show_form.set(false) /> }.into_any()
            } else {
                view! {
                    <div class="habit-library">
                        <div class="library-header">
                            <h3>"Habit Library"</h3>
                            <button class="winamp-btn" on:click=move |_| set_show_form.set(true)>"NEW HABIT"</button>
                        </div>
                        <div class="habit-grid">
                            <For
                                each=move || habits.get()
                                key=|h| h.id
                                children=move |habit| {
                                    view! { <HabitItem habit=habit /> }
                                }
                            />
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}

#[component]
pub fn HabitItem(habit: Habit) -> impl IntoView {
    let set_view = use_context::<WriteSignal<crate::AppView>>().expect("AppView setter not found");
    let id = habit.id;
    
    view! {
        <div 
            class="habit-item tracker-item" 
            on:click=move |_| set_view.set(crate::AppView::HabitDetail(id))
        >
            <span class="icon">{habit.icon}</span>
            <span class="name">{habit.name}</span>
            <span class="details">{format!("{}", habit.unit_name)}</span>
        </div>
    }
}

#[component]
pub fn HabitForm(#[prop(into)] on_close: Callback<()>) -> impl IntoView {
    let store = use_context::<HabitStore>().expect("HabitStore not found");
    
    let (name, set_name) = signal("".to_string());
    let (slug, set_slug) = signal("".to_string());
    let (icon, set_icon) = signal("🍺".to_string());
    let (unit, set_unit) = signal("Beers".to_string());

    let create = move |_| {
        let h = Habit::new(
            &name.get(),
            &slug.get(),
            &icon.get(),
            &unit.get(),
        );
        store.add_habit(h);
        on_close.run(());
    };

    view! {
        <div class="creator-form">
            <h3>"Define New Habit"</h3>
            <div class="form-group">
                <label>"Name:"</label>
                <input type="text" on:input=move |ev| set_name.set(event_target_value(&ev)) value=name />
            </div>
             <div class="form-group">
                <label>"URL Slug (unique):"</label>
                <input type="text" on:input=move |ev| set_slug.set(event_target_value(&ev)) value=slug />
            </div>
            <div class="form-group">
                <label>"Icon (Emoji):"</label>
                <input type="text" on:input=move |ev| set_icon.set(event_target_value(&ev)) value=icon />
            </div>
             <div class="form-group">
                <label>"Unit Name:"</label>
                <input type="text" on:input=move |ev| set_unit.set(event_target_value(&ev)) value=unit />
            </div>
            
            <div class="actions">
                <button class="winamp-btn" on:click=create>"SAVE TO DB"</button>
                <button class="winamp-btn" on:click=move |_| on_close.run(())>"CANCEL"</button>
            </div>
        </div>
    }
}

// ========================
// TRACKER CREATION
// ========================

#[component]
pub fn TrackerCreator(#[prop(into)] on_close: Callback<()>) -> impl IntoView {
    let h_store = use_context::<HabitStore>().expect("HabitStore not found");
    let t_store = use_context::<TrackerStore>().expect("TrackerStore not found");
    let theme_store = use_context::<crate::store::ThemeStore>().expect("ThemeStore not found");
    let habits = h_store.habits;
    let themes = theme_store.themes;
    
    let (selected_habit_id, set_selected_habit_id) = signal::<Option<String>>(None);
    let (units_per_day, set_units_per_day) = signal("1.0".to_string());
    let (user_name, set_user_name) = signal("Bogdan".to_string());
    let (theme_id, set_theme_id) = signal("".to_string());
    let (start_date_str, set_start_date_str) = signal(
        Utc::now().format("%Y-%m-%dT%H:%M").to_string()
    );

    let create = move |_| {
        if let Some(id_str) = selected_habit_id.get() {
            if let Ok(habit_id) = uuid::Uuid::parse_str(&id_str) {
                let dt_str = start_date_str.get();
                let naive = chrono::NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%dT%H:%M")
                    .unwrap_or_else(|_| Utc::now().naive_local());
                let dt_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

                let upd: f64 = units_per_day.get().parse().unwrap_or(1.0);
                let u_name = user_name.get();
                let t_id = if theme_id.get().is_empty() {
                    None
                } else {
                    uuid::Uuid::parse_str(&theme_id.get()).ok()
                };

                let t = Tracker::new_abstinence(habit_id, dt_utc, upd, t_id, u_name.clone());
                t_store.add_tracker(t);

                // Update URL to reflect this new tracker
                if let Some(h) = habits.get().iter().find(|h| h.id == habit_id) {
                    let state = TrackerState {
                        start_timestamp: dt_utc.timestamp(),
                        units_per_day: upd,
                        theme_id: t_id.map(|id| id.to_string()),
                        user_name: u_name,
                    };
                    let encoded = state.encode_to_url();
                    let url = format!("/{}#{}", h.slug, encoded);
                    
                    if let Ok(history) = web_sys::window().unwrap().history() {
                        let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url));
                    }
                }
                
                on_close.run(());
            }
        }
    };

    view! {
        <div class="creator-form">
            <h3>"New Tracker"</h3>
            
            <div class="form-group">
                <label>"Select Habit from DB:"</label>
                <select on:change=move |ev| set_selected_habit_id.set(Some(event_target_value(&ev)))>
                    <option value="">"-- Choose a Habit --"</option>
                    <For
                        each=move || habits.get()
                        key=|h| h.id
                        children=move |habit| {
                            view! { <option value=habit.id.to_string()>{habit.name}</option> }
                        }
                    />
                </select>
                {move || if habits.get().is_empty() {
                    view! { <p class="hint">"Loading habits from server..."</p> }.into_any()
                } else {
                    view! { <div/> }.into_any()
                }}
            </div>

            {move || if selected_habit_id.get().is_some() {
                view! {
                    <div class="tracker-config">
                        <div class="form-group">
                            <label>"Your Name:"</label>
                            <input type="text" on:input=move |ev| set_user_name.set(event_target_value(&ev)) value=user_name />
                        </div>
                        <div class="form-group">
                            <label>"Quit Date:"</label>
                            <input 
                                type="datetime-local" 
                                on:input=move |ev| set_start_date_str.set(event_target_value(&ev)) 
                                value=start_date_str 
                            />
                        </div>

                        <div class="form-group">
                            <label>"Units per Day (to save):"</label>
                            <input 
                                type="number" 
                                step="0.1"
                                on:input=move |ev| set_units_per_day.set(event_target_value(&ev)) 
                                value=units_per_day 
                            />
                        </div>

                        <div class="form-group">
                            <label>"Theme:"</label>
                            <select on:change=move |ev| set_theme_id.set(event_target_value(&ev))>
                                <option value="">"Default (None)"</option>
                                <For
                                    each=move || themes.get()
                                    key=|t| t.id
                                    children=move |theme| {
                                        view! { <option value=theme.id.to_string()>{theme.name}</option> }
                                    }
                                />
                            </select>
                        </div>
                        
                        <div class="actions">
                            <button class="winamp-btn" on:click=create>"START TRACKING"</button>
                            <button class="winamp-btn" on:click=move |_| on_close.run(())>"CANCEL"</button>
                        </div>
                    </div>
                }.into_any()
            } else {
                 view! { 
                    <div class="actions">
                        <button class="winamp-btn" on:click=move |_| on_close.run(())>"CANCEL"</button>
                    </div> 
                 }.into_any()
            }}
        </div>
    }
}

#[component]
pub fn HabitDetailView(habit_id: uuid::Uuid) -> impl IntoView {
    let h_store = use_context::<HabitStore>().expect("HabitStore not found");
    let set_view = use_context::<WriteSignal<crate::AppView>>().expect("AppView setter not found");

    let habit = Memo::new(move |_| {
        h_store.get_habit(habit_id)
    });

    view! {
        {move || match habit.get() {
            Some(h) => {
                view! {
                    <div class="habit-card full-view">
                        <div class="habit-header">
                            <span class="habit-name">"Habit Details"</span>
                        </div>
                        
                        <div class="habit-info-container">
                            <h2>{h.icon} " " {h.name}</h2>
                            <p><strong>"Slug:"</strong> " " {h.slug}</p>
                            <p><strong>"Unit:"</strong> " " {h.unit_name}</p>
                        </div>
                        
                        <div class="habit-detail-actions">
                             <button class="winamp-btn" on:click=move |_| set_view.set(crate::AppView::Habits)>"< BACK"</button>
                        </div>
                    </div>
                }.into_any()
            },
            None => view! { <div class="error">"Habit not found"</div> }.into_any()
        }}
    }
}

#[component]
pub fn ThemeItemDisplay(theme: crate::model::Theme) -> impl IntoView {
    let (expanded, set_expanded) = signal(false);
    
    view! {
        <div class="theme-item">
            <div class="theme-item-header" style="display: flex; justify-content: space-between; align-items: center;">
                <span>{theme.name} " (" {move || theme.icon_limit.map(|l| l.to_string()).unwrap_or("No Limit".to_string())} " icons)"</span>
                <button class="winamp-btn-small" on:click=move |_| set_expanded.update(|e| *e = !*e)>
                    {move || if expanded.get() { "HIDE" } else { "SHOW" }}
                </button>
            </div>
            {move || if expanded.get() {
                view! {
                    <pre class="theme-item-css">
                        {theme.css.clone()}
                    </pre>
                }.into_any()
            } else {
                view! { <div/> }.into_any()
            }}
        </div>
    }
}

#[component]
pub fn ThemeManager() -> impl IntoView {
    let theme_store = use_context::<crate::store::ThemeStore>().expect("ThemeStore not found");
    let set_view = use_context::<WriteSignal<crate::AppView>>().expect("AppView setter not found");
    
    let (name, set_name) = signal("".to_string());
    let (css, set_css) = signal("".to_string());
    let (icon_limit, set_icon_limit) = signal("500".to_string());

    let create = move |_| {
        let n = name.get();
        let c = css.get();
        let limit_str = icon_limit.get();
        let limit: Option<i32> = if limit_str.trim().is_empty() {
            None
        } else {
            limit_str.parse().ok()
        };
        
        if n.is_empty() || c.is_empty() { return; }
        
        let proto = ProtoTheme {
            id: uuid::Uuid::new_v4().to_string(),
            name: n,
            css: c,
            icon_limit: limit,
        };
        
        let mut buf = Vec::new();
        proto.encode(&mut buf).unwrap();
        
        leptos::task::spawn_local(async move {
            let _ = gloo_net::http::Request::post("/api/themes")
                .header("Content-Type", "application/octet-stream")
                .body(buf)
                .unwrap()
                .send()
                .await;
            let _ = web_sys::window().unwrap().location().reload();
        });
    };

    view! {
        <div class="creator-form">
            <div class="library-header">
                <h3>"Theme Manager"</h3>
                <button class="winamp-btn" on:click=move |_| set_view.set(crate::AppView::Trackers)>"BACK"</button>
            </div>
            
            <div class="theme-form-section">
                <h4>"Create New Theme"</h4>
                <div class="form-group">
                    <label>"Theme Name:"</label>
                    <input type="text" on:input=move |ev| set_name.set(event_target_value(&ev)) value=name />
                </div>
                <div class="form-group">
                    <label>"Icon Limit (Performance, leave empty for none):"</label>
                    <input type="number" on:input=move |ev| set_icon_limit.set(event_target_value(&ev)) value=icon_limit />
                </div>
                <div class="form-group">
                    <label>"CSS Content:"</label>
                    <textarea 
                        class="theme-css-textarea"
                        on:input=move |ev| set_css.set(event_target_value(&ev)) 
                        prop:value=css 
                    />
                </div>
                <div class="actions">
                    <button class="winamp-btn" on:click=create>"SAVE THEME"</button>
                </div>
            </div>
            
            <div style="margin-top: 20px;">
                <h4>"Existing Themes:"</h4>
                <div class="theme-list">
                    <For
                        each=move || theme_store.themes.get()
                        key=|t| t.id
                        children=move |theme| {
                            view! { <ThemeItemDisplay theme=theme /> }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
