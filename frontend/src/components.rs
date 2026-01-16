use leptos::prelude::*;
use crate::model::{Habit, Tracker};
use crate::store::{HabitStore, TrackerStore};
use chrono::{DateTime, Utc};
use leptos::web_sys;
use shared::tracker::TrackerState;

// ========================
// DASHBOARD (TRACKERS)
// ========================

#[component]
pub fn Dashboard() -> impl IntoView {
    let t_store = use_context::<TrackerStore>().expect("TrackerStore not found");
    let h_store = use_context::<HabitStore>().expect("HabitStore not found");
    let trackers = t_store.trackers;

    view! {
        <div class="dashboard">
            <For
                each=move || trackers.get()
                key=|t| t.id
                children=move |tracker| {
                    let h = h_store.get_habit(tracker.habit_id);
                    view! { <TrackerCard tracker=tracker habit=h /> }
                }
            />
            {move || if trackers.get().is_empty() {
                 view! { <div class="empty-state">"No active trackers. Go to 'New Tracker' to start."</div> }.into_any()
            } else {
                 view! { <div/> }.into_any()
            }}
        </div>
    }
}

#[component]
pub fn TrackerCard(tracker: Tracker, habit: Option<Habit>) -> impl IntoView {
    let t_store = use_context::<TrackerStore>().expect("TrackerStore not found");
    let (expanded, set_expanded) = signal(false);
    
    if habit.is_none() {
        return view! { <div class="error">"Loading or Orphaned Tracker"</div> }.into_any();
    }
    let habit = habit.unwrap();

    let (now, set_now) = signal(chrono::Utc::now());
    
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::leptos_dom::helpers::set_interval_with_handle;
        use std::time::Duration;
        let _ = set_interval_with_handle(
            move || set_now.set(chrono::Utc::now()),
            Duration::from_secs(1),
        );
    }

    let habit_icon = habit.icon.clone();
    let habit_name = habit.name.clone();
    let habit_icon_viz = habit.icon.clone();
    let habit_unit = habit.unit_name.clone();
    let habit_color = habit.theme_color.clone();
    let freq_hours = habit.frequency_hours;
    
    let tracker_for_calc = tracker.clone();
    
    let count = Memo::new(move |_| {
        now.track(); 
        tracker_for_calc.get_abstinence_count(freq_hours)
    });
    
    let tracker_id = tracker.id;
    let delete_tracker = move |_| {
        if web_sys::window().and_then(|w| w.confirm_with_message("Remove this tracker?").ok()).unwrap_or(false) {
            t_store.delete_tracker(tracker_id);
        }
    };

    let share_tracker = {
        let habit_id = habit.id.to_string();
        let start_date = match tracker.tracker_type {
            crate::model::TrackerType::Abstinence { start_date } => start_date.timestamp(),
            _ => 0,
        };
        move |_| {
            let state = TrackerState {
                habit_id: habit_id.clone(),
                start_timestamp: start_date,
                user_name: "Anon".to_string(),
            };
            let encoded = state.encode_to_url();
            let url = format!("{}#{}", web_sys::window().unwrap().location().origin().unwrap(), encoded);
            let _ = web_sys::window().unwrap().navigator().clipboard().write_text(&url);
            let _ = web_sys::window().unwrap().alert_with_message("Share link copied to clipboard!");
        }
    };

    let theme_style = format!("border-color: {}; color: {};", habit_color, habit_color);

    view! {
        <div class="habit-card" style=theme_style>
            <div class="habit-header">
                <span class="habit-icon"> {move || habit_icon.clone()} </span>
                <span class="habit-name"> {move || habit_name.clone()} </span>
                <div class="card-actions">
                    <button class="winamp-btn-small" on:click=share_tracker>"SHARE"</button>
                    <button class="winamp-btn-small delete" on:click=delete_tracker>"X"</button>
                </div>
            </div>
            
            <div class="habit-stats">
                 {move || {
                    let c = count.get();
                    format!("{:.2} {} saved", c, habit_unit)
                 }}
            </div>

            <button class="winamp-btn" on:click=move |_| set_expanded.update(|v| *v = !*v)>
                {move || if expanded.get() { "HIDE VISUALS" } else { "SHOW VISUALS" }}
            </button>

            {move || if expanded.get() {
                 view! { <Visualizer count=count.get() icon=habit_icon_viz.clone() /> }.into_any()
            } else {
                 view! { <div/> }.into_any()
            }}
        </div>
    }.into_any()
}

