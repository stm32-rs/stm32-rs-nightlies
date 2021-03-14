#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG1_IT();
    fn PVD_AVD();
    fn TAMP();
    fn RTC_WKUP_ALARM();
    fn TZC_IT();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_STR0();
    fn DMA1_STR1();
    fn DMA1_STR2();
    fn DMA1_STR3();
    fn DMA1_STR4();
    fn DMA1_STR5();
    fn DMA1_STR6();
    fn ADC1();
    fn FDCAN1_IT0();
    fn FDCAN2_IT0();
    fn FDCAN1_IT1();
    fn FDCAN2_IT1();
    fn EXTI5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EVT();
    fn I2C1_ERR();
    fn I2C2_EVT();
    fn I2C2_ERR();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI10();
    fn RTC_TS();
    fn EXTI11();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn DMA1_STR7();
    fn FMC();
    fn SDMMC1();
    fn TIM5();
    fn SPI3();
    fn USART4();
    fn USART5();
    fn TIM6();
    fn TIM7();
    fn DMA2_STR0();
    fn DMA2_STR1();
    fn DMA2_STR2();
    fn DMA2_STR3();
    fn DMA2_STR4();
    fn ETH1();
    fn ETH1_WKUP();
    fn EXTI6();
    fn EXTI7();
    fn EXTI8();
    fn EXTI9();
    fn DMA2_STR5();
    fn DMA2_STR6();
    fn DMA2_STR7();
    fn USART6();
    fn I2C3_EVT();
    fn I2C3_ERR();
    fn EXTI12();
    fn EXTI13();
    fn DCMI();
    fn CRYP1();
    fn HASH1();
    fn USART7();
    fn USART8();
    fn SPI4();
    fn SPI5();
    fn SPI6();
    fn SAI1();
    fn LTDC();
    fn LTDC_ER();
    fn ADC2();
    fn SAI2();
    fn QUADSPI();
    fn LPTIM1();
    fn CEC();
    fn I2C4_EVT();
    fn I2C4_ERR();
    fn SPDIFRX();
    fn OTG();
    fn IPCC_RX0();
    fn IPCC_TX0();
    fn DMAMUX1_OVR_REQ();
    fn IPCC_RX1();
    fn IPCC_TX1();
    fn CRYP2();
    fn HASH2();
    fn I2C5_EVT();
    fn I2C5_ERR();
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn DFSDM1_FLT3();
    fn SAI3();
    fn DFSDM1_FLT4();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn TIM12();
    fn MDIOS();
    fn EXTI14();
    fn MDMA();
    fn DSI();
    fn SDMMC2();
    fn HSEM_IT2();
    fn DFSDM1_FLT5();
    fn EXTI15();
    fn TIM13();
    fn TIM14();
    fn DAC();
    fn RNG1();
    fn RNG2();
    fn I2C6_EVT();
    fn I2C6_ERR();
    fn SDMMC3();
    fn LPTIM2();
    fn LPTIM3();
    fn LPTIM4();
    fn LPTIM5();
    fn ETH1_LPI();
    fn RCC_WAKEUP();
    fn SAI4();
    fn DTS();
    fn IWDG1_IT();
    fn IWDG2_IT();
    fn TAMP_S();
    fn RTC_WKUP_ALARM_S();
    fn RTC_TS_S();
    fn DDRPERFM();
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
pub static __INTERRUPTS: [Vector; 214] = [
    Vector { _handler: WWDG1_IT },
    Vector { _handler: PVD_AVD },
    Vector { _handler: TAMP },
    Vector {
        _handler: RTC_WKUP_ALARM,
    },
    Vector { _handler: TZC_IT },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_STR0,
    },
    Vector {
        _handler: DMA1_STR1,
    },
    Vector {
        _handler: DMA1_STR2,
    },
    Vector {
        _handler: DMA1_STR3,
    },
    Vector {
        _handler: DMA1_STR4,
    },
    Vector {
        _handler: DMA1_STR5,
    },
    Vector {
        _handler: DMA1_STR6,
    },
    Vector { _handler: ADC1 },
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
    Vector { _handler: EXTI5 },
    Vector { _handler: TIM1_BRK },
    Vector { _handler: TIM1_UP },
    Vector {
        _handler: TIM1_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EVT },
    Vector { _handler: I2C1_ERR },
    Vector { _handler: I2C2_EVT },
    Vector { _handler: I2C2_ERR },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector { _handler: EXTI10 },
    Vector { _handler: RTC_TS },
    Vector { _handler: EXTI11 },
    Vector { _handler: TIM8_BRK },
    Vector { _handler: TIM8_UP },
    Vector {
        _handler: TIM8_TRG_COM,
    },
    Vector { _handler: TIM8_CC },
    Vector {
        _handler: DMA1_STR7,
    },
    Vector { _handler: FMC },
    Vector { _handler: SDMMC1 },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: USART4 },
    Vector { _handler: USART5 },
    Vector { _handler: TIM6 },
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
    Vector { _handler: ETH1 },
    Vector {
        _handler: ETH1_WKUP,
    },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI6 },
    Vector { _handler: EXTI7 },
    Vector { _handler: EXTI8 },
    Vector { _handler: EXTI9 },
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
    Vector { _handler: I2C3_EVT },
    Vector { _handler: I2C3_ERR },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: EXTI12 },
    Vector { _handler: EXTI13 },
    Vector { _handler: DCMI },
    Vector { _handler: CRYP1 },
    Vector { _handler: HASH1 },
    Vector { _reserved: 0 },
    Vector { _handler: USART7 },
    Vector { _handler: USART8 },
    Vector { _handler: SPI4 },
    Vector { _handler: SPI5 },
    Vector { _handler: SPI6 },
    Vector { _handler: SAI1 },
    Vector { _handler: LTDC },
    Vector { _handler: LTDC_ER },
    Vector { _handler: ADC2 },
    Vector { _handler: SAI2 },
    Vector { _handler: QUADSPI },
    Vector { _handler: LPTIM1 },
    Vector { _handler: CEC },
    Vector { _handler: I2C4_EVT },
    Vector { _handler: I2C4_ERR },
    Vector { _handler: SPDIFRX },
    Vector { _handler: OTG },
    Vector { _reserved: 0 },
    Vector { _handler: IPCC_RX0 },
    Vector { _handler: IPCC_TX0 },
    Vector {
        _handler: DMAMUX1_OVR_REQ,
    },
    Vector { _handler: IPCC_RX1 },
    Vector { _handler: IPCC_TX1 },
    Vector { _handler: CRYP2 },
    Vector { _handler: HASH2 },
    Vector { _handler: I2C5_EVT },
    Vector { _handler: I2C5_ERR },
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
    Vector { _handler: SAI3 },
    Vector {
        _handler: DFSDM1_FLT4,
    },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: TIM12 },
    Vector { _handler: MDIOS },
    Vector { _handler: EXTI14 },
    Vector { _handler: MDMA },
    Vector { _handler: DSI },
    Vector { _handler: SDMMC2 },
    Vector { _handler: HSEM_IT2 },
    Vector {
        _handler: DFSDM1_FLT5,
    },
    Vector { _handler: EXTI15 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM13 },
    Vector { _handler: TIM14 },
    Vector { _handler: DAC },
    Vector { _handler: RNG1 },
    Vector { _handler: RNG2 },
    Vector { _handler: I2C6_EVT },
    Vector { _handler: I2C6_ERR },
    Vector { _handler: SDMMC3 },
    Vector { _handler: LPTIM2 },
    Vector { _handler: LPTIM3 },
    Vector { _handler: LPTIM4 },
    Vector { _handler: LPTIM5 },
    Vector { _handler: ETH1_LPI },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: RCC_WAKEUP,
    },
    Vector { _handler: SAI4 },
    Vector { _handler: DTS },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: IWDG1_IT },
    Vector { _handler: IWDG2_IT },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TAMP_S },
    Vector {
        _handler: RTC_WKUP_ALARM_S,
    },
    Vector { _handler: RTC_TS_S },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DDRPERFM },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG1_IT = 0,
    #[doc = "1 - PVD AND AVD detector through EXTI"]
    PVD_AVD = 1,
    #[doc = "2 - Tamper interrupt (include LSECSS interrupts)"]
    TAMP = 2,
    #[doc = "3 - RTC Tamper or TimeStamp"]
    RTC_WKUP_ALARM = 3,
    #[doc = "4 - TrustZone DDR address space controller"]
    TZC_IT = 4,
    #[doc = "5 - RCC global interrupt"]
    RCC = 5,
    #[doc = "6 - EXTI Line 0 interrupt"]
    EXTI0 = 6,
    #[doc = "7 - EXTI Line 1 interrupt"]
    EXTI1 = 7,
    #[doc = "8 - EXTI Line 2 interrupt"]
    EXTI2 = 8,
    #[doc = "9 - EXTI Line 3 interrupt"]
    EXTI3 = 9,
    #[doc = "10 - EXTI Line 4 interrupt"]
    EXTI4 = 10,
    #[doc = "11 - DMA1 stream0 global interrupt"]
    DMA1_STR0 = 11,
    #[doc = "12 - DMA1 stream1 global interrupt"]
    DMA1_STR1 = 12,
    #[doc = "13 - DMA1 stream2 global interrupt"]
    DMA1_STR2 = 13,
    #[doc = "14 - DMA1 stream3 global interrupt"]
    DMA1_STR3 = 14,
    #[doc = "15 - DMA1 stream4 global interrupt"]
    DMA1_STR4 = 15,
    #[doc = "16 - DMA1 stream5 global interrupt"]
    DMA1_STR5 = 16,
    #[doc = "17 - DMA1 stream6 global interrupt"]
    DMA1_STR6 = 17,
    #[doc = "18 - ADC1 global interrupt"]
    ADC1 = 18,
    #[doc = "19 - FDCAN1 interrupt 0"]
    FDCAN1_IT0 = 19,
    #[doc = "20 - FDCAN2 interrupt 0"]
    FDCAN2_IT0 = 20,
    #[doc = "21 - FDCAN1 interrupt 1"]
    FDCAN1_IT1 = 21,
    #[doc = "22 - FDCAN2 interrupt 1"]
    FDCAN2_IT1 = 22,
    #[doc = "23 - EXTI line 5 interrupt"]
    EXTI5 = 23,
    #[doc = "24 - TIM1 break interrupt"]
    TIM1_BRK = 24,
    #[doc = "25 - TIM1 update interrupt"]
    TIM1_UP = 25,
    #[doc = "26 - TIM1 trigger and commutation interrupt"]
    TIM1_TRG_COM = 26,
    #[doc = "27 - TIM1 capture compare interrupt"]
    TIM1_CC = 27,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2 = 28,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3 = 29,
    #[doc = "30 - TIM4 global interrupt"]
    TIM4 = 30,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EVT = 31,
    #[doc = "32 - I2C1 global error interrupt"]
    I2C1_ERR = 32,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EVT = 33,
    #[doc = "34 - I2C2 global error interrupt"]
    I2C2_ERR = 34,
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
    #[doc = "40 - EXTI line 10 interrupt"]
    EXTI10 = 40,
    #[doc = "41 - RTC timestamp interrupt"]
    RTC_TS = 41,
    #[doc = "42 - EXTI line 11 interrupt"]
    EXTI11 = 42,
    #[doc = "43 - TIM8 break interrupt"]
    TIM8_BRK = 43,
    #[doc = "44 - TIM8 update interrupt"]
    TIM8_UP = 44,
    #[doc = "45 - TIM8 trigger and commutation interrupt"]
    TIM8_TRG_COM = 45,
    #[doc = "46 - TIM8 capture compare interrupt"]
    TIM8_CC = 46,
    #[doc = "47 - DMA1 stream7 global interrupt"]
    DMA1_STR7 = 47,
    #[doc = "48 - FMC global interrupt"]
    FMC = 48,
    #[doc = "49 - SDMMC1 global interrupt"]
    SDMMC1 = 49,
    #[doc = "50 - TIM5 global interrupt"]
    TIM5 = 50,
    #[doc = "51 - SPI3 global interrupt"]
    SPI3 = 51,
    #[doc = "52 - USART4 global interrupt"]
    USART4 = 52,
    #[doc = "53 - USART5 global interrupt"]
    USART5 = 53,
    #[doc = "54 - TIM6 global interrupt"]
    TIM6 = 54,
    #[doc = "55 - TIM7 global interrupt"]
    TIM7 = 55,
    #[doc = "56 - DMA2 stream0 global interrupt"]
    DMA2_STR0 = 56,
    #[doc = "57 - DMA2 stream1 global interrupt"]
    DMA2_STR1 = 57,
    #[doc = "58 - DMA2 stream2 global interrupt"]
    DMA2_STR2 = 58,
    #[doc = "59 - DMA2 stream3 global interrupt"]
    DMA2_STR3 = 59,
    #[doc = "60 - DMA2 stream4 global interrupt"]
    DMA2_STR4 = 60,
    #[doc = "61 - ETH1 global interrupt"]
    ETH1 = 61,
    #[doc = "62 - ETH1 wakeup interrupt"]
    ETH1_WKUP = 62,
    #[doc = "64 - EXTI line 6 interrupt"]
    EXTI6 = 64,
    #[doc = "65 - EXTI line 7 interrupt"]
    EXTI7 = 65,
    #[doc = "66 - EXTI line 8 interrupt"]
    EXTI8 = 66,
    #[doc = "67 - EXTI line 9 interrupt"]
    EXTI9 = 67,
    #[doc = "68 - DMA2 stream5 global interrupt"]
    DMA2_STR5 = 68,
    #[doc = "69 - DMA2 stream6 global interrupt"]
    DMA2_STR6 = 69,
    #[doc = "70 - DMA2 stream7 global interrupt"]
    DMA2_STR7 = 70,
    #[doc = "71 - USART6 global interrupt"]
    USART6 = 71,
    #[doc = "72 - I2C3 event interrupt"]
    I2C3_EVT = 72,
    #[doc = "73 - I2C3 global error interrupt"]
    I2C3_ERR = 73,
    #[doc = "76 - EXTI line 12 interrupt"]
    EXTI12 = 76,
    #[doc = "77 - EXTI line 13 interrupt"]
    EXTI13 = 77,
    #[doc = "78 - DCMI global interrupt"]
    DCMI = 78,
    #[doc = "79 - CRYP1 global interrupt"]
    CRYP1 = 79,
    #[doc = "80 - HASH1 interrupt"]
    HASH1 = 80,
    #[doc = "82 - USART7 global interrupt"]
    USART7 = 82,
    #[doc = "83 - USART8 global interrupt"]
    USART8 = 83,
    #[doc = "84 - SPI4 global interrupt"]
    SPI4 = 84,
    #[doc = "85 - SPI5 global interrupt"]
    SPI5 = 85,
    #[doc = "86 - SPI6 global interrupt"]
    SPI6 = 86,
    #[doc = "87 - SAI1 global interrupt"]
    SAI1 = 87,
    #[doc = "88 - LTCD global interrupt"]
    LTDC = 88,
    #[doc = "89 - LTCD global error interrupt"]
    LTDC_ER = 89,
    #[doc = "90 - ADC2 global interrupt"]
    ADC2 = 90,
    #[doc = "91 - SAI2 global interrupt"]
    SAI2 = 91,
    #[doc = "92 - QUADSPI global interrupt"]
    QUADSPI = 92,
    #[doc = "93 - LPTIMER1 global interrupt"]
    LPTIM1 = 93,
    #[doc = "94 - HDMI-CEC global interrupt"]
    CEC = 94,
    #[doc = "95 - I2C4 event interrupt"]
    I2C4_EVT = 95,
    #[doc = "96 - I2C4 global error interrupt"]
    I2C4_ERR = 96,
    #[doc = "97 - SPDIFRX global interrupt"]
    SPDIFRX = 97,
    #[doc = "98 - USB On-The-Go global interrupt"]
    OTG = 98,
    #[doc = "100 - IPCC RX0 occupied interrupt"]
    IPCC_RX0 = 100,
    #[doc = "101 - IPCC TX0 free interrupt"]
    IPCC_TX0 = 101,
    #[doc = "102 - DMAMUX1 overrun interrupt"]
    DMAMUX1_OVR_REQ = 102,
    #[doc = "103 - IPCC RX1 occupied interrupt"]
    IPCC_RX1 = 103,
    #[doc = "104 - IPCC TX1 free interrupt"]
    IPCC_TX1 = 104,
    #[doc = "105 - CRYP2 global interrupt"]
    CRYP2 = 105,
    #[doc = "106 - HASH2 interrupt"]
    HASH2 = 106,
    #[doc = "107 - I2C5 event interrupt"]
    I2C5_EVT = 107,
    #[doc = "108 - I2C5 global error interrupt"]
    I2C5_ERR = 108,
    #[doc = "110 - DFSDM1 filter0 Interrupt"]
    DFSDM1_FLT0 = 110,
    #[doc = "111 - DFSDM1 filter1 Interrupt"]
    DFSDM1_FLT1 = 111,
    #[doc = "112 - DFSDM1 filter2 Interrupt"]
    DFSDM1_FLT2 = 112,
    #[doc = "113 - DFSDM1 filter3 Interrupt"]
    DFSDM1_FLT3 = 113,
    #[doc = "114 - SAI3 global interrupt"]
    SAI3 = 114,
    #[doc = "115 - DFSDM1 filter4 Interrupt"]
    DFSDM1_FLT4 = 115,
    #[doc = "116 - TIM15 global interrupt"]
    TIM15 = 116,
    #[doc = "117 - TIM16 global interrupt"]
    TIM16 = 117,
    #[doc = "118 - TIM17 global interrupt"]
    TIM17 = 118,
    #[doc = "119 - TIM12 gloabl interrupt"]
    TIM12 = 119,
    #[doc = "120 - MDIOS global interrupt"]
    MDIOS = 120,
    #[doc = "121 - EXTI line 14 interrupt"]
    EXTI14 = 121,
    #[doc = "122 - MDMA global interrupt"]
    MDMA = 122,
    #[doc = "123 - DSI Host controller global interrupt"]
    DSI = 123,
    #[doc = "124 - SDMMC2 global interrupt"]
    SDMMC2 = 124,
    #[doc = "125 - HSEM semaphore interrupt 2"]
    HSEM_IT2 = 125,
    #[doc = "126 - DFSDM1 filter5 Interrupt"]
    DFSDM1_FLT5 = 126,
    #[doc = "127 - EXTI line 15 interrupt"]
    EXTI15 = 127,
    #[doc = "130 - TIM13 global interrupt"]
    TIM13 = 130,
    #[doc = "131 - TIM14 global interrupt"]
    TIM14 = 131,
    #[doc = "132 - DAC1 and DAC2 underrun error interrupts"]
    DAC = 132,
    #[doc = "133 - RNG1 interrupt"]
    RNG1 = 133,
    #[doc = "134 - RNG2 interrupt"]
    RNG2 = 134,
    #[doc = "135 - I2C6 event interrupt"]
    I2C6_EVT = 135,
    #[doc = "136 - I2C6 global error interrupt"]
    I2C6_ERR = 136,
    #[doc = "137 - SDMMC3 global interrupt"]
    SDMMC3 = 137,
    #[doc = "138 - LPTIMER2 global interrupt"]
    LPTIM2 = 138,
    #[doc = "139 - LPTIMER3 global interrupt"]
    LPTIM3 = 139,
    #[doc = "140 - LPTIMER4 global interrupt"]
    LPTIM4 = 140,
    #[doc = "141 - LPTIMER5 global interrupt"]
    LPTIM5 = 141,
    #[doc = "142 - ETH1 LPI interrupt"]
    ETH1_LPI = 142,
    #[doc = "145 - RCC MPU wakeup interrupt"]
    RCC_WAKEUP = 145,
    #[doc = "146 - SAI4 global interrupt"]
    SAI4 = 146,
    #[doc = "147 - Digital temperature sensor interrupt"]
    DTS = 147,
    #[doc = "150 - IWDG1 early wake"]
    IWDG1_IT = 150,
    #[doc = "151 - IWDG2 early wake"]
    IWDG2_IT = 151,
    #[doc = "197 - TAMP tamper secure interrupt"]
    TAMP_S = 197,
    #[doc = "198 - RTC wakeup timer and alarms (A and B) secure interrupt"]
    RTC_WKUP_ALARM_S = 198,
    #[doc = "199 - RTC timestamp secure interrupt"]
    RTC_TS_S = 199,
    #[doc = "213 - DDR performance monitor interrupt"]
    DDRPERFM = 213,
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
#[doc = "ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4800_3000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "ADC"]
pub mod adc;
#[doc = "ADC2"]
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc2::RegisterBlock {
        0x4800_3100 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC2::ptr() }
    }
}
#[doc = "ADC2"]
pub mod adc2;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC_COMMON {}
impl ADC_COMMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc_common::RegisterBlock {
        0x4800_3300 as *const _
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
#[doc = "AXIMC_Mx"]
pub struct AXIMC_MX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AXIMC_MX {}
impl AXIMC_MX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aximc_mx::RegisterBlock {
        0x5704_2024 as *const _
    }
}
impl Deref for AXIMC_MX {
    type Target = aximc_mx::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AXIMC_MX::ptr() }
    }
}
#[doc = "AXIMC_Mx"]
pub mod aximc_mx;
#[doc = "BSEC2"]
pub struct BSEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BSEC {}
impl BSEC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bsec::RegisterBlock {
        0x5c00_5000 as *const _
    }
}
impl Deref for BSEC {
    type Target = bsec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BSEC::ptr() }
    }
}
#[doc = "BSEC2"]
pub mod bsec;
#[doc = "CCU"]
pub struct CCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU {}
impl CCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu::RegisterBlock {
        0x4401_0000 as *const _
    }
}
impl Deref for CCU {
    type Target = ccu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU::ptr() }
    }
}
#[doc = "CCU"]
pub mod ccu;
#[doc = "CRC1"]
pub struct CRC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC1 {}
impl CRC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc1::RegisterBlock {
        0x5800_9000 as *const _
    }
}
impl Deref for CRC1 {
    type Target = crc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC1::ptr() }
    }
}
#[doc = "CRC1"]
pub mod crc1;
#[doc = "CRC1"]
pub struct CRC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC2 {}
impl CRC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc1::RegisterBlock {
        0x4c00_4000 as *const _
    }
}
impl Deref for CRC2 {
    type Target = crc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC2::ptr() }
    }
}
#[doc = "CRYP1"]
pub struct CRYP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYP1 {}
impl CRYP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cryp1::RegisterBlock {
        0x5400_1000 as *const _
    }
}
impl Deref for CRYP1 {
    type Target = cryp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYP1::ptr() }
    }
}
#[doc = "CRYP1"]
pub mod cryp1;
#[doc = "CRYP1"]
pub struct CRYP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYP2 {}
impl CRYP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cryp1::RegisterBlock {
        0x4c00_5000 as *const _
    }
}
impl Deref for CRYP2 {
    type Target = cryp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYP2::ptr() }
    }
}
#[doc = "DAC1"]
pub struct DAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC1 {}
impl DAC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac1::RegisterBlock {
        0x4001_7000 as *const _
    }
}
impl Deref for DAC1 {
    type Target = dac1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC1::ptr() }
    }
}
#[doc = "DAC1"]
pub mod dac1;
#[doc = "DCMI"]
pub struct DCMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCMI {}
impl DCMI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dcmi::RegisterBlock {
        0x4c00_6000 as *const _
    }
}
impl Deref for DCMI {
    type Target = dcmi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DCMI::ptr() }
    }
}
#[doc = "DCMI"]
pub mod dcmi;
#[doc = "DDRCTRL"]
pub struct DDRCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DDRCTRL {}
impl DDRCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ddrctrl::RegisterBlock {
        0x5a00_3000 as *const _
    }
}
impl Deref for DDRCTRL {
    type Target = ddrctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DDRCTRL::ptr() }
    }
}
#[doc = "DDRCTRL"]
pub mod ddrctrl;
#[doc = "DDRPERFM"]
pub struct DDRPERFM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DDRPERFM {}
impl DDRPERFM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ddrperfm::RegisterBlock {
        0x5a00_7000 as *const _
    }
}
impl Deref for DDRPERFM {
    type Target = ddrperfm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DDRPERFM::ptr() }
    }
}
#[doc = "DDRPERFM"]
pub mod ddrperfm;
#[doc = "DDRPHYC"]
pub struct DDRPHYC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DDRPHYC {}
impl DDRPHYC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ddrphyc::RegisterBlock {
        0x5a00_4000 as *const _
    }
}
impl Deref for DDRPHYC {
    type Target = ddrphyc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DDRPHYC::ptr() }
    }
}
#[doc = "DDRPHYC"]
pub mod ddrphyc;
#[doc = "DFSDM1"]
pub struct DFSDM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DFSDM1 {}
impl DFSDM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dfsdm1::RegisterBlock {
        0x4400_d000 as *const _
    }
}
impl Deref for DFSDM1 {
    type Target = dfsdm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DFSDM1::ptr() }
    }
}
#[doc = "DFSDM1"]
pub mod dfsdm1;
#[doc = "DLYBQS"]
pub struct DLYBQS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLYBQS {}
impl DLYBQS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dlybqs::RegisterBlock {
        0x5800_4000 as *const _
    }
}
impl Deref for DLYBQS {
    type Target = dlybqs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DLYBQS::ptr() }
    }
}
#[doc = "DLYBQS"]
pub mod dlybqs;
#[doc = "DLYBSD1"]
pub struct DLYBSD1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLYBSD1 {}
impl DLYBSD1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dlybsd1::RegisterBlock {
        0x5800_6000 as *const _
    }
}
impl Deref for DLYBSD1 {
    type Target = dlybsd1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DLYBSD1::ptr() }
    }
}
#[doc = "DLYBSD1"]
pub mod dlybsd1;
#[doc = "DLYBSD1"]
pub struct DLYBSD2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLYBSD2 {}
impl DLYBSD2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dlybsd1::RegisterBlock {
        0x5800_8000 as *const _
    }
}
impl Deref for DLYBSD2 {
    type Target = dlybsd1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DLYBSD2::ptr() }
    }
}
#[doc = "DLYBSD1"]
pub struct DLYBSD3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLYBSD3 {}
impl DLYBSD3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dlybsd1::RegisterBlock {
        0x4800_5000 as *const _
    }
}
impl Deref for DLYBSD3 {
    type Target = dlybsd1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DLYBSD3::ptr() }
    }
}
#[doc = "DMA1"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4800_0000 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "DMA1"]
pub mod dma1;
#[doc = "DMA1"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4800_1000 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "DMAMUX1"]
pub struct DMAMUX1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX1 {}
impl DMAMUX1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmamux1::RegisterBlock {
        0x4800_2000 as *const _
    }
}
impl Deref for DMAMUX1 {
    type Target = dmamux1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAMUX1::ptr() }
    }
}
#[doc = "DMAMUX1"]
pub mod dmamux1;
#[doc = "DSIHOST1"]
pub struct DSIHOST1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSIHOST1 {}
impl DSIHOST1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dsihost1::RegisterBlock {
        0x5a00_0000 as *const _
    }
}
impl Deref for DSIHOST1 {
    type Target = dsihost1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DSIHOST1::ptr() }
    }
}
#[doc = "DSIHOST1"]
pub mod dsihost1;
#[doc = "DTS register block"]
pub struct DTS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DTS {}
impl DTS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dts::RegisterBlock {
        0x5002_8000 as *const _
    }
}
impl Deref for DTS {
    type Target = dts::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DTS::ptr() }
    }
}
#[doc = "DTS register block"]
pub mod dts;
#[doc = "ETH_MAC_MMC"]
pub struct ETH_MAC_MMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH_MAC_MMC {}
impl ETH_MAC_MMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eth_mac_mmc::RegisterBlock {
        0x5800_a000 as *const _
    }
}
impl Deref for ETH_MAC_MMC {
    type Target = eth_mac_mmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETH_MAC_MMC::ptr() }
    }
}
#[doc = "ETH_MAC_MMC"]
pub mod eth_mac_mmc;
#[doc = "ETH_MTL"]
pub struct ETH_MTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH_MTL {}
impl ETH_MTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eth_mtl::RegisterBlock {
        0x5800_ac00 as *const _
    }
}
impl Deref for ETH_MTL {
    type Target = eth_mtl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETH_MTL::ptr() }
    }
}
#[doc = "ETH_MTL"]
pub mod eth_mtl;
#[doc = "ETH_DMA"]
pub struct ETH_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH_DMA {}
impl ETH_DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eth_dma::RegisterBlock {
        0x5800_b000 as *const _
    }
}
impl Deref for ETH_DMA {
    type Target = eth_dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETH_DMA::ptr() }
    }
}
#[doc = "ETH_DMA"]
pub mod eth_dma;
#[doc = "ETZPC"]
pub struct ETZPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETZPC {}
impl ETZPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const etzpc::RegisterBlock {
        0x5c00_7000 as *const _
    }
}
impl Deref for ETZPC {
    type Target = etzpc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETZPC::ptr() }
    }
}
#[doc = "ETZPC"]
pub mod etzpc;
#[doc = "EXTI"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x5000_d000 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "EXTI"]
pub mod exti;
#[doc = "FDCAN1"]
pub struct FDCAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN1 {}
impl FDCAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fdcan1::RegisterBlock {
        0x4400_e000 as *const _
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
pub struct FDCAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN2 {}
impl FDCAN2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fdcan1::RegisterBlock {
        0x4400_f000 as *const _
    }
}
impl Deref for FDCAN2 {
    type Target = fdcan1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FDCAN2::ptr() }
    }
}
#[doc = "FMC register block"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x5800_2000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "FMC register block"]
pub mod fmc;
#[doc = "GICC"]
pub struct GICC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GICC {}
impl GICC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gicc::RegisterBlock {
        0xa002_2000 as *const _
    }
}
impl Deref for GICC {
    type Target = gicc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GICC::ptr() }
    }
}
#[doc = "GICC"]
pub mod gicc;
#[doc = "GICD"]
pub struct GICD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GICD {}
impl GICD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gicd::RegisterBlock {
        0xa002_1000 as *const _
    }
}
impl Deref for GICD {
    type Target = gicd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GICD::ptr() }
    }
}
#[doc = "GICD"]
pub mod gicd;
#[doc = "GICH"]
pub struct GICH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GICH {}
impl GICH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gich::RegisterBlock {
        0xa002_4000 as *const _
    }
}
impl Deref for GICH {
    type Target = gich::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GICH::ptr() }
    }
}
#[doc = "GICH"]
pub mod gich;
#[doc = "GICV"]
pub struct GICV {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GICV {}
impl GICV {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gicv::RegisterBlock {
        0xa002_6000 as *const _
    }
}
impl Deref for GICV {
    type Target = gicv::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GICV::ptr() }
    }
}
#[doc = "GICV"]
pub mod gicv;
#[doc = "GPIOA"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_2000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "GPIOA"]
pub mod gpioa;
#[doc = "GPIOB"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x5000_3000 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "GPIOB"]
pub mod gpiob;
#[doc = "GPIOC"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "GPIOC"]
pub mod gpioc;
#[doc = "GPIOD"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x5000_5000 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIOD"]
pub mod gpiod;
#[doc = "GPIOE"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioe::RegisterBlock {
        0x5000_6000 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioe::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "GPIOE"]
