/*!Peripheral access API for STM32G070 microcontrollers (generated using svd2rust v0.36.1 (4052ce6 2025-04-04))

You can find an overview of the generated API [here].

API features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.

[here]: https://docs.rs/svd2rust/0.36.1/svd2rust/#peripheral-api
[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased
[repository]: https://github.com/rust-embedded/svd2rust*/
///Number available in the NVIC for configuring priority
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn RTC_STAMP();
    fn FLASH();
    fn RCC();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn DMA1_CHANNEL1();
    fn DMA1_CHANNEL2_3();
    fn DMA1_CHANNEL4_5_6_7_DMAMUX();
    fn ADC();
    fn TIM1_BRK_UP_TRG_COM();
    fn TIM1_CC();
    fn TIM3();
    fn TIM6();
    fn TIM7();
    fn TIM14();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn I2C1();
    fn I2C2();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3_USART4();
    fn CEC();
}
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 31] = [
    Vector { _handler: WWDG },
    Vector { _reserved: 0 },
    Vector {
        _handler: RTC_STAMP,
    },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA1_CHANNEL1,
    },
    Vector {
        _handler: DMA1_CHANNEL2_3,
    },
    Vector {
        _handler: DMA1_CHANNEL4_5_6_7_DMAMUX,
    },
    Vector { _handler: ADC },
    Vector {
        _handler: TIM1_BRK_UP_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _reserved: 0 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM6 },
    Vector { _handler: TIM7 },
    Vector { _handler: TIM14 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector {
        _handler: USART3_USART4,
    },
    Vector { _handler: CEC },
];
///Enumeration of all the interrupts.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    ///0 - Window watchdog interrupt
    WWDG = 0,
    ///2 - RTC and TAMP interrupts
    RTC_STAMP = 2,
    ///3 - Flash global interrupt
    FLASH = 3,
    ///4 - RCC global interrupt
    RCC = 4,
    ///5 - EXTI line 0 and 1 interrupt
    EXTI0_1 = 5,
    ///6 - EXTI line 2 and 3 interrupt
    EXTI2_3 = 6,
    ///7 - EXTI line 4 to 15 interrupt
    EXTI4_15 = 7,
    ///9 - DMA channel 1 interrupt
    DMA1_CHANNEL1 = 9,
    ///10 - DMA channel 2 and 3 interrupts
    DMA1_CHANNEL2_3 = 10,
    ///11 - interrupts for DMA1 channels 4-7 and DMAMUX
    DMA1_CHANNEL4_5_6_7_DMAMUX = 11,
    ///12 - ADC interrupt (ADC combined with EXTI 17 and 18)
    ADC = 12,
    ///13 - TIM1 break, update, trigger and commutation interrupts
    TIM1_BRK_UP_TRG_COM = 13,
    ///14 - TIM1 Capture Compare interrupt
    TIM1_CC = 14,
    ///16 - TIM3 global interrupt
    TIM3 = 16,
    ///17 - TIM6 global interrupt
    TIM6 = 17,
    ///18 - TIM7 global interrupt
    TIM7 = 18,
    ///19 - TIM14 global interrupt
    TIM14 = 19,
    ///20 - Timer 15 global interrupt
    TIM15 = 20,
    ///21 - TIM16 global interrupt
    TIM16 = 21,
    ///22 - TIM17 global interrupt
    TIM17 = 22,
    ///23 - I2C1 global interrupt
    I2C1 = 23,
    ///24 - I2C2 global interrupt
    I2C2 = 24,
    ///25 - SPI1 global interrupt
    SPI1 = 25,
    ///26 - SPI2 global interrupt
    SPI2 = 26,
    ///27 - USART1 global interrupt
    USART1 = 27,
    ///28 - USART2 global interrupt
    USART2 = 28,
    ///29 - USART3 + USART4 interrupt
    USART3_USART4 = 29,
    ///30 - CEC global interrupt
    CEC = 30,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
