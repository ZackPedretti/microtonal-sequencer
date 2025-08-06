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
        let start = match self.last_played_tick {
            None => { 0 }
            Some(l) => { l }
        };
        (self.tick - start) % duration as u16 == 0
    }

    pub(crate) fn reset_tick(&mut self) {
        self.tick = 0;
    }
}