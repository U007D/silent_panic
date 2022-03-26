#[cfg(test)]
mod unit_tests;

use std::mem;
use std::panic::{PanicInfo, set_hook, take_hook};

pub struct SilentPanic(Option<Box::<dyn Fn(&PanicInfo<'_>) + Send + Sync + 'static>>);

impl SilentPanic {
    #[must_use]
    pub fn on() -> Self {
        // Save current panic handler
        let original_handler = take_hook();

        // Install silent panic handler
        set_hook(Box::new(|_info| {}));

        Self(Some(original_handler))
    }
}

impl Drop for SilentPanic {
    fn drop(&mut self) {
        // Retrieve original panic handler
        let original_handler = mem::replace(&mut self.0, None).unwrap_or_else(|| unreachable!());

        // Restore original panic handler
        set_hook(original_handler);
    }
}

// TODO: figure out how to test this (!). Specifically how to get the unit test to confirm the existence of panic spew
//       when I don't activate `SilentPanic` and the absence of the panic spew when I do activate it.  For that the unit
//       tests need to be able to get the test spew.
