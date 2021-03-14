#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD_PVM();
    fn RTC();
    fn RTC_S();
    fn TAMP();
    fn TAMP_S();
    fn FLASH();
    fn FLASH_S();
    fn RCC();
    fn RCC_S();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn EXTI5();
    fn EXTI6();
    fn EXTI7();
    fn EXTI8();
    fn EXTI9();
    fn EXTI10();
    fn EXTI11();
    fn EXTI12();
    fn EXTI13();
    fn EXTI14();
    fn EXTI15();
    fn DMAMUX1_OVR();
    fn DMAMUX1_OVR_S();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
    fn DMA1_CH4();
    fn DMA1_CH5();
    fn DMA1_CH6();
    fn DMA1_CH7();
    fn DMA1_CHANNEL8();
    fn ADC1_2();
    fn FDCAN1_IT0();
    fn FDCAN1_IT1();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn TIM5();
    fn TIM6();
    fn TIM7();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn UART4();
    fn UART5();
    fn LPUART1();
    fn LPTIM1();
    fn LPTIM2();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn COMP();
    fn USB_FS();
    fn FMC();
    fn OCTOSPI1();
    fn SDMMC1();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn DMA2_CH6();
    fn DMA2_CH7();
    fn DMA2_CH8();
    fn I2C3_EV();
    fn I2C3_ER();
    fn SAI1();
    fn SAI2();
    fn TSC();
    fn RNG();
    fn LPTIM3();
    fn SPI3();
    fn I2C4_ER();
    fn I2C4_EV();
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn DFSDM1_FLT3();
    fn UCPD1();
    fn ICACHE();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 108] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD_PVM },
    Vector { _handler: RTC },
    Vector { _handler: RTC_S },
    Vector { _handler: TAMP },
    Vector { _handler: TAMP_S },
    Vector { _handler: FLASH },
    Vector { _handler: FLASH_S },
    Vector { _reserved: 0 },
    Vector { _handler: RCC },
    Vector { _handler: RCC_S },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: EXTI5 },
    Vector { _handler: EXTI6 },
    Vector { _handler: EXTI7 },
    Vector { _handler: EXTI8 },
    Vector { _handler: EXTI9 },
    Vector { _handler: EXTI10 },
    Vector { _handler: EXTI11 },
    Vector { _handler: EXTI12 },
    Vector { _handler: EXTI13 },
    Vector { _handler: EXTI14 },
    Vector { _handler: EXTI15 },
    Vector {
        _handler: DMAMUX1_OVR,
    },
    Vector {
        _handler: DMAMUX1_OVR_S,
    },
    Vector { _handler: DMA1_CH1 },
    Vector { _handler: DMA1_CH2 },
    Vector { _handler: DMA1_CH3 },
    Vector { _handler: DMA1_CH4 },
    Vector { _handler: DMA1_CH5 },
    Vector { _handler: DMA1_CH6 },
    Vector { _handler: DMA1_CH7 },
    Vector {
        _handler: DMA1_CHANNEL8,
    },
    Vector { _handler: ADC1_2 },
    Vector { _reserved: 0 },
    Vector {
        _handler: FDCAN1_IT0,
    },
    Vector {
        _handler: FDCAN1_IT1,
    },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: TIM5 },
    Vector { _handler: TIM6 },
    Vector { _handler: TIM7 },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: LPUART1 },
    Vector { _handler: LPTIM1 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: COMP },
    Vector { _handler: USB_FS },
    Vector { _reserved: 0 },
    Vector { _handler: FMC },
    Vector { _handler: OCTOSPI1 },
    Vector { _reserved: 0 },
    Vector { _handler: SDMMC1 },
    Vector { _reserved: 0 },
    Vector { _handler: DMA2_CH1 },
    Vector { _handler: DMA2_CH2 },
    Vector { _handler: DMA2_CH3 },
    Vector { _handler: DMA2_CH4 },
    Vector { _handler: DMA2_CH5 },
    Vector { _handler: DMA2_CH6 },
    Vector { _handler: DMA2_CH7 },
    Vector { _handler: DMA2_CH8 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector { _handler: SAI1 },
    Vector { _handler: SAI2 },
    Vector { _handler: TSC },
    Vector { _reserved: 0 },
    Vector { _handler: RNG },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: LPTIM3 },
    Vector { _handler: SPI3 },
    Vector { _handler: I2C4_ER },
    Vector { _handler: I2C4_EV },
    Vector {
        _handler: DFSDM1_FLT0,
    },
    Vector {
        _handler: DFSDM1_FLT1,
    },
    Vector {
        _handler: DFSDM1_FLT2,
    },
    Vector {
        _handler: DFSDM1_FLT3,
    },
    Vector { _handler: UCPD1 },
    Vector { _handler: ICACHE },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG = 0,
    #[doc = "1 - PVD/PVM1/PVM2/PVM3/PVM4 through EXTI"]
    PVD_PVM = 1,
    #[doc = "2 - RTC global interrupts (EXTI line 17)"]
    RTC = 2,
    #[doc = "3 - RTC secure global interrupts (EXTI line 18)"]
    RTC_S = 3,
    #[doc = "4 - TAMPTamper global interrupt (EXTI line 19)"]
    TAMP = 4,
    #[doc = "5 - Tamper secure global interrupt (EXTI line 20)"]
    TAMP_S = 5,
    #[doc = "6 - Flash global interrupt"]
    FLASH = 6,
    #[doc = "7 - Flash memory secure global interrupt"]
    FLASH_S = 7,
    #[doc = "9 - RCC global interrupt"]
    RCC = 9,
    #[doc = "10 - RCC SECURE GLOBAL INTERRUPT"]
    RCC_S = 10,
    #[doc = "11 - EXTI line0 interrupt"]
    EXTI0 = 11,
    #[doc = "12 - EXTI line1 interrupt"]
    EXTI1 = 12,
    #[doc = "13 - EXTI line2 interrupt"]
    EXTI2 = 13,
    #[doc = "14 - EXTI line3 interrupt"]
    EXTI3 = 14,
    #[doc = "15 - EXTI line4 interrupt"]
    EXTI4 = 15,
    #[doc = "16 - EXTI line5 interrupt"]
    EXTI5 = 16,
    #[doc = "17 - EXTI line6 interrupt"]
    EXTI6 = 17,
    #[doc = "18 - EXTI line7 interrupt"]
    EXTI7 = 18,
    #[doc = "19 - EXTI line8 interrupt"]
    EXTI8 = 19,
    #[doc = "20 - EXTI line9 interrupt"]
    EXTI9 = 20,
    #[doc = "21 - EXTI line10 interrupt"]
    EXTI10 = 21,
    #[doc = "22 - EXTI line11 interrupt"]
    EXTI11 = 22,
    #[doc = "23 - EXTI line12 interrupt"]
    EXTI12 = 23,
    #[doc = "24 - EXTI line13 interrupt"]
    EXTI13 = 24,
    #[doc = "25 - EXTI line14 interrupt"]
    EXTI14 = 25,
    #[doc = "26 - EXTI line15 interrupt"]
    EXTI15 = 26,
    #[doc = "27 - DMAMUX overrun interrupt"]
    DMAMUX1_OVR = 27,
    #[doc = "28 - DMAMUX1 secure overRun interrupt"]
    DMAMUX1_OVR_S = 28,
    #[doc = "29 - DMA1 Channel1 global interrupt"]
    DMA1_CH1 = 29,
    #[doc = "30 - DMA1 Channel2 global interrupt"]
    DMA1_CH2 = 30,
    #[doc = "31 - DMA1 Channel3 interrupt"]
    DMA1_CH3 = 31,
    #[doc = "32 - DMA1 Channel4 interrupt"]
    DMA1_CH4 = 32,
    #[doc = "33 - DMA1 Channel5 interrupt"]
    DMA1_CH5 = 33,
    #[doc = "34 - DMA1 Channel6 interrupt"]
    DMA1_CH6 = 34,
    #[doc = "35 - DMA1 Channel 7 interrupt"]
    DMA1_CH7 = 35,
    #[doc = "36 - DMA1_Channel8"]
    DMA1_CHANNEL8 = 36,
    #[doc = "37 - ADC1_2 global interrupt"]
    ADC1_2 = 37,
    #[doc = "39 - FDCAN1 Interrupt 0"]
    FDCAN1_IT0 = 39,
    #[doc = "40 - FDCAN1 Interrupt 1"]
    FDCAN1_IT1 = 40,
    #[doc = "41 - TIM1 Break"]
    TIM1_BRK = 41,
    #[doc = "42 - TIM1 Update"]
    TIM1_UP = 42,
    #[doc = "43 - TIM1 Trigger and Commutation"]
    TIM1_TRG_COM = 43,
    #[doc = "44 - TIM1 Capture Compare interrupt"]
    TIM1_CC = 44,
    #[doc = "45 - TIM2 global interrupt"]
    TIM2 = 45,
    #[doc = "46 - TIM3 global interrupt"]
    TIM3 = 46,
    #[doc = "47 - TIM4 global interrupt"]
    TIM4 = 47,
    #[doc = "48 - TIM5 global interrupt"]
    TIM5 = 48,
    #[doc = "49 - TIM6 global interrupt"]
    TIM6 = 49,
    #[doc = "50 - TIM7 global interrupt"]
    TIM7 = 50,
    #[doc = "51 - TIM8 Break Interrupt"]
    TIM8_BRK = 51,
    #[doc = "52 - TIM8 Update Interrupt"]
    TIM8_UP = 52,
    #[doc = "53 - TIM8 Trigger and Commutation Interrupt"]
    TIM8_TRG_COM = 53,
    #[doc = "54 - TIM8 Capture Compare Interrupt"]
    TIM8_CC = 54,
    #[doc = "55 - I2C1 event interrupt"]
    I2C1_EV = 55,
    #[doc = "56 - I2C1 error interrupt"]
    I2C1_ER = 56,
    #[doc = "57 - I2C2 event interrupt"]
    I2C2_EV = 57,
    #[doc = "58 - I2C2 error interrupt"]
    I2C2_ER = 58,
    #[doc = "59 - SPI1 global interrupt"]
    SPI1 = 59,
    #[doc = "60 - SPI2 global interrupt"]
    SPI2 = 60,
    #[doc = "61 - USART1 global interrupt"]
    USART1 = 61,
    #[doc = "62 - USART2 global interrupt"]
    USART2 = 62,
    #[doc = "63 - USART3 global interrupt"]
    USART3 = 63,
    #[doc = "64 - UART4 global interrupt"]
    UART4 = 64,
    #[doc = "65 - UART5 global interrupt"]
    UART5 = 65,
    #[doc = "66 - LPUART1 global interrupt"]
    LPUART1 = 66,
    #[doc = "67 - LP TIM1 interrupt"]
    LPTIM1 = 67,
    #[doc = "68 - LP TIM2 interrupt"]
    LPTIM2 = 68,
    #[doc = "69 - TIM15 global interrupt"]
    TIM15 = 69,
    #[doc = "70 - TIM16 global interrupt"]
    TIM16 = 70,
    #[doc = "71 - TIM17 global interrupt"]
    TIM17 = 71,
    #[doc = "72 - COMP1 and COMP2 interrupts"]
    COMP = 72,
    #[doc = "73 - USB FS global interrupt"]
    USB_FS = 73,
    #[doc = "75 - FMC global interrupt"]
    FMC = 75,
    #[doc = "76 - OCTOSPI1 global interrupt"]
    OCTOSPI1 = 76,
    #[doc = "78 - SDMMC1 global interrupt"]
    SDMMC1 = 78,
    #[doc = "80 - DMA2_CH1"]
    DMA2_CH1 = 80,
    #[doc = "81 - DMA2_CH2"]
    DMA2_CH2 = 81,
    #[doc = "82 - DMA2_CH3"]
    DMA2_CH3 = 82,
    #[doc = "83 - DMA2_CH4"]
    DMA2_CH4 = 83,
    #[doc = "84 - DMA2_CH5"]
    DMA2_CH5 = 84,
    #[doc = "85 - DMA2_CH6"]
    DMA2_CH6 = 85,
    #[doc = "86 - DMA2_CH7"]
    DMA2_CH7 = 86,
    #[doc = "87 - DMA2_CH8"]
    DMA2_CH8 = 87,
    #[doc = "88 - I2C3 event interrupt"]
    I2C3_EV = 88,
    #[doc = "89 - I2C3 error interrupt"]
    I2C3_ER = 89,
    #[doc = "90 - SAI1 global interrupt"]
    SAI1 = 90,
    #[doc = "91 - SAI2 global interrupt"]
    SAI2 = 91,
    #[doc = "92 - TSC global interrupt"]
    TSC = 92,
    #[doc = "94 - RNG global interrupt"]
    RNG = 94,
    #[doc = "98 - LPTIM3"]
    LPTIM3 = 98,
    #[doc = "99 - SPI3"]
    SPI3 = 99,
    #[doc = "100 - I2C4 error interrupt"]
    I2C4_ER = 100,
    #[doc = "101 - I2C4 event interrupt"]
    I2C4_EV = 101,
    #[doc = "102 - DFSDM1_FLT0 global interrupt"]
    DFSDM1_FLT0 = 102,
    #[doc = "103 - DFSDM1_FLT1 global interrupt"]
    DFSDM1_FLT1 = 103,
    #[doc = "104 - DFSDM1_FLT2 global interrupt"]
    DFSDM1_FLT2 = 104,
    #[doc = "105 - DFSDM1_FLT3 global interrupt"]
    DFSDM1_FLT3 = 105,
    #[doc = "106 - UCPD global interrupt"]
    UCPD1 = 106,
    #[doc = "107 - ICACHE"]
    ICACHE = 107,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Digital filter for sigma delta modulators"]
