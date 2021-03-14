#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG1();
    fn PVD_PVM();
    fn RTC_TAMP_STAMP_CSS_LSE();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA_STR0();
    fn DMA_STR1();
    fn DMA_STR2();
    fn DMA_STR3();
    fn DMA_STR4();
    fn DMA_STR5();
    fn DMA_STR6();
    fn ADC1_2();
    fn FDCAN1_IT0();
    fn FDCAN2_IT0();
    fn FDCAN1_IT1();
    fn FDCAN2_IT1();
    fn EXTI9_5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn TIM8_BRK_TIM12();
    fn TIM8_UP_TIM13();
    fn TIM8_TRG_COM_TIM14();
    fn TIM8_CC();
    fn DMA1_STR7();
    fn FMC();
    fn SDMMC1();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DAC();
    fn TIM7();
    fn DMA2_STR0();
    fn DMA2_STR1();
    fn DMA2_STR2();
    fn DMA2_STR3();
    fn DMA2_STR4();
    fn DMA2_STR5();
    fn DMA2_STR6();
    fn DMA2_STR7();
    fn USART6();
    fn I2C3_EV();
    fn I2C3_ER();
    fn OTG_HS_EP1_OUT();
    fn OTG_HS_EP1_IN();
    fn OTG_HS_WKUP();
    fn OTG_HS();
    fn DCMI();
    fn CRYP();
    fn HASH_RNG();
    fn FPU();
    fn UART7();
    fn UART8();
    fn SPI4();
    fn SPI5();
    fn SPI6();
    fn SAI1();
    fn LTDC();
    fn LTDC_ER();
    fn DMA2D();
    fn SAI2();
    fn QUADSPI();
    fn LPTIM1();
    fn CEC();
    fn I2C4_EV();
    fn I2C4_ER();
    fn SPDIFRX();
    fn DMAMUX1_OV();
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn DFSDM1_FLT3();
    fn SWPMI1();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn MDIOS_WKUP();
    fn MDIOS();
    fn JPEG();
    fn MDMA();
    fn SDMMC();
    fn HSEM0();
    fn DAC2();
    fn DMAMUX2_OVR();
    fn BDMA_CH1();
    fn BDMA_CH2();
    fn BDMA_CH3();
    fn BDMA_CH4();
    fn BDMA_CH5();
    fn BDMA_CH6();
    fn BDMA_CH7();
    fn BDMA_CH8();
    fn COMP();
    fn LPTIM2();
    fn LPTIM3();
    fn UART9();
    fn USART10();
    fn LPUART();
    fn WWDG1_RST();
    fn CRS();
    fn RAMECC();
    fn WKUP();
    fn OCTOSPI2();
    fn OTFDEC1();
    fn OTFDEC2();
    fn BDMA1();
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
pub static __INTERRUPTS: [Vector; 155] = [
    Vector { _handler: WWDG1 },
    Vector { _handler: PVD_PVM },
    Vector {
        _handler: RTC_TAMP_STAMP_CSS_LSE,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: DMA_STR0 },
    Vector { _handler: DMA_STR1 },
    Vector { _handler: DMA_STR2 },
    Vector { _handler: DMA_STR3 },
    Vector { _handler: DMA_STR4 },
    Vector { _handler: DMA_STR5 },
    Vector { _handler: DMA_STR6 },
    Vector { _handler: ADC1_2 },
    Vector {
        _handler: FDCAN1_IT0,
    },
    Vector {
        _handler: FDCAN2_IT0,
    },
    Vector {
        _handler: FDCAN1_IT1,
    },
    Vector {
        _handler: FDCAN2_IT1,
    },
    Vector { _handler: EXTI9_5 },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: TIM8_BRK_TIM12,
    },
    Vector {
        _handler: TIM8_UP_TIM13,
    },
    Vector {
        _handler: TIM8_TRG_COM_TIM14,
    },
    Vector { _handler: TIM8_CC },
    Vector {
        _handler: DMA1_STR7,
    },
    Vector { _handler: FMC },
    Vector { _handler: SDMMC1 },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: TIM6_DAC },
    Vector { _handler: TIM7 },
    Vector {
        _handler: DMA2_STR0,
    },
    Vector {
        _handler: DMA2_STR1,
    },
    Vector {
        _handler: DMA2_STR2,
    },
    Vector {
        _handler: DMA2_STR3,
    },
    Vector {
        _handler: DMA2_STR4,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA2_STR5,
    },
    Vector {
        _handler: DMA2_STR6,
    },
    Vector {
        _handler: DMA2_STR7,
    },
    Vector { _handler: USART6 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector {
        _handler: OTG_HS_EP1_OUT,
    },
    Vector {
        _handler: OTG_HS_EP1_IN,
    },
    Vector {
        _handler: OTG_HS_WKUP,
    },
    Vector { _handler: OTG_HS },
    Vector { _handler: DCMI },
    Vector { _handler: CRYP },
    Vector { _handler: HASH_RNG },
    Vector { _handler: FPU },
    Vector { _handler: UART7 },
    Vector { _handler: UART8 },
    Vector { _handler: SPI4 },
    Vector { _handler: SPI5 },
    Vector { _handler: SPI6 },
    Vector { _handler: SAI1 },
    Vector { _handler: LTDC },
    Vector { _handler: LTDC_ER },
    Vector { _handler: DMA2D },
    Vector { _handler: SAI2 },
    Vector { _handler: QUADSPI },
    Vector { _handler: LPTIM1 },
    Vector { _handler: CEC },
    Vector { _handler: I2C4_EV },
    Vector { _handler: I2C4_ER },
    Vector { _handler: SPDIFRX },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMAMUX1_OV,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
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
    Vector { _reserved: 0 },
    Vector { _handler: SWPMI1 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector {
        _handler: MDIOS_WKUP,
    },
    Vector { _handler: MDIOS },
    Vector { _handler: JPEG },
    Vector { _handler: MDMA },
    Vector { _reserved: 0 },
    Vector { _handler: SDMMC },
    Vector { _handler: HSEM0 },
    Vector { _reserved: 0 },
    Vector { _handler: DAC2 },
    Vector {
        _handler: DMAMUX2_OVR,
    },
    Vector { _handler: BDMA_CH1 },
    Vector { _handler: BDMA_CH2 },
    Vector { _handler: BDMA_CH3 },
    Vector { _handler: BDMA_CH4 },
    Vector { _handler: BDMA_CH5 },
    Vector { _handler: BDMA_CH6 },
    Vector { _handler: BDMA_CH7 },
    Vector { _handler: BDMA_CH8 },
    Vector { _handler: COMP },
    Vector { _handler: LPTIM2 },
    Vector { _handler: LPTIM3 },
    Vector { _handler: UART9 },
    Vector { _handler: USART10 },
    Vector { _handler: LPUART },
    Vector {
        _handler: WWDG1_RST,
    },
    Vector { _handler: CRS },
    Vector { _handler: RAMECC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: WKUP },
    Vector { _handler: OCTOSPI2 },
    Vector { _handler: OTFDEC1 },
    Vector { _handler: OTFDEC2 },
    Vector { _reserved: 0 },
    Vector { _handler: BDMA1 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG1 = 0,
    #[doc = "1 - PVD through EXTI line"]
    PVD_PVM = 1,
    #[doc = "2 - RTC tamper, timestamp"]
    RTC_TAMP_STAMP_CSS_LSE = 2,
    #[doc = "3 - RTC Wakeup interrupt"]
    RTC_WKUP = 3,
    #[doc = "4 - Flash memory"]
    FLASH = 4,
    #[doc = "5 - RCC global interrupt"]
    RCC = 5,
    #[doc = "6 - EXTI Line 0 interrupt"]
    EXTI0 = 6,
    #[doc = "7 - EXTI Line 1 interrupt"]
    EXTI1 = 7,
    #[doc = "8 - EXTI Line 2 interrupt"]
    EXTI2 = 8,
    #[doc = "9 - EXTI Line 3interrupt"]
    EXTI3 = 9,
    #[doc = "10 - EXTI Line 4interrupt"]
    EXTI4 = 10,
    #[doc = "11 - DMA1 Stream0"]
    DMA_STR0 = 11,
    #[doc = "12 - DMA1 Stream1"]
    DMA_STR1 = 12,
    #[doc = "13 - DMA1 Stream2"]
    DMA_STR2 = 13,
    #[doc = "14 - DMA1 Stream3"]
    DMA_STR3 = 14,
    #[doc = "15 - DMA1 Stream4"]
    DMA_STR4 = 15,
    #[doc = "16 - DMA1 Stream5"]
    DMA_STR5 = 16,
    #[doc = "17 - DMA1 Stream6"]
    DMA_STR6 = 17,
    #[doc = "18 - ADC1 and ADC2"]
    ADC1_2 = 18,
    #[doc = "19 - FDCAN1 Interrupt 0"]
    FDCAN1_IT0 = 19,
    #[doc = "20 - FDCAN2 Interrupt 0"]
    FDCAN2_IT0 = 20,
    #[doc = "21 - FDCAN1 Interrupt 1"]
    FDCAN1_IT1 = 21,
    #[doc = "22 - FDCAN2 Interrupt 1"]
    FDCAN2_IT1 = 22,
    #[doc = "23 - EXTI Line\\[9:5\\]
interrupts"]
    EXTI9_5 = 23,
    #[doc = "24 - TIM1 break interrupt"]
    TIM1_BRK = 24,
    #[doc = "25 - TIM1 update interrupt"]
    TIM1_UP = 25,
    #[doc = "26 - TIM1 trigger and commutation"]
    TIM1_TRG_COM = 26,
    #[doc = "27 - TIM1 capture / compare"]
    TIM_CC = 27,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2 = 28,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3 = 29,
    #[doc = "30 - TIM4 global interrupt"]
    TIM4 = 30,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EV = 31,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER = 32,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EV = 33,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER = 34,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1 = 35,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2 = 36,
    #[doc = "37 - USART1 global interrupt"]
    USART1 = 37,
    #[doc = "38 - USART2 global interrupt"]
    USART2 = 38,
    #[doc = "39 - USART3 global interrupt"]
    USART3 = 39,
    #[doc = "40 - EXTI Line\\[15:10\\]
interrupts"]
    EXTI15_10 = 40,
    #[doc = "41 - RTC alarms (A and B)"]
    RTC_ALARM = 41,
    #[doc = "43 - TIM8 and 12 break global"]
    TIM8_BRK_TIM12 = 43,
    #[doc = "44 - TIM8 and 13 update global"]
    TIM8_UP_TIM13 = 44,
    #[doc = "45 - TIM8 and 14 trigger /commutation and global"]
    TIM8_TRG_COM_TIM14 = 45,
    #[doc = "46 - TIM8 capture / compare"]
    TIM8_CC = 46,
    #[doc = "47 - DMA1 Stream7"]
    DMA1_STR7 = 47,
    #[doc = "48 - FMC global interrupt"]
    FMC = 48,
    #[doc = "49 - SDMMC global interrupt"]
    SDMMC1 = 49,
    #[doc = "50 - TIM5 global interrupt"]
    TIM5 = 50,
    #[doc = "51 - SPI3 global interrupt"]
    SPI3 = 51,
    #[doc = "52 - UART4 global interrupt"]
    UART4 = 52,
    #[doc = "53 - UART5 global interrupt"]
    UART5 = 53,
    #[doc = "54 - TIM6 global interrupt"]
    TIM6_DAC = 54,
    #[doc = "55 - TIM7 global interrupt"]
    TIM7 = 55,
    #[doc = "56 - DMA2 Stream0"]
    DMA2_STR0 = 56,
    #[doc = "57 - DMA2 Stream1"]
    DMA2_STR1 = 57,
    #[doc = "58 - DMA2 Stream2"]
    DMA2_STR2 = 58,
    #[doc = "59 - DMA2 Stream3"]
    DMA2_STR3 = 59,
    #[doc = "60 - DMA2 Stream4"]
    DMA2_STR4 = 60,
    #[doc = "68 - DMA2 Stream5"]
    DMA2_STR5 = 68,
    #[doc = "69 - DMA2 Stream6"]
    DMA2_STR6 = 69,
    #[doc = "70 - DMA2 Stream7"]
    DMA2_STR7 = 70,
    #[doc = "71 - USART6 global interrupt"]
    USART6 = 71,
    #[doc = "72 - I2C3 event interrupt"]
    I2C3_EV = 72,
    #[doc = "73 - I2C3 error interrupt"]
    I2C3_ER = 73,
    #[doc = "74 - OTG_HS out global interrupt"]
    OTG_HS_EP1_OUT = 74,
    #[doc = "75 - OTG_HS in global interrupt"]
    OTG_HS_EP1_IN = 75,
    #[doc = "76 - OTG_HS wakeup interrupt"]
    OTG_HS_WKUP = 76,
    #[doc = "77 - OTG_HS global interrupt"]
    OTG_HS = 77,
    #[doc = "78 - DCMI global interrupt"]
    DCMI = 78,
    #[doc = "79 - CRYP global interrupt"]
    CRYP = 79,
    #[doc = "80 - HASH and RNG"]
    HASH_RNG = 80,
    #[doc = "81 - Floating point unit interrupt"]
    FPU = 81,
    #[doc = "82 - UART7 global interrupt"]
    UART7 = 82,
    #[doc = "83 - UART8 global interrupt"]
    UART8 = 83,
    #[doc = "84 - SPI4 global interrupt"]
    SPI4 = 84,
    #[doc = "85 - SPI5 global interrupt"]
    SPI5 = 85,
    #[doc = "86 - SPI6 global interrupt"]
    SPI6 = 86,
    #[doc = "87 - SAI1 global interrupt"]
    SAI1 = 87,
    #[doc = "88 - LCD-TFT global interrupt"]
    LTDC = 88,
    #[doc = "89 - LCD-TFT error interrupt"]
    LTDC_ER = 89,
    #[doc = "90 - DMA2D global interrupt"]
    DMA2D = 90,
    #[doc = "91 - SAI2 global interrupt"]
    SAI2 = 91,
    #[doc = "92 - QuadSPI global interrupt"]
    QUADSPI = 92,
    #[doc = "93 - LPTIM1 global interrupt"]
    LPTIM1 = 93,
    #[doc = "94 - HDMI-CEC global interrupt"]
    CEC = 94,
    #[doc = "95 - I2C4 event interrupt"]
    I2C4_EV = 95,
    #[doc = "96 - I2C4 error interrupt"]
    I2C4_ER = 96,
    #[doc = "97 - SPDIFRX global interrupt"]
    SPDIFRX = 97,
    #[doc = "102 - DMAMUX1 overrun interrupt"]
    DMAMUX1_OV = 102,
    #[doc = "110 - DFSDM1 filter 0 interrupt"]
    DFSDM1_FLT0 = 110,
    #[doc = "111 - DFSDM1 filter 1 interrupt"]
    DFSDM1_FLT1 = 111,
    #[doc = "112 - DFSDM1 filter 2 interrupt"]
    DFSDM1_FLT2 = 112,
    #[doc = "113 - DFSDM1 filter 3 interrupt"]
    DFSDM1_FLT3 = 113,
    #[doc = "115 - SWPMI global interrupt"]
    SWPMI1 = 115,
    #[doc = "116 - TIM15 global interrupt"]
    TIM15 = 116,
    #[doc = "117 - TIM16 global interrupt"]
    TIM16 = 117,
    #[doc = "118 - TIM17 global interrupt"]
    TIM17 = 118,
    #[doc = "119 - MDIOS wakeup"]
    MDIOS_WKUP = 119,
    #[doc = "120 - MDIOS global interrupt"]
    MDIOS = 120,
    #[doc = "121 - JPEG global interrupt"]
    JPEG = 121,
    #[doc = "122 - MDMA"]
    MDMA = 122,
    #[doc = "124 - SDMMC global interrupt"]
    SDMMC = 124,
    #[doc = "125 - HSEM global interrupt 1"]
    HSEM0 = 125,
    #[doc = "127 - DAC2 underrun interrupt"]
    DAC2 = 127,
    #[doc = "128 - DMAMUX2 overrun interrupt"]
    DMAMUX2_OVR = 128,
    #[doc = "129 - BDMA channel 1 interrupt"]
    BDMA_CH1 = 129,
    #[doc = "130 - BDMA channel 2 interrupt"]
    BDMA_CH2 = 130,
    #[doc = "131 - BDMA channel 3 interrupt"]
    BDMA_CH3 = 131,
    #[doc = "132 - BDMA channel 4 interrupt"]
    BDMA_CH4 = 132,
    #[doc = "133 - BDMA channel 5 interrupt"]
    BDMA_CH5 = 133,
    #[doc = "134 - BDMA channel 6 interrupt"]
    BDMA_CH6 = 134,
    #[doc = "135 - BDMA channel 7 interrupt"]
    BDMA_CH7 = 135,
    #[doc = "136 - BDMA channel 8 interrupt"]
    BDMA_CH8 = 136,
    #[doc = "137 - COMP1 and COMP2"]
    COMP = 137,
    #[doc = "138 - LPTIM2 timer interrupt"]
    LPTIM2 = 138,
    #[doc = "139 - LPTIM2 timer interrupt"]
    LPTIM3 = 139,
    #[doc = "140 - UART9 global interrupt"]
    UART9 = 140,
    #[doc = "141 - USART10 global interrupt"]
    USART10 = 141,
    #[doc = "142 - LPUART global interrupt"]
    LPUART = 142,
    #[doc = "143 - Window Watchdog interrupt"]
    WWDG1_RST = 143,
    #[doc = "144 - Clock Recovery System globa"]
    CRS = 144,
    #[doc = "145 - ECC diagnostic global interrupt"]
    RAMECC = 145,
    #[doc = "149 - WKUP1 to WKUP6 pins"]
    WKUP = 149,
    #[doc = "150 - OCTOSPI2 global interrupt"]
    OCTOSPI2 = 150,
    #[doc = "151 - OTFDEC1 interrupt"]
    OTFDEC1 = 151,
    #[doc = "152 - OTFDEC2 interrupt"]
    OTFDEC2 = 152,
    #[doc = "154 - BDMA1"]
    BDMA1 = 154,
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
#[doc = "COMP1"]
pub struct COMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP1 {}
impl COMP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp1::RegisterBlock {
        0x5800_3800 as *const _
    }
}
impl Deref for COMP1 {
    type Target = comp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP1::ptr() }
    }
}
#[doc = "COMP1"]
pub mod comp1;
#[doc = "CRS"]
pub struct CRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRS {}
impl CRS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crs::RegisterBlock {
        0x4000_8400 as *const _
    }
}
impl Deref for CRS {
    type Target = crs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRS::ptr() }
    }
}
#[doc = "CRS"]
pub mod crs;
#[doc = "DAC"]
pub struct DAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC1 {}
impl DAC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac1::RegisterBlock {
        0x4000_7400 as *const _
    }
}
impl Deref for DAC1 {
    type Target = dac1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC1::ptr() }
    }
}
#[doc = "DAC"]
pub mod dac1;
#[doc = "DMA2D"]
pub struct DMA2D {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2D {}
impl DMA2D {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma2d::RegisterBlock {
        0x5200_1000 as *const _
    }
}
impl Deref for DMA2D {
    type Target = dma2d::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2D::ptr() }
    }
}
#[doc = "DMA2D"]
pub mod dma2d;
#[doc = "DMAMUX"]
pub struct DMAMUX2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX2 {}
impl DMAMUX2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmamux2::RegisterBlock {
        0x5802_5800 as *const _
    }
}
impl Deref for DMAMUX2 {
    type Target = dmamux2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAMUX2::ptr() }
    }
}
#[doc = "DMAMUX"]
pub mod dmamux2;
#[doc = "FMC"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x5200_4000 as *const _
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
#[doc = "CEC"]
pub struct CEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CEC {}
impl CEC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cec::RegisterBlock {
        0x4000_6c00 as *const _
    }
}
impl Deref for CEC {
    type Target = cec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CEC::ptr() }
    }
}
#[doc = "CEC"]
pub mod cec;
#[doc = "HSEM"]
pub struct HSEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSEM {}
impl HSEM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsem::RegisterBlock {
        0x5802_6400 as *const _
    }
}
impl Deref for HSEM {
    type Target = hsem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HSEM::ptr() }
    }
}
#[doc = "HSEM"]
pub mod hsem;
#[doc = "GPIO"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_0000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpioa;
#[doc = "GPIO"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_0400 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_0800 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_0c00 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_1000 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_1400 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_1800 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_1c00 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIOI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOI {}
impl GPIOI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_2000 as *const _
    }
}
impl Deref for GPIOI {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOI::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIOJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOJ {}
impl GPIOJ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_2400 as *const _
    }
}
impl Deref for GPIOJ {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOJ::ptr() }
    }
}
#[doc = "GPIO"]
pub struct GPIOK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOK {}
impl GPIOK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5802_2800 as *const _
    }
}
impl Deref for GPIOK {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOK::ptr() }
    }
}
#[doc = "JPEG"]
pub struct JPEG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for JPEG {}
impl JPEG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const jpeg::RegisterBlock {
        0x5200_3000 as *const _
    }
}
impl Deref for JPEG {
    type Target = jpeg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*JPEG::ptr() }
    }
}
#[doc = "JPEG"]
pub mod jpeg;
#[doc = "MDMA"]
pub struct MDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MDMA {}
impl MDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mdma::RegisterBlock {
        0x5200_0000 as *const _
    }
}
impl Deref for MDMA {
    type Target = mdma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MDMA::ptr() }
    }
}
#[doc = "MDMA"]
pub mod mdma;
#[doc = "RNG"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x4802_1800 as *const _
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
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x5800_4000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "SAI"]
pub struct SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI1 {}
impl SAI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4001_5800 as *const _
    }
}
impl Deref for SAI1 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI1::ptr() }
    }
}
#[doc = "SAI"]
pub mod sai1;
#[doc = "SAI"]
pub struct SAI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI2 {}
impl SAI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4001_5c00 as *const _
    }
}
impl Deref for SAI2 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI2::ptr() }
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
        0x5200_7000 as *const _
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
pub struct SDMMC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC2 {}
impl SDMMC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc1::RegisterBlock {
        0x4802_2400 as *const _
    }
}
impl Deref for SDMMC2 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC2::ptr() }
    }
}
#[doc = "VREFBUF"]
pub struct VREFBUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREFBUF {}
impl VREFBUF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vrefbuf::RegisterBlock {
        0x5800_3c00 as *const _
    }
}
impl Deref for VREFBUF {
    type Target = vrefbuf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VREFBUF::ptr() }
    }
}
#[doc = "VREFBUF"]
pub mod vrefbuf;
#[doc = "IWDG"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iwdg::RegisterBlock {
        0x5800_4800 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "IWDG"]
