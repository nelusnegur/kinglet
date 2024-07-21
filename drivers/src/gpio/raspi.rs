use core::u32;

use crate::mmio::Register;

const GPIO_BASE_REG_ADDR: u32 = 0x3f200000;

/// This register is used to define the opration of the GPIO pins
/// according to the Function 1 table.
const FSEL1: u32 = GPIO_BASE_REG_ADDR + 0x04;

/// GPIO pin pull up/down controls the actuation of the internal pull-up/down
/// control line to all GPIO pins. This register must be used in conjunction
/// with the two PUDCLKn registers.
const PUD: u32 = GPIO_BASE_REG_ADDR + 0x94;

/// GPIO pin pull up/down clock controls the actuation of internal pull-downs on
/// the respective GPIO pins. These registers must be used in conjunction with
/// the PUD register to apply GPIO Pull-up/down changes.
///
/// The following sequence of events is required:
///     1. write to PUD to set the required control signal
///     2. wait 150 cycles – this provides the required set-up time for the
///     control signal
///     3. write to PUDCLK0/1 to clock the control signal into the GPIO pads
///     you wish to modify. Only the pads which receive a clock will be modified,
///     all others will retain their previous state.
///     4. wait 150 cycles
///     5. write to PUD to remove the control signal
///     6. write to PUDCLK0/1 to remove the clock
const PUDCLK0: u32 = GPIO_BASE_REG_ADDR + 0x98;

const PUD_CPU_CYCLES: u32 = 150;

const GPIO_PAD_MASK: u32 = 0x1;
const CLEAR_PIN_MASK: u32 = 0x7;

const PINS: [Pin; 2] = [
    Pin {
        nr: 14,
        start_bit: 12,
    },
    Pin {
        nr: 15,
        start_bit: 15,
    },
];

struct Pin {
    nr: u32,
    start_bit: u32,
}

#[derive(Debug)]
#[repr(u8)]
pub enum GpioPin {
    Pin14 = 0,
    Pin15 = 1,
}

#[derive(Debug)]
#[repr(u32)]
pub enum GpioPinOp {
    Input = 0,
    Output = 1,
    Alt0 = 4,
    Alt1 = 5,
    Alt2 = 6,
    Alt3 = 7,
    Alt4 = 3,
    Alt5 = 2,
}

/// Represents the GPIO peripheral. It has 41 pins. All GPIO pins have at least
/// two alternative functions within BCM.
#[derive(Debug)]
pub struct Gpio {
    fsel1: Register<u32>,
    pud: Register<u32>,
    pudclk0: Register<u32>,
    wait: fn(cpu_cycles: u32),
}

impl Gpio {
    pub fn new(wait: fn(cpu_cycles: u32)) -> Gpio {
        // SAFETY
        // The following addresses are statically known, aligned
        // and valid for the entire executio of the kernel.
        unsafe {
            Self {
                fsel1: Register::new(FSEL1 as *const u32),
                pud: Register::new(PUD as *const u32),
                pudclk0: Register::new(PUDCLK0 as *const u32),
                wait,
            }
        }
    }

    pub fn enable_pin(&mut self, pin: GpioPin, pin_op: GpioPinOp) {
        let mut fsel1 = self.fsel1.read();
        let pin = &PINS[pin as u8 as usize];

        fsel1 &= !(CLEAR_PIN_MASK << pin.start_bit);
        fsel1 |= (pin_op as u32) << pin.start_bit;

        self.fsel1.write(fsel1);

        // Disable pull-up/down
        self.pud.write(0);
        (self.wait)(PUD_CPU_CYCLES);

        // Assert clock on the given pad
        self.pudclk0.write(GPIO_PAD_MASK << pin.nr);
        (self.wait)(PUD_CPU_CYCLES);

        // Clear the clock
        self.pudclk0.write(0);
    }
}
