use alloc::rc::Rc;
use core::cell::RefCell;
use cortex_m::delay::Delay;
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::gpio::DynPin;

use crate::pin_consts::num_to_led_char;

pub struct RegisterController {
    /// srclock
    sr_clock: DynPin,
    /// ser
    sr_data: DynPin,
    /// rclock
    r_clock: DynPin,

    /// delay controller
    delay: Rc<RefCell<Delay>>,
}

impl RegisterController {
    pub fn new(
        sr_clock: DynPin,
        r_clock: DynPin,
        sr_data: DynPin,
        delay: Rc<RefCell<Delay>>,
    ) -> Self {
        RegisterController {
            sr_clock,
            r_clock,
            sr_data,
            delay,
        }
    }
    pub fn load_byte(&mut self, b: u8) {
        if b == 0 {
            return;
        }
        self.r_clock.set_low();
        self.sr_data.set_low();

        let b = b ^ 0xFF;
        for idx in 0..8 {
            let load = (b >> idx) & 1 == 1;
            self.load_bit(load);
        }

        self.r_clock.set_low();
    }
    fn load_bit(&mut self, b: bool) {
        self.r_clock.set_low();
        match b {
            true => self.sr_data.set_high(),
            false => self.sr_data.set_low(),
        };
        self.sr_clock.set_high();
        self.sr_clock.set_low();
        self.r_clock.set_high();
    }
}

pub struct SegmentController {
    led_latch_pin: DynPin,
    register_controller: Rc<RefCell<RegisterController>>,
    delay: Rc<RefCell<Delay>>,
}

impl SegmentController {
    pub fn new(
        led_pin: DynPin,
        register_controller: Rc<RefCell<RegisterController>>,
        delay: Rc<RefCell<Delay>>,
    ) -> Self {
        SegmentController {
            led_latch_pin: led_pin,
            register_controller,
            delay,
        }
    }

    pub fn load_and_display_number(&mut self, number: u8) {
        let led_config = num_to_led_char(number);
        let mut locked_controller = self.register_controller.as_ref().borrow_mut();
        let mut locked_delay = self.delay.as_ref().borrow_mut();

        locked_controller.load_byte(led_config);
        self.led_latch_pin.set_high();
        locked_delay.delay_us(5);
        self.led_latch_pin.set_low();
    }

    pub fn disable_segment(&mut self) {
        self.led_latch_pin.set_low();
    }
}

pub struct DisplayController {
    led_controllers: [SegmentController; 4],
    register_controller: Rc<RefCell<RegisterController>>,
    delay: Rc<RefCell<Delay>>,
}

impl DisplayController {
    pub fn new(
        led_pin_1: DynPin,
        led_pin_2: DynPin,
        led_pin_3: DynPin,
        led_pin_4: DynPin,
        sr_clock_pin: DynPin,
        sr_data_pin: DynPin,
        sr_rclock_pin: DynPin,
        delay: Rc<RefCell<Delay>>,
    ) -> Self {
        let register_controller = Rc::new(RefCell::new(RegisterController::new(
            sr_clock_pin,
            sr_rclock_pin,
            sr_data_pin,
            Rc::clone(&delay),
        )));

        let led_controllers = [
            SegmentController::new(
                led_pin_1,
                Rc::clone(&register_controller),
                Rc::clone(&delay),
            ),
            SegmentController::new(
                led_pin_2,
                Rc::clone(&register_controller),
                Rc::clone(&delay),
            ),
            SegmentController::new(
                led_pin_3,
                Rc::clone(&register_controller),
                Rc::clone(&delay),
            ),
            SegmentController::new(
                led_pin_4,
                Rc::clone(&register_controller),
                Rc::clone(&delay),
            ),
        ];

        Self {
            register_controller,
            led_controllers,
            delay,
        }
    }
    pub fn show_number(&mut self, number: u8) {
        self.led_controllers[0].load_and_display_number(number);
        self.led_controllers[1].load_and_display_number(number);
        self.led_controllers[2].load_and_display_number(number);
        self.led_controllers[3].load_and_display_number(number);
    }
}
