#[cfg(test)]
mod tests {
    use super::tracker::TrackerState;

    #[test]
    fn debug_specific_hash() {
        let hash = "CiQzYzA1NjA1My1hMDlhLTQwZDItOWJkNC1hYzlhNzk0Y2QwYmMQgJaWywYaBEFub24";
        let state = TrackerState::decode_from_url(hash);
        match state {
            Ok(s) => {
                println!("Decoded Successfully:");
                println!("Habit ID: {}", s.habit_id);
                println!("Timestamp: {}", s.start_timestamp);
                println!("User: {}", s.user_name);
            },
            Err(e) => panic!("Failed to decode: {}", e),
        }
    }
}