pub struct DFSDM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DFSDM1 {}
impl DFSDM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dfsdm1::RegisterBlock {
        0x4001_6000 as *const _
    }
}
impl Deref for DFSDM1 {
    type Target = dfsdm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DFSDM1::ptr() }
    }
}
#[doc = "Digital filter for sigma delta modulators"]
pub mod dfsdm1;
#[doc = "Digital filter for sigma delta modulators"]
pub struct SEC_DFSDM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_DFSDM1 {}
impl SEC_DFSDM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dfsdm1::RegisterBlock {
        0x5001_6000 as *const _
    }
}
impl Deref for SEC_DFSDM1 {
    type Target = dfsdm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_DFSDM1::ptr() }
    }
}
#[doc = "Direct memory access Multiplexer"]
pub struct DMAMUX1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX1 {}
impl DMAMUX1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmamux1::RegisterBlock {
        0x4002_0800 as *const _
    }
}
impl Deref for DMAMUX1 {
    type Target = dmamux1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAMUX1::ptr() }
    }
}
#[doc = "Direct memory access Multiplexer"]
pub mod dmamux1;
#[doc = "Direct memory access Multiplexer"]
pub struct SEC_DMAMUX1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_DMAMUX1 {}
impl SEC_DMAMUX1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmamux1::RegisterBlock {
        0x5002_0800 as *const _
    }
}
impl Deref for SEC_DMAMUX1 {
    type Target = dmamux1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_DMAMUX1::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x4002_f400 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "External interrupt/event controller"]