pub mod iwdg;
#[doc = "WWDG"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdg::RegisterBlock {
        0x5000_3000 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "WWDG"]
pub mod wwdg;
#[doc = "PWR"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        0x5802_4800 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "PWR"]
pub mod pwr;
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
pub struct SPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI4 {}
impl SPI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_3400 as *const _
    }
}
impl Deref for SPI4 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI4::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub struct SPI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI5 {}
impl SPI5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for SPI5 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI5::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub struct SPI6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI6 {}
impl SPI6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x5800_1400 as *const _
    }
}
impl Deref for SPI6 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI6::ptr() }
    }
}
#[doc = "LCD-TFT Controller"]
pub struct LTDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LTDC {}
impl LTDC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ltdc::RegisterBlock {
        0x5000_1000 as *const _
    }
}
impl Deref for LTDC {
    type Target = ltdc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LTDC::ptr() }
    }
}
#[doc = "LCD-TFT Controller"]
pub mod ltdc;
#[doc = "Receiver Interface"]
pub struct SPDIFRX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPDIFRX {}
impl SPDIFRX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spdifrx::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SPDIFRX {
    type Target = spdifrx::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPDIFRX::ptr() }
    }
}
#[doc = "Receiver Interface"]
pub mod spdifrx;
#[doc = "DMAMUX"]
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
#[doc = "DMAMUX"]
pub mod dmamux1;
#[doc = "Cryptographic processor"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x5802_4c00 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cryptographic processor"]
pub mod crc;
#[doc = "Low power timer"]
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_2400 as *const _
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
        0x5800_2400 as *const _
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
    pub const fn ptr() -> *const lptim3::RegisterBlock {
        0x5800_2800 as *const _
    }
}
impl Deref for LPTIM3 {
    type Target = lptim3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM3::ptr() }
    }
}
#[doc = "Low power timer"]
pub mod lptim3;
#[doc = "LPUART1"]
pub struct LPUART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART1 {}
impl LPUART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart1::RegisterBlock {
        0x5800_0c00 as *const _
    }
}
impl Deref for LPUART1 {
    type Target = lpuart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART1::ptr() }
    }
}
#[doc = "LPUART1"]
pub mod lpuart1;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x5800_0000 as *const _
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
#[doc = "DELAY_Block_SDMMC1"]
pub struct DELAY_BLOCK_SDMMC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DELAY_BLOCK_SDMMC1 {}
impl DELAY_BLOCK_SDMMC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const delay_block_sdmmc1::RegisterBlock {
        0x5200_8000 as *const _
    }
}
impl Deref for DELAY_BLOCK_SDMMC1 {
    type Target = delay_block_sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DELAY_BLOCK_SDMMC1::ptr() }
    }
}
#[doc = "DELAY_Block_SDMMC1"]
pub mod delay_block_sdmmc1;
#[doc = "DELAY_Block_SDMMC1"]
pub struct DELAY_BLOCK_SDMMC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DELAY_BLOCK_SDMMC2 {}
impl DELAY_BLOCK_SDMMC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const delay_block_sdmmc1::RegisterBlock {
        0x4802_2800 as *const _
    }
}
impl Deref for DELAY_BLOCK_SDMMC2 {
    type Target = delay_block_sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DELAY_BLOCK_SDMMC2::ptr() }
    }
}
#[doc = "DELAY_Block_SDMMC1"]
pub struct DELAY_BLOCK_OCTOSPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DELAY_BLOCK_OCTOSPI1 {}
impl DELAY_BLOCK_OCTOSPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const delay_block_sdmmc1::RegisterBlock {
        0x5200_6000 as *const _
    }
}
impl Deref for DELAY_BLOCK_OCTOSPI1 {
    type Target = delay_block_sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DELAY_BLOCK_OCTOSPI1::ptr() }
    }
}
#[doc = "DELAY_Block_SDMMC1"]
pub struct DELAY_BLOCK_OCTOSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DELAY_BLOCK_OCTOSPI2 {}
impl DELAY_BLOCK_OCTOSPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const delay_block_sdmmc1::RegisterBlock {
        0x5200_b000 as *const _
    }
}
impl Deref for DELAY_BLOCK_OCTOSPI2 {
    type Target = delay_block_sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DELAY_BLOCK_OCTOSPI2::ptr() }
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
        0x5200_2000 as *const _
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
#[doc = "AXI interconnect registers"]
pub struct AXI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AXI {}
impl AXI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const axi::RegisterBlock {
        0x5100_0000 as *const _
    }
}
impl Deref for AXI {
    type Target = axi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AXI::ptr() }
    }
}
#[doc = "AXI interconnect registers"]
pub mod axi;
#[doc = "Hash processor"]
pub struct HASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HASH {}
impl HASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hash::RegisterBlock {
        0x4802_1400 as *const _
    }
}
impl Deref for HASH {
    type Target = hash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HASH::ptr() }
    }
}
#[doc = "Hash processor"]
pub mod hash;
#[doc = "Cryptographic processor"]
pub struct CRYP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYP {}
impl CRYP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cryp::RegisterBlock {
        0x4802_1000 as *const _
    }
}
impl Deref for CRYP {
    type Target = cryp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYP::ptr() }
    }
}
#[doc = "Cryptographic processor"]
pub mod cryp;
#[doc = "Digital camera interface"]
pub struct DCMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCMI {}
impl DCMI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dcmi::RegisterBlock {
        0x4802_0000 as *const _
    }
}
impl Deref for DCMI {
    type Target = dcmi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DCMI::ptr() }
    }
}
#[doc = "Digital camera interface"]
pub mod dcmi;
#[doc = "USB 1 on the go high speed"]
pub struct OTG1_HS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG1_HS_GLOBAL {}
impl OTG1_HS_GLOBAL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otg1_hs_global::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for OTG1_HS_GLOBAL {
    type Target = otg1_hs_global::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG1_HS_GLOBAL::ptr() }
    }
}
#[doc = "USB 1 on the go high speed"]
pub mod otg1_hs_global;
#[doc = "USB 1 on the go high speed"]
pub struct OTG2_HS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG2_HS_GLOBAL {}
impl OTG2_HS_GLOBAL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otg1_hs_global::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for OTG2_HS_GLOBAL {
    type Target = otg1_hs_global::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG2_HS_GLOBAL::ptr() }
    }
}
#[doc = "USB 1 on the go high speed"]
pub struct OTG1_HS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG1_HS_HOST {}
impl OTG1_HS_HOST {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otg1_hs_host::RegisterBlock {
        0x4004_0400 as *const _
    }
}
impl Deref for OTG1_HS_HOST {
    type Target = otg1_hs_host::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG1_HS_HOST::ptr() }
    }
}
#[doc = "USB 1 on the go high speed"]
pub mod otg1_hs_host;
#[doc = "USB 1 on the go high speed"]
pub struct OTG2_HS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG2_HS_HOST {}
impl OTG2_HS_HOST {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otg1_hs_host::RegisterBlock {
        0x4008_0400 as *const _
    }
}
impl Deref for OTG2_HS_HOST {
    type Target = otg1_hs_host::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG2_HS_HOST::ptr() }
    }
}
#[doc = "USB 1 on the go high speed"]
pub struct OTG1_HS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG1_HS_DEVICE {}
impl OTG1_HS_DEVICE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otg1_hs_device::RegisterBlock {
        0x4004_0800 as *const _
    }
}
impl Deref for OTG1_HS_DEVICE {
    type Target = otg1_hs_device::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG1_HS_DEVICE::ptr() }
    }
}
#[doc = "USB 1 on the go high speed"]
pub mod otg1_hs_device;
#[doc = "USB 1 on the go high speed"]
pub struct OTG2_HS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG2_HS_DEVICE {}
impl OTG2_HS_DEVICE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otg1_hs_device::RegisterBlock {
        0x4008_0800 as *const _
    }
}
impl Deref for OTG2_HS_DEVICE {
    type Target = otg1_hs_device::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG2_HS_DEVICE::ptr() }
    }
}
#[doc = "USB 1 on the go high speed"]
pub struct OTG1_HS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG1_HS_PWRCLK {}
impl OTG1_HS_PWRCLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otg1_hs_pwrclk::RegisterBlock {
        0x4004_0e00 as *const _
    }
}
impl Deref for OTG1_HS_PWRCLK {
    type Target = otg1_hs_pwrclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG1_HS_PWRCLK::ptr() }
    }
}
#[doc = "USB 1 on the go high speed"]
pub mod otg1_hs_pwrclk;
#[doc = "USB 1 on the go high speed"]
pub struct OTG2_HS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG2_HS_PWRCLK {}
impl OTG2_HS_PWRCLK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otg1_hs_pwrclk::RegisterBlock {
        0x4008_0e00 as *const _
    }
}
impl Deref for OTG2_HS_PWRCLK {
    type Target = otg1_hs_pwrclk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG2_HS_PWRCLK::ptr() }
    }
}
#[doc = "Ethernet: media access control (MAC)"]
pub struct ETHERNET_MAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_MAC {}
impl ETHERNET_MAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ethernet_mac::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for ETHERNET_MAC {
    type Target = ethernet_mac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETHERNET_MAC::ptr() }
    }
}
#[doc = "Ethernet: media access control (MAC)"]
pub mod ethernet_mac;
#[doc = "DMA controller"]
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
#[doc = "DMA controller"]
pub mod dma1;
#[doc = "High Resolution Timer: Master Timers"]
pub struct HRTIM_MASTER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_MASTER {}
impl HRTIM_MASTER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrtim_master::RegisterBlock {
        0x4001_7400 as *const _
    }
}
impl Deref for HRTIM_MASTER {
    type Target = hrtim_master::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRTIM_MASTER::ptr() }
    }
}
#[doc = "High Resolution Timer: Master Timers"]
pub mod hrtim_master;
#[doc = "High Resolution Timer: TIMA"]
pub struct HRTIM_TIMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_TIMA {}
impl HRTIM_TIMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrtim_tima::RegisterBlock {
        0x4001_7480 as *const _
    }
}
impl Deref for HRTIM_TIMA {
    type Target = hrtim_tima::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRTIM_TIMA::ptr() }
    }
}
#[doc = "High Resolution Timer: TIMA"]
pub mod hrtim_tima;
#[doc = "High Resolution Timer: TIMB"]
pub struct HRTIM_TIMB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_TIMB {}
impl HRTIM_TIMB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrtim_timb::RegisterBlock {
        0x4001_7500 as *const _
    }
}
impl Deref for HRTIM_TIMB {
    type Target = hrtim_timb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRTIM_TIMB::ptr() }
    }
}
#[doc = "High Resolution Timer: TIMB"]
pub mod hrtim_timb;
#[doc = "High Resolution Timer: TIMC"]
pub struct HRTIM_TIMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_TIMC {}
impl HRTIM_TIMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrtim_timc::RegisterBlock {
        0x4001_7580 as *const _
    }
}
impl Deref for HRTIM_TIMC {
    type Target = hrtim_timc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRTIM_TIMC::ptr() }
    }
}
#[doc = "High Resolution Timer: TIMC"]
pub mod hrtim_timc;
#[doc = "High Resolution Timer: TIMD"]
pub struct HRTIM_TIMD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_TIMD {}
impl HRTIM_TIMD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrtim_timd::RegisterBlock {
        0x4001_7600 as *const _
    }
}
impl Deref for HRTIM_TIMD {
    type Target = hrtim_timd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRTIM_TIMD::ptr() }
    }
}
#[doc = "High Resolution Timer: TIMD"]
pub mod hrtim_timd;
#[doc = "High Resolution Timer: TIME"]
pub struct HRTIM_TIME {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_TIME {}
impl HRTIM_TIME {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrtim_time::RegisterBlock {
        0x4001_7680 as *const _
    }
}
impl Deref for HRTIM_TIME {
    type Target = hrtim_time::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRTIM_TIME::ptr() }
    }
}
#[doc = "High Resolution Timer: TIME"]
pub mod hrtim_time;
#[doc = "High Resolution Timer: Common functions"]
pub struct HRTIM_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HRTIM_COMMON {}
impl HRTIM_COMMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hrtim_common::RegisterBlock {
        0x4001_7780 as *const _
    }
}
impl Deref for HRTIM_COMMON {
    type Target = hrtim_common::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HRTIM_COMMON::ptr() }
    }
}
#[doc = "High Resolution Timer: Common functions"]
pub mod hrtim_common;
#[doc = "Digital filter for sigma delta modulators"]
pub struct DFSDM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DFSDM {}
impl DFSDM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dfsdm::RegisterBlock {
        0x4001_7000 as *const _
    }
}
impl Deref for DFSDM {
    type Target = dfsdm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DFSDM::ptr() }
    }
}
#[doc = "Digital filter for sigma delta modulators"]
pub mod dfsdm;
#[doc = "General-purpose-timers"]
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
#[doc = "General-purpose-timers"]
pub mod tim16;
#[doc = "General-purpose-timers"]
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
#[doc = "General-purpose-timers"]
pub mod tim17;
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
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_1000 as *const _
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
pub struct USART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART6 {}
impl USART6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_1400 as *const _
    }
}
impl Deref for USART6 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART6::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART7 {}
impl UART7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_7800 as *const _
    }
}
impl Deref for UART7 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART7::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART8 {}
impl UART8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_7c00 as *const _
    }
}
impl Deref for UART8 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART8::ptr() }
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
        0x4001_0000 as *const _
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
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim8::RegisterBlock {
        0x4001_0400 as *const _
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
#[doc = "FDCAN1"]
pub struct FDCAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN2 {}
impl FDCAN2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fdcan2::RegisterBlock {
        0x4000_a400 as *const _
    }
}
impl Deref for FDCAN2 {
    type Target = fdcan2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FDCAN2::ptr() }
    }
}
#[doc = "FDCAN1"]
pub mod fdcan2;
#[doc = "FDCAN1"]
pub struct FDCAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN1 {}
impl FDCAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fdcan1::RegisterBlock {
        0x4000_a000 as *const _
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
#[doc = "CCU registers"]
pub struct CAN_CCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN_CCU {}
impl CAN_CCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can_ccu::RegisterBlock {
        0x4000_a800 as *const _
    }
}
impl Deref for CAN_CCU {
    type Target = can_ccu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN_CCU::ptr() }
    }
}
#[doc = "CCU registers"]
pub mod can_ccu;
#[doc = "Management data input/output slave"]
pub struct MDIOS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MDIOS {}
impl MDIOS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mdios::RegisterBlock {
        0x4000_9400 as *const _
    }
}
impl Deref for MDIOS {
    type Target = mdios::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MDIOS::ptr() }
    }
}
#[doc = "Management data input/output slave"]
pub mod mdios;
#[doc = "Operational amplifiers"]
pub struct OPAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OPAMP {}
impl OPAMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const opamp::RegisterBlock {
        0x4000_9000 as *const _
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
#[doc = "Single Wire Protocol Master Interface"]
pub struct SWPMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWPMI {}
impl SWPMI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const swpmi::RegisterBlock {
        0x4000_8800 as *const _
    }
}
impl Deref for SWPMI {
    type Target = swpmi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SWPMI::ptr() }
    }
}
#[doc = "Single Wire Protocol Master Interface"]
pub mod swpmi;
#[doc = "General purpose timers"]
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
#[doc = "General purpose timers"]
pub mod tim2;
#[doc = "General purpose timers"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0c00 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "Basic timers"]
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
#[doc = "Basic timers"]
pub mod tim6;
#[doc = "Basic timers"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_1400 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "SysTick timer"]
pub struct STK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STK {}
impl STK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const stk::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for STK {
    type Target = stk::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STK::ptr() }
    }
}
#[doc = "SysTick timer"]
pub mod stk;
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
#[doc = "Floating point unit CPACR"]
pub struct FPU_CPACR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_CPACR {}
impl FPU_CPACR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fpu_cpacr::RegisterBlock {
        0xe000_ed88 as *const _
    }
}
impl Deref for FPU_CPACR {
    type Target = fpu_cpacr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FPU_CPACR::ptr() }
    }
}
#[doc = "Floating point unit CPACR"]
pub mod fpu_cpacr;
#[doc = "System control block ACTLR"]
pub struct SCB_ACTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB_ACTRL {}
impl SCB_ACTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scb_actrl::RegisterBlock {
        0xe000_e008 as *const _
    }
}
impl Deref for SCB_ACTRL {
    type Target = scb_actrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCB_ACTRL::ptr() }
    }
}
#[doc = "System control block ACTLR"]
pub mod scb_actrl;
#[doc = "Processor features"]
pub struct PF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PF {}
impl PF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pf::RegisterBlock {
        0xe000_ed78 as *const _
    }
}
impl Deref for PF {
    type Target = pf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PF::ptr() }
    }
}
#[doc = "Processor features"]
pub mod pf;
#[doc = "Access control"]
pub struct AC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AC {}
impl AC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ac::RegisterBlock {
        0xe000_ef90 as *const _
    }
}
impl Deref for AC {
    type Target = ac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AC::ptr() }
    }
}
#[doc = "Access control"]
pub mod ac;
#[doc = "OctoSPI"]
pub struct OCTOSPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCTOSPI2 {}
impl OCTOSPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const octospi2::RegisterBlock {
        0x5200_a000 as *const _
    }
}
impl Deref for OCTOSPI2 {
    type Target = octospi2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCTOSPI2::ptr() }
    }
}
#[doc = "OctoSPI"]
pub mod octospi2;
#[doc = "OctoSPI"]
pub struct OCTOSPI1_CONTROL_REGISTER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCTOSPI1_CONTROL_REGISTER {}
impl OCTOSPI1_CONTROL_REGISTER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const octospi2::RegisterBlock {
        0x5200_5000 as *const _
    }
}
impl Deref for OCTOSPI1_CONTROL_REGISTER {
    type Target = octospi2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCTOSPI1_CONTROL_REGISTER::ptr() }
    }
}
#[doc = "OctoSPI IO Manager"]
pub struct OCTOSPII_O_MANAGER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCTOSPII_O_MANAGER {}
impl OCTOSPII_O_MANAGER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const octo_spii_o_manager::RegisterBlock {
        0x5200_b400 as *const _
    }
}
impl Deref for OCTOSPII_O_MANAGER {
    type Target = octo_spii_o_manager::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCTOSPII_O_MANAGER::ptr() }
    }
}
#[doc = "OctoSPI IO Manager"]
pub mod octo_spii_o_manager;
#[doc = "On-The-Fly Decryption engine"]
pub struct OTFDEC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTFDEC1 {}
impl OTFDEC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otfdec1::RegisterBlock {
        0x5200_b800 as *const _
    }
}
impl Deref for OTFDEC1 {
    type Target = otfdec1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTFDEC1::ptr() }
    }
}
#[doc = "On-The-Fly Decryption engine"]
pub mod otfdec1;
#[doc = "On-The-Fly Decryption engine"]
pub struct OTFDEC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTFDEC2 {}
impl OTFDEC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otfdec1::RegisterBlock {
        0x5200_bc00 as *const _
    }
}
impl Deref for OTFDEC2 {
    type Target = otfdec1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTFDEC2::ptr() }
    }
}
#[doc = "BDMA"]
pub struct BDMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BDMA2 {}
impl BDMA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bdma2::RegisterBlock {
        0x5802_5400 as *const _
    }
}
impl Deref for BDMA2 {
    type Target = bdma2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BDMA2::ptr() }
    }
}
#[doc = "BDMA"]
pub mod bdma2;
#[doc = "BDMA"]
pub struct BDMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BDMA1 {}
impl BDMA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bdma2::RegisterBlock {
        0x4802_2c00 as *const _
    }
}
impl Deref for BDMA1 {
    type Target = bdma2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BDMA1::ptr() }
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
        0x5802_4400 as *const _
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
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x5800_0400 as *const _
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
#[doc = "ECC controller is associated to each RAM area"]
pub struct RAMECC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RAMECC {}
impl RAMECC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ramecc::RegisterBlock {
        0x5200_9000 as *const _
    }
}
impl Deref for RAMECC {
    type Target = ramecc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RAMECC::ptr() }
    }
}
#[doc = "ECC controller is associated to each RAM area"]
pub mod ramecc;
#[doc = "I2C1"]
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
#[doc = "I2C1"]
pub mod i2c1;
#[doc = "I2C1"]
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
#[doc = "I2C3"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c3::RegisterBlock {
        0x4000_5c00 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "I2C3"]