///Analog to Digital ConverteR
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#ADC)
pub type ADC = crate::Periph<adc::RegisterBlock, 0x4001_2400>;
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
///Analog to Digital ConverteR
pub mod adc;
///Independent watchdog
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#IWDG)
pub type IWDG = crate::Periph<iwdg::RegisterBlock, 0x4000_3000>;
impl core::fmt::Debug for IWDG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG").finish()
    }
}
///Independent watchdog
pub mod iwdg;
///System window watchdog
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#WWDG)
pub type WWDG = crate::Periph<wwdg::RegisterBlock, 0x4000_2c00>;
impl core::fmt::Debug for WWDG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDG").finish()
    }
}
///System window watchdog
pub mod wwdg;
///Flash
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#FLASH)
pub type FLASH = crate::Periph<flash::RegisterBlock, 0x4002_2000>;
impl core::fmt::Debug for FLASH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH").finish()
    }
}
///Flash
pub mod flash;
///Debug support
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DBG)
pub type DBG = crate::Periph<dbg::RegisterBlock, 0x4001_5800>;
impl core::fmt::Debug for DBG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG").finish()
    }
}
///Debug support
pub mod dbg;
///Reset and clock control
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#RCC)
pub type RCC = crate::Periph<rcc::RegisterBlock, 0x4002_1000>;
impl core::fmt::Debug for RCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC").finish()
    }
}
///Reset and clock control
pub mod rcc;
///Power control
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#PWR)
pub type PWR = crate::Periph<pwr::RegisterBlock, 0x4000_7000>;
impl core::fmt::Debug for PWR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR").finish()
    }
}
///Power control
pub mod pwr;
///DMA controller
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMA1)
pub type DMA1 = crate::Periph<dma1::RegisterBlock, 0x4002_0000>;
impl core::fmt::Debug for DMA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1").finish()
    }
}
///DMA controller
pub mod dma1;
///DMAMUX
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX)
pub type DMAMUX = crate::Periph<dmamux::RegisterBlock, 0x4002_0800>;
impl core::fmt::Debug for DMAMUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMUX").finish()
    }
}
///DMAMUX
pub mod dmamux;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#GPIOA)
pub type GPIOA = crate::Periph<gpioa::RegisterBlock, 0x5000_0000>;
impl core::fmt::Debug for GPIOA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOA").finish()
    }
}
///General-purpose I/Os
pub mod gpioa;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#GPIOB)
pub type GPIOB = crate::Periph<gpiob::RegisterBlock, 0x5000_0400>;
impl core::fmt::Debug for GPIOB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOB").finish()
    }
}
///General-purpose I/Os
pub mod gpiob;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#GPIOB)
pub type GPIOC = crate::Periph<gpiob::RegisterBlock, 0x5000_0800>;
impl core::fmt::Debug for GPIOC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOC").finish()
    }
}
///General-purpose I/Os
pub use self::gpiob as gpioc;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#GPIOB)
pub type GPIOD = crate::Periph<gpiob::RegisterBlock, 0x5000_0c00>;
impl core::fmt::Debug for GPIOD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOD").finish()
    }
}
///General-purpose I/Os
pub use self::gpiob as gpiod;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#GPIOB)
pub type GPIOF = crate::Periph<gpiob::RegisterBlock, 0x5000_1400>;
impl core::fmt::Debug for GPIOF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOF").finish()
    }
}
///General-purpose I/Os
pub use self::gpiob as gpiof;
///Cyclic redundancy check calculation unit
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#CRC)
pub type CRC = crate::Periph<crc::RegisterBlock, 0x4002_3000>;
impl core::fmt::Debug for CRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC").finish()
    }
}
///Cyclic redundancy check calculation unit
pub mod crc;
///External interrupt/event controller
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#EXTI)
pub type EXTI = crate::Periph<exti::RegisterBlock, 0x4002_1800>;
impl core::fmt::Debug for EXTI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI").finish()
    }
}
///External interrupt/event controller
pub mod exti;
///General purpose timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#TIM15)
pub type TIM15 = crate::Periph<tim15::RegisterBlock, 0x4001_4000>;
impl core::fmt::Debug for TIM15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15").finish()
    }
}
///General purpose timers
pub mod tim15;
///General purpose timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#TIM16)
pub type TIM16 = crate::Periph<tim16::RegisterBlock, 0x4001_4400>;
impl core::fmt::Debug for TIM16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM16").finish()
    }
}
///General purpose timers
pub mod tim16;
///General purpose timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#TIM16)
pub type TIM17 = crate::Periph<tim16::RegisterBlock, 0x4001_4800>;
impl core::fmt::Debug for TIM17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM17").finish()
    }
}
///General purpose timers
pub use self::tim16 as tim17;
///Universal synchronous asynchronous receiver transmitter
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#USART1)
pub type USART1 = crate::Periph<usart1::RegisterBlock, 0x4001_3800>;
impl core::fmt::Debug for USART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART1").finish()
    }
}
///Universal synchronous asynchronous receiver transmitter
pub mod usart1;
///Universal synchronous asynchronous receiver transmitter
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#USART1)
pub type USART2 = crate::Periph<usart1::RegisterBlock, 0x4000_4400>;
impl core::fmt::Debug for USART2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART2").finish()
    }
}
///Universal synchronous asynchronous receiver transmitter
pub use self::usart1 as usart2;
///Universal synchronous asynchronous receiver transmitter
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#USART1)
pub type USART3 = crate::Periph<usart1::RegisterBlock, 0x4000_4800>;
impl core::fmt::Debug for USART3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART3").finish()
    }
}
///Universal synchronous asynchronous receiver transmitter
pub use self::usart1 as usart3;
///Universal synchronous asynchronous receiver transmitter
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#USART1)
pub type USART4 = crate::Periph<usart1::RegisterBlock, 0x4000_4c00>;
impl core::fmt::Debug for USART4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART4").finish()
    }
}
///Universal synchronous asynchronous receiver transmitter
pub use self::usart1 as usart4;
///Serial peripheral interface/Inter-IC sound
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#SPI1)
pub type SPI1 = crate::Periph<spi1::RegisterBlock, 0x4001_3000>;
impl core::fmt::Debug for SPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI1").finish()
    }
}
///Serial peripheral interface/Inter-IC sound
pub mod spi1;
///Serial peripheral interface/Inter-IC sound
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#SPI1)
pub type SPI2 = crate::Periph<spi1::RegisterBlock, 0x4000_3800>;
impl core::fmt::Debug for SPI2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2").finish()
    }
}
///Serial peripheral interface/Inter-IC sound
pub use self::spi1 as spi2;
///Advanced-timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#TIM1)
pub type TIM1 = crate::Periph<tim1::RegisterBlock, 0x4001_2c00>;
impl core::fmt::Debug for TIM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1").finish()
    }
}
///Advanced-timers
pub mod tim1;
///System configuration controller
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#SYSCFG)
pub type SYSCFG = crate::Periph<syscfg::RegisterBlock, 0x4001_0000>;
impl core::fmt::Debug for SYSCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG").finish()
    }
}
///System configuration controller
pub mod syscfg;
///Tamper and backup registers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#TAMP)
pub type TAMP = crate::Periph<tamp::RegisterBlock, 0x4000_b000>;
impl core::fmt::Debug for TAMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMP").finish()
    }
}
///Tamper and backup registers
pub mod tamp;
///Inter-integrated circuit
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#I2C1)
pub type I2C1 = crate::Periph<i2c1::RegisterBlock, 0x4000_5400>;
impl core::fmt::Debug for I2C1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C1").finish()
    }
}
///Inter-integrated circuit
pub mod i2c1;
///Inter-integrated circuit
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#I2C1)
pub type I2C2 = crate::Periph<i2c1::RegisterBlock, 0x4000_5800>;
impl core::fmt::Debug for I2C2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C2").finish()
    }
}
///Inter-integrated circuit
pub use self::i2c1 as i2c2;
///Real-time clock
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#RTC)
pub type RTC = crate::Periph<rtc::RegisterBlock, 0x4000_2800>;
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
///Real-time clock
pub mod rtc;
///General purpose timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#TIM14)
pub type TIM14 = crate::Periph<tim14::RegisterBlock, 0x4000_2000>;
impl core::fmt::Debug for TIM14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM14").finish()
    }
}
///General purpose timers
pub mod tim14;
///Basic timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#TIM6)
pub type TIM6 = crate::Periph<tim6::RegisterBlock, 0x4000_1000>;
impl core::fmt::Debug for TIM6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM6").finish()
    }
}
///Basic timers
pub mod tim6;
///Basic timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#TIM6)
pub type TIM7 = crate::Periph<tim6::RegisterBlock, 0x4000_1400>;
impl core::fmt::Debug for TIM7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM7").finish()
    }
}
///Basic timers
pub use self::tim6 as tim7;
///General-purpose-timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#TIM3)
pub type TIM3 = crate::Periph<tim3::RegisterBlock, 0x4000_0400>;
impl core::fmt::Debug for TIM3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3").finish()
    }
}
///General-purpose-timers
pub mod tim3;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
/// All the peripherals.
#[allow(non_snake_case)]
pub struct Peripherals {
    ///ADC
    pub ADC: ADC,
    ///IWDG
    pub IWDG: IWDG,
    ///WWDG
    pub WWDG: WWDG,
    ///FLASH
    pub FLASH: FLASH,
    ///DBG
    pub DBG: DBG,
    ///RCC
    pub RCC: RCC,
    ///PWR
    pub PWR: PWR,
    ///DMA1
    pub DMA1: DMA1,
    ///DMAMUX
    pub DMAMUX: DMAMUX,
    ///GPIOA
    pub GPIOA: GPIOA,
    ///GPIOB
    pub GPIOB: GPIOB,
    ///GPIOC
    pub GPIOC: GPIOC,
    ///GPIOD
    pub GPIOD: GPIOD,
    ///GPIOF
    pub GPIOF: GPIOF,
    ///CRC
    pub CRC: CRC,
    ///EXTI
    pub EXTI: EXTI,
    ///TIM15
    pub TIM15: TIM15,
    ///TIM16
    pub TIM16: TIM16,
    ///TIM17
    pub TIM17: TIM17,
    ///USART1
    pub USART1: USART1,
    ///USART2
    pub USART2: USART2,
    ///USART3
    pub USART3: USART3,
    ///USART4
    pub USART4: USART4,
    ///SPI1
    pub SPI1: SPI1,
    ///SPI2
    pub SPI2: SPI2,
    ///TIM1
    pub TIM1: TIM1,
    ///SYSCFG
    pub SYSCFG: SYSCFG,
    ///TAMP
    pub TAMP: TAMP,
    ///I2C1
    pub I2C1: I2C1,
    ///I2C2
    pub I2C2: I2C2,
    ///RTC
    pub RTC: RTC,
    ///TIM14
    pub TIM14: TIM14,
    ///TIM6
    pub TIM6: TIM6,
    ///TIM7
    pub TIM7: TIM7,
    ///TIM3
    pub TIM3: TIM3,
}
impl Peripherals {
    /// Returns all the peripherals *once*.
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    /// Unchecked version of `Peripherals::take`.
    ///
    /// # Safety
    ///
    /// Each of the returned peripherals must be used at most once.
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ADC: ADC::steal(),
            IWDG: IWDG::steal(),
            WWDG: WWDG::steal(),
            FLASH: FLASH::steal(),
            DBG: DBG::steal(),
            RCC: RCC::steal(),
            PWR: PWR::steal(),
            DMA1: DMA1::steal(),
            DMAMUX: DMAMUX::steal(),
            GPIOA: GPIOA::steal(),
            GPIOB: GPIOB::steal(),
            GPIOC: GPIOC::steal(),
            GPIOD: GPIOD::steal(),
            GPIOF: GPIOF::steal(),
            CRC: CRC::steal(),
            EXTI: EXTI::steal(),
            TIM15: TIM15::steal(),
            TIM16: TIM16::steal(),
            TIM17: TIM17::steal(),
            USART1: USART1::steal(),
            USART2: USART2::steal(),
            USART3: USART3::steal(),
            USART4: USART4::steal(),
            SPI1: SPI1::steal(),
            SPI2: SPI2::steal(),
            TIM1: TIM1::steal(),
            SYSCFG: SYSCFG::steal(),
            TAMP: TAMP::steal(),
            I2C1: I2C1::steal(),
            I2C2: I2C2::steal(),
            RTC: RTC::steal(),
            TIM14: TIM14::steal(),
            TIM6: TIM6::steal(),
            TIM7: TIM7::steal(),
            TIM3: TIM3::steal(),
        }
    }
}
