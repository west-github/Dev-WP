#[derive(Debug)]
pub struct Track {
    pub title: String,
    pub duration: u32,
    cursor: u32,
}

impl Track {
    pub fn new(title: String, duration: u32) -> Self {
        Self {
            title,
            duration,
            cursor: 0,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    playlist: Vec<Track>,
    current_track: usize,
    _volume: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            playlist: vec![
                Track::new("Track 1".into(), 180),
                Track::new("Track 2".into(), 160),
                Track::new("Track 3".into(), 140),
                Track::new("Track 4".into(), 120),
                Track::new("Track 5".into(), 100),
            ],
            current_track: 0,
            _volume: 25,
        }
    }
}

impl Player {
    pub fn next_track(&mut self) {
        self.current_track = (self.current_track + 1) % self.playlist.len();
    }

    pub fn prev_track(&mut self) {
        self.current_track = (self.playlist.len() + self.current_track - 1) % self.playlist.len();
    }
    pub fn play(&mut self) {
        self.track_mut().cursor = 10;
    }

    pub fn pause(&mut self) {
        self.track_mut().cursor = 43;
    }

    pub fn rewind(&mut self) {
        self.track_mut().cursor = 0;
    }

    pub fn track(&self) -> &Track {
        &self.playlist[self.current_track]
    }

    pub fn track_mut(&mut self) -> &mut Track {
        &mut self.playlist[self.current_track]
    }
}
