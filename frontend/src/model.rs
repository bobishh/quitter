use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub id: Uuid,
    pub name: String,
    pub css: String,
    pub icon_limit: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Habit {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub icon: String,
    pub unit_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TrackerType {
    Abstinence { 
        start_date: DateTime<Utc>,
        units_per_day: f64,
        theme_id: Option<Uuid>,
        user_name: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tracker {
    pub id: Uuid,
    pub habit_id: Uuid,
    pub tracker_type: TrackerType,
}

impl Habit {
    pub fn new(name: &str, slug: &str, icon: &str, unit: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            slug: slug.to_string(),
            name: name.to_string(),
            icon: icon.to_string(),
            unit_name: unit.to_string(),
        }
    }
}

impl Tracker {
    pub fn new_abstinence(habit_id: Uuid, start_date: DateTime<Utc>, units_per_day: f64, theme_id: Option<Uuid>, user_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            habit_id,
            tracker_type: TrackerType::Abstinence { 
                start_date,
                units_per_day,
                theme_id,
                user_name,
            },
        }
    }

    pub fn get_abstinence_count(&self) -> f64 {
        match &self.tracker_type {
            TrackerType::Abstinence { start_date, units_per_day, .. } => {
                let duration = Utc::now() - *start_date;
                let hours = duration.num_minutes() as f64 / 60.0;
                let habit_freq_hours = if *units_per_day > 0.0 {
                    24.0 / units_per_day
                } else {
                    0.0
                };

                if habit_freq_hours > 0.0 {
                    hours / habit_freq_hours
                } else {
                    0.0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_habit_creation() {
        let habit = Habit::new("Test", "test-slug", "🧪", "vials");
        assert_eq!(habit.name, "Test");
        assert_eq!(habit.slug, "test-slug");
    }

    #[test]
    fn test_tracker_abstinence_creation() {
        let habit_id = Uuid::new_v4();
        let start = Utc::now();
        let tracker = Tracker::new_abstinence(habit_id, start, 2.0, None, "User".to_string());
        
        match tracker.tracker_type {
            TrackerType::Abstinence { start_date, units_per_day, .. } => {
                assert_eq!(start_date, start);
                assert_eq!(units_per_day, 2.0);
            }
        }
    }

    #[test]
    fn test_abstinence_calculation() {
        // Mock current time by creating a start date in the past
        let now = Utc::now();
        let two_days_ago = now - chrono::Duration::hours(48);
        
        let tracker = Tracker {
            id: Uuid::new_v4(),
            habit_id: Uuid::new_v4(),
            tracker_type: TrackerType::Abstinence { 
                start_date: two_days_ago,
                units_per_day: 1.0, // 1 unit per day = 24h frequency
                theme_id: None,
                user_name: "User".to_string(),
            },
        };

        // 48h duration with 1 unit/day = 2.0 units
        let count = tracker.get_abstinence_count();
        
        // Allow small float error due to execution time
        assert!((count - 2.0).abs() < 0.01);
    }
}