pub mod i2c3;
#[doc = "I2C3"]
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c3::RegisterBlock {
        0x5800_1c00 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod adc1;
#[doc = "Analog to Digital Converter"]
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc2::RegisterBlock {
        0x4002_2100 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC2::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod adc2;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC12_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC12_COMMON {}
impl ADC12_COMMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc12_common::RegisterBlock {
        0x4002_2300 as *const _
    }
}
impl Deref for ADC12_COMMON {
    type Target = adc12_common::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC12_COMMON::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc12_common;
#[doc = "General purpose timers"]
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
#[doc = "General purpose timers"]
pub mod tim3;
#[doc = "General purpose timers"]
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
#[doc = "General purpose timers"]
pub mod tim4;
#[doc = "General purpose timers"]
pub struct TIM12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM12 {}
impl TIM12 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim12::RegisterBlock {
        0x4000_1800 as *const _
    }
}
impl Deref for TIM12 {
    type Target = tim12::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM12::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim12;
#[doc = "General purpose timers"]
pub struct TIM13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM13 {}
impl TIM13 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim13::RegisterBlock {
        0x4000_1c00 as *const _
    }
}
impl Deref for TIM13 {
    type Target = tim13::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM13::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim13;
#[doc = "General purpose timers"]
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM14 {}
impl TIM14 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim14::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for TIM14 {
    type Target = tim14::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM14::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim14;
#[doc = "DMA controller"]
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
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART9 {}
impl UART9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for UART9 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART9::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART10 {}
impl USART10 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for USART10 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART10::ptr() }
    }
}
#[doc = "DAC"]
pub struct DAC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC2 {}
impl DAC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac1::RegisterBlock {
        0x5800_3400 as *const _
    }
}
impl Deref for DAC2 {
    type Target = dac1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC2::ptr() }
    }
}
#[doc = "Debug support"]
pub struct DBGMCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBGMCU {}
impl DBGMCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dbgmcu::RegisterBlock {
        0x5c00_1000 as *const _
    }
}
impl Deref for DBGMCU {
    type Target = dbgmcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DBGMCU::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbgmcu;
#[doc = "MIPI DSI Host"]
pub struct DSIHOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSIHOST {}
impl DSIHOST {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dsihost::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for DSIHOST {
    type Target = dsihost::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DSIHOST::ptr() }
    }
}
#[doc = "MIPI DSI Host"]
pub mod dsihost;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "COMP1"]
    pub COMP1: COMP1,
    #[doc = "CRS"]
    pub CRS: CRS,
    #[doc = "DAC1"]
    pub DAC1: DAC1,
    #[doc = "DMA2D"]
    pub DMA2D: DMA2D,
    #[doc = "DMAMUX2"]
    pub DMAMUX2: DMAMUX2,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "CEC"]
    pub CEC: CEC,
    #[doc = "HSEM"]
    pub HSEM: HSEM,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
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
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "GPIOI"]
    pub GPIOI: GPIOI,
    #[doc = "GPIOJ"]
    pub GPIOJ: GPIOJ,
    #[doc = "GPIOK"]
    pub GPIOK: GPIOK,
    #[doc = "JPEG"]
    pub JPEG: JPEG,
    #[doc = "MDMA"]
    pub MDMA: MDMA,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SAI1"]
    pub SAI1: SAI1,
    #[doc = "SAI2"]
    pub SAI2: SAI2,
    #[doc = "SDMMC1"]
    pub SDMMC1: SDMMC1,
    #[doc = "SDMMC2"]
    pub SDMMC2: SDMMC2,
    #[doc = "VREFBUF"]
    pub VREFBUF: VREFBUF,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "SPI4"]
    pub SPI4: SPI4,
    #[doc = "SPI5"]
    pub SPI5: SPI5,
    #[doc = "SPI6"]
    pub SPI6: SPI6,
    #[doc = "LTDC"]
    pub LTDC: LTDC,
    #[doc = "SPDIFRX"]
    pub SPDIFRX: SPDIFRX,
    #[doc = "DMAMUX1"]
    pub DMAMUX1: DMAMUX1,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "LPTIM2"]
    pub LPTIM2: LPTIM2,
    #[doc = "LPTIM3"]
    pub LPTIM3: LPTIM3,
    #[doc = "LPUART1"]
    pub LPUART1: LPUART1,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "DELAY_BLOCK_SDMMC1"]
    pub DELAY_BLOCK_SDMMC1: DELAY_BLOCK_SDMMC1,
    #[doc = "DELAY_BLOCK_SDMMC2"]
    pub DELAY_BLOCK_SDMMC2: DELAY_BLOCK_SDMMC2,
    #[doc = "DELAY_BLOCK_OCTOSPI1"]
    pub DELAY_BLOCK_OCTOSPI1: DELAY_BLOCK_OCTOSPI1,
    #[doc = "DELAY_BLOCK_OCTOSPI2"]
    pub DELAY_BLOCK_OCTOSPI2: DELAY_BLOCK_OCTOSPI2,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "AXI"]
    pub AXI: AXI,
    #[doc = "HASH"]
    pub HASH: HASH,
    #[doc = "CRYP"]
    pub CRYP: CRYP,
    #[doc = "DCMI"]
    pub DCMI: DCMI,
    #[doc = "OTG1_HS_GLOBAL"]
    pub OTG1_HS_GLOBAL: OTG1_HS_GLOBAL,
    #[doc = "OTG2_HS_GLOBAL"]
    pub OTG2_HS_GLOBAL: OTG2_HS_GLOBAL,
    #[doc = "OTG1_HS_HOST"]
    pub OTG1_HS_HOST: OTG1_HS_HOST,
    #[doc = "OTG2_HS_HOST"]
    pub OTG2_HS_HOST: OTG2_HS_HOST,
    #[doc = "OTG1_HS_DEVICE"]
    pub OTG1_HS_DEVICE: OTG1_HS_DEVICE,
    #[doc = "OTG2_HS_DEVICE"]
    pub OTG2_HS_DEVICE: OTG2_HS_DEVICE,
    #[doc = "OTG1_HS_PWRCLK"]
    pub OTG1_HS_PWRCLK: OTG1_HS_PWRCLK,
    #[doc = "OTG2_HS_PWRCLK"]
    pub OTG2_HS_PWRCLK: OTG2_HS_PWRCLK,
    #[doc = "ETHERNET_MAC"]
    pub ETHERNET_MAC: ETHERNET_MAC,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "HRTIM_MASTER"]
    pub HRTIM_MASTER: HRTIM_MASTER,
    #[doc = "HRTIM_TIMA"]
    pub HRTIM_TIMA: HRTIM_TIMA,
    #[doc = "HRTIM_TIMB"]
    pub HRTIM_TIMB: HRTIM_TIMB,
    #[doc = "HRTIM_TIMC"]
    pub HRTIM_TIMC: HRTIM_TIMC,
    #[doc = "HRTIM_TIMD"]
    pub HRTIM_TIMD: HRTIM_TIMD,
    #[doc = "HRTIM_TIME"]
    pub HRTIM_TIME: HRTIM_TIME,
    #[doc = "HRTIM_COMMON"]
    pub HRTIM_COMMON: HRTIM_COMMON,
    #[doc = "DFSDM"]
    pub DFSDM: DFSDM,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "TIM15"]
    pub TIM15: TIM15,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "USART6"]
    pub USART6: USART6,
    #[doc = "UART7"]
    pub UART7: UART7,
    #[doc = "UART8"]
    pub UART8: UART8,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "FDCAN2"]
    pub FDCAN2: FDCAN2,
    #[doc = "FDCAN1"]
    pub FDCAN1: FDCAN1,
    #[doc = "CAN_CCU"]
    pub CAN_CCU: CAN_CCU,
    #[doc = "MDIOS"]
    pub MDIOS: MDIOS,
    #[doc = "OPAMP"]
    pub OPAMP: OPAMP,
    #[doc = "SWPMI"]
    pub SWPMI: SWPMI,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "STK"]
    pub STK: STK,
    #[doc = "NVIC_STIR"]
    pub NVIC_STIR: NVIC_STIR,
    #[doc = "FPU_CPACR"]
    pub FPU_CPACR: FPU_CPACR,
    #[doc = "SCB_ACTRL"]
    pub SCB_ACTRL: SCB_ACTRL,
    #[doc = "PF"]
    pub PF: PF,
    #[doc = "AC"]
    pub AC: AC,
    #[doc = "OCTOSPI2"]
    pub OCTOSPI2: OCTOSPI2,
    #[doc = "OCTOSPI1_CONTROL_REGISTER"]
    pub OCTOSPI1_CONTROL_REGISTER: OCTOSPI1_CONTROL_REGISTER,
    #[doc = "OCTOSPII_O_MANAGER"]
    pub OCTOSPII_O_MANAGER: OCTOSPII_O_MANAGER,
    #[doc = "OTFDEC1"]
    pub OTFDEC1: OTFDEC1,
    #[doc = "OTFDEC2"]
    pub OTFDEC2: OTFDEC2,
    #[doc = "BDMA2"]
    pub BDMA2: BDMA2,
    #[doc = "BDMA1"]
    pub BDMA1: BDMA1,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "RAMECC"]
    pub RAMECC: RAMECC,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[doc = "ADC12_COMMON"]
    pub ADC12_COMMON: ADC12_COMMON,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "TIM12"]
    pub TIM12: TIM12,
    #[doc = "TIM13"]
    pub TIM13: TIM13,
    #[doc = "TIM14"]
    pub TIM14: TIM14,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "UART9"]
    pub UART9: UART9,
    #[doc = "USART10"]
    pub USART10: USART10,
    #[doc = "DAC2"]
    pub DAC2: DAC2,
    #[doc = "DBGMCU"]
    pub DBGMCU: DBGMCU,
    #[doc = "DSIHOST"]
    pub DSIHOST: DSIHOST,
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
            COMP1: COMP1 {
                _marker: PhantomData,
            },
            CRS: CRS {
                _marker: PhantomData,
            },
            DAC1: DAC1 {
                _marker: PhantomData,
            },
            DMA2D: DMA2D {
                _marker: PhantomData,
            },
            DMAMUX2: DMAMUX2 {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            CEC: CEC {
                _marker: PhantomData,
            },
            HSEM: HSEM {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
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
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            GPIOI: GPIOI {
                _marker: PhantomData,
            },
            GPIOJ: GPIOJ {
                _marker: PhantomData,
            },
            GPIOK: GPIOK {
                _marker: PhantomData,
            },
            JPEG: JPEG {
                _marker: PhantomData,
            },
            MDMA: MDMA {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SAI1: SAI1 {
                _marker: PhantomData,
            },
            SAI2: SAI2 {
                _marker: PhantomData,
            },
            SDMMC1: SDMMC1 {
                _marker: PhantomData,
            },
            SDMMC2: SDMMC2 {
                _marker: PhantomData,
            },
            VREFBUF: VREFBUF {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            PWR: PWR {
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
            SPI4: SPI4 {
                _marker: PhantomData,
            },
            SPI5: SPI5 {
                _marker: PhantomData,
            },
            SPI6: SPI6 {
                _marker: PhantomData,
            },
            LTDC: LTDC {
                _marker: PhantomData,
            },
            SPDIFRX: SPDIFRX {
                _marker: PhantomData,
            },
            DMAMUX1: DMAMUX1 {
                _marker: PhantomData,
            },
            CRC: CRC {
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
            LPUART1: LPUART1 {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            DELAY_BLOCK_SDMMC1: DELAY_BLOCK_SDMMC1 {
                _marker: PhantomData,
            },
            DELAY_BLOCK_SDMMC2: DELAY_BLOCK_SDMMC2 {
                _marker: PhantomData,
            },
            DELAY_BLOCK_OCTOSPI1: DELAY_BLOCK_OCTOSPI1 {
                _marker: PhantomData,
            },
            DELAY_BLOCK_OCTOSPI2: DELAY_BLOCK_OCTOSPI2 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            AXI: AXI {
                _marker: PhantomData,
            },
            HASH: HASH {
                _marker: PhantomData,
            },
            CRYP: CRYP {
                _marker: PhantomData,
            },
            DCMI: DCMI {
                _marker: PhantomData,
            },
            OTG1_HS_GLOBAL: OTG1_HS_GLOBAL {
                _marker: PhantomData,
            },
            OTG2_HS_GLOBAL: OTG2_HS_GLOBAL {
                _marker: PhantomData,
            },
            OTG1_HS_HOST: OTG1_HS_HOST {
                _marker: PhantomData,
            },
            OTG2_HS_HOST: OTG2_HS_HOST {
                _marker: PhantomData,
            },
            OTG1_HS_DEVICE: OTG1_HS_DEVICE {
                _marker: PhantomData,
            },
            OTG2_HS_DEVICE: OTG2_HS_DEVICE {
                _marker: PhantomData,
            },
            OTG1_HS_PWRCLK: OTG1_HS_PWRCLK {
                _marker: PhantomData,
            },
            OTG2_HS_PWRCLK: OTG2_HS_PWRCLK {
                _marker: PhantomData,
            },
            ETHERNET_MAC: ETHERNET_MAC {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            HRTIM_MASTER: HRTIM_MASTER {
                _marker: PhantomData,
            },
            HRTIM_TIMA: HRTIM_TIMA {
                _marker: PhantomData,
            },
            HRTIM_TIMB: HRTIM_TIMB {
                _marker: PhantomData,
            },
            HRTIM_TIMC: HRTIM_TIMC {
                _marker: PhantomData,
            },
            HRTIM_TIMD: HRTIM_TIMD {
                _marker: PhantomData,
            },
            HRTIM_TIME: HRTIM_TIME {
                _marker: PhantomData,
            },
            HRTIM_COMMON: HRTIM_COMMON {
                _marker: PhantomData,
            },
            DFSDM: DFSDM {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            TIM15: TIM15 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            USART6: USART6 {
                _marker: PhantomData,
            },
            UART7: UART7 {
                _marker: PhantomData,
            },
            UART8: UART8 {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            FDCAN2: FDCAN2 {
                _marker: PhantomData,
            },
            FDCAN1: FDCAN1 {
                _marker: PhantomData,
            },
            CAN_CCU: CAN_CCU {
                _marker: PhantomData,
            },
            MDIOS: MDIOS {
                _marker: PhantomData,
            },
            OPAMP: OPAMP {
                _marker: PhantomData,
            },
            SWPMI: SWPMI {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            STK: STK {
                _marker: PhantomData,
            },
            NVIC_STIR: NVIC_STIR {
                _marker: PhantomData,
            },
            FPU_CPACR: FPU_CPACR {
                _marker: PhantomData,
            },
            SCB_ACTRL: SCB_ACTRL {
                _marker: PhantomData,
            },
            PF: PF {
                _marker: PhantomData,
            },
            AC: AC {
                _marker: PhantomData,
            },
            OCTOSPI2: OCTOSPI2 {
                _marker: PhantomData,
            },
            OCTOSPI1_CONTROL_REGISTER: OCTOSPI1_CONTROL_REGISTER {
                _marker: PhantomData,
            },
            OCTOSPII_O_MANAGER: OCTOSPII_O_MANAGER {
                _marker: PhantomData,
            },
            OTFDEC1: OTFDEC1 {
                _marker: PhantomData,
            },
            OTFDEC2: OTFDEC2 {
                _marker: PhantomData,
            },
            BDMA2: BDMA2 {
                _marker: PhantomData,
            },
            BDMA1: BDMA1 {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            RAMECC: RAMECC {
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
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            ADC12_COMMON: ADC12_COMMON {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            TIM12: TIM12 {
                _marker: PhantomData,
            },
            TIM13: TIM13 {
                _marker: PhantomData,
            },
            TIM14: TIM14 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            UART9: UART9 {
                _marker: PhantomData,
            },
            USART10: USART10 {
                _marker: PhantomData,
            },
            DAC2: DAC2 {
                _marker: PhantomData,
            },
            DBGMCU: DBGMCU {
                _marker: PhantomData,
            },
            DSIHOST: DSIHOST {
                _marker: PhantomData,
            },
        }
    }
}