pub mod gpioe;
#[doc = "GPIOF"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiof::RegisterBlock {
        0x5000_7000 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpiof::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "GPIOF"]
pub mod gpiof;
#[doc = "GPIOG"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiog::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpiog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "GPIOG"]
pub mod gpiog;
#[doc = "GPIOH"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioh::RegisterBlock {
        0x5000_9000 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioh::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "GPIOH"]
pub mod gpioh;
#[doc = "GPIOI"]
pub struct GPIOI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOI {}
impl GPIOI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioi::RegisterBlock {
        0x5000_a000 as *const _
    }
}
impl Deref for GPIOI {
    type Target = gpioi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOI::ptr() }
    }
}
#[doc = "GPIOI"]
pub mod gpioi;
#[doc = "GPIOJ"]
pub struct GPIOJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOJ {}
impl GPIOJ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioj::RegisterBlock {
        0x5000_b000 as *const _
    }
}
impl Deref for GPIOJ {
    type Target = gpioj::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOJ::ptr() }
    }
}
#[doc = "GPIOJ"]
pub mod gpioj;
#[doc = "GPIOK"]
pub struct GPIOK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOK {}
impl GPIOK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiok::RegisterBlock {
        0x5000_c000 as *const _
    }
}
impl Deref for GPIOK {
    type Target = gpiok::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOK::ptr() }
    }
}
#[doc = "GPIOK"]
pub mod gpiok;
#[doc = "GPIOZ"]
pub struct GPIOZ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOZ {}
impl GPIOZ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioz::RegisterBlock {
        0x5400_4000 as *const _
    }
}
impl Deref for GPIOZ {
    type Target = gpioz::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOZ::ptr() }
    }
}
#[doc = "GPIOZ"]
pub mod gpioz;
#[doc = "HASH register block"]
pub struct HASH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HASH1 {}
impl HASH1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hash1::RegisterBlock {
        0x5400_2000 as *const _
    }
}
impl Deref for HASH1 {
    type Target = hash1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HASH1::ptr() }
    }
}
#[doc = "HASH register block"]
pub mod hash1;
#[doc = "HASH register block"]
pub struct HASH2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HASH2 {}
impl HASH2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hash2::RegisterBlock {
        0x4c00_2000 as *const _
    }
}
impl Deref for HASH2 {
    type Target = hash2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HASH2::ptr() }
    }
}
#[doc = "HASH register block"]
pub mod hash2;
#[doc = "HDMI_CEC"]
pub struct HDMI_CEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HDMI_CEC {}
impl HDMI_CEC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hdmi_cec::RegisterBlock {
        0x4001_6000 as *const _
    }
}
impl Deref for HDMI_CEC {
    type Target = hdmi_cec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HDMI_CEC::ptr() }
    }
}
#[doc = "HDMI_CEC"]
pub mod hdmi_cec;
#[doc = "HDP"]
pub struct HDP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HDP {}
impl HDP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hdp::RegisterBlock {
        0x5002_a000 as *const _
    }
}
impl Deref for HDP {
    type Target = hdp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HDP::ptr() }
    }
}
#[doc = "HDP"]
pub mod hdp;
#[doc = "HSEM"]
pub struct HSEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSEM {}
impl HSEM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsem::RegisterBlock {
        0x4c00_0000 as *const _
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
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4001_2000 as *const _
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
        0x4001_3000 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C1"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "I2C1"]
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x5c00_2000 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
#[doc = "I2C1"]
pub struct I2C5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C5 {}
impl I2C5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for I2C5 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C5::ptr() }
    }
}
#[doc = "I2C1"]
pub struct I2C6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C6 {}
impl I2C6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x5c00_9000 as *const _
    }
}
impl Deref for I2C6 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C6::ptr() }
    }
}
#[doc = "IPCC"]
pub struct IPCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IPCC {}
impl IPCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ipcc::RegisterBlock {
        0x4c00_1000 as *const _
    }
}
impl Deref for IPCC {
    type Target = ipcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IPCC::ptr() }
    }
}
#[doc = "IPCC"]
pub mod ipcc;
#[doc = "IWDG1"]
pub struct IWDG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG1 {}
impl IWDG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iwdg1::RegisterBlock {
        0x5c00_3000 as *const _
    }
}
impl Deref for IWDG1 {
    type Target = iwdg1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IWDG1::ptr() }
    }
}
#[doc = "IWDG1"]
pub mod iwdg1;
#[doc = "IWDG1"]
pub struct IWDG2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG2 {}
impl IWDG2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iwdg1::RegisterBlock {
        0x5a00_2000 as *const _
    }
}
impl Deref for IWDG2 {
    type Target = iwdg1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IWDG2::ptr() }
    }
}
#[doc = "LPTIM1"]
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM1::ptr() }
    }
}
#[doc = "LPTIM1"]
pub mod lptim1;
#[doc = "LPTIM1"]
pub struct LPTIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM2 {}
impl LPTIM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5002_1000 as *const _
    }
}
impl Deref for LPTIM2 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM2::ptr() }
    }
}
#[doc = "LPTIM1"]
pub struct LPTIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM3 {}
impl LPTIM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5002_2000 as *const _
    }
}
impl Deref for LPTIM3 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM3::ptr() }
    }
}
#[doc = "LPTIM1"]
pub struct LPTIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM4 {}
impl LPTIM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5002_3000 as *const _
    }
}
impl Deref for LPTIM4 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM4::ptr() }
    }
}
#[doc = "LPTIM1"]
pub struct LPTIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM5 {}
impl LPTIM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5002_4000 as *const _
    }
}
impl Deref for LPTIM5 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM5::ptr() }
    }
}
#[doc = "LTDC"]
pub struct LTDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LTDC {}
impl LTDC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ltdc::RegisterBlock {
        0x5a00_1000 as *const _
    }
}
impl Deref for LTDC {
    type Target = ltdc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LTDC::ptr() }
    }
}
#[doc = "LTDC"]
pub mod ltdc;
#[doc = "MDIOS"]
pub struct MDIOS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MDIOS {}
impl MDIOS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mdios::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for MDIOS {
    type Target = mdios::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MDIOS::ptr() }
    }
}
#[doc = "MDIOS"]
pub mod mdios;
#[doc = "MDMA1"]
pub struct MDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MDMA {}
impl MDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mdma::RegisterBlock {
        0x5800_0000 as *const _
    }
}
impl Deref for MDMA {
    type Target = mdma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MDMA::ptr() }
    }
}
#[doc = "MDMA1"]
pub mod mdma;
#[doc = "OTG"]
pub struct OTG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG {}
impl OTG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const otg::RegisterBlock {
        0x4900_0000 as *const _
    }
}
impl Deref for OTG {
    type Target = otg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OTG::ptr() }
    }
}
#[doc = "OTG"]
pub mod otg;
#[doc = "PWR"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        0x5000_1000 as *const _
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
#[doc = "QUADSPI1"]
pub struct QUADSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QUADSPI {}
impl QUADSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const quadspi::RegisterBlock {
        0x5800_3000 as *const _
    }
}
impl Deref for QUADSPI {
    type Target = quadspi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QUADSPI::ptr() }
    }
}
#[doc = "QUADSPI1"]
pub mod quadspi;
#[doc = "RCC"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "RCC"]
pub mod rcc;
#[doc = "RNG1"]
pub struct RNG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG1 {}
impl RNG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng1::RegisterBlock {
        0x5400_3000 as *const _
    }
}
impl Deref for RNG1 {
    type Target = rng1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG1::ptr() }
    }
}
#[doc = "RNG1"]
pub mod rng1;
#[doc = "RNG1"]
pub struct RNG2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG2 {}
impl RNG2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng1::RegisterBlock {
        0x4c00_3000 as *const _
    }
}
impl Deref for RNG2 {
    type Target = rng1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG2::ptr() }
    }
}
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x5c00_4000 as *const _
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
#[doc = "SAI1 register block"]
pub struct SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI1 {}
impl SAI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4400_a000 as *const _
    }
}
impl Deref for SAI1 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI1::ptr() }
    }
}
#[doc = "SAI1 register block"]
pub mod sai1;
#[doc = "SAI1 register block"]
pub struct SAI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI2 {}
impl SAI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4400_b000 as *const _
    }
}
impl Deref for SAI2 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI2::ptr() }
    }
}
#[doc = "SAI1 register block"]
pub struct SAI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI3 {}
impl SAI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4400_c000 as *const _
    }
}
impl Deref for SAI3 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI3::ptr() }
    }
}
#[doc = "SAI1 register block"]
pub struct SAI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI4 {}
impl SAI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x5002_7000 as *const _
    }
}
impl Deref for SAI4 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI4::ptr() }
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
        0x5800_5000 as *const _
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
        0x5800_7000 as *const _
    }
}
impl Deref for SDMMC2 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC2::ptr() }
    }
}
#[doc = "SDMMC1"]
pub struct SDMMC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC3 {}
impl SDMMC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc1::RegisterBlock {
        0x4800_4000 as *const _
    }
}
impl Deref for SDMMC3 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC3::ptr() }
    }
}
#[doc = "SPDIFRX"]
pub struct SPDIFRX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPDIFRX {}
impl SPDIFRX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spdifrx::RegisterBlock {
        0x4000_d000 as *const _
    }
}
impl Deref for SPDIFRX {
    type Target = spdifrx::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPDIFRX::ptr() }
    }
}
#[doc = "SPDIFRX"]
pub mod spdifrx;
#[doc = "SPI1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4400_4000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "SPI1"]
pub mod spi1;
#[doc = "SPI1"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_b000 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "SPI1"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "SPI1"]
pub struct SPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI4 {}
impl SPI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4400_5000 as *const _
    }
}
impl Deref for SPI4 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI4::ptr() }
    }
}
#[doc = "SPI1"]
pub struct SPI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI5 {}
impl SPI5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4400_9000 as *const _
    }
}
impl Deref for SPI5 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI5::ptr() }
    }
}
#[doc = "SPI1"]
pub struct SPI6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI6 {}
impl SPI6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x5c00_1000 as *const _
    }
}
impl Deref for SPI6 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI6::ptr() }
    }
}
#[doc = "STGENC"]
pub struct STGENC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STGENC {}
impl STGENC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const stgenc::RegisterBlock {
        0x5c00_8000 as *const _
    }
}
impl Deref for STGENC {
    type Target = stgenc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STGENC::ptr() }
    }
}
#[doc = "STGENC"]
pub mod stgenc;
#[doc = "STGENR"]
pub struct STGENR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STGENR {}
impl STGENR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const stgenr::RegisterBlock {
        0x5a00_5000 as *const _
    }
}
impl Deref for STGENR {
    type Target = stgenr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STGENR::ptr() }
    }
}
#[doc = "STGENR"]
pub mod stgenr;
#[doc = "SYSCFG"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x5002_0000 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "SYSCFG"]
pub mod syscfg;
#[doc = "TAMP"]
pub struct TAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TAMP {}
impl TAMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tamp::RegisterBlock {
        0x5c00_a000 as *const _
    }
}
impl Deref for TAMP {
    type Target = tamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TAMP::ptr() }
    }
}
#[doc = "TAMP"]
pub mod tamp;
#[doc = "TIM1"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4400_0000 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "TIM1"]
pub mod tim1;
#[doc = "TIM2"]
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
#[doc = "TIM2"]
pub mod tim2;
#[doc = "TIM3"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim3::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "TIM3"]
pub mod tim3;
#[doc = "TIM4"]
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim4::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM4::ptr() }
    }
}
#[doc = "TIM4"]
pub mod tim4;
#[doc = "TIM5"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim5::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "TIM5"]
pub mod tim5;
#[doc = "TIM6"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "TIM6"]
pub mod tim6;
#[doc = "TIM7"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim7::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim7::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "TIM7"]
pub mod tim7;
#[doc = "TIM8"]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim8::RegisterBlock {
        0x4400_1000 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim8::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM8::ptr() }
    }
}
#[doc = "TIM8"]
pub mod tim8;
#[doc = "TIM12"]
pub struct TIM12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM12 {}
impl TIM12 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim12::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for TIM12 {
    type Target = tim12::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM12::ptr() }
    }
}
#[doc = "TIM12"]
pub mod tim12;
#[doc = "TIM13"]
pub struct TIM13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM13 {}
impl TIM13 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim13::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for TIM13 {
    type Target = tim13::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM13::ptr() }
    }
}
#[doc = "TIM13"]
pub mod tim13;
#[doc = "TIM14"]
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM14 {}
impl TIM14 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim14::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for TIM14 {
    type Target = tim14::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM14::ptr() }
    }
}
#[doc = "TIM14"]
pub mod tim14;
#[doc = "TIM15"]
pub struct TIM15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM15 {}
impl TIM15 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim15::RegisterBlock {
        0x4400_6000 as *const _
    }
}
impl Deref for TIM15 {
    type Target = tim15::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM15::ptr() }
    }
}
#[doc = "TIM15"]
pub mod tim15;
#[doc = "TIM16"]
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x4400_7000 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM16::ptr() }
    }
}
#[doc = "TIM16"]
pub mod tim16;
#[doc = "TIM16"]
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x4400_8000 as *const _
    }
}
impl Deref for TIM17 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM17::ptr() }
    }
}
#[doc = "TZC"]
pub struct TZC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TZC {}
impl TZC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tzc::RegisterBlock {
        0x5c00_6000 as *const _
    }
}
impl Deref for TZC {
    type Target = tzc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TZC::ptr() }
    }
}
#[doc = "TZC"]
pub mod tzc;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x5c00_0000 as *const _
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
        0x4000_e000 as *const _
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
        0x4000_f000 as *const _
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
pub struct USART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART4 {}
impl USART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for USART4 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART4::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART5 {}
impl USART5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for USART5 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART5::ptr() }
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
        0x4400_3000 as *const _
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
pub struct USART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART7 {}
impl USART7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for USART7 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART7::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART8 {}
impl USART8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_9000 as *const _
    }
}
impl Deref for USART8 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART8::ptr() }
    }
}
#[doc = "USBPHYC"]
pub struct USBPHYC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBPHYC {}
impl USBPHYC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbphyc::RegisterBlock {
        0x5a00_6000 as *const _
    }
}
impl Deref for USBPHYC {
    type Target = usbphyc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBPHYC::ptr() }
    }
}
#[doc = "USBPHYC"]
pub mod usbphyc;
#[doc = "VREFBUF"]
pub struct VREFBUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREFBUF {}
impl VREFBUF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vrefbuf::RegisterBlock {
        0x5002_5000 as *const _
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
#[doc = "WWDG1"]
pub struct WWDG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG1 {}
impl WWDG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdg1::RegisterBlock {
        0x4000_a000 as *const _
    }
}
impl Deref for WWDG1 {
    type Target = wwdg1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDG1::ptr() }
    }
}
#[doc = "WWDG1"]
pub mod wwdg1;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[doc = "ADC_COMMON"]
    pub ADC_COMMON: ADC_COMMON,
    #[doc = "AXIMC_MX"]
    pub AXIMC_MX: AXIMC_MX,
    #[doc = "BSEC"]
    pub BSEC: BSEC,
    #[doc = "CCU"]
    pub CCU: CCU,
    #[doc = "CRC1"]
    pub CRC1: CRC1,
    #[doc = "CRC2"]
    pub CRC2: CRC2,
    #[doc = "CRYP1"]
    pub CRYP1: CRYP1,
    #[doc = "CRYP2"]
    pub CRYP2: CRYP2,
    #[doc = "DAC1"]
    pub DAC1: DAC1,
    #[doc = "DCMI"]
    pub DCMI: DCMI,
    #[doc = "DDRCTRL"]
    pub DDRCTRL: DDRCTRL,
    #[doc = "DDRPERFM"]
    pub DDRPERFM: DDRPERFM,
    #[doc = "DDRPHYC"]
    pub DDRPHYC: DDRPHYC,
    #[doc = "DFSDM1"]
    pub DFSDM1: DFSDM1,
    #[doc = "DLYBQS"]
    pub DLYBQS: DLYBQS,
    #[doc = "DLYBSD1"]
    pub DLYBSD1: DLYBSD1,
    #[doc = "DLYBSD2"]
    pub DLYBSD2: DLYBSD2,
    #[doc = "DLYBSD3"]
    pub DLYBSD3: DLYBSD3,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "DMAMUX1"]
    pub DMAMUX1: DMAMUX1,
    #[doc = "DSIHOST1"]
    pub DSIHOST1: DSIHOST1,
    #[doc = "DTS"]
    pub DTS: DTS,
    #[doc = "ETH_MAC_MMC"]
    pub ETH_MAC_MMC: ETH_MAC_MMC,
    #[doc = "ETH_MTL"]
    pub ETH_MTL: ETH_MTL,
    #[doc = "ETH_DMA"]
    pub ETH_DMA: ETH_DMA,
    #[doc = "ETZPC"]
    pub ETZPC: ETZPC,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "FDCAN1"]
    pub FDCAN1: FDCAN1,
    #[doc = "FDCAN2"]
    pub FDCAN2: FDCAN2,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "GICC"]
    pub GICC: GICC,
    #[doc = "GICD"]
    pub GICD: GICD,
    #[doc = "GICH"]
    pub GICH: GICH,
    #[doc = "GICV"]
    pub GICV: GICV,
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
    #[doc = "GPIOZ"]
    pub GPIOZ: GPIOZ,
    #[doc = "HASH1"]
    pub HASH1: HASH1,
    #[doc = "HASH2"]
    pub HASH2: HASH2,
    #[doc = "HDMI_CEC"]
    pub HDMI_CEC: HDMI_CEC,
    #[doc = "HDP"]
    pub HDP: HDP,
    #[doc = "HSEM"]
    pub HSEM: HSEM,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "I2C5"]
    pub I2C5: I2C5,
    #[doc = "I2C6"]
    pub I2C6: I2C6,
    #[doc = "IPCC"]
    pub IPCC: IPCC,
    #[doc = "IWDG1"]
    pub IWDG1: IWDG1,
    #[doc = "IWDG2"]
    pub IWDG2: IWDG2,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "LPTIM2"]
    pub LPTIM2: LPTIM2,
    #[doc = "LPTIM3"]
    pub LPTIM3: LPTIM3,
    #[doc = "LPTIM4"]
    pub LPTIM4: LPTIM4,
    #[doc = "LPTIM5"]
    pub LPTIM5: LPTIM5,
    #[doc = "LTDC"]
    pub LTDC: LTDC,
    #[doc = "MDIOS"]
    pub MDIOS: MDIOS,
    #[doc = "MDMA"]
    pub MDMA: MDMA,
    #[doc = "OTG"]
    pub OTG: OTG,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "QUADSPI"]
    pub QUADSPI: QUADSPI,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "RNG1"]
    pub RNG1: RNG1,
    #[doc = "RNG2"]
    pub RNG2: RNG2,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SAI1"]
    pub SAI1: SAI1,
    #[doc = "SAI2"]
    pub SAI2: SAI2,
    #[doc = "SAI3"]
    pub SAI3: SAI3,
    #[doc = "SAI4"]
    pub SAI4: SAI4,
    #[doc = "SDMMC1"]
    pub SDMMC1: SDMMC1,
    #[doc = "SDMMC2"]
    pub SDMMC2: SDMMC2,
    #[doc = "SDMMC3"]
    pub SDMMC3: SDMMC3,
    #[doc = "SPDIFRX"]
    pub SPDIFRX: SPDIFRX,
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
    #[doc = "STGENC"]
    pub STGENC: STGENC,
    #[doc = "STGENR"]
    pub STGENR: STGENR,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "TAMP"]
    pub TAMP: TAMP,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "TIM12"]
    pub TIM12: TIM12,
    #[doc = "TIM13"]
    pub TIM13: TIM13,
    #[doc = "TIM14"]
    pub TIM14: TIM14,
    #[doc = "TIM15"]
    pub TIM15: TIM15,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "TZC"]
    pub TZC: TZC,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "USART4"]
    pub USART4: USART4,
    #[doc = "USART5"]
    pub USART5: USART5,
    #[doc = "USART6"]
    pub USART6: USART6,
    #[doc = "USART7"]
    pub USART7: USART7,
    #[doc = "USART8"]
    pub USART8: USART8,
    #[doc = "USBPHYC"]
    pub USBPHYC: USBPHYC,
    #[doc = "VREFBUF"]
    pub VREFBUF: VREFBUF,
    #[doc = "WWDG1"]
    pub WWDG1: WWDG1,
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
            ADC: ADC {
                _marker: PhantomData,
            },
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            ADC_COMMON: ADC_COMMON {
                _marker: PhantomData,
            },
            AXIMC_MX: AXIMC_MX {
                _marker: PhantomData,
            },
            BSEC: BSEC {
                _marker: PhantomData,
            },
            CCU: CCU {
                _marker: PhantomData,
            },
            CRC1: CRC1 {
                _marker: PhantomData,
            },
            CRC2: CRC2 {
                _marker: PhantomData,
            },
            CRYP1: CRYP1 {
                _marker: PhantomData,
            },
            CRYP2: CRYP2 {
                _marker: PhantomData,
            },
            DAC1: DAC1 {
                _marker: PhantomData,
            },
            DCMI: DCMI {
                _marker: PhantomData,
            },
            DDRCTRL: DDRCTRL {
                _marker: PhantomData,
            },
            DDRPERFM: DDRPERFM {
                _marker: PhantomData,
            },
            DDRPHYC: DDRPHYC {
                _marker: PhantomData,
            },
            DFSDM1: DFSDM1 {
                _marker: PhantomData,
            },
            DLYBQS: DLYBQS {
                _marker: PhantomData,
            },
            DLYBSD1: DLYBSD1 {
                _marker: PhantomData,
            },
            DLYBSD2: DLYBSD2 {
                _marker: PhantomData,
            },
            DLYBSD3: DLYBSD3 {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            DMAMUX1: DMAMUX1 {
                _marker: PhantomData,
            },
            DSIHOST1: DSIHOST1 {
                _marker: PhantomData,
            },
            DTS: DTS {
                _marker: PhantomData,
            },
            ETH_MAC_MMC: ETH_MAC_MMC {
                _marker: PhantomData,
            },
            ETH_MTL: ETH_MTL {
                _marker: PhantomData,
            },
            ETH_DMA: ETH_DMA {
                _marker: PhantomData,
            },
            ETZPC: ETZPC {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            FDCAN1: FDCAN1 {
                _marker: PhantomData,
            },
            FDCAN2: FDCAN2 {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            GICC: GICC {
                _marker: PhantomData,
            },
            GICD: GICD {
                _marker: PhantomData,
            },
            GICH: GICH {
                _marker: PhantomData,
            },
            GICV: GICV {
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
            GPIOZ: GPIOZ {
                _marker: PhantomData,
            },
            HASH1: HASH1 {
                _marker: PhantomData,
            },
            HASH2: HASH2 {
                _marker: PhantomData,
            },
            HDMI_CEC: HDMI_CEC {
                _marker: PhantomData,
            },
            HDP: HDP {
                _marker: PhantomData,
            },
            HSEM: HSEM {
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
            I2C5: I2C5 {
                _marker: PhantomData,
            },
            I2C6: I2C6 {
                _marker: PhantomData,
            },
            IPCC: IPCC {
                _marker: PhantomData,
            },
            IWDG1: IWDG1 {
                _marker: PhantomData,
            },
            IWDG2: IWDG2 {
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
            LPTIM4: LPTIM4 {
                _marker: PhantomData,
            },
            LPTIM5: LPTIM5 {
                _marker: PhantomData,
            },
            LTDC: LTDC {
                _marker: PhantomData,
            },
            MDIOS: MDIOS {
                _marker: PhantomData,
            },
            MDMA: MDMA {
                _marker: PhantomData,
            },
            OTG: OTG {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            QUADSPI: QUADSPI {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            RNG1: RNG1 {
                _marker: PhantomData,
            },
            RNG2: RNG2 {
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
            SAI3: SAI3 {
                _marker: PhantomData,
            },
            SAI4: SAI4 {
                _marker: PhantomData,
            },
            SDMMC1: SDMMC1 {
                _marker: PhantomData,
            },
            SDMMC2: SDMMC2 {
                _marker: PhantomData,
            },
            SDMMC3: SDMMC3 {
                _marker: PhantomData,
            },
            SPDIFRX: SPDIFRX {
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
            STGENC: STGENC {
                _marker: PhantomData,
            },
            STGENR: STGENR {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            TAMP: TAMP {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
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
            TIM8: TIM8 {
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
            TIM15: TIM15 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            TZC: TZC {
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
            USART4: USART4 {
                _marker: PhantomData,
            },
            USART5: USART5 {
                _marker: PhantomData,
            },
            USART6: USART6 {
                _marker: PhantomData,
            },
            USART7: USART7 {
                _marker: PhantomData,
            },
            USART8: USART8 {
                _marker: PhantomData,
            },
            USBPHYC: USBPHYC {
                _marker: PhantomData,
            },
            VREFBUF: VREFBUF {
                _marker: PhantomData,
            },
            WWDG1: WWDG1 {
                _marker: PhantomData,
            },
        }
    }
}