pub struct SEC_EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_EXTI {}
impl SEC_EXTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x5002_f400 as *const _
    }
}
impl Deref for SEC_EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_EXTI::ptr() }
    }
}
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "Flash"]
pub struct SEC_FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_FLASH {}
impl SEC_FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x5002_2000 as *const _
    }
}
impl Deref for SEC_FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_FLASH::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4202_0000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct SEC_GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GPIOA {}
impl SEC_GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5202_0000 as *const _
    }
}
impl Deref for SEC_GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x4202_0400 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct SEC_GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GPIOB {}
impl SEC_GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x5202_0400 as *const _
    }
}
impl Deref for SEC_GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4202_0800 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioc;
#[doc = "General-purpose I/Os"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4202_0c00 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4202_1000 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4202_1400 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4202_1800 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct SEC_GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GPIOC {}
impl SEC_GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x5202_0800 as *const _
    }
}
impl Deref for SEC_GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct SEC_GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GPIOD {}
impl SEC_GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x5202_0c00 as *const _
    }
}
impl Deref for SEC_GPIOD {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GPIOD::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct SEC_GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GPIOE {}
impl SEC_GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x5202_1000 as *const _
    }
}
impl Deref for SEC_GPIOE {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GPIOE::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct SEC_GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GPIOF {}
impl SEC_GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x5202_1400 as *const _
    }
}
impl Deref for SEC_GPIOF {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GPIOF::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct SEC_GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GPIOG {}
impl SEC_GPIOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x5202_1800 as *const _
    }
}
impl Deref for SEC_GPIOG {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GPIOG::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioh::RegisterBlock {
        0x4202_1c00 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioh;
#[doc = "General-purpose I/Os"]
pub struct SEC_GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GPIOH {}
impl SEC_GPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioh::RegisterBlock {
        0x5202_1c00 as *const _
    }
}
impl Deref for SEC_GPIOH {
    type Target = gpioh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GPIOH::ptr() }
    }
}
#[doc = "Tamper and backup registers"]
pub struct TAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TAMP {}
impl TAMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tamp::RegisterBlock {
        0x4000_3400 as *const _
    }
}
impl Deref for TAMP {
    type Target = tamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TAMP::ptr() }
    }
}
#[doc = "Tamper and backup registers"]
pub mod tamp;
#[doc = "Tamper and backup registers"]
pub struct SEC_TAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TAMP {}
impl SEC_TAMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tamp::RegisterBlock {
        0x5000_3400 as *const _
    }
}
impl Deref for SEC_TAMP {
    type Target = tamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TAMP::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5400 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "Inter-integrated circuit"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5800 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_5c00 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4000_8400 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct SEC_I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_I2C1 {}
impl SEC_I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x5000_5400 as *const _
    }
}
impl Deref for SEC_I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_I2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct SEC_I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_I2C2 {}
impl SEC_I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x5000_5800 as *const _
    }
}
impl Deref for SEC_I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_I2C2::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct SEC_I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_I2C3 {}
impl SEC_I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x5000_5c00 as *const _
    }
}
impl Deref for SEC_I2C3 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_I2C3::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct SEC_I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_I2C4 {}
impl SEC_I2C4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x5000_8400 as *const _
    }
}
impl Deref for SEC_I2C4 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_I2C4::ptr() }
    }
}
#[doc = "ICache"]
pub struct ICACHE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICACHE {}
impl ICACHE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icache::RegisterBlock {
        0x4003_0400 as *const _
    }
}
impl Deref for ICACHE {
    type Target = icache::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICACHE::ptr() }
    }
}
#[doc = "ICache"]
pub mod icache;
#[doc = "ICache"]
pub struct SEC_ICACHE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_ICACHE {}
impl SEC_ICACHE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icache::RegisterBlock {
        0x5003_0400 as *const _
    }
}
impl Deref for SEC_ICACHE {
    type Target = icache::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_ICACHE::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iwdg::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "Independent watchdog"]
