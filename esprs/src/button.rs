use esp_idf_hal::gpio::*;
use debouncr::{debounce_4,Edge};
use esp_idf_hal::gpio::{AnyIOPin, Input};
use debouncr::Debouncer;
use debouncr::Repeat4;

pub struct Button<'a> {
    pin_driver: PinDriver<'a, AnyIOPin, Input>,
    debouncer: Debouncer<u8, Repeat4>,
    pub on_rising_edge: Option<fn()>,
}

impl Button<'_> {
    pub fn new(pin: AnyIOPin) -> Result<Self, esp_idf_hal::sys::EspError> {
        let mut pin_driver = PinDriver::input(pin)?;
        pin_driver.set_pull(Pull::Down).unwrap();
        Ok(Self {
            pin_driver,
            debouncer: debounce_4(false),
            on_rising_edge: None,
        })
    }

    pub fn button_interrupt(&mut self) {
        let button_state: Option<Edge> = self.debouncer.update(self.pin_driver.is_high());
        match button_state {
            Some(Edge::Rising) => {
                    if self.on_rising_edge.is_some() {
                        self.on_rising_edge.unwrap()();
                    }
            }
            _ => {}
        }
    }
}