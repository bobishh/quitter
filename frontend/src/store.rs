use gloo_storage::{LocalStorage, Storage};
use gloo_net::http::Request;
use crate::model::{Habit, Tracker, Theme};
use leptos::prelude::*;
use leptos::task::spawn_local;
use shared::tracker::Habit as ProtoHabit;
use prost::Message;

const TRACKERS_KEY: &str = "addict_trackers";

#[derive(Clone, Copy, Debug)]
pub struct ThemeStore {
    pub themes: RwSignal<Vec<Theme>>,
}

impl ThemeStore {
    pub fn new() -> Self {
        let store = Self {
            themes: RwSignal::new(Vec::new()),
        };
        
        spawn_local(async move {
            if let Ok(res) = Request::get("/api/themes").send().await {
                if let Ok(themes) = res.json::<Vec<Theme>>().await {
                    store.themes.set(themes);
                }
            }
        });
        
        store
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HabitStore {
    pub habits: RwSignal<Vec<Habit>>,
    pub loading: RwSignal<bool>,
}

impl HabitStore {
    pub fn new() -> Self {
        let store = Self {
            habits: RwSignal::new(Vec::new()),
            loading: RwSignal::new(true),
        };
        
        // Initial fetch from server
        spawn_local(async move {
            match Request::get("/api/habits").send().await {
                Ok(response) => {
                    match response.json::<Vec<Habit>>().await {
                        Ok(habits) => {
                            leptos::logging::log!("Loaded {} habits from API", habits.len());
                            store.habits.set(habits);
                        },
                        Err(e) => leptos::logging::error!("Failed to parse habits JSON: {:?}", e),
                    }
                },
                Err(e) => leptos::logging::error!("Failed to fetch habits: {:?}", e),
            }
            store.loading.set(false);
        });
        
        store
    }

    pub fn add_habit(&self, habit: Habit) {
        let h = habit.clone();
        
        let proto = ProtoHabit {
            id: h.id.to_string(),
            slug: h.slug.clone(),
            name: h.name.clone(),
            icon: h.icon.clone(),
            unit_name: h.unit_name.clone(),
        };
        
        let mut buf = Vec::new();
        proto.encode(&mut buf).unwrap();

        spawn_local(async move {
            let _ = Request::post("/api/habits")
                .header("Content-Type", "application/octet-stream")
                .body(buf)
                .unwrap()
                .send()
                .await;
        });
        self.habits.update(|h| h.push(habit));
    }

    pub fn get_habit(&self, id: uuid::Uuid) -> Option<Habit> {
        self.habits.get().iter().find(|h| h.id == id).cloned()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TrackerStore {
    pub trackers: RwSignal<Vec<Tracker>>,
}

impl TrackerStore {
    pub fn new() -> Self {
        let stored_trackers: Vec<Tracker> = LocalStorage::get(TRACKERS_KEY).unwrap_or_default();
        Self {
            trackers: RwSignal::new(stored_trackers),
        }
    }

    pub fn save(&self) {
        let _ = LocalStorage::set(TRACKERS_KEY, self.trackers.get());
    }

    pub fn add_tracker(&self, tracker: Tracker) {
        self.trackers.update(|t| t.push(tracker));
        self.save();
    }

    pub fn update_tracker(&self, tracker: Tracker) {
        self.trackers.update(|t| {
            if let Some(index) = t.iter().position(|x| x.id == tracker.id) {
                t[index] = tracker;
            }
        });
        self.save();
    }

    pub fn delete_tracker(&self, id: uuid::Uuid) {
        self.trackers.update(|t| t.retain(|x| x.id != id));
        self.save();
    }
}