pub struct SEC_IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_IWDG {}
impl SEC_IWDG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iwdg::RegisterBlock {
        0x5000_3000 as *const _
    }
}
impl Deref for SEC_IWDG {
    type Target = iwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_IWDG::ptr() }
    }
}
#[doc = "Low power timer"]
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_7c00 as *const _
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM1::ptr() }
    }
}
#[doc = "Low power timer"]
pub mod lptim1;
#[doc = "Low power timer"]
pub struct LPTIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM2 {}
impl LPTIM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_9400 as *const _
    }
}
impl Deref for LPTIM2 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM2::ptr() }
    }
}
#[doc = "Low power timer"]
pub struct LPTIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM3 {}
impl LPTIM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_9800 as *const _
    }
}
impl Deref for LPTIM3 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM3::ptr() }
    }
}
#[doc = "Low power timer"]
pub struct SEC_LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_LPTIM1 {}
impl SEC_LPTIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5000_7c00 as *const _
    }
}
impl Deref for SEC_LPTIM1 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_LPTIM1::ptr() }
    }
}
#[doc = "Low power timer"]
pub struct SEC_LPTIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_LPTIM2 {}
impl SEC_LPTIM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5000_9400 as *const _
    }
}
impl Deref for SEC_LPTIM2 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_LPTIM2::ptr() }
    }
}
#[doc = "Low power timer"]
pub struct SEC_LPTIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_LPTIM3 {}
impl SEC_LPTIM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5000_9800 as *const _
    }
}
impl Deref for SEC_LPTIM3 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_LPTIM3::ptr() }
    }
}
#[doc = "GTZC_MPCBB1"]
pub struct GTZC_MPCBB1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GTZC_MPCBB1 {}
impl GTZC_MPCBB1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gtzc_mpcbb1::RegisterBlock {
        0x4003_2c00 as *const _
    }
}
impl Deref for GTZC_MPCBB1 {
    type Target = gtzc_mpcbb1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GTZC_MPCBB1::ptr() }
    }
}
#[doc = "GTZC_MPCBB1"]
pub mod gtzc_mpcbb1;
#[doc = "GTZC_MPCBB2"]
pub struct GTZC_MPCBB2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GTZC_MPCBB2 {}
impl GTZC_MPCBB2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gtzc_mpcbb2::RegisterBlock {
        0x4003_3000 as *const _
    }
}
impl Deref for GTZC_MPCBB2 {
    type Target = gtzc_mpcbb2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GTZC_MPCBB2::ptr() }
    }
}
#[doc = "GTZC_MPCBB2"]
pub mod gtzc_mpcbb2;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "Power control"]
pub struct SEC_PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_PWR {}
impl SEC_PWR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        0x5000_7000 as *const _
    }
}
impl Deref for SEC_PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_PWR::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "Reset and clock control"]
pub struct SEC_RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_RCC {}
impl SEC_RCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        0x5002_1000 as *const _
    }
}
impl Deref for SEC_RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_RCC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4000_2800 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Real-time clock"]
pub struct SEC_RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_RTC {}
impl SEC_RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x5000_2800 as *const _
    }
}
impl Deref for SEC_RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_RTC::ptr() }
    }
}
#[doc = "Serial audio interface"]
pub struct SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI1 {}
impl SAI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4001_5400 as *const _
    }
}
impl Deref for SAI1 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI1::ptr() }
    }
}
#[doc = "Serial audio interface"]
pub mod sai1;
#[doc = "Serial audio interface"]
pub struct SAI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI2 {}
impl SAI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4001_5800 as *const _
    }
}
impl Deref for SAI2 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI2::ptr() }
    }
}
#[doc = "Serial audio interface"]
pub struct SEC_SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_SAI1 {}
impl SEC_SAI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x5001_5400 as *const _
    }
}
impl Deref for SEC_SAI1 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_SAI1::ptr() }
    }
}
#[doc = "Serial audio interface"]
pub struct SEC_SAI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_SAI2 {}
impl SEC_SAI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x5001_5800 as *const _
    }
}
impl Deref for SEC_SAI2 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_SAI2::ptr() }
    }
}
#[doc = "Direct memory access controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "Direct memory access controller"]
pub mod dma1;
#[doc = "Direct memory access controller"]
pub struct SEC_DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_DMA1 {}
impl SEC_DMA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x5002_0000 as *const _
    }
}
impl Deref for SEC_DMA1 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_DMA1::ptr() }
    }
}
#[doc = "Direct memory access controller"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4002_0400 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "Direct memory access controller"]
pub struct SEC_DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_DMA2 {}
impl SEC_DMA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x5002_0400 as *const _
    }
}
impl Deref for SEC_DMA2 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_DMA2::ptr() }
    }
}
#[doc = "SEC_GTZC_MPCBB1"]
pub struct SEC_GTZC_MPCBB1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GTZC_MPCBB1 {}
impl SEC_GTZC_MPCBB1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sec_gtzc_mpcbb1::RegisterBlock {
        0x5003_2c00 as *const _
    }
}
impl Deref for SEC_GTZC_MPCBB1 {
    type Target = sec_gtzc_mpcbb1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GTZC_MPCBB1::ptr() }
    }
}
#[doc = "SEC_GTZC_MPCBB1"]
pub mod sec_gtzc_mpcbb1;
#[doc = "SEC_GTZC_MPCBB2"]
pub struct SEC_GTZC_MPCBB2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GTZC_MPCBB2 {}
impl SEC_GTZC_MPCBB2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sec_gtzc_mpcbb2::RegisterBlock {
        0x5003_3000 as *const _
    }
}
impl Deref for SEC_GTZC_MPCBB2 {
    type Target = sec_gtzc_mpcbb2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GTZC_MPCBB2::ptr() }
    }
}
#[doc = "SEC_GTZC_MPCBB2"]
pub mod sec_gtzc_mpcbb2;
#[doc = "Serial peripheral interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi1;
#[doc = "Serial peripheral interface"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_3800 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_3c00 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub struct SEC_SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_SPI1 {}
impl SEC_SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x5001_3000 as *const _
    }
}
impl Deref for SEC_SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub struct SEC_SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_SPI2 {}
impl SEC_SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x5000_3800 as *const _
    }
}
impl Deref for SEC_SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_SPI2::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub struct SEC_SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_SPI3 {}
impl SEC_SPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x5000_3c00 as *const _
    }
}
impl Deref for SEC_SPI3 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_SPI3::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4001_2c00 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "Advanced-timers"]
pub struct SEC_TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM1 {}
impl SEC_TIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x5001_2c00 as *const _
    }
}
impl Deref for SEC_TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM1::ptr() }
    }
}
#[doc = "General purpose timers"]
pub struct TIM15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM15 {}
impl TIM15 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim15::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for TIM15 {
    type Target = tim15::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM15::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim15;
#[doc = "General purpose timers"]
pub struct SEC_TIM15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM15 {}
impl SEC_TIM15 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim15::RegisterBlock {
        0x5001_4000 as *const _
    }
}
impl Deref for SEC_TIM15 {
    type Target = tim15::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM15::ptr() }
    }
}
#[doc = "General purpose timers"]
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x4001_4400 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM16::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim16;
#[doc = "General purpose timers"]
pub struct SEC_TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM16 {}
impl SEC_TIM16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x5001_4400 as *const _
    }
}
impl Deref for SEC_TIM16 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM16::ptr() }
    }
}
#[doc = "General purpose timers"]
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim17::RegisterBlock {
        0x4001_4800 as *const _
    }
}
impl Deref for TIM17 {
    type Target = tim17::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM17::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim17;
#[doc = "General purpose timers"]
pub struct SEC_TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM17 {}
impl SEC_TIM17 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim17::RegisterBlock {
        0x5001_4800 as *const _
    }
}
impl Deref for SEC_TIM17 {
    type Target = tim17::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM17::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim2;
#[doc = "General-purpose-timers"]
pub struct SEC_TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM2 {}
impl SEC_TIM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for SEC_TIM2 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM2::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim3::RegisterBlock {
        0x4000_0400 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim3;
#[doc = "General-purpose-timers"]
pub struct SEC_TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM3 {}
impl SEC_TIM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim3::RegisterBlock {
        0x5000_0400 as *const _
    }
}
impl Deref for SEC_TIM3 {
    type Target = tim3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM3::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim4::RegisterBlock {
        0x4000_0800 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM4::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim4;
#[doc = "General-purpose-timers"]
pub struct SEC_TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM4 {}
impl SEC_TIM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim4::RegisterBlock {
        0x5000_0800 as *const _
    }
}
impl Deref for SEC_TIM4 {
    type Target = tim4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM4::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim4::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct SEC_TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM5 {}
impl SEC_TIM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim4::RegisterBlock {
        0x5000_0c00 as *const _
    }
}
impl Deref for SEC_TIM5 {
    type Target = tim4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM5::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim6;
#[doc = "General-purpose-timers"]
pub struct SEC_TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM6 {}
impl SEC_TIM6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x5000_1000 as *const _
    }
}
impl Deref for SEC_TIM6 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM6::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim7::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim7::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim7;
#[doc = "General-purpose-timers"]
pub struct SEC_TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM7 {}
impl SEC_TIM7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim7::RegisterBlock {
        0x5000_1400 as *const _
    }
}
impl Deref for SEC_TIM7 {
    type Target = tim7::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM7::ptr() }
    }
}
#[doc = "DAC"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4000_7400 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "DAC"]
pub mod dac;
#[doc = "DAC"]
pub struct SEC_DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_DAC {}
impl SEC_DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x5000_7400 as *const _
    }
}
impl Deref for SEC_DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_DAC::ptr() }
    }
}
#[doc = "Operational amplifiers"]
pub struct OPAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OPAMP {}
impl OPAMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const opamp::RegisterBlock {
        0x4000_7800 as *const _
    }
}
impl Deref for OPAMP {
    type Target = opamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OPAMP::ptr() }
    }
}
#[doc = "Operational amplifiers"]
pub mod opamp;
#[doc = "Operational amplifiers"]
pub struct SEC_OPAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_OPAMP {}
impl SEC_OPAMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const opamp::RegisterBlock {
        0x5000_7800 as *const _
    }
}
impl Deref for SEC_OPAMP {
    type Target = opamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_OPAMP::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim8::RegisterBlock {
        0x4001_3400 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim8::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM8::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim8;
#[doc = "Advanced-timers"]
pub struct SEC_TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TIM8 {}
impl SEC_TIM8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim8::RegisterBlock {
        0x5001_3400 as *const _
    }
}
impl Deref for SEC_TIM8 {
    type Target = tim8::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TIM8::ptr() }
    }
}
#[doc = "GTZC_TZIC"]
pub struct GTZC_TZIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GTZC_TZIC {}
impl GTZC_TZIC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gtzc_tzic::RegisterBlock {
        0x4003_2800 as *const _
    }
}
impl Deref for GTZC_TZIC {
    type Target = gtzc_tzic::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GTZC_TZIC::ptr() }
    }
}
#[doc = "GTZC_TZIC"]
pub mod gtzc_tzic;
#[doc = "GTZC_TZIC"]
pub struct SEC_GTZC_TZIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GTZC_TZIC {}
impl SEC_GTZC_TZIC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gtzc_tzic::RegisterBlock {
        0x5003_2800 as *const _
    }
}
impl Deref for SEC_GTZC_TZIC {
    type Target = gtzc_tzic::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GTZC_TZIC::ptr() }
    }
}
#[doc = "GTZC_TZSC"]
pub struct GTZC_TZSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GTZC_TZSC {}
impl GTZC_TZSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gtzc_tzsc::RegisterBlock {
        0x4003_2400 as *const _
    }
}
impl Deref for GTZC_TZSC {
    type Target = gtzc_tzsc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GTZC_TZSC::ptr() }
    }
}
#[doc = "GTZC_TZSC"]
pub mod gtzc_tzsc;
#[doc = "GTZC_TZSC"]
pub struct SEC_GTZC_TZSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_GTZC_TZSC {}
impl SEC_GTZC_TZSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gtzc_tzsc::RegisterBlock {
        0x5003_2400 as *const _
    }
}
impl Deref for SEC_GTZC_TZSC {
    type Target = gtzc_tzsc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_GTZC_TZSC::ptr() }
    }
}
#[doc = "System window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdg::RegisterBlock {
        0x4000_2c00 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "System window watchdog"]
