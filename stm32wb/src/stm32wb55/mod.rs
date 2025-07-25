/*!Peripheral access API for STM32WB55 microcontrollers (generated using svd2rust v0.36.1 (4052ce6 2025-04-04))

You can find an overview of the generated API [here].

API features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.

[here]: https://docs.rs/svd2rust/0.36.1/svd2rust/#peripheral-api
[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased
[repository]: https://github.com/rust-embedded/svd2rust*/
///Number available in the NVIC for configuring priority
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn RTC_TAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CHANNEL1();
    fn DMA1_CHANNEL2();
    fn DMA1_CHANNEL3();
    fn DMA1_CHANNEL4();
    fn DMA1_CHANNEL5();
    fn DMA1_CHANNEL6();
    fn DMA1_CHANNEL7();
    fn ADC1();
    fn USB_HP();
    fn USB_LP();
    fn C2SEV();
    fn COMP();
    fn EXTI9_5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM_TIM17();
    fn TIM1_CC();
    fn TIM2();
    fn PKA();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn LPUART1();
    fn SAI1();
    fn TSC();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn CRS_IT();
    fn PWR_SOTF();
    fn IPCC_C1_RX_IT();
    fn IPCC_C1_TX_IT();
    fn HSEM();
    fn LPTIM1();
    fn LPTIM2();
    fn LCD();
    fn QUADSPI();
    fn AES1();
    fn AES2();
    fn TRUE_RNG();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn DMAMUX_OVR();
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
pub static __INTERRUPTS: [Vector; 63] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector { _handler: RTC_TAMP },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_CHANNEL1,
    },
    Vector {
        _handler: DMA1_CHANNEL2,
    },
    Vector {
        _handler: DMA1_CHANNEL3,
    },
    Vector {
        _handler: DMA1_CHANNEL4,
    },
    Vector {
        _handler: DMA1_CHANNEL5,
    },
    Vector {
        _handler: DMA1_CHANNEL6,
    },
    Vector {
        _handler: DMA1_CHANNEL7,
    },
    Vector { _handler: ADC1 },
    Vector { _handler: USB_HP },
    Vector { _handler: USB_LP },
    Vector { _handler: C2SEV },
    Vector { _handler: COMP },
    Vector { _handler: EXTI9_5 },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM_TIM17,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: PKA },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: LPUART1 },
    Vector { _handler: SAI1 },
    Vector { _handler: TSC },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _handler: CRS_IT },
    Vector { _handler: PWR_SOTF },
    Vector {
        _handler: IPCC_C1_RX_IT,
    },
    Vector {
        _handler: IPCC_C1_TX_IT,
    },
    Vector { _handler: HSEM },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: LCD },
    Vector { _handler: QUADSPI },
    Vector { _handler: AES1 },
    Vector { _handler: AES2 },
    Vector { _handler: TRUE_RNG },
    Vector { _reserved: 0 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector {
        _handler: DMAMUX_OVR,
    },
];
///Enumeration of all the interrupts.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    ///0 - Window Watchdog interrupt
    WWDG = 0,
    ///1 - PVD through EXTI\[16\] (C1IMR2\[20\])
    PVD = 1,
    ///2 - RTC/TAMP/CSS on LSE through EXTI line 19 interrupt
    RTC_TAMP = 2,
    ///3 - RTC wakeup interrupt through EXTI\[19\]
    RTC_WKUP = 3,
    ///4 - Flash global interrupt
    FLASH = 4,
    ///5 - RCC global interrupt
    RCC = 5,
    ///6 - EXTI line 0 interrupt through EXTI\[0\]
    EXTI0 = 6,
    ///7 - EXTI line 0 interrupt through EXTI\[1\]
    EXTI1 = 7,
    ///8 - EXTI line 0 interrupt through EXTI\[2\]
    EXTI2 = 8,
    ///9 - EXTI line 0 interrupt through EXTI\[3\]
    EXTI3 = 9,
    ///10 - EXTI line 0 interrupt through EXTI\[4\]
    EXTI4 = 10,
    ///11 - DMA1 Channel1 global interrupt
    DMA1_CHANNEL1 = 11,
    ///12 - DMA1 Channel2 global interrupt
    DMA1_CHANNEL2 = 12,
    ///13 - DMA1 Channel3 interrupt
    DMA1_CHANNEL3 = 13,
    ///14 - DMA1 Channel4 interrupt
    DMA1_CHANNEL4 = 14,
    ///15 - DMA1 Channel5 interrupt
    DMA1_CHANNEL5 = 15,
    ///16 - DMA1 Channel6 interrupt
    DMA1_CHANNEL6 = 16,
    ///17 - DMA1 Channel 7 interrupt
    DMA1_CHANNEL7 = 17,
    ///18 - ADC1 global interrupt
    ADC1 = 18,
    ///19 - USB high priority interrupt
    USB_HP = 19,
    ///20 - USB low priority interrupt (including USB wakeup)
    USB_LP = 20,
    ///21 - CPU2 SEV through EXTI\[40\]
    C2SEV = 21,
    ///22 - COMP2 & COMP1 interrupt through AIEC\[21:20\]
    COMP = 22,
    ///23 - EXTI line \[9:5\] interrupt through EXTI\[9:5\]
    EXTI9_5 = 23,
    ///24 - Timer 1 break interrupt
    TIM1_BRK = 24,
    ///25 - Timer 1 Update
    TIM1_UP = 25,
    ///26 - TIM1 Trigger and Commutation interrupts and TIM17 global interrupt
    TIM1_TRG_COM_TIM17 = 26,
    ///27 - TIM1 Capture Compare interrupt
    TIM1_CC = 27,
    ///28 - TIM2 global interrupt
    TIM2 = 28,
    ///29 - Private key accelerator interrupt
    PKA = 29,
    ///30 - I2C1 event interrupt
    I2C1_EV = 30,
    ///31 - I2C1 error interrupt
    I2C1_ER = 31,
    ///32 - I2C3 event interrupt
    I2C3_EV = 32,
    ///33 - I2C3 error interrupt
    I2C3_ER = 33,
    ///34 - SPI 1 global interrupt
    SPI1 = 34,
    ///35 - SPI1 global interrupt
    SPI2 = 35,
    ///36 - USART1 global interrupt
    USART1 = 36,
    ///37 - LPUART1 global interrupt
    LPUART1 = 37,
    ///38 - SAI1 global interrupt
    SAI1 = 38,
    ///39 - TSC global interrupt
    TSC = 39,
    ///40 - EXTI line \[15:10\] interrupt through EXTI\[15:10\]
    EXTI15_10 = 40,
    ///41 - RTC Alarms (A and B) interrupt through AIEC
    RTC_ALARM = 41,
    ///42 - CRS interrupt
    CRS_IT = 42,
    ///43 - PWR switching on the fly interrupt
    PWR_SOTF = 43,
    ///44 - IPCC CPU1 RX occupied interrupt
    IPCC_C1_RX_IT = 44,
    ///45 - IPCC CPU1 TX free interrupt
    IPCC_C1_TX_IT = 45,
    ///46 - Semaphore interrupt 0 to CPU1
    HSEM = 46,
    ///47 - LPtimer 1 global interrupt
    LPTIM1 = 47,
    ///48 - LPtimer 2 global interrupt
    LPTIM2 = 48,
    ///49 - LCD global interrupt
    LCD = 49,
    ///50 - QSPI global interrupt
    QUADSPI = 50,
    ///51 - AES1 global interrupt
    AES1 = 51,
    ///52 - AES2 global interrupt
    AES2 = 52,
    ///53 - True random number generator interrupt
    TRUE_RNG = 53,
    ///55 - DMA2 channel 1 interrupt
    DMA2_CH1 = 55,
    ///56 - DMA2 channel 2 interrupt
    DMA2_CH2 = 56,
    ///57 - DMA2 channel 3 interrupt
    DMA2_CH3 = 57,
    ///58 - DMA2 channel 4 interrupt
    DMA2_CH4 = 58,
    ///59 - DMA2 channel 5 interrupt
    DMA2_CH5 = 59,
    ///60 - DMA2 channel 6 interrupt
    DMA2_CH6 = 60,
    ///61 - DMA2 channel 7 interrupt
    DMA2_CH7 = 61,
    ///62 - DMAMUX overrun interrupt
    DMAMUX_OVR = 62,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
