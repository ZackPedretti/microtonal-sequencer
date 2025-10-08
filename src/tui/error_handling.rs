use crate::tui::entities::App;
use std::io;

pub(crate) trait MidiSequencerTUIResult<T> {
    fn unwrap_or_display_err(self, app: &mut App);

    fn unwrap_or_default_val_and_display_err(self, app: &mut App, default: T) -> T;
}

impl<T> MidiSequencerTUIResult<T> for Result<T, io::Error> {
    fn unwrap_or_display_err(self, app: &mut App) {
        match self {
            Ok(..) => {}
            Err(e) => {
                app.error = Some(e);
            }
        }
    }

    fn unwrap_or_default_val_and_display_err(self, app: &mut App, default: T) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                app.error = Some(e);
                default
            }
        }
    }
}
