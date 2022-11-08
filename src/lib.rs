#![no_std]

pub mod bluetooth {
    use cortex_m::delay::Delay;
    use embedded_hal::digital::v2::OutputPin;
    use rp2040_hal::{
        clocks::{init_clocks_and_plls, ClocksManager},
        gpio::{bank0::Gpio0, bank0::Gpio1, pin::Pin, FunctionUart, Pins, PushPullOutput},
        pac,
        sio::Sio,
        uart::{self, UartPeripheral},
        watchdog::Watchdog,
        Clock,
    };

    // Setup function
    pub fn setup(
        clocks: &mut ClocksManager,
        pins: (
            rp2040_hal::gpio::Pin<Gpio0, rp2040_hal::gpio::Function<rp2040_hal::gpio::Uart>>,
            rp2040_hal::gpio::Pin<Gpio1, rp2040_hal::gpio::Function<rp2040_hal::gpio::Uart>>,
        ),
    ) -> Result<
        UartPeripheral<
            rp2040_hal::uart::Enabled,
            rp2040_hal::pac::UART0,
            (
                rp2040_hal::gpio::Pin<Gpio0, rp2040_hal::gpio::Function<rp2040_hal::gpio::Uart>>,
                rp2040_hal::gpio::Pin<Gpio1, rp2040_hal::gpio::Function<rp2040_hal::gpio::Uart>>,
            ),
        >,
        rp2040_hal::uart::Error,
    > {
        let mut peripherals = pac::Peripherals::take().unwrap();
        let uart = UartPeripheral::new(peripherals.UART0, pins, &mut peripherals.RESETS).enable(
            uart::common_configs::_9600_8_N_1,
            clocks.peripheral_clock.freq(),
        );

        return uart;
    }
}