///Direct memory access controller
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DMA1)
pub type DMA1 = crate::Periph<dma1::RegisterBlock, 0x4002_0000>;
impl core::fmt::Debug for DMA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1").finish()
    }
}
///Direct memory access controller
pub mod dma1;
///Direct memory access controller
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DMA1)
pub type DMA2 = crate::Periph<dma1::RegisterBlock, 0x4002_0400>;
impl core::fmt::Debug for DMA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA2").finish()
    }
}
///Direct memory access controller
pub use self::dma1 as dma2;
///Direct memory access Multiplexer
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DMAMUX1)
pub type DMAMUX1 = crate::Periph<dmamux1::RegisterBlock, 0x4002_0800>;
impl core::fmt::Debug for DMAMUX1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAMUX1").finish()
    }
}
///Direct memory access Multiplexer
pub mod dmamux1;
///Cyclic redundancy check calculation unit
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#CRC)
pub type CRC = crate::Periph<crc::RegisterBlock, 0x4002_3000>;
impl core::fmt::Debug for CRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRC").finish()
    }
}
///Cyclic redundancy check calculation unit
pub mod crc;
///Liquid crystal display controller
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#LCD)
pub type LCD = crate::Periph<lcd::RegisterBlock, 0x4000_2400>;
impl core::fmt::Debug for LCD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCD").finish()
    }
}
///Liquid crystal display controller
pub mod lcd;
///Touch sensing controller
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TSC)
pub type TSC = crate::Periph<tsc::RegisterBlock, 0x4002_4000>;
impl core::fmt::Debug for TSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSC").finish()
    }
}
///Touch sensing controller
pub mod tsc;
///Independent watchdog
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IWDG)
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
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#WWDG)
pub type WWDG = crate::Periph<wwdg::RegisterBlock, 0x4000_2c00>;
impl core::fmt::Debug for WWDG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDG").finish()
    }
}
///System window watchdog
pub mod wwdg;
///Inter-integrated circuit
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#I2C1)
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
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#I2C1)
pub type I2C3 = crate::Periph<i2c1::RegisterBlock, 0x4000_5c00>;
impl core::fmt::Debug for I2C3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C3").finish()
    }
}
///Inter-integrated circuit
pub use self::i2c1 as i2c3;
///Flash
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#Flash)
pub type FLASH = crate::Periph<flash::RegisterBlock, 0x5800_4000>;
impl core::fmt::Debug for FLASH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH").finish()
    }
}
///Flash
pub mod flash;
///QuadSPI interface
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#QUADSPI)
pub type QUADSPI = crate::Periph<quadspi::RegisterBlock, 0xa000_1000>;
impl core::fmt::Debug for QUADSPI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUADSPI").finish()
    }
}
///QuadSPI interface
pub mod quadspi;
///Reset and clock control
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC)
pub type RCC = crate::Periph<rcc::RegisterBlock, 0x5800_0000>;
impl core::fmt::Debug for RCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC").finish()
    }
}
///Reset and clock control
pub mod rcc;
///Power control
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR)
pub type PWR = crate::Periph<pwr::RegisterBlock, 0x5800_0400>;
impl core::fmt::Debug for PWR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR").finish()
    }
}
///Power control
pub mod pwr;
///SYSCFG_VREFBUF
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SYSCFG)
pub type SYSCFG = crate::Periph<syscfg::RegisterBlock, 0x4001_0000>;
impl core::fmt::Debug for SYSCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG").finish()
    }
}
///SYSCFG_VREFBUF
pub mod syscfg;
///Comparator instance 1
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#COMP)
pub type COMP = crate::Periph<comp::RegisterBlock, 0x4001_0200>;
impl core::fmt::Debug for COMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP").finish()
    }
}
///Comparator instance 1
pub mod comp;
///Random number generator
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RNG)
pub type RNG = crate::Periph<rng::RegisterBlock, 0x5800_1000>;
impl core::fmt::Debug for RNG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG").finish()
    }
}
///Random number generator
pub mod rng;
///Advanced encryption standard hardware accelerator 1
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#AES1)
pub type AES1 = crate::Periph<aes1::RegisterBlock, 0x5006_0000>;
impl core::fmt::Debug for AES1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES1").finish()
    }
}
///Advanced encryption standard hardware accelerator 1
pub mod aes1;
///Advanced encryption standard hardware accelerator 1
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#AES2)
pub type AES2 = crate::Periph<aes2::RegisterBlock, 0x5800_1800>;
impl core::fmt::Debug for AES2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES2").finish()
    }
}
///Advanced encryption standard hardware accelerator 1
pub mod aes2;
///HSEM
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#HSEM)
pub type HSEM = crate::Periph<hsem::RegisterBlock, 0x5800_1400>;
impl core::fmt::Debug for HSEM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM").finish()
    }
}
///HSEM
pub mod hsem;
///Analog to Digital Converter instance 1
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#ADC1)
pub type ADC1 = crate::Periph<adc1::RegisterBlock, 0x5004_0000>;
impl core::fmt::Debug for ADC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC1").finish()
    }
}
///Analog to Digital Converter instance 1
pub mod adc1;
///ADC common registers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#ADC_Common)
pub type ADC_COMMON = crate::Periph<adc_common::RegisterBlock, 0x5004_0300>;
impl core::fmt::Debug for ADC_COMMON {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_COMMON").finish()
    }
}
///ADC common registers
pub mod adc_common;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOA)
pub type GPIOA = crate::Periph<gpioa::RegisterBlock, 0x4800_0000>;
impl core::fmt::Debug for GPIOA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOA").finish()
    }
}
///General-purpose I/Os
pub mod gpioa;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOB)
pub type GPIOB = crate::Periph<gpiob::RegisterBlock, 0x4800_0400>;
impl core::fmt::Debug for GPIOB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOB").finish()
    }
}
///General-purpose I/Os
pub mod gpiob;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOC)
pub type GPIOC = crate::Periph<gpioc::RegisterBlock, 0x4800_0800>;
impl core::fmt::Debug for GPIOC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOC").finish()
    }
}
///General-purpose I/Os
pub mod gpioc;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOC)
pub type GPIOD = crate::Periph<gpioc::RegisterBlock, 0x4800_0c00>;
impl core::fmt::Debug for GPIOD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOD").finish()
    }
}
///General-purpose I/Os
pub use self::gpioc as gpiod;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE)
pub type GPIOE = crate::Periph<gpioe::RegisterBlock, 0x4800_1000>;
impl core::fmt::Debug for GPIOE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOE").finish()
    }
}
///General-purpose I/Os
pub mod gpioe;
///General-purpose I/Os
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOH)
pub type GPIOH = crate::Periph<gpioh::RegisterBlock, 0x4800_1c00>;
impl core::fmt::Debug for GPIOH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOH").finish()
    }
}
///General-purpose I/Os
pub mod gpioh;
///Serial audio interface
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SAI1)
pub type SAI1 = crate::Periph<sai1::RegisterBlock, 0x4001_5400>;
impl core::fmt::Debug for SAI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAI1").finish()
    }
}
///Serial audio interface
pub mod sai1;
///General-purpose-timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TIM2)
pub type TIM2 = crate::Periph<tim2::RegisterBlock, 0x4000_0000>;
impl core::fmt::Debug for TIM2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2").finish()
    }
}
///General-purpose-timers
pub mod tim2;
///General purpose timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TIM16)
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
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TIM17)
pub type TIM17 = crate::Periph<tim17::RegisterBlock, 0x4001_4800>;
impl core::fmt::Debug for TIM17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM17").finish()
    }
}
///General purpose timers
pub mod tim17;
///Advanced-timers
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#TIM1)
pub type TIM1 = crate::Periph<tim1::RegisterBlock, 0x4001_2c00>;
impl core::fmt::Debug for TIM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1").finish()
    }
}
///Advanced-timers
pub mod tim1;
///Low power timer
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#LPTIM1)
pub type LPTIM1 = crate::Periph<lptim1::RegisterBlock, 0x4000_7c00>;
impl core::fmt::Debug for LPTIM1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM1").finish()
    }
}
///Low power timer
pub mod lptim1;
///Low power timer
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#LPTIM1)
pub type LPTIM2 = crate::Periph<lptim1::RegisterBlock, 0x4000_9400>;
impl core::fmt::Debug for LPTIM2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTIM2").finish()
    }
}
///Low power timer
pub use self::lptim1 as lptim2;
///Universal synchronous asynchronous receiver transmitter
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USART1)
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
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USART1)
pub type LPUART1 = crate::Periph<usart1::RegisterBlock, 0x4000_8000>;
impl core::fmt::Debug for LPUART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART1").finish()
    }
}
///Universal synchronous asynchronous receiver transmitter
pub use self::usart1 as lpuart1;
///Serial peripheral interface/Inter-IC sound
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SPI1)
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
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#SPI1)
pub type SPI2 = crate::Periph<spi1::RegisterBlock, 0x4000_3800>;
impl core::fmt::Debug for SPI2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI2").finish()
    }
}
///Serial peripheral interface/Inter-IC sound
pub use self::spi1 as spi2;
///Real-time clock
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RTC)
pub type RTC = crate::Periph<rtc::RegisterBlock, 0x4000_2800>;
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
///Real-time clock
pub mod rtc;
///Debug support
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU)
pub type DBGMCU = crate::Periph<dbgmcu::RegisterBlock, 0xe004_2000>;
impl core::fmt::Debug for DBGMCU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU").finish()
    }
}
///Debug support
pub mod dbgmcu;
///PKA
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PKA)
pub type PKA = crate::Periph<pka::RegisterBlock, 0x5800_2000>;
impl core::fmt::Debug for PKA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKA").finish()
    }
}
///PKA
pub mod pka;
///IPCC
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#IPCC)
pub type IPCC = crate::Periph<ipcc::RegisterBlock, 0x5800_0c00>;
impl core::fmt::Debug for IPCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPCC").finish()
    }
}
///IPCC
pub mod ipcc;
///External interrupt/event controller
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI)
pub type EXTI = crate::Periph<exti::RegisterBlock, 0x5800_0800>;
impl core::fmt::Debug for EXTI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI").finish()
    }
}
///External interrupt/event controller
pub mod exti;
///Clock recovery system
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#CRS)
pub type CRS = crate::Periph<crs::RegisterBlock, 0x4000_6000>;
impl core::fmt::Debug for CRS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRS").finish()
    }
}
///Clock recovery system
pub mod crs;
///Universal serial bus full-speed device interface
///
///See peripheral [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB)
pub type USB = crate::Periph<usb::RegisterBlock, 0x4000_6800>;
impl core::fmt::Debug for USB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB").finish()
    }
}
///Universal serial bus full-speed device interface
pub mod usb;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
/// All the peripherals.
#[allow(non_snake_case)]
pub struct Peripherals {
    ///DMA1
    pub DMA1: DMA1,
    ///DMA2
    pub DMA2: DMA2,
    ///DMAMUX1
    pub DMAMUX1: DMAMUX1,
    ///CRC
    pub CRC: CRC,
    ///LCD
    pub LCD: LCD,
    ///TSC
    pub TSC: TSC,
    ///IWDG
    pub IWDG: IWDG,
    ///WWDG
    pub WWDG: WWDG,
    ///I2C1
    pub I2C1: I2C1,
    ///I2C3
    pub I2C3: I2C3,
    ///Flash
    pub FLASH: FLASH,
    ///QUADSPI
    pub QUADSPI: QUADSPI,
    ///RCC
    pub RCC: RCC,
    ///PWR
    pub PWR: PWR,
    ///SYSCFG
    pub SYSCFG: SYSCFG,
    ///COMP
    pub COMP: COMP,
    ///RNG
    pub RNG: RNG,
    ///AES1
    pub AES1: AES1,
    ///AES2
    pub AES2: AES2,
    ///HSEM
    pub HSEM: HSEM,
    ///ADC1
    pub ADC1: ADC1,
    ///ADC_Common
    pub ADC_COMMON: ADC_COMMON,
    ///GPIOA
    pub GPIOA: GPIOA,
    ///GPIOB
    pub GPIOB: GPIOB,
    ///GPIOC
    pub GPIOC: GPIOC,
    ///GPIOD
    pub GPIOD: GPIOD,
    ///GPIOE
    pub GPIOE: GPIOE,
    ///GPIOH
    pub GPIOH: GPIOH,
    ///SAI1
    pub SAI1: SAI1,
    ///TIM2
    pub TIM2: TIM2,
    ///TIM16
    pub TIM16: TIM16,
    ///TIM17
    pub TIM17: TIM17,
    ///TIM1
    pub TIM1: TIM1,
    ///LPTIM1
    pub LPTIM1: LPTIM1,
    ///LPTIM2
    pub LPTIM2: LPTIM2,
    ///USART1
    pub USART1: USART1,
    ///LPUART1
    pub LPUART1: LPUART1,
    ///SPI1
    pub SPI1: SPI1,
    ///SPI2
    pub SPI2: SPI2,
    ///RTC
    pub RTC: RTC,
    ///DBGMCU
    pub DBGMCU: DBGMCU,
    ///PKA
    pub PKA: PKA,
    ///IPCC
    pub IPCC: IPCC,
    ///EXTI
    pub EXTI: EXTI,
    ///CRS
    pub CRS: CRS,
    ///USB
    pub USB: USB,
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
            DMA1: DMA1::steal(),
            DMA2: DMA2::steal(),
            DMAMUX1: DMAMUX1::steal(),
            CRC: CRC::steal(),
            LCD: LCD::steal(),
            TSC: TSC::steal(),
            IWDG: IWDG::steal(),
            WWDG: WWDG::steal(),
            I2C1: I2C1::steal(),
            I2C3: I2C3::steal(),
            FLASH: FLASH::steal(),
            QUADSPI: QUADSPI::steal(),
            RCC: RCC::steal(),
            PWR: PWR::steal(),
            SYSCFG: SYSCFG::steal(),
            COMP: COMP::steal(),
            RNG: RNG::steal(),
            AES1: AES1::steal(),
            AES2: AES2::steal(),
            HSEM: HSEM::steal(),
            ADC1: ADC1::steal(),
            ADC_COMMON: ADC_COMMON::steal(),
            GPIOA: GPIOA::steal(),
            GPIOB: GPIOB::steal(),
            GPIOC: GPIOC::steal(),
            GPIOD: GPIOD::steal(),
            GPIOE: GPIOE::steal(),
            GPIOH: GPIOH::steal(),
            SAI1: SAI1::steal(),
            TIM2: TIM2::steal(),
            TIM16: TIM16::steal(),
            TIM17: TIM17::steal(),
            TIM1: TIM1::steal(),
            LPTIM1: LPTIM1::steal(),
            LPTIM2: LPTIM2::steal(),
            USART1: USART1::steal(),
            LPUART1: LPUART1::steal(),
            SPI1: SPI1::steal(),
            SPI2: SPI2::steal(),
            RTC: RTC::steal(),
            DBGMCU: DBGMCU::steal(),
            PKA: PKA::steal(),
            IPCC: IPCC::steal(),
            EXTI: EXTI::steal(),
            CRS: CRS::steal(),
            USB: USB::steal(),
        }
    }
}
