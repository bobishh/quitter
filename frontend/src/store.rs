use gloo_storage::{LocalStorage, Storage};
use gloo_net::http::Request;
use crate::model::{Habit, Tracker};
use leptos::prelude::*;
use leptos::task::spawn_local;

const TRACKERS_KEY: &str = "addict_trackers";

#[derive(Clone, Copy, Debug)]
pub struct HabitStore {
    pub habits: RwSignal<Vec<Habit>>,
}

impl HabitStore {
    pub fn new() -> Self {
        let store = Self {
            habits: RwSignal::new(Vec::new()),
        };
        
        // Initial fetch from server
        spawn_local(async move {
            if let Ok(response) = Request::get("/api/habits").send().await {
                if let Ok(habits) = response.json::<Vec<Habit>>().await {
                    store.habits.set(habits);
                }
            }
        });
        
        store
    }

    pub fn add_habit(&self, habit: Habit) {
        let h = habit.clone();
        spawn_local(async move {
            let _ = Request::post("/api/habits")
                .json(&h)
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

    pub fn delete_tracker(&self, id: uuid::Uuid) {
        self.trackers.update(|t| t.retain(|x| x.id != id));
        self.save();
    }
}