pub mod wwdg;
#[doc = "System window watchdog"]
pub struct SEC_WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_WWDG {}
impl SEC_WWDG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdg::RegisterBlock {
        0x5000_2c00 as *const _
    }
}
impl Deref for SEC_WWDG {
    type Target = wwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_WWDG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "System configuration controller"]
pub struct SEC_SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_SYSCFG {}
impl SEC_SYSCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x5001_0000 as *const _
    }
}
impl Deref for SEC_SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_SYSCFG::ptr() }
    }
}
#[doc = "MCU debug component"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dbgmcu::RegisterBlock {
        0xe004_4000 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "MCU debug component"]
pub mod dbgmcu;
#[doc = "Universal serial bus full-speed device interface"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x4000_d400 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "Universal serial bus full-speed device interface"]
pub mod usb;
#[doc = "Universal serial bus full-speed device interface"]
pub struct SEC_USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_USB {}
impl SEC_USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x5000_d400 as *const _
    }
}
impl Deref for SEC_USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_USB::ptr() }
    }
}
#[doc = "OctoSPI"]
pub struct OCTOSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCTOSPI1 {}
impl OCTOSPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const octospi1::RegisterBlock {
        0x4402_1000 as *const _
    }
}
impl Deref for OCTOSPI1 {
    type Target = octospi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCTOSPI1::ptr() }
    }
}
#[doc = "OctoSPI"]
pub mod octospi1;
#[doc = "OctoSPI"]
pub struct SEC_OCTOSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_OCTOSPI1 {}
impl SEC_OCTOSPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const octospi1::RegisterBlock {
        0x5402_1000 as *const _
    }
}
impl Deref for SEC_OCTOSPI1 {
    type Target = octospi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_OCTOSPI1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for LPUART1 {
    type Target = lpuart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod lpuart1;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct SEC_LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_LPUART1 {}
impl SEC_LPUART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for SEC_LPUART1 {
    type Target = lpuart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_LPUART1::ptr() }
    }
}
#[doc = "Comparator"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp::RegisterBlock {
        0x4001_0200 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Comparator"]
