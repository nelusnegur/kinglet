use core::fmt::Result;
use core::fmt::Write;

use crate::gpio::raspi::GpioPinOp;
use crate::gpio::raspi::Gpio;
use crate::gpio::raspi::GpioPin;
use crate::mmio::Register;

const UART_BASE_REG_ADDR: u32 = 0x3f215000;

/// This register enables the mini UART and access to its registers.
const ENABLES: u32 = UART_BASE_REG_ADDR + 0x04;

/// This register is used to write data to and read data from the UART FIFOs.
const IO: u32 = UART_BASE_REG_ADDR + 0x40;

/// This register enables/disables transmit/receive interrupts.
const IER: u32 = UART_BASE_REG_ADDR + 0x44;

/// This register shows interrupt status. It also has two FIFO enable status
/// and FIFO clear bits.
const IIR: u32 = UART_BASE_REG_ADDR + 0x48;

/// This register controls the line data format and gives access to the
/// baudrate register.
const LCR: u32 = UART_BASE_REG_ADDR + 0x4c;

/// This register controls the modem signals.
const MCR: u32 = UART_BASE_REG_ADDR + 0x50;

/// This register shows the data status.
const LSR: u32 = UART_BASE_REG_ADDR + 0x54;

/// The control register provides access to some extra features not found on
/// a normal 16550 UART.
const CNTL: u32 = UART_BASE_REG_ADDR + 0x60;

/// Baud register allows direct access to the 16-bit wide baudrate counter.
/// Baud is the number of symbols transferred per second. 
/// One baud is equivalent to one bit per second.
const BAUD: u32 = UART_BASE_REG_ADDR + 0x68;

/// A bit mask used to check whether the transmit FIFO
/// can accept at least one byte.
const TRANSMITTER_EMPTY: u32 = 0x20;

/// A bit mask used to check whether the receive FIFO
/// holds at least one symbol.
// TODO: Implement reading from the console
#[allow(dead_code)]
const DATA_READY: u32 = 0x01;

#[derive(Debug)]
pub struct MiniUart {
    io: Register<u32>,
    lsr: Register<u32>,
    mcr: Register<u32>,
    enables: Register<u32>,
    cntl: Register<u32>,
    ier: Register<u32>,
    iir: Register<u32>,
    lcr: Register<u32>,
    baud: Register<u32>,
}

impl MiniUart {
    pub fn init(gpio: &mut Gpio) -> MiniUart {
        // SAFETY
        // The following addresses are statically known, aligned
        // and valid for the entire executio of the kernel.
        let mut uart = unsafe {
            MiniUart {
                io: Register::new(IO as *const u32),
                lsr: Register::new(LSR as *const u32),
                mcr: Register::new(MCR as *const u32),
                enables: Register::new(ENABLES as *const u32),
                cntl: Register::new(CNTL as *const u32),
                ier: Register::new(IER as *const u32),
                iir: Register::new(IIR as *const u32),
                lcr: Register::new(LCR as *const u32),
                baud: Register::new(BAUD as *const u32),
            }
        };

        // Enable GPIO UART1 pins
        gpio.enable_pin(GpioPin::Pin14, GpioPinOp::Alt5);
        gpio.enable_pin(GpioPin::Pin15, GpioPinOp::Alt5);

        // Enable UART1
        uart.enables.write(0x01);

        // Disable the auto flow control. The transmitter will ignore
        // the status of CTS (Clear to Send) line.
        // Temporarily disable the transmitter and receiver.
        uart.cntl.write(0);

        // Enable 8-bit mode
        uart.lcr.write(0x03);

        // Set the RTS (Request to Send) line to high
        uart.mcr.write(0);

        // Disable the transmit and receive interrupts
        uart.ier.write(0);

        // Enable and clear the transmit and receive FIFOs
        uart.iir.write(0xc6);

        // Set the baud rate to 115200 bps.
        // The value 270 (0x10e) is the baud rate register.
        uart.baud.write(0x10e);

        // Enable the transmitter and receiver
        uart.cntl.write(0x03);

        uart
    }

    fn write(&mut self, c: char) {
        loop {
            let data_status = self.lsr.read();
            if data_status & TRANSMITTER_EMPTY != 0 {
                break;
            }
        }
        // SAFETY
        // TODO: The register must be behind a lock!
        self.io.write(c as u32);
    }
}

impl Write for MiniUart {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            // SAFETY
            // TODO: The register must be behind a lock!
            self.write(c)
        }

        Ok(())
    }
}
