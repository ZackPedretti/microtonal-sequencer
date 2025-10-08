pub(crate) struct Clock {
    tick: u16,
    last_played_tick: Option<u16>
}

impl Clock {
    pub(crate) fn new() -> Self {
        Self {
            tick: 0,
            last_played_tick: None
        }
    }

    pub(crate) fn next(&mut self) {
        self.tick = (self.tick + 1) % (24 * 8);
    }

    pub(crate) fn has_time_passed_note(&self, duration: u8) -> bool {
        if duration == 0 {
            return true;
        }
        let start = self.last_played_tick.unwrap_or(0);
        (self.tick - start) % duration as u16 == 0
    }

    pub(crate) fn note_played(&mut self) {
        self.last_played_tick = Some(self.tick);
    }

    pub(crate) fn reset_tick(&mut self) {
        self.tick = 0;
        self.last_played_tick = None;
    }
}