#[component]
pub fn Visualizer(count: f64, icon: String) -> impl IntoView {
    let int_count = count.floor() as usize;
    let display_count = int_count.min(1000); 
    let remainder = display_count < int_count;

    view! {
        <div class="visualizer-container">
            <div class="visualizer-grid">
                {(0..display_count).map(|_| {
                    let i = icon.clone();
                    view! { <span class="viz-item">{move || i.clone()}</span> }
                }).collect::<Vec<_>>()}
                
                {if remainder {
                    view! { <span class="viz-overflow">"...and many more"</span> }.into_any()
                } else {
                    view! { <span/> }.into_any()
                }}
            </div>
        </div>
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
    let theme_style = format!("border-color: {}; color: {};", habit.theme_color, habit.theme_color);
    
    view! {
        <div class="habit-item" style=theme_style>
            <span class="icon">{habit.icon}</span>
            <span class="name">{habit.name}</span>
            <span class="details">{format!("{} / {}h", habit.unit_name, habit.frequency_hours)}</span>
        </div>
    }
}

#[component]
pub fn HabitForm(#[prop(into)] on_close: Callback<()>) -> impl IntoView {
    let store = use_context::<HabitStore>().expect("HabitStore not found");
    
    let (name, set_name) = signal("".to_string());
    let (icon, set_icon) = signal("🍺".to_string());
    let (color, set_color) = signal("#00ff00".to_string());
    let (freq, set_freq) = signal("24".to_string());
    let (unit, set_unit) = signal("Beers".to_string());

    let create = move |_| {
        let f: f64 = freq.get().parse().unwrap_or(24.0);
        let h = Habit::new(
            &name.get(),
            &icon.get(),
            &color.get(),
            f,
            &unit.get()
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
                <label>"Icon (Emoji):"</label>
                <input type="text" on:input=move |ev| set_icon.set(event_target_value(&ev)) value=icon />
            </div>
            <div class="form-group">
                <label>"Theme Color:"</label>
                <input type="color" on:input=move |ev| set_color.set(event_target_value(&ev)) value=color />
            </div>
             <div class="form-group">
                <label>"Frequency (Hours):"</label>
                <input type="number" on:input=move |ev| set_freq.set(event_target_value(&ev)) value=freq />
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
    let habits = h_store.habits;
    
    let (selected_habit_id, set_selected_habit_id) = signal::<Option<String>>(None);
    let (mode, set_mode) = signal("abstinence".to_string());
    let (start_date_str, set_start_date_str) = signal(
        Utc::now().format("%Y-%m-%dT%H:%M").to_string()
    );

    let create = move |_| {
        if let Some(id_str) = selected_habit_id.get() {
            if let Ok(habit_id) = uuid::Uuid::parse_str(&id_str) {
                if mode.get() == "abstinence" {
                    let dt_str = start_date_str.get();
                    let naive = chrono::NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%dT%H:%M")
                        .unwrap_or_else(|_| Utc::now().naive_local());
                    let dt_utc = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

                    let t = Tracker::new_abstinence(habit_id, dt_utc);
                    t_store.add_tracker(t);
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
                            <label>"Mode:"</label>
                            <select on:change=move |ev| set_mode.set(event_target_value(&ev))>
                                <option value="abstinence">"Abstinence (Quit Date)"</option>
                            </select>
                        </div>
                        
                        <div class="form-group">
                            <label>"Quit Date:"</label>
                            <input 
                                type="datetime-local" 
                                on:input=move |ev| set_start_date_str.set(event_target_value(&ev)) 
                                value=start_date_str 
                            />
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
