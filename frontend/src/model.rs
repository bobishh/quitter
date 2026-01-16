use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Habit {
    pub id: Uuid,
    pub name: String,
    pub icon: String,
    pub theme_color: String,
    pub unit_name: String,
    pub cost_per_unit: f64,
    pub frequency_hours: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TrackerType {
    Abstinence { start_date: DateTime<Utc> },
    Usage { events: Vec<DateTime<Utc>> },
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tracker {
    pub id: Uuid,
    pub habit_id: Uuid,
    pub tracker_type: TrackerType,
}

impl Habit {
    pub fn new(name: &str, icon: &str, color: &str, freq_hours: f64, unit: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            icon: icon.to_string(),
            theme_color: color.to_string(),
            unit_name: unit.to_string(),
            cost_per_unit: 0.0,
            frequency_hours: freq_hours,
        }
    }
}

impl Tracker {
    pub fn new_abstinence(habit_id: Uuid, start_date: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            habit_id,
            tracker_type: TrackerType::Abstinence { start_date },
        }
    }

    pub fn new_usage(habit_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            habit_id,
            tracker_type: TrackerType::Usage { events: Vec::new() },
        }
    }

    pub fn get_abstinence_count(&self, habit_freq_hours: f64) -> f64 {
        match &self.tracker_type {
            TrackerType::Abstinence { start_date } => {
                let duration = Utc::now() - *start_date;
                let hours = duration.num_minutes() as f64 / 60.0;
                if habit_freq_hours > 0.0 {
                    hours / habit_freq_hours
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_habit_creation() {
        let habit = Habit::new("Test", "🧪", "#000", 2.0, "vials");
        assert_eq!(habit.name, "Test");
        assert_eq!(habit.frequency_hours, 2.0);
    }

    #[test]
    fn test_tracker_abstinence_creation() {
        let habit_id = Uuid::new_v4();
        let start = Utc::now();
        let tracker = Tracker::new_abstinence(habit_id, start);
        
        match tracker.tracker_type {
            TrackerType::Abstinence { start_date } => assert_eq!(start_date, start),
            _ => panic!("Wrong tracker type"),
        }
    }

    #[test]
    fn test_tracker_usage_creation() {
        let habit_id = Uuid::new_v4();
        let tracker = Tracker::new_usage(habit_id);
        
        match tracker.tracker_type {
            TrackerType::Usage { events } => assert!(events.is_empty()),
            _ => panic!("Wrong tracker type"),
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
            tracker_type: TrackerType::Abstinence { start_date: two_days_ago },
        };

        // If frequency is 24h, 48h duration = 2.0 units
        let count = tracker.get_abstinence_count(24.0);
        
        // Allow small float error due to execution time
        assert!((count - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_abstinence_zero_frequency() {
        let tracker = Tracker {
            id: Uuid::new_v4(),
            habit_id: Uuid::new_v4(),
            tracker_type: TrackerType::Abstinence { start_date: Utc::now() },
        };
        
        let count = tracker.get_abstinence_count(0.0);
        assert_eq!(count, 0.0);
    }
}