pub mod comp;
#[doc = "Comparator"]
pub struct SEC_COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_COMP {}
impl SEC_COMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp::RegisterBlock {
        0x5001_0200 as *const _
    }
}
impl Deref for SEC_COMP {
    type Target = comp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_COMP::ptr() }
    }
}
#[doc = "Voltage reference buffer"]
pub struct VREFBUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREFBUF {}
impl VREFBUF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vrefbuf::RegisterBlock {
        0x4001_0030 as *const _
    }
}
impl Deref for VREFBUF {
    type Target = vrefbuf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VREFBUF::ptr() }
    }
}
#[doc = "Voltage reference buffer"]
pub mod vrefbuf;
#[doc = "Voltage reference buffer"]
pub struct SEC_VREFBUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_VREFBUF {}
impl SEC_VREFBUF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vrefbuf::RegisterBlock {
        0x5001_0030 as *const _
    }
}
impl Deref for SEC_VREFBUF {
    type Target = vrefbuf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_VREFBUF::ptr() }
    }
}
#[doc = "Touch sensing controller"]
pub struct TSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSC {}
impl TSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tsc::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for TSC {
    type Target = tsc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TSC::ptr() }
    }
}
#[doc = "Touch sensing controller"]
pub mod tsc;
#[doc = "Touch sensing controller"]
pub struct SEC_TSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_TSC {}
impl SEC_TSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tsc::RegisterBlock {
        0x5002_4000 as *const _
    }
}
impl Deref for SEC_TSC {
    type Target = tsc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_TSC::ptr() }
    }
}
#[doc = "USB Power Delivery interface"]
pub struct UCPD1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UCPD1 {}
impl UCPD1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ucpd1::RegisterBlock {
        0x4000_dc00 as *const _
    }
}
impl Deref for UCPD1 {
    type Target = ucpd1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UCPD1::ptr() }
    }
}
#[doc = "USB Power Delivery interface"]
pub mod ucpd1;
#[doc = "USB Power Delivery interface"]
pub struct SEC_UCPD1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_UCPD1 {}
impl SEC_UCPD1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ucpd1::RegisterBlock {
        0x5000_dc00 as *const _
    }
}
impl Deref for SEC_UCPD1 {
    type Target = ucpd1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_UCPD1::ptr() }
    }
}
#[doc = "FDCAN1"]
pub struct FDCAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN1 {}
impl FDCAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fdcan1::RegisterBlock {
        0x4000_a400 as *const _
    }
}
impl Deref for FDCAN1 {
    type Target = fdcan1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FDCAN1::ptr() }
    }
}
#[doc = "FDCAN1"]
pub mod fdcan1;
#[doc = "FDCAN1"]
pub struct SEC_FDCAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_FDCAN1 {}
impl SEC_FDCAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fdcan1::RegisterBlock {
        0x5000_a400 as *const _
    }
}
impl Deref for SEC_FDCAN1 {
    type Target = fdcan1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_FDCAN1::ptr() }
    }
}
#[doc = "Cyclic redundancy check calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4002_3000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic redundancy check calculation unit"]
pub mod crc;
#[doc = "Cyclic redundancy check calculation unit"]
pub struct SEC_CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_CRC {}
impl SEC_CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x5002_3000 as *const _
    }
}
impl Deref for SEC_CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_CRC::ptr() }
    }
}
#[doc = "Clock recovery system"]
pub struct CRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRS {}
impl CRS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crs::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for CRS {
    type Target = crs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRS::ptr() }
    }
}
#[doc = "Clock recovery system"]
pub mod crs;
#[doc = "Clock recovery system"]
pub struct SEC_CRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_CRS {}
impl SEC_CRS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crs::RegisterBlock {
        0x5000_6000 as *const _
    }
}
impl Deref for SEC_CRS {
    type Target = crs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_CRS::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_3800 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct SEC_USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_USART1 {}
impl SEC_USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x5001_3800 as *const _
    }
}
impl Deref for SEC_USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4400 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct SEC_USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_USART2 {}
impl SEC_USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x5000_4400 as *const _
    }
}
impl Deref for SEC_USART2 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_USART2::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4800 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct SEC_USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_USART3 {}
impl SEC_USART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x5000_4800 as *const _
    }
}
impl Deref for SEC_USART3 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_USART3::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_4c00 as *const _
    }
}
impl Deref for UART4 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for UART5 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART5::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct SEC_UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_UART4 {}
impl SEC_UART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x5000_4c00 as *const _
    }
}
impl Deref for SEC_UART4 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_UART4::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct SEC_UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_UART5 {}
impl SEC_UART5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x5000_5000 as *const _
    }
}
impl Deref for SEC_UART5 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_UART5::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub struct ADC_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC_COMMON {}
impl ADC_COMMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc_common::RegisterBlock {
        0x4202_8300 as *const _
    }
}
impl Deref for ADC_COMMON {
    type Target = adc_common::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC_COMMON::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc_common;
#[doc = "Analog-to-Digital Converter"]
pub struct SEC_ADC_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_ADC_COMMON {}
impl SEC_ADC_COMMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc_common::RegisterBlock {
        0x5202_8300 as *const _
    }
}
impl Deref for SEC_ADC_COMMON {
    type Target = adc_common::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_ADC_COMMON::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4202_8000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc;
#[doc = "Analog-to-Digital Converter"]
pub struct SEC_ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_ADC {}
impl SEC_ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x5202_8000 as *const _
    }
}
impl Deref for SEC_ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_ADC::ptr() }
    }
}
#[doc = "Nested vectored interrupt controller"]
pub struct NVIC_STIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVIC_STIR {}
impl NVIC_STIR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvic_stir::RegisterBlock {
        0xe000_ef00 as *const _
    }
}
impl Deref for NVIC_STIR {
    type Target = nvic_stir::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVIC_STIR::ptr() }
    }
}
#[doc = "Nested vectored interrupt controller"]
pub mod nvic_stir;
#[doc = "FMC"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x4402_0000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "FMC"]
pub mod fmc;
#[doc = "FMC"]
pub struct SEC_FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_FMC {}
impl SEC_FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x5402_0000 as *const _
    }
}
impl Deref for SEC_FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_FMC::ptr() }
    }
}
#[doc = "RNG"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x420c_0800 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "RNG"]
pub mod rng;
#[doc = "RNG"]
pub struct SEC_RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_RNG {}
impl SEC_RNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x520c_0800 as *const _
    }
}
impl Deref for SEC_RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_RNG::ptr() }
    }
}
#[doc = "SDMMC1"]
pub struct SDMMC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC1 {}
impl SDMMC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc1::RegisterBlock {
        0x420c_8000 as *const _
    }
}
impl Deref for SDMMC1 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC1::ptr() }
    }
}
#[doc = "SDMMC1"]
pub mod sdmmc1;
#[doc = "SDMMC1"]
pub struct SEC_SDMMC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SEC_SDMMC1 {}
impl SEC_SDMMC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc1::RegisterBlock {
        0x520c_8000 as *const _
    }
}
impl Deref for SEC_SDMMC1 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SEC_SDMMC1::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "DFSDM1"]
    pub DFSDM1: DFSDM1,
    #[doc = "SEC_DFSDM1"]
    pub SEC_DFSDM1: SEC_DFSDM1,
    #[doc = "DMAMUX1"]
    pub DMAMUX1: DMAMUX1,
    #[doc = "SEC_DMAMUX1"]
    pub SEC_DMAMUX1: SEC_DMAMUX1,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "SEC_EXTI"]
    pub SEC_EXTI: SEC_EXTI,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "SEC_FLASH"]
    pub SEC_FLASH: SEC_FLASH,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "SEC_GPIOA"]
    pub SEC_GPIOA: SEC_GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "SEC_GPIOB"]
    pub SEC_GPIOB: SEC_GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
    #[doc = "SEC_GPIOC"]
    pub SEC_GPIOC: SEC_GPIOC,
    #[doc = "SEC_GPIOD"]
    pub SEC_GPIOD: SEC_GPIOD,
    #[doc = "SEC_GPIOE"]
    pub SEC_GPIOE: SEC_GPIOE,
    #[doc = "SEC_GPIOF"]
    pub SEC_GPIOF: SEC_GPIOF,
    #[doc = "SEC_GPIOG"]
    pub SEC_GPIOG: SEC_GPIOG,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "SEC_GPIOH"]
    pub SEC_GPIOH: SEC_GPIOH,
    #[doc = "TAMP"]
    pub TAMP: TAMP,
    #[doc = "SEC_TAMP"]
    pub SEC_TAMP: SEC_TAMP,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "SEC_I2C1"]
    pub SEC_I2C1: SEC_I2C1,
    #[doc = "SEC_I2C2"]
    pub SEC_I2C2: SEC_I2C2,
    #[doc = "SEC_I2C3"]
    pub SEC_I2C3: SEC_I2C3,
    #[doc = "SEC_I2C4"]
    pub SEC_I2C4: SEC_I2C4,
    #[doc = "ICACHE"]
    pub ICACHE: ICACHE,
    #[doc = "SEC_ICACHE"]
    pub SEC_ICACHE: SEC_ICACHE,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "SEC_IWDG"]
    pub SEC_IWDG: SEC_IWDG,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "LPTIM2"]
    pub LPTIM2: LPTIM2,
    #[doc = "LPTIM3"]
    pub LPTIM3: LPTIM3,
    #[doc = "SEC_LPTIM1"]
    pub SEC_LPTIM1: SEC_LPTIM1,
    #[doc = "SEC_LPTIM2"]
    pub SEC_LPTIM2: SEC_LPTIM2,
    #[doc = "SEC_LPTIM3"]
    pub SEC_LPTIM3: SEC_LPTIM3,
    #[doc = "GTZC_MPCBB1"]
    pub GTZC_MPCBB1: GTZC_MPCBB1,
    #[doc = "GTZC_MPCBB2"]
    pub GTZC_MPCBB2: GTZC_MPCBB2,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "SEC_PWR"]
    pub SEC_PWR: SEC_PWR,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "SEC_RCC"]
    pub SEC_RCC: SEC_RCC,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SEC_RTC"]
    pub SEC_RTC: SEC_RTC,
    #[doc = "SAI1"]
    pub SAI1: SAI1,
    #[doc = "SAI2"]
    pub SAI2: SAI2,
    #[doc = "SEC_SAI1"]
    pub SEC_SAI1: SEC_SAI1,
    #[doc = "SEC_SAI2"]
    pub SEC_SAI2: SEC_SAI2,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "SEC_DMA1"]
    pub SEC_DMA1: SEC_DMA1,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "SEC_DMA2"]
    pub SEC_DMA2: SEC_DMA2,
    #[doc = "SEC_GTZC_MPCBB1"]
    pub SEC_GTZC_MPCBB1: SEC_GTZC_MPCBB1,
    #[doc = "SEC_GTZC_MPCBB2"]
    pub SEC_GTZC_MPCBB2: SEC_GTZC_MPCBB2,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "SEC_SPI1"]
    pub SEC_SPI1: SEC_SPI1,
    #[doc = "SEC_SPI2"]
    pub SEC_SPI2: SEC_SPI2,
    #[doc = "SEC_SPI3"]
    pub SEC_SPI3: SEC_SPI3,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "SEC_TIM1"]
    pub SEC_TIM1: SEC_TIM1,
    #[doc = "TIM15"]
    pub TIM15: TIM15,
    #[doc = "SEC_TIM15"]
    pub SEC_TIM15: SEC_TIM15,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "SEC_TIM16"]
    pub SEC_TIM16: SEC_TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "SEC_TIM17"]
    pub SEC_TIM17: SEC_TIM17,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "SEC_TIM2"]
    pub SEC_TIM2: SEC_TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "SEC_TIM3"]
    pub SEC_TIM3: SEC_TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "SEC_TIM4"]
    pub SEC_TIM4: SEC_TIM4,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "SEC_TIM5"]
    pub SEC_TIM5: SEC_TIM5,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "SEC_TIM6"]
    pub SEC_TIM6: SEC_TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "SEC_TIM7"]
    pub SEC_TIM7: SEC_TIM7,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "SEC_DAC"]
    pub SEC_DAC: SEC_DAC,
    #[doc = "OPAMP"]
    pub OPAMP: OPAMP,
    #[doc = "SEC_OPAMP"]
    pub SEC_OPAMP: SEC_OPAMP,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "SEC_TIM8"]
    pub SEC_TIM8: SEC_TIM8,
    #[doc = "GTZC_TZIC"]
    pub GTZC_TZIC: GTZC_TZIC,
    #[doc = "SEC_GTZC_TZIC"]
    pub SEC_GTZC_TZIC: SEC_GTZC_TZIC,
    #[doc = "GTZC_TZSC"]
    pub GTZC_TZSC: GTZC_TZSC,
    #[doc = "SEC_GTZC_TZSC"]
    pub SEC_GTZC_TZSC: SEC_GTZC_TZSC,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "SEC_WWDG"]
    pub SEC_WWDG: SEC_WWDG,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "SEC_SYSCFG"]
    pub SEC_SYSCFG: SEC_SYSCFG,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "SEC_USB"]
    pub SEC_USB: SEC_USB,
    #[doc = "OCTOSPI1"]
    pub OCTOSPI1: OCTOSPI1,
    #[doc = "SEC_OCTOSPI1"]
    pub SEC_OCTOSPI1: SEC_OCTOSPI1,
    #[doc = "LPUART1"]
    pub LPUART1: LPUART1,
    #[doc = "SEC_LPUART1"]
    pub SEC_LPUART1: SEC_LPUART1,
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "SEC_COMP"]
    pub SEC_COMP: SEC_COMP,
    #[doc = "VREFBUF"]
    pub VREFBUF: VREFBUF,
    #[doc = "SEC_VREFBUF"]
    pub SEC_VREFBUF: SEC_VREFBUF,
    #[doc = "TSC"]
    pub TSC: TSC,
    #[doc = "SEC_TSC"]
    pub SEC_TSC: SEC_TSC,
    #[doc = "UCPD1"]
    pub UCPD1: UCPD1,
    #[doc = "SEC_UCPD1"]
    pub SEC_UCPD1: SEC_UCPD1,
    #[doc = "FDCAN1"]
    pub FDCAN1: FDCAN1,
    #[doc = "SEC_FDCAN1"]
    pub SEC_FDCAN1: SEC_FDCAN1,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "SEC_CRC"]
    pub SEC_CRC: SEC_CRC,
    #[doc = "CRS"]
    pub CRS: CRS,
    #[doc = "SEC_CRS"]
    pub SEC_CRS: SEC_CRS,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "SEC_USART1"]
    pub SEC_USART1: SEC_USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "SEC_USART2"]
    pub SEC_USART2: SEC_USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "SEC_USART3"]
    pub SEC_USART3: SEC_USART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "SEC_UART4"]
    pub SEC_UART4: SEC_UART4,
    #[doc = "SEC_UART5"]
    pub SEC_UART5: SEC_UART5,
    #[doc = "ADC_COMMON"]
    pub ADC_COMMON: ADC_COMMON,
    #[doc = "SEC_ADC_COMMON"]
    pub SEC_ADC_COMMON: SEC_ADC_COMMON,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "SEC_ADC"]
    pub SEC_ADC: SEC_ADC,
    #[doc = "NVIC_STIR"]
    pub NVIC_STIR: NVIC_STIR,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "SEC_FMC"]
    pub SEC_FMC: SEC_FMC,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "SEC_RNG"]
    pub SEC_RNG: SEC_RNG,
    #[doc = "SDMMC1"]
    pub SDMMC1: SDMMC1,
    #[doc = "SEC_SDMMC1"]
    pub SEC_SDMMC1: SEC_SDMMC1,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            DFSDM1: DFSDM1 {
                _marker: PhantomData,
            },
            SEC_DFSDM1: SEC_DFSDM1 {
                _marker: PhantomData,
            },
            DMAMUX1: DMAMUX1 {
                _marker: PhantomData,
            },
            SEC_DMAMUX1: SEC_DMAMUX1 {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            SEC_EXTI: SEC_EXTI {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            SEC_FLASH: SEC_FLASH {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            SEC_GPIOA: SEC_GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            SEC_GPIOB: SEC_GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            SEC_GPIOC: SEC_GPIOC {
                _marker: PhantomData,
            },
            SEC_GPIOD: SEC_GPIOD {
                _marker: PhantomData,
            },
            SEC_GPIOE: SEC_GPIOE {
                _marker: PhantomData,
            },
            SEC_GPIOF: SEC_GPIOF {
                _marker: PhantomData,
            },
            SEC_GPIOG: SEC_GPIOG {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            SEC_GPIOH: SEC_GPIOH {
                _marker: PhantomData,
            },
            TAMP: TAMP {
                _marker: PhantomData,
            },
            SEC_TAMP: SEC_TAMP {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C4: I2C4 {
                _marker: PhantomData,
            },
            SEC_I2C1: SEC_I2C1 {
                _marker: PhantomData,
            },
            SEC_I2C2: SEC_I2C2 {
                _marker: PhantomData,
            },
            SEC_I2C3: SEC_I2C3 {
                _marker: PhantomData,
            },
            SEC_I2C4: SEC_I2C4 {
                _marker: PhantomData,
            },
            ICACHE: ICACHE {
                _marker: PhantomData,
            },
            SEC_ICACHE: SEC_ICACHE {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            SEC_IWDG: SEC_IWDG {
                _marker: PhantomData,
            },
            LPTIM1: LPTIM1 {
                _marker: PhantomData,
            },
            LPTIM2: LPTIM2 {
                _marker: PhantomData,
            },
            LPTIM3: LPTIM3 {
                _marker: PhantomData,
            },
            SEC_LPTIM1: SEC_LPTIM1 {
                _marker: PhantomData,
            },
            SEC_LPTIM2: SEC_LPTIM2 {
                _marker: PhantomData,
            },
            SEC_LPTIM3: SEC_LPTIM3 {
                _marker: PhantomData,
            },
            GTZC_MPCBB1: GTZC_MPCBB1 {
                _marker: PhantomData,
            },
            GTZC_MPCBB2: GTZC_MPCBB2 {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            SEC_PWR: SEC_PWR {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            SEC_RCC: SEC_RCC {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SEC_RTC: SEC_RTC {
                _marker: PhantomData,
            },
            SAI1: SAI1 {
                _marker: PhantomData,
            },
            SAI2: SAI2 {
                _marker: PhantomData,
            },
            SEC_SAI1: SEC_SAI1 {
                _marker: PhantomData,
            },
            SEC_SAI2: SEC_SAI2 {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            SEC_DMA1: SEC_DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            SEC_DMA2: SEC_DMA2 {
                _marker: PhantomData,
            },
            SEC_GTZC_MPCBB1: SEC_GTZC_MPCBB1 {
                _marker: PhantomData,
            },
            SEC_GTZC_MPCBB2: SEC_GTZC_MPCBB2 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            SEC_SPI1: SEC_SPI1 {
                _marker: PhantomData,
            },
            SEC_SPI2: SEC_SPI2 {
                _marker: PhantomData,
            },
            SEC_SPI3: SEC_SPI3 {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            SEC_TIM1: SEC_TIM1 {
                _marker: PhantomData,
            },
            TIM15: TIM15 {
                _marker: PhantomData,
            },
            SEC_TIM15: SEC_TIM15 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            SEC_TIM16: SEC_TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            SEC_TIM17: SEC_TIM17 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            SEC_TIM2: SEC_TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            SEC_TIM3: SEC_TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            SEC_TIM4: SEC_TIM4 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            SEC_TIM5: SEC_TIM5 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            SEC_TIM6: SEC_TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            SEC_TIM7: SEC_TIM7 {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            SEC_DAC: SEC_DAC {
                _marker: PhantomData,
            },
            OPAMP: OPAMP {
                _marker: PhantomData,
            },
            SEC_OPAMP: SEC_OPAMP {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            SEC_TIM8: SEC_TIM8 {
                _marker: PhantomData,
            },
            GTZC_TZIC: GTZC_TZIC {
                _marker: PhantomData,
            },
            SEC_GTZC_TZIC: SEC_GTZC_TZIC {
                _marker: PhantomData,
            },
            GTZC_TZSC: GTZC_TZSC {
                _marker: PhantomData,
            },
            SEC_GTZC_TZSC: SEC_GTZC_TZSC {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            SEC_WWDG: SEC_WWDG {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            SEC_SYSCFG: SEC_SYSCFG {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            SEC_USB: SEC_USB {
                _marker: PhantomData,
            },
            OCTOSPI1: OCTOSPI1 {
                _marker: PhantomData,
            },
            SEC_OCTOSPI1: SEC_OCTOSPI1 {
                _marker: PhantomData,
            },
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            SEC_LPUART1: SEC_LPUART1 {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            SEC_COMP: SEC_COMP {
                _marker: PhantomData,
            },
            VREFBUF: VREFBUF {
                _marker: PhantomData,
            },
            SEC_VREFBUF: SEC_VREFBUF {
                _marker: PhantomData,
            },
            TSC: TSC {
                _marker: PhantomData,
            },
            SEC_TSC: SEC_TSC {
                _marker: PhantomData,
            },
            UCPD1: UCPD1 {
                _marker: PhantomData,
            },
            SEC_UCPD1: SEC_UCPD1 {
                _marker: PhantomData,
            },
            FDCAN1: FDCAN1 {
                _marker: PhantomData,
            },
            SEC_FDCAN1: SEC_FDCAN1 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            SEC_CRC: SEC_CRC {
                _marker: PhantomData,
            },
            CRS: CRS {
                _marker: PhantomData,
            },
            SEC_CRS: SEC_CRS {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            SEC_USART1: SEC_USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            SEC_USART2: SEC_USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            SEC_USART3: SEC_USART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            SEC_UART4: SEC_UART4 {
                _marker: PhantomData,
            },
            SEC_UART5: SEC_UART5 {
                _marker: PhantomData,
            },
            ADC_COMMON: ADC_COMMON {
                _marker: PhantomData,
            },
            SEC_ADC_COMMON: SEC_ADC_COMMON {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            SEC_ADC: SEC_ADC {
                _marker: PhantomData,
            },
            NVIC_STIR: NVIC_STIR {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            SEC_FMC: SEC_FMC {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            SEC_RNG: SEC_RNG {
                _marker: PhantomData,
            },
            SDMMC1: SDMMC1 {
                _marker: PhantomData,
            },
            SEC_SDMMC1: SEC_SDMMC1 {
                _marker: PhantomData,
            },
        }
    }
}
