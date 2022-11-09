#![no_std]

pub mod bluetooth {
    use core::fmt::Write;
    use rp2040_hal::{
        clocks::ClocksManager,
        gpio::{bank0::Gpio0, bank0::Gpio1},
        pac,
        uart::{self, UartPeripheral},
        Clock,
    };

    pub fn setup(
        clocks: &mut ClocksManager,
        pins: (
            rp2040_hal::gpio::Pin<Gpio0, rp2040_hal::gpio::Function<rp2040_hal::gpio::Uart>>,
            rp2040_hal::gpio::Pin<Gpio1, rp2040_hal::gpio::Function<rp2040_hal::gpio::Uart>>,
        ),
    ) -> UartPeripheral<
        rp2040_hal::uart::Enabled,
        rp2040_hal::pac::UART0,
        (
            rp2040_hal::gpio::Pin<Gpio0, rp2040_hal::gpio::Function<rp2040_hal::gpio::Uart>>,
            rp2040_hal::gpio::Pin<Gpio1, rp2040_hal::gpio::Function<rp2040_hal::gpio::Uart>>,
        ),
    > {
        let mut peripherals = pac::Peripherals::take().unwrap();

        let uart = UartPeripheral::new(peripherals.UART0, pins, &mut peripherals.RESETS)
            .enable(
                uart::common_configs::_9600_8_N_1,
                clocks.peripheral_clock.freq(),
            )
            .unwrap();

        uart
    }

    pub fn ble_write(
        value: u8,
        ble: &mut UartPeripheral<
            rp2040_hal::uart::Enabled,
            rp2040_hal::pac::UART0,
            (
                rp2040_hal::gpio::Pin<Gpio0, rp2040_hal::gpio::Function<rp2040_hal::gpio::Uart>>,
                rp2040_hal::gpio::Pin<Gpio1, rp2040_hal::gpio::Function<rp2040_hal::gpio::Uart>>,
            ),
        >,
    ) {
        writeln!(ble, "value: {:02}\r", value).unwrap();
    }
}
