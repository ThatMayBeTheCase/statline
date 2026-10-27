use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct Game {
    pub name: String,
    pub executable: String,
    pub total_playtime: u64,
    pub was_running: bool,
    start_time: Option<Instant>,
    start_timestamp: Option<i64>,
}

pub struct Session {
    pub started_at: i64,
    pub ended_at: i64,
    pub duration_seconds: i64,
}

impl Game {
    pub fn new(name: String, executable: String) -> Self {
        Self {
            name,
            executable,
            total_playtime: 0,
            was_running: false,
            start_time: None,
            start_timestamp: None,
        }
    }

    pub fn start_session(&mut self) {
        self.start_time = Some(Instant::now());

        self.start_timestamp = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("System clock is before Unix epoch")
                .as_secs() as i64,
        );

        self.was_running = true;

        println!("{} Started!", self.name);
    }

    pub fn stop_session(&mut self) -> Option<Session> {
        self.was_running = false;

        if let (Some(start), Some(started_at)) =
            (self.start_time.take(), self.start_timestamp.take())
        {
            let total_seconds = start.elapsed().as_secs();

            self.total_playtime += total_seconds;

            let ended_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("System clock is before Unix epoch")
                .as_secs() as i64;

            Some(Session {
                started_at,
                ended_at,
                duration_seconds: total_seconds as i64,
            })
        } else {
            None
        }
    }
}
