#[doc = "Reader of register CCR%s"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register CCR%s"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Input DMA request line selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMAREQ_ID_A {
    #[doc = "0: No signal selected as request input"]
    NONE = 0,
    #[doc = "1: Signal `dmamux1_req_gen0` selected as request input"]
    DMAMUX1_REQ_GEN0 = 1,
    #[doc = "2: Signal `dmamux1_req_gen1` selected as request input"]
    DMAMUX1_REQ_GEN1 = 2,
    #[doc = "3: Signal `dmamux1_req_gen2` selected as request input"]
    DMAMUX1_REQ_GEN2 = 3,
    #[doc = "4: Signal `dmamux1_req_gen3` selected as request input"]
    DMAMUX1_REQ_GEN3 = 4,
    #[doc = "5: Signal `dmamux1_req_gen4` selected as request input"]
    DMAMUX1_REQ_GEN4 = 5,
    #[doc = "6: Signal `dmamux1_req_gen5` selected as request input"]
    DMAMUX1_REQ_GEN5 = 6,
    #[doc = "7: Signal `dmamux1_req_gen6` selected as request input"]
    DMAMUX1_REQ_GEN6 = 7,
    #[doc = "8: Signal `dmamux1_req_gen7` selected as request input"]
    DMAMUX1_REQ_GEN7 = 8,
    #[doc = "9: Signal `adc1_dma` selected as request input"]
    ADC1_DMA = 9,
    #[doc = "10: Signal `adc2_dma` selected as request input"]
    ADC2_DMA = 10,
    #[doc = "11: Signal `tim1_ch1` selected as request input"]
    TIM1_CH1 = 11,
    #[doc = "12: Signal `tim1_ch2` selected as request input"]
    TIM1_CH2 = 12,
    #[doc = "13: Signal `tim1_ch3` selected as request input"]
    TIM1_CH3 = 13,
    #[doc = "14: Signal `tim1_ch4` selected as request input"]
    TIM1_CH4 = 14,
    #[doc = "15: Signal `tim1_up` selected as request input"]
    TIM1_UP = 15,
    #[doc = "16: Signal `tim1_trig` selected as request input"]
    TIM1_TRIG = 16,
    #[doc = "17: Signal `tim1_com` selected as request input"]
    TIM1_COM = 17,
    #[doc = "18: Signal `tim2_ch1` selected as request input"]
    TIM2_CH1 = 18,
    #[doc = "19: Signal `tim2_ch2` selected as request input"]
    TIM2_CH2 = 19,
    #[doc = "20: Signal `tim2_ch3` selected as request input"]
    TIM2_CH3 = 20,
    #[doc = "21: Signal `tim2_ch4` selected as request input"]
    TIM2_CH4 = 21,
    #[doc = "22: Signal `tim2_up` selected as request input"]
    TIM2_UP = 22,
    #[doc = "23: Signal `tim3_ch1` selected as request input"]
    TIM3_CH1 = 23,
    #[doc = "24: Signal `tim3_ch2` selected as request input"]
    TIM3_CH2 = 24,
    #[doc = "25: Signal `tim3_ch3` selected as request input"]
    TIM3_CH3 = 25,
    #[doc = "26: Signal `tim3_ch4` selected as request input"]
    TIM3_CH4 = 26,
    #[doc = "27: Signal `tim3_up` selected as request input"]
    TIM3_UP = 27,
    #[doc = "28: Signal `tim3_trig` selected as request input"]
    TIM3_TRIG = 28,
    #[doc = "29: Signal `tim4_ch1` selected as request input"]
    TIM4_CH1 = 29,
    #[doc = "30: Signal `tim4_ch2` selected as request input"]
    TIM4_CH2 = 30,
    #[doc = "31: Signal `tim4_ch3` selected as request input"]
    TIM4_CH3 = 31,
    #[doc = "32: Signal `tim4_up` selected as request input"]
    TIM4_UP = 32,
    #[doc = "33: Signal `i2c1_rx_dma` selected as request input"]
    I2C1_RX_DMA = 33,
    #[doc = "34: Signal `i2c1_tx_dma` selected as request input"]
    I2C1_TX_DMA = 34,
    #[doc = "35: Signal `i2c2_rx_dma` selected as request input"]
    I2C2_RX_DMA = 35,
    #[doc = "36: Signal `i2c2_tx_dma` selected as request input"]
    I2C2_TX_DMA = 36,
    #[doc = "37: Signal `spi1_rx_dma` selected as request input"]
    SPI1_RX_DMA = 37,
    #[doc = "38: Signal `spi1_tx_dma` selected as request input"]
    SPI1_TX_DMA = 38,
    #[doc = "39: Signal `spi2_rx_dma` selected as request input"]
    SPI2_RX_DMA = 39,
    #[doc = "40: Signal `spi2_tx_dma` selected as request input"]
    SPI2_TX_DMA = 40,
    #[doc = "41: Signal `usart1_rx_dma` selected as request input"]
    USART1_RX_DMA = 41,
    #[doc = "42: Signal `usart1_tx_dma` selected as request input"]
    USART1_TX_DMA = 42,
    #[doc = "43: Signal `usart2_rx_dma` selected as request input"]
    USART2_RX_DMA = 43,
    #[doc = "44: Signal `usart2_tx_dma` selected as request input"]
    USART2_TX_DMA = 44,
    #[doc = "45: Signal `usart3_rx_dma` selected as request input"]
    USART3_RX_DMA = 45,
    #[doc = "46: Signal `usart3_tx_dma` selected as request input"]
    USART3_TX_DMA = 46,
    #[doc = "47: Signal `tim8_ch1` selected as request input"]
    TIM8_CH1 = 47,
    #[doc = "48: Signal `tim8_ch2` selected as request input"]
    TIM8_CH2 = 48,
    #[doc = "49: Signal `tim8_ch3` selected as request input"]
    TIM8_CH3 = 49,
    #[doc = "50: Signal `tim8_ch4` selected as request input"]
    TIM8_CH4 = 50,
    #[doc = "51: Signal `tim8_up` selected as request input"]
    TIM8_UP = 51,
    #[doc = "52: Signal `tim8_trig` selected as request input"]
    TIM8_TRIG = 52,
    #[doc = "53: Signal `tim8_com` selected as request input"]
    TIM8_COM = 53,
    #[doc = "55: Signal `tim5_ch1` selected as request input"]
    TIM5_CH1 = 55,
    #[doc = "56: Signal `tim5_ch2` selected as request input"]
    TIM5_CH2 = 56,
    #[doc = "57: Signal `tim5_ch3` selected as request input"]
    TIM5_CH3 = 57,
    #[doc = "58: Signal `tim5_ch4` selected as request input"]
    TIM5_CH4 = 58,
    #[doc = "59: Signal `tim5_up` selected as request input"]
    TIM5_UP = 59,
    #[doc = "60: Signal `tim5_trig` selected as request input"]
    TIM5_TRIG = 60,
    #[doc = "61: Signal `spi3_rx_dma` selected as request input"]
    SPI3_RX_DMA = 61,
    #[doc = "62: Signal `spi3_tx_dma` selected as request input"]
    SPI3_TX_DMA = 62,
    #[doc = "63: Signal `uart4_rx_dma` selected as request input"]
    UART4_RX_DMA = 63,
    #[doc = "64: Signal `uart4_tx_dma` selected as request input"]
    UART4_TX_DMA = 64,
    #[doc = "65: Signal `uart5_rx_dma` selected as request input"]
    UART5_RX_DMA = 65,
    #[doc = "66: Signal `uart5_tx_dma` selected as request input"]
    UART5_TX_DMA = 66,
    #[doc = "67: Signal `dac_ch1_dma` selected as request input"]
    DAC_CH1_DMA = 67,
    #[doc = "68: Signal `dac_ch2_dma` selected as request input"]
    DAC_CH2_DMA = 68,
    #[doc = "69: Signal `tim6_up` selected as request input"]
    TIM6_UP = 69,
    #[doc = "70: Signal `tim7_up` selected as request input"]
    TIM7_UP = 70,
    #[doc = "71: Signal `usart6_rx_dma` selected as request input"]
    USART6_RX_DMA = 71,
    #[doc = "72: Signal `usart6_tx_dma` selected as request input"]
    USART6_TX_DMA = 72,
    #[doc = "73: Signal `i2c3_rx_dma` selected as request input"]
    I2C3_RX_DMA = 73,
    #[doc = "74: Signal `i2c3_tx_dma` selected as request input"]
    I2C3_TX_DMA = 74,
    #[doc = "75: Signal `dcmi_dma` selected as request input"]
    DCMI_DMA = 75,
    #[doc = "76: Signal `cryp_in_dma` selected as request input"]
    CRYP_IN_DMA = 76,
    #[doc = "77: Signal `cryp_out_dma` selected as request input"]
    CRYP_OUT_DMA = 77,
    #[doc = "78: Signal `hash_in_dma` selected as request input"]
    HASH_IN_DMA = 78,
    #[doc = "79: Signal `uart7_rx_dma` selected as request input"]
    UART7_RX_DMA = 79,
    #[doc = "80: Signal `uart7_tx_dma` selected as request input"]
    UART7_TX_DMA = 80,
    #[doc = "81: Signal `uart8_rx_dma` selected as request input"]
    UART8_RX_DMA = 81,
    #[doc = "82: Signal `uart8_tx_dma` selected as request input"]
    UART8_TX_DMA = 82,
    #[doc = "83: Signal `spi4_rx_dma` selected as request input"]
    SPI4_RX_DMA = 83,
    #[doc = "84: Signal `spi4_tx_dma` selected as request input"]
    SPI4_TX_DMA = 84,
    #[doc = "85: Signal `spi5_rx_dma` selected as request input"]
    SPI5_RX_DMA = 85,
    #[doc = "86: Signal `spi5_tx_dma` selected as request input"]
    SPI5_TX_DMA = 86,
    #[doc = "87: Signal `sai1a_dma` selected as request input"]
    SAI1A_DMA = 87,
    #[doc = "88: Signal `sai1b_dma` selected as request input"]
    SAI1B_DMA = 88,
    #[doc = "89: Signal `sai2a_dma` selected as request input"]
    SAI2A_DMA = 89,
    #[doc = "90: Signal `sai2b_dma` selected as request input"]
    SAI2B_DMA = 90,
    #[doc = "91: Signal `swpmi_rx_dma` selected as request input"]
    SWPMI_RX_DMA = 91,
    #[doc = "92: Signal `swpmi_tx_dma` selected as request input"]
    SWPMI_TX_DMA = 92,
    #[doc = "93: Signal `spdifrx_dat_dma` selected as request input"]
    SPDIFRX_DAT_DMA = 93,
    #[doc = "94: Signal `spdifrx_ctrl_dma` selected as request input"]
    SPDIFRX_CTRL_DMA = 94,
    #[doc = "95: Signal `hr_req(1)` selected as request input"]
    HR_REQ1 = 95,
    #[doc = "96: Signal `hr_req(2)` selected as request input"]
    HR_REQ2 = 96,
    #[doc = "97: Signal `hr_req(3)` selected as request input"]
    HR_REQ3 = 97,
    #[doc = "98: Signal `hr_req(4)` selected as request input"]
    HR_REQ4 = 98,
    #[doc = "99: Signal `hr_req(5)` selected as request input"]
    HR_REQ5 = 99,
    #[doc = "100: Signal `hr_req(6)` selected as request input"]
    HR_REQ6 = 100,
    #[doc = "101: Signal `dfsdm1_dma0` selected as request input"]
    DFSDM1_DMA0 = 101,
    #[doc = "102: Signal `dfsdm1_dma1` selected as request input"]
    DFSDM1_DMA1 = 102,
    #[doc = "103: Signal `dfsdm1_dma2` selected as request input"]
    DFSDM1_DMA2 = 103,
    #[doc = "104: Signal `dfsdm1_dma3` selected as request input"]
    DFSDM1_DMA3 = 104,
    #[doc = "105: Signal `tim15_ch1` selected as request input"]
    TIM15_CH1 = 105,
    #[doc = "106: Signal `tim15_up` selected as request input"]
    TIM15_UP = 106,
    #[doc = "107: Signal `tim15_trig` selected as request input"]
    TIM15_TRIG = 107,
    #[doc = "108: Signal `tim15_com` selected as request input"]
    TIM15_COM = 108,
    #[doc = "109: Signal `tim16_ch1` selected as request input"]
    TIM16_CH1 = 109,
    #[doc = "110: Signal `tim16_up` selected as request input"]
    TIM16_UP = 110,
    #[doc = "111: Signal `tim17_ch1` selected as request input"]
    TIM17_CH1 = 111,
    #[doc = "112: Signal `tim17_up` selected as request input"]
    TIM17_UP = 112,
    #[doc = "113: Signal `sai3_a_dma` selected as request input"]
    SAI3_A_DMA = 113,
    #[doc = "114: Signal `sai3_b_dma` selected as request input"]
    SAI3_B_DMA = 114,
    #[doc = "115: Signal `adc3_dma` selected as request input"]
    ADC3_DMA = 115,
}
impl From<DMAREQ_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: DMAREQ_ID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMAREQ_ID`"]
pub type DMAREQ_ID_R = crate::R<u8, DMAREQ_ID_A>;
impl DMAREQ_ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMAREQ_ID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMAREQ_ID_A::NONE),
            1 => Val(DMAREQ_ID_A::DMAMUX1_REQ_GEN0),
            2 => Val(DMAREQ_ID_A::DMAMUX1_REQ_GEN1),
            3 => Val(DMAREQ_ID_A::DMAMUX1_REQ_GEN2),
            4 => Val(DMAREQ_ID_A::DMAMUX1_REQ_GEN3),
            5 => Val(DMAREQ_ID_A::DMAMUX1_REQ_GEN4),
            6 => Val(DMAREQ_ID_A::DMAMUX1_REQ_GEN5),
            7 => Val(DMAREQ_ID_A::DMAMUX1_REQ_GEN6),
            8 => Val(DMAREQ_ID_A::DMAMUX1_REQ_GEN7),
            9 => Val(DMAREQ_ID_A::ADC1_DMA),
            10 => Val(DMAREQ_ID_A::ADC2_DMA),
            11 => Val(DMAREQ_ID_A::TIM1_CH1),
            12 => Val(DMAREQ_ID_A::TIM1_CH2),
            13 => Val(DMAREQ_ID_A::TIM1_CH3),
            14 => Val(DMAREQ_ID_A::TIM1_CH4),
            15 => Val(DMAREQ_ID_A::TIM1_UP),
            16 => Val(DMAREQ_ID_A::TIM1_TRIG),
            17 => Val(DMAREQ_ID_A::TIM1_COM),
            18 => Val(DMAREQ_ID_A::TIM2_CH1),
            19 => Val(DMAREQ_ID_A::TIM2_CH2),
            20 => Val(DMAREQ_ID_A::TIM2_CH3),
            21 => Val(DMAREQ_ID_A::TIM2_CH4),
            22 => Val(DMAREQ_ID_A::TIM2_UP),
            23 => Val(DMAREQ_ID_A::TIM3_CH1),
            24 => Val(DMAREQ_ID_A::TIM3_CH2),
            25 => Val(DMAREQ_ID_A::TIM3_CH3),
            26 => Val(DMAREQ_ID_A::TIM3_CH4),
            27 => Val(DMAREQ_ID_A::TIM3_UP),
            28 => Val(DMAREQ_ID_A::TIM3_TRIG),
            29 => Val(DMAREQ_ID_A::TIM4_CH1),
            30 => Val(DMAREQ_ID_A::TIM4_CH2),
            31 => Val(DMAREQ_ID_A::TIM4_CH3),
            32 => Val(DMAREQ_ID_A::TIM4_UP),
            33 => Val(DMAREQ_ID_A::I2C1_RX_DMA),
            34 => Val(DMAREQ_ID_A::I2C1_TX_DMA),
            35 => Val(DMAREQ_ID_A::I2C2_RX_DMA),
            36 => Val(DMAREQ_ID_A::I2C2_TX_DMA),
            37 => Val(DMAREQ_ID_A::SPI1_RX_DMA),
            38 => Val(DMAREQ_ID_A::SPI1_TX_DMA),
            39 => Val(DMAREQ_ID_A::SPI2_RX_DMA),
            40 => Val(DMAREQ_ID_A::SPI2_TX_DMA),
            41 => Val(DMAREQ_ID_A::USART1_RX_DMA),
            42 => Val(DMAREQ_ID_A::USART1_TX_DMA),
            43 => Val(DMAREQ_ID_A::USART2_RX_DMA),
            44 => Val(DMAREQ_ID_A::USART2_TX_DMA),
            45 => Val(DMAREQ_ID_A::USART3_RX_DMA),
            46 => Val(DMAREQ_ID_A::USART3_TX_DMA),
            47 => Val(DMAREQ_ID_A::TIM8_CH1),
            48 => Val(DMAREQ_ID_A::TIM8_CH2),
            49 => Val(DMAREQ_ID_A::TIM8_CH3),
            50 => Val(DMAREQ_ID_A::TIM8_CH4),
            51 => Val(DMAREQ_ID_A::TIM8_UP),
            52 => Val(DMAREQ_ID_A::TIM8_TRIG),
            53 => Val(DMAREQ_ID_A::TIM8_COM),
            55 => Val(DMAREQ_ID_A::TIM5_CH1),
            56 => Val(DMAREQ_ID_A::TIM5_CH2),
            57 => Val(DMAREQ_ID_A::TIM5_CH3),
            58 => Val(DMAREQ_ID_A::TIM5_CH4),
            59 => Val(DMAREQ_ID_A::TIM5_UP),
            60 => Val(DMAREQ_ID_A::TIM5_TRIG),
            61 => Val(DMAREQ_ID_A::SPI3_RX_DMA),
            62 => Val(DMAREQ_ID_A::SPI3_TX_DMA),
            63 => Val(DMAREQ_ID_A::UART4_RX_DMA),
            64 => Val(DMAREQ_ID_A::UART4_TX_DMA),
            65 => Val(DMAREQ_ID_A::UART5_RX_DMA),
            66 => Val(DMAREQ_ID_A::UART5_TX_DMA),
            67 => Val(DMAREQ_ID_A::DAC_CH1_DMA),
            68 => Val(DMAREQ_ID_A::DAC_CH2_DMA),
            69 => Val(DMAREQ_ID_A::TIM6_UP),
            70 => Val(DMAREQ_ID_A::TIM7_UP),
            71 => Val(DMAREQ_ID_A::USART6_RX_DMA),
            72 => Val(DMAREQ_ID_A::USART6_TX_DMA),
            73 => Val(DMAREQ_ID_A::I2C3_RX_DMA),
            74 => Val(DMAREQ_ID_A::I2C3_TX_DMA),
            75 => Val(DMAREQ_ID_A::DCMI_DMA),
            76 => Val(DMAREQ_ID_A::CRYP_IN_DMA),
            77 => Val(DMAREQ_ID_A::CRYP_OUT_DMA),
            78 => Val(DMAREQ_ID_A::HASH_IN_DMA),
            79 => Val(DMAREQ_ID_A::UART7_RX_DMA),
            80 => Val(DMAREQ_ID_A::UART7_TX_DMA),
            81 => Val(DMAREQ_ID_A::UART8_RX_DMA),
            82 => Val(DMAREQ_ID_A::UART8_TX_DMA),
            83 => Val(DMAREQ_ID_A::SPI4_RX_DMA),
            84 => Val(DMAREQ_ID_A::SPI4_TX_DMA),
            85 => Val(DMAREQ_ID_A::SPI5_RX_DMA),
            86 => Val(DMAREQ_ID_A::SPI5_TX_DMA),
            87 => Val(DMAREQ_ID_A::SAI1A_DMA),
            88 => Val(DMAREQ_ID_A::SAI1B_DMA),
            89 => Val(DMAREQ_ID_A::SAI2A_DMA),
            90 => Val(DMAREQ_ID_A::SAI2B_DMA),
            91 => Val(DMAREQ_ID_A::SWPMI_RX_DMA),
            92 => Val(DMAREQ_ID_A::SWPMI_TX_DMA),
            93 => Val(DMAREQ_ID_A::SPDIFRX_DAT_DMA),
            94 => Val(DMAREQ_ID_A::SPDIFRX_CTRL_DMA),
            95 => Val(DMAREQ_ID_A::HR_REQ1),
            96 => Val(DMAREQ_ID_A::HR_REQ2),
            97 => Val(DMAREQ_ID_A::HR_REQ3),
            98 => Val(DMAREQ_ID_A::HR_REQ4),
            99 => Val(DMAREQ_ID_A::HR_REQ5),
            100 => Val(DMAREQ_ID_A::HR_REQ6),
            101 => Val(DMAREQ_ID_A::DFSDM1_DMA0),
            102 => Val(DMAREQ_ID_A::DFSDM1_DMA1),
            103 => Val(DMAREQ_ID_A::DFSDM1_DMA2),
            104 => Val(DMAREQ_ID_A::DFSDM1_DMA3),
            105 => Val(DMAREQ_ID_A::TIM15_CH1),
            106 => Val(DMAREQ_ID_A::TIM15_UP),
            107 => Val(DMAREQ_ID_A::TIM15_TRIG),
            108 => Val(DMAREQ_ID_A::TIM15_COM),
            109 => Val(DMAREQ_ID_A::TIM16_CH1),
            110 => Val(DMAREQ_ID_A::TIM16_UP),
            111 => Val(DMAREQ_ID_A::TIM17_CH1),
            112 => Val(DMAREQ_ID_A::TIM17_UP),
            113 => Val(DMAREQ_ID_A::SAI3_A_DMA),
            114 => Val(DMAREQ_ID_A::SAI3_B_DMA),
            115 => Val(DMAREQ_ID_A::ADC3_DMA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DMAREQ_ID_A::NONE
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_REQ_GEN0`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen0(&self) -> bool {
        *self == DMAREQ_ID_A::DMAMUX1_REQ_GEN0
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_REQ_GEN1`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen1(&self) -> bool {
        *self == DMAREQ_ID_A::DMAMUX1_REQ_GEN1
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_REQ_GEN2`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen2(&self) -> bool {
        *self == DMAREQ_ID_A::DMAMUX1_REQ_GEN2
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_REQ_GEN3`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen3(&self) -> bool {
        *self == DMAREQ_ID_A::DMAMUX1_REQ_GEN3
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_REQ_GEN4`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen4(&self) -> bool {
        *self == DMAREQ_ID_A::DMAMUX1_REQ_GEN4
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_REQ_GEN5`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen5(&self) -> bool {
        *self == DMAREQ_ID_A::DMAMUX1_REQ_GEN5
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_REQ_GEN6`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen6(&self) -> bool {
        *self == DMAREQ_ID_A::DMAMUX1_REQ_GEN6
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_REQ_GEN7`"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen7(&self) -> bool {
        *self == DMAREQ_ID_A::DMAMUX1_REQ_GEN7
    }
    #[doc = "Checks if the value of the field is `ADC1_DMA`"]
    #[inline(always)]
    pub fn is_adc1_dma(&self) -> bool {
        *self == DMAREQ_ID_A::ADC1_DMA
    }
    #[doc = "Checks if the value of the field is `ADC2_DMA`"]
    #[inline(always)]
    pub fn is_adc2_dma(&self) -> bool {
        *self == DMAREQ_ID_A::ADC2_DMA
    }
    #[doc = "Checks if the value of the field is `TIM1_CH1`"]
    #[inline(always)]
    pub fn is_tim1_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::TIM1_CH1
    }
    #[doc = "Checks if the value of the field is `TIM1_CH2`"]
    #[inline(always)]
    pub fn is_tim1_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::TIM1_CH2
    }
    #[doc = "Checks if the value of the field is `TIM1_CH3`"]
    #[inline(always)]
    pub fn is_tim1_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::TIM1_CH3
    }
    #[doc = "Checks if the value of the field is `TIM1_CH4`"]
    #[inline(always)]
    pub fn is_tim1_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::TIM1_CH4
    }
    #[doc = "Checks if the value of the field is `TIM1_UP`"]
    #[inline(always)]
    pub fn is_tim1_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM1_UP
    }
    #[doc = "Checks if the value of the field is `TIM1_TRIG`"]
    #[inline(always)]
    pub fn is_tim1_trig(&self) -> bool {
        *self == DMAREQ_ID_A::TIM1_TRIG
    }
    #[doc = "Checks if the value of the field is `TIM1_COM`"]
    #[inline(always)]
    pub fn is_tim1_com(&self) -> bool {
        *self == DMAREQ_ID_A::TIM1_COM
    }
    #[doc = "Checks if the value of the field is `TIM2_CH1`"]
    #[inline(always)]
    pub fn is_tim2_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::TIM2_CH1
    }
    #[doc = "Checks if the value of the field is `TIM2_CH2`"]
    #[inline(always)]
    pub fn is_tim2_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::TIM2_CH2
    }
    #[doc = "Checks if the value of the field is `TIM2_CH3`"]
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::TIM2_CH3
    }
    #[doc = "Checks if the value of the field is `TIM2_CH4`"]
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::TIM2_CH4
    }
    #[doc = "Checks if the value of the field is `TIM2_UP`"]
    #[inline(always)]
    pub fn is_tim2_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM2_UP
    }
    #[doc = "Checks if the value of the field is `TIM3_CH1`"]
    #[inline(always)]
    pub fn is_tim3_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::TIM3_CH1
    }
    #[doc = "Checks if the value of the field is `TIM3_CH2`"]
    #[inline(always)]
    pub fn is_tim3_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::TIM3_CH2
    }
    #[doc = "Checks if the value of the field is `TIM3_CH3`"]
    #[inline(always)]
    pub fn is_tim3_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::TIM3_CH3
    }
    #[doc = "Checks if the value of the field is `TIM3_CH4`"]
    #[inline(always)]
    pub fn is_tim3_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::TIM3_CH4
    }
    #[doc = "Checks if the value of the field is `TIM3_UP`"]
    #[inline(always)]
    pub fn is_tim3_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM3_UP
    }
    #[doc = "Checks if the value of the field is `TIM3_TRIG`"]
    #[inline(always)]
    pub fn is_tim3_trig(&self) -> bool {
        *self == DMAREQ_ID_A::TIM3_TRIG
    }
    #[doc = "Checks if the value of the field is `TIM4_CH1`"]
    #[inline(always)]
    pub fn is_tim4_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::TIM4_CH1
    }
    #[doc = "Checks if the value of the field is `TIM4_CH2`"]
    #[inline(always)]
    pub fn is_tim4_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::TIM4_CH2
    }
    #[doc = "Checks if the value of the field is `TIM4_CH3`"]
    #[inline(always)]
    pub fn is_tim4_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::TIM4_CH3
    }
    #[doc = "Checks if the value of the field is `TIM4_UP`"]
    #[inline(always)]
    pub fn is_tim4_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM4_UP
    }
    #[doc = "Checks if the value of the field is `I2C1_RX_DMA`"]
    #[inline(always)]
    pub fn is_i2c1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2C1_RX_DMA
    }
    #[doc = "Checks if the value of the field is `I2C1_TX_DMA`"]
    #[inline(always)]
    pub fn is_i2c1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2C1_TX_DMA
    }
    #[doc = "Checks if the value of the field is `I2C2_RX_DMA`"]
    #[inline(always)]
    pub fn is_i2c2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2C2_RX_DMA
    }
    #[doc = "Checks if the value of the field is `I2C2_TX_DMA`"]
    #[inline(always)]
    pub fn is_i2c2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2C2_TX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI1_RX_DMA`"]
    #[inline(always)]
    pub fn is_spi1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPI1_RX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI1_TX_DMA`"]
    #[inline(always)]
    pub fn is_spi1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPI1_TX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI2_RX_DMA`"]
    #[inline(always)]
    pub fn is_spi2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPI2_RX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI2_TX_DMA`"]
    #[inline(always)]
    pub fn is_spi2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPI2_TX_DMA
    }
    #[doc = "Checks if the value of the field is `USART1_RX_DMA`"]
    #[inline(always)]
    pub fn is_usart1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::USART1_RX_DMA
    }
    #[doc = "Checks if the value of the field is `USART1_TX_DMA`"]
    #[inline(always)]
    pub fn is_usart1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::USART1_TX_DMA
    }
    #[doc = "Checks if the value of the field is `USART2_RX_DMA`"]
    #[inline(always)]
    pub fn is_usart2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::USART2_RX_DMA
    }
    #[doc = "Checks if the value of the field is `USART2_TX_DMA`"]
    #[inline(always)]
    pub fn is_usart2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::USART2_TX_DMA
    }
    #[doc = "Checks if the value of the field is `USART3_RX_DMA`"]
    #[inline(always)]
    pub fn is_usart3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::USART3_RX_DMA
    }
    #[doc = "Checks if the value of the field is `USART3_TX_DMA`"]
    #[inline(always)]
    pub fn is_usart3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::USART3_TX_DMA
    }
    #[doc = "Checks if the value of the field is `TIM8_CH1`"]
    #[inline(always)]
    pub fn is_tim8_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::TIM8_CH1
    }
    #[doc = "Checks if the value of the field is `TIM8_CH2`"]
    #[inline(always)]
    pub fn is_tim8_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::TIM8_CH2
    }
    #[doc = "Checks if the value of the field is `TIM8_CH3`"]
    #[inline(always)]
    pub fn is_tim8_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::TIM8_CH3
    }
    #[doc = "Checks if the value of the field is `TIM8_CH4`"]
    #[inline(always)]
    pub fn is_tim8_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::TIM8_CH4
    }
    #[doc = "Checks if the value of the field is `TIM8_UP`"]
    #[inline(always)]
    pub fn is_tim8_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM8_UP
    }
    #[doc = "Checks if the value of the field is `TIM8_TRIG`"]
    #[inline(always)]
    pub fn is_tim8_trig(&self) -> bool {
        *self == DMAREQ_ID_A::TIM8_TRIG
    }
    #[doc = "Checks if the value of the field is `TIM8_COM`"]
    #[inline(always)]
    pub fn is_tim8_com(&self) -> bool {
        *self == DMAREQ_ID_A::TIM8_COM
    }
    #[doc = "Checks if the value of the field is `TIM5_CH1`"]
    #[inline(always)]
    pub fn is_tim5_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::TIM5_CH1
    }
    #[doc = "Checks if the value of the field is `TIM5_CH2`"]
    #[inline(always)]
    pub fn is_tim5_ch2(&self) -> bool {
        *self == DMAREQ_ID_A::TIM5_CH2
    }
    #[doc = "Checks if the value of the field is `TIM5_CH3`"]
    #[inline(always)]
    pub fn is_tim5_ch3(&self) -> bool {
        *self == DMAREQ_ID_A::TIM5_CH3
    }
    #[doc = "Checks if the value of the field is `TIM5_CH4`"]
    #[inline(always)]
    pub fn is_tim5_ch4(&self) -> bool {
        *self == DMAREQ_ID_A::TIM5_CH4
    }
    #[doc = "Checks if the value of the field is `TIM5_UP`"]
    #[inline(always)]
    pub fn is_tim5_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM5_UP
    }
    #[doc = "Checks if the value of the field is `TIM5_TRIG`"]
    #[inline(always)]
    pub fn is_tim5_trig(&self) -> bool {
        *self == DMAREQ_ID_A::TIM5_TRIG
    }
    #[doc = "Checks if the value of the field is `SPI3_RX_DMA`"]
    #[inline(always)]
    pub fn is_spi3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPI3_RX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI3_TX_DMA`"]
    #[inline(always)]
    pub fn is_spi3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPI3_TX_DMA
    }
    #[doc = "Checks if the value of the field is `UART4_RX_DMA`"]
    #[inline(always)]
    pub fn is_uart4_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::UART4_RX_DMA
    }
    #[doc = "Checks if the value of the field is `UART4_TX_DMA`"]
    #[inline(always)]
    pub fn is_uart4_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::UART4_TX_DMA
    }
    #[doc = "Checks if the value of the field is `UART5_RX_DMA`"]
    #[inline(always)]
    pub fn is_uart5_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::UART5_RX_DMA
    }
    #[doc = "Checks if the value of the field is `UART5_TX_DMA`"]
    #[inline(always)]
    pub fn is_uart5_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::UART5_TX_DMA
    }
    #[doc = "Checks if the value of the field is `DAC_CH1_DMA`"]
    #[inline(always)]
    pub fn is_dac_ch1_dma(&self) -> bool {
        *self == DMAREQ_ID_A::DAC_CH1_DMA
    }
    #[doc = "Checks if the value of the field is `DAC_CH2_DMA`"]
    #[inline(always)]
    pub fn is_dac_ch2_dma(&self) -> bool {
        *self == DMAREQ_ID_A::DAC_CH2_DMA
    }
    #[doc = "Checks if the value of the field is `TIM6_UP`"]
    #[inline(always)]
    pub fn is_tim6_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM6_UP
    }
    #[doc = "Checks if the value of the field is `TIM7_UP`"]
    #[inline(always)]
    pub fn is_tim7_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM7_UP
    }
    #[doc = "Checks if the value of the field is `USART6_RX_DMA`"]
    #[inline(always)]
    pub fn is_usart6_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::USART6_RX_DMA
    }
    #[doc = "Checks if the value of the field is `USART6_TX_DMA`"]
    #[inline(always)]
    pub fn is_usart6_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::USART6_TX_DMA
    }
    #[doc = "Checks if the value of the field is `I2C3_RX_DMA`"]
    #[inline(always)]
    pub fn is_i2c3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2C3_RX_DMA
    }
    #[doc = "Checks if the value of the field is `I2C3_TX_DMA`"]
    #[inline(always)]
    pub fn is_i2c3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::I2C3_TX_DMA
    }
    #[doc = "Checks if the value of the field is `DCMI_DMA`"]
    #[inline(always)]
    pub fn is_dcmi_dma(&self) -> bool {
        *self == DMAREQ_ID_A::DCMI_DMA
    }
    #[doc = "Checks if the value of the field is `CRYP_IN_DMA`"]
    #[inline(always)]
    pub fn is_cryp_in_dma(&self) -> bool {
        *self == DMAREQ_ID_A::CRYP_IN_DMA
    }
    #[doc = "Checks if the value of the field is `CRYP_OUT_DMA`"]
    #[inline(always)]
    pub fn is_cryp_out_dma(&self) -> bool {
        *self == DMAREQ_ID_A::CRYP_OUT_DMA
    }
    #[doc = "Checks if the value of the field is `HASH_IN_DMA`"]
    #[inline(always)]
    pub fn is_hash_in_dma(&self) -> bool {
        *self == DMAREQ_ID_A::HASH_IN_DMA
    }
    #[doc = "Checks if the value of the field is `UART7_RX_DMA`"]
    #[inline(always)]
    pub fn is_uart7_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::UART7_RX_DMA
    }
    #[doc = "Checks if the value of the field is `UART7_TX_DMA`"]
    #[inline(always)]
    pub fn is_uart7_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::UART7_TX_DMA
    }
    #[doc = "Checks if the value of the field is `UART8_RX_DMA`"]
    #[inline(always)]
    pub fn is_uart8_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::UART8_RX_DMA
    }
    #[doc = "Checks if the value of the field is `UART8_TX_DMA`"]
    #[inline(always)]
    pub fn is_uart8_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::UART8_TX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI4_RX_DMA`"]
    #[inline(always)]
    pub fn is_spi4_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPI4_RX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI4_TX_DMA`"]
    #[inline(always)]
    pub fn is_spi4_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPI4_TX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI5_RX_DMA`"]
    #[inline(always)]
    pub fn is_spi5_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPI5_RX_DMA
    }
    #[doc = "Checks if the value of the field is `SPI5_TX_DMA`"]
    #[inline(always)]
    pub fn is_spi5_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPI5_TX_DMA
    }
    #[doc = "Checks if the value of the field is `SAI1A_DMA`"]
    #[inline(always)]
    pub fn is_sai1a_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SAI1A_DMA
    }
    #[doc = "Checks if the value of the field is `SAI1B_DMA`"]
    #[inline(always)]
    pub fn is_sai1b_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SAI1B_DMA
    }
    #[doc = "Checks if the value of the field is `SAI2A_DMA`"]
    #[inline(always)]
    pub fn is_sai2a_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SAI2A_DMA
    }
    #[doc = "Checks if the value of the field is `SAI2B_DMA`"]
    #[inline(always)]
    pub fn is_sai2b_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SAI2B_DMA
    }
    #[doc = "Checks if the value of the field is `SWPMI_RX_DMA`"]
    #[inline(always)]
    pub fn is_swpmi_rx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SWPMI_RX_DMA
    }
    #[doc = "Checks if the value of the field is `SWPMI_TX_DMA`"]
    #[inline(always)]
    pub fn is_swpmi_tx_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SWPMI_TX_DMA
    }
    #[doc = "Checks if the value of the field is `SPDIFRX_DAT_DMA`"]
    #[inline(always)]
    pub fn is_spdifrx_dat_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPDIFRX_DAT_DMA
    }
    #[doc = "Checks if the value of the field is `SPDIFRX_CTRL_DMA`"]
    #[inline(always)]
    pub fn is_spdifrx_ctrl_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SPDIFRX_CTRL_DMA
    }
    #[doc = "Checks if the value of the field is `HR_REQ1`"]
    #[inline(always)]
    pub fn is_hr_req1(&self) -> bool {
        *self == DMAREQ_ID_A::HR_REQ1
    }
    #[doc = "Checks if the value of the field is `HR_REQ2`"]
    #[inline(always)]
    pub fn is_hr_req2(&self) -> bool {
        *self == DMAREQ_ID_A::HR_REQ2
    }
    #[doc = "Checks if the value of the field is `HR_REQ3`"]
    #[inline(always)]
    pub fn is_hr_req3(&self) -> bool {
        *self == DMAREQ_ID_A::HR_REQ3
    }
    #[doc = "Checks if the value of the field is `HR_REQ4`"]
    #[inline(always)]
    pub fn is_hr_req4(&self) -> bool {
        *self == DMAREQ_ID_A::HR_REQ4
    }
    #[doc = "Checks if the value of the field is `HR_REQ5`"]
    #[inline(always)]
    pub fn is_hr_req5(&self) -> bool {
        *self == DMAREQ_ID_A::HR_REQ5
    }
    #[doc = "Checks if the value of the field is `HR_REQ6`"]
    #[inline(always)]
    pub fn is_hr_req6(&self) -> bool {
        *self == DMAREQ_ID_A::HR_REQ6
    }
    #[doc = "Checks if the value of the field is `DFSDM1_DMA0`"]
    #[inline(always)]
    pub fn is_dfsdm1_dma0(&self) -> bool {
        *self == DMAREQ_ID_A::DFSDM1_DMA0
    }
    #[doc = "Checks if the value of the field is `DFSDM1_DMA1`"]
    #[inline(always)]
    pub fn is_dfsdm1_dma1(&self) -> bool {
        *self == DMAREQ_ID_A::DFSDM1_DMA1
    }
    #[doc = "Checks if the value of the field is `DFSDM1_DMA2`"]
    #[inline(always)]
    pub fn is_dfsdm1_dma2(&self) -> bool {
        *self == DMAREQ_ID_A::DFSDM1_DMA2
    }
    #[doc = "Checks if the value of the field is `DFSDM1_DMA3`"]
    #[inline(always)]
    pub fn is_dfsdm1_dma3(&self) -> bool {
        *self == DMAREQ_ID_A::DFSDM1_DMA3
    }
    #[doc = "Checks if the value of the field is `TIM15_CH1`"]
    #[inline(always)]
    pub fn is_tim15_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::TIM15_CH1
    }
    #[doc = "Checks if the value of the field is `TIM15_UP`"]
    #[inline(always)]
    pub fn is_tim15_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM15_UP
    }
    #[doc = "Checks if the value of the field is `TIM15_TRIG`"]
    #[inline(always)]
    pub fn is_tim15_trig(&self) -> bool {
        *self == DMAREQ_ID_A::TIM15_TRIG
    }
    #[doc = "Checks if the value of the field is `TIM15_COM`"]
    #[inline(always)]
    pub fn is_tim15_com(&self) -> bool {
        *self == DMAREQ_ID_A::TIM15_COM
    }
    #[doc = "Checks if the value of the field is `TIM16_CH1`"]
    #[inline(always)]
    pub fn is_tim16_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::TIM16_CH1
    }
    #[doc = "Checks if the value of the field is `TIM16_UP`"]
    #[inline(always)]
    pub fn is_tim16_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM16_UP
    }
    #[doc = "Checks if the value of the field is `TIM17_CH1`"]
    #[inline(always)]
    pub fn is_tim17_ch1(&self) -> bool {
        *self == DMAREQ_ID_A::TIM17_CH1
    }
    #[doc = "Checks if the value of the field is `TIM17_UP`"]
    #[inline(always)]
    pub fn is_tim17_up(&self) -> bool {
        *self == DMAREQ_ID_A::TIM17_UP
    }
    #[doc = "Checks if the value of the field is `SAI3_A_DMA`"]
    #[inline(always)]
    pub fn is_sai3_a_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SAI3_A_DMA
    }
    #[doc = "Checks if the value of the field is `SAI3_B_DMA`"]
    #[inline(always)]
    pub fn is_sai3_b_dma(&self) -> bool {
        *self == DMAREQ_ID_A::SAI3_B_DMA
    }
    #[doc = "Checks if the value of the field is `ADC3_DMA`"]
    #[inline(always)]
    pub fn is_adc3_dma(&self) -> bool {
        *self == DMAREQ_ID_A::ADC3_DMA
    }
}
#[doc = "Write proxy for field `DMAREQ_ID`"]
pub struct DMAREQ_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREQ_ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAREQ_ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No signal selected as request input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::NONE)
    }
    #[doc = "Signal `dmamux1_req_gen0` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen0(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN0)
    }
    #[doc = "Signal `dmamux1_req_gen1` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN1)
    }
    #[doc = "Signal `dmamux1_req_gen2` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN2)
    }
    #[doc = "Signal `dmamux1_req_gen3` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN3)
    }
    #[doc = "Signal `dmamux1_req_gen4` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN4)
    }
    #[doc = "Signal `dmamux1_req_gen5` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen5(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN5)
    }
    #[doc = "Signal `dmamux1_req_gen6` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen6(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN6)
    }
    #[doc = "Signal `dmamux1_req_gen7` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen7(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DMAMUX1_REQ_GEN7)
    }
    #[doc = "Signal `adc1_dma` selected as request input"]
    #[inline(always)]
    pub fn adc1_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::ADC1_DMA)
    }
    #[doc = "Signal `adc2_dma` selected as request input"]
    #[inline(always)]
    pub fn adc2_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::ADC2_DMA)
    }
    #[doc = "Signal `tim1_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_CH1)
    }
    #[doc = "Signal `tim1_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_CH2)
    }
    #[doc = "Signal `tim1_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_CH3)
    }
    #[doc = "Signal `tim1_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_CH4)
    }
    #[doc = "Signal `tim1_up` selected as request input"]
    #[inline(always)]
    pub fn tim1_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_UP)
    }
    #[doc = "Signal `tim1_trig` selected as request input"]
    #[inline(always)]
    pub fn tim1_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_TRIG)
    }
    #[doc = "Signal `tim1_com` selected as request input"]
    #[inline(always)]
    pub fn tim1_com(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM1_COM)
    }
    #[doc = "Signal `tim2_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM2_CH1)
    }
    #[doc = "Signal `tim2_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM2_CH2)
    }
    #[doc = "Signal `tim2_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM2_CH3)
    }
    #[doc = "Signal `tim2_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM2_CH4)
    }
    #[doc = "Signal `tim2_up` selected as request input"]
    #[inline(always)]
    pub fn tim2_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM2_UP)
    }
    #[doc = "Signal `tim3_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim3_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM3_CH1)
    }
    #[doc = "Signal `tim3_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim3_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM3_CH2)
    }
    #[doc = "Signal `tim3_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim3_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM3_CH3)
    }
    #[doc = "Signal `tim3_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim3_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM3_CH4)
    }
    #[doc = "Signal `tim3_up` selected as request input"]
    #[inline(always)]
    pub fn tim3_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM3_UP)
    }
    #[doc = "Signal `tim3_trig` selected as request input"]
    #[inline(always)]
    pub fn tim3_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM3_TRIG)
    }
    #[doc = "Signal `tim4_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim4_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM4_CH1)
    }
    #[doc = "Signal `tim4_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim4_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM4_CH2)
    }
    #[doc = "Signal `tim4_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim4_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM4_CH3)
    }
    #[doc = "Signal `tim4_up` selected as request input"]
    #[inline(always)]
    pub fn tim4_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM4_UP)
    }
    #[doc = "Signal `i2c1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C1_RX_DMA)
    }
    #[doc = "Signal `i2c1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C1_TX_DMA)
    }
    #[doc = "Signal `i2c2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C2_RX_DMA)
    }
    #[doc = "Signal `i2c2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C2_TX_DMA)
    }
    #[doc = "Signal `spi1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI1_RX_DMA)
    }
    #[doc = "Signal `spi1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI1_TX_DMA)
    }
    #[doc = "Signal `spi2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI2_RX_DMA)
    }
    #[doc = "Signal `spi2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI2_TX_DMA)
    }
    #[doc = "Signal `usart1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart1_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART1_RX_DMA)
    }
    #[doc = "Signal `usart1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart1_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART1_TX_DMA)
    }
    #[doc = "Signal `usart2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart2_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART2_RX_DMA)
    }
    #[doc = "Signal `usart2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart2_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART2_TX_DMA)
    }
    #[doc = "Signal `usart3_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart3_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART3_RX_DMA)
    }
    #[doc = "Signal `usart3_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart3_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART3_TX_DMA)
    }
    #[doc = "Signal `tim8_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim8_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM8_CH1)
    }
    #[doc = "Signal `tim8_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim8_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM8_CH2)
    }
    #[doc = "Signal `tim8_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim8_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM8_CH3)
    }
    #[doc = "Signal `tim8_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim8_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM8_CH4)
    }
    #[doc = "Signal `tim8_up` selected as request input"]
    #[inline(always)]
    pub fn tim8_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM8_UP)
    }
    #[doc = "Signal `tim8_trig` selected as request input"]
    #[inline(always)]
    pub fn tim8_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM8_TRIG)
    }
    #[doc = "Signal `tim8_com` selected as request input"]
    #[inline(always)]
    pub fn tim8_com(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM8_COM)
    }
    #[doc = "Signal `tim5_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim5_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM5_CH1)
    }
    #[doc = "Signal `tim5_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim5_ch2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM5_CH2)
    }
    #[doc = "Signal `tim5_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim5_ch3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM5_CH3)
    }
    #[doc = "Signal `tim5_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim5_ch4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM5_CH4)
    }
    #[doc = "Signal `tim5_up` selected as request input"]
    #[inline(always)]
    pub fn tim5_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM5_UP)
    }
    #[doc = "Signal `tim5_trig` selected as request input"]
    #[inline(always)]
    pub fn tim5_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM5_TRIG)
    }
    #[doc = "Signal `spi3_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi3_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI3_RX_DMA)
    }
    #[doc = "Signal `spi3_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi3_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI3_TX_DMA)
    }
    #[doc = "Signal `uart4_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart4_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::UART4_RX_DMA)
    }
    #[doc = "Signal `uart4_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart4_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::UART4_TX_DMA)
    }
    #[doc = "Signal `uart5_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart5_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::UART5_RX_DMA)
    }
    #[doc = "Signal `uart5_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart5_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::UART5_TX_DMA)
    }
    #[doc = "Signal `dac_ch1_dma` selected as request input"]
    #[inline(always)]
    pub fn dac_ch1_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DAC_CH1_DMA)
    }
    #[doc = "Signal `dac_ch2_dma` selected as request input"]
    #[inline(always)]
    pub fn dac_ch2_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DAC_CH2_DMA)
    }
    #[doc = "Signal `tim6_up` selected as request input"]
    #[inline(always)]
    pub fn tim6_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM6_UP)
    }
    #[doc = "Signal `tim7_up` selected as request input"]
    #[inline(always)]
    pub fn tim7_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM7_UP)
    }
    #[doc = "Signal `usart6_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart6_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART6_RX_DMA)
    }
    #[doc = "Signal `usart6_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart6_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::USART6_TX_DMA)
    }
    #[doc = "Signal `i2c3_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c3_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C3_RX_DMA)
    }
    #[doc = "Signal `i2c3_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c3_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::I2C3_TX_DMA)
    }
    #[doc = "Signal `dcmi_dma` selected as request input"]
    #[inline(always)]
    pub fn dcmi_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DCMI_DMA)
    }
    #[doc = "Signal `cryp_in_dma` selected as request input"]
    #[inline(always)]
    pub fn cryp_in_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::CRYP_IN_DMA)
    }
    #[doc = "Signal `cryp_out_dma` selected as request input"]
    #[inline(always)]
    pub fn cryp_out_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::CRYP_OUT_DMA)
    }
    #[doc = "Signal `hash_in_dma` selected as request input"]
    #[inline(always)]
    pub fn hash_in_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HASH_IN_DMA)
    }
    #[doc = "Signal `uart7_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart7_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::UART7_RX_DMA)
    }
    #[doc = "Signal `uart7_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart7_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::UART7_TX_DMA)
    }
    #[doc = "Signal `uart8_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart8_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::UART8_RX_DMA)
    }
    #[doc = "Signal `uart8_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart8_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::UART8_TX_DMA)
    }
    #[doc = "Signal `spi4_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi4_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI4_RX_DMA)
    }
    #[doc = "Signal `spi4_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi4_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI4_TX_DMA)
    }
    #[doc = "Signal `spi5_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi5_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI5_RX_DMA)
    }
    #[doc = "Signal `spi5_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi5_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPI5_TX_DMA)
    }
    #[doc = "Signal `sai1a_dma` selected as request input"]
    #[inline(always)]
    pub fn sai1a_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SAI1A_DMA)
    }
    #[doc = "Signal `sai1b_dma` selected as request input"]
    #[inline(always)]
    pub fn sai1b_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SAI1B_DMA)
    }
    #[doc = "Signal `sai2a_dma` selected as request input"]
    #[inline(always)]
    pub fn sai2a_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SAI2A_DMA)
    }
    #[doc = "Signal `sai2b_dma` selected as request input"]
    #[inline(always)]
    pub fn sai2b_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SAI2B_DMA)
    }
    #[doc = "Signal `swpmi_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn swpmi_rx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SWPMI_RX_DMA)
    }
    #[doc = "Signal `swpmi_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn swpmi_tx_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SWPMI_TX_DMA)
    }
    #[doc = "Signal `spdifrx_dat_dma` selected as request input"]
    #[inline(always)]
    pub fn spdifrx_dat_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPDIFRX_DAT_DMA)
    }
    #[doc = "Signal `spdifrx_ctrl_dma` selected as request input"]
    #[inline(always)]
    pub fn spdifrx_ctrl_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SPDIFRX_CTRL_DMA)
    }
    #[doc = "Signal `hr_req(1)` selected as request input"]
    #[inline(always)]
    pub fn hr_req1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HR_REQ1)
    }
    #[doc = "Signal `hr_req(2)` selected as request input"]
    #[inline(always)]
    pub fn hr_req2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HR_REQ2)
    }
    #[doc = "Signal `hr_req(3)` selected as request input"]
    #[inline(always)]
    pub fn hr_req3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HR_REQ3)
    }
    #[doc = "Signal `hr_req(4)` selected as request input"]
    #[inline(always)]
    pub fn hr_req4(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HR_REQ4)
    }
    #[doc = "Signal `hr_req(5)` selected as request input"]
    #[inline(always)]
    pub fn hr_req5(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HR_REQ5)
    }
    #[doc = "Signal `hr_req(6)` selected as request input"]
    #[inline(always)]
    pub fn hr_req6(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::HR_REQ6)
    }
    #[doc = "Signal `dfsdm1_dma0` selected as request input"]
    #[inline(always)]
    pub fn dfsdm1_dma0(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DFSDM1_DMA0)
    }
    #[doc = "Signal `dfsdm1_dma1` selected as request input"]
    #[inline(always)]
    pub fn dfsdm1_dma1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DFSDM1_DMA1)
    }
    #[doc = "Signal `dfsdm1_dma2` selected as request input"]
    #[inline(always)]
    pub fn dfsdm1_dma2(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DFSDM1_DMA2)
    }
    #[doc = "Signal `dfsdm1_dma3` selected as request input"]
    #[inline(always)]
    pub fn dfsdm1_dma3(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::DFSDM1_DMA3)
    }
    #[doc = "Signal `tim15_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim15_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM15_CH1)
    }
    #[doc = "Signal `tim15_up` selected as request input"]
    #[inline(always)]
    pub fn tim15_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM15_UP)
    }
    #[doc = "Signal `tim15_trig` selected as request input"]
    #[inline(always)]
    pub fn tim15_trig(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM15_TRIG)
    }
    #[doc = "Signal `tim15_com` selected as request input"]
    #[inline(always)]
    pub fn tim15_com(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM15_COM)
    }
    #[doc = "Signal `tim16_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim16_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM16_CH1)
    }
    #[doc = "Signal `tim16_up` selected as request input"]
    #[inline(always)]
    pub fn tim16_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM16_UP)
    }
    #[doc = "Signal `tim17_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim17_ch1(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM17_CH1)
    }
    #[doc = "Signal `tim17_up` selected as request input"]
    #[inline(always)]
    pub fn tim17_up(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::TIM17_UP)
    }
    #[doc = "Signal `sai3_a_dma` selected as request input"]
    #[inline(always)]
    pub fn sai3_a_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SAI3_A_DMA)
    }
    #[doc = "Signal `sai3_b_dma` selected as request input"]
    #[inline(always)]
    pub fn sai3_b_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::SAI3_B_DMA)
    }
    #[doc = "Signal `adc3_dma` selected as request input"]
    #[inline(always)]
    pub fn adc3_dma(self) -> &'a mut W {
        self.variant(DMAREQ_ID_A::ADC3_DMA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Interrupt enable at synchronization event overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOIE_A {
    #[doc = "0: Synchronization overrun interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Synchronization overrun interrupt enabled"]
    ENABLED = 1,
}
impl From<SOIE_A> for bool {
    #[inline(always)]
    fn from(variant: SOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOIE`"]
pub type SOIE_R = crate::R<bool, SOIE_A>;
impl SOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOIE_A {
        match self.bits {
            false => SOIE_A::DISABLED,
            true => SOIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SOIE`"]
pub struct SOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOIE_A::DISABLED)
    }
    #[doc = "Synchronization overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOIE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Event generation enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EGE_A {
    #[doc = "0: Event generation disabled"]
    DISABLED = 0,
    #[doc = "1: Event generation enabled"]
    ENABLED = 1,
}
impl From<EGE_A> for bool {
    #[inline(always)]
    fn from(variant: EGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EGE`"]
pub type EGE_R = crate::R<bool, EGE_A>;
impl EGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EGE_A {
        match self.bits {
            false => EGE_A::DISABLED,
            true => EGE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EGE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EGE_A::ENABLED
    }
}
#[doc = "Write proxy for field `EGE`"]
pub struct EGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EGE_A::DISABLED)
    }
    #[doc = "Event generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EGE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Synchronous operating mode enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SE_A {
    #[doc = "0: Synchronization disabled"]
    DISABLED = 0,
    #[doc = "1: Synchronization enabled"]
    ENABLED = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SE`"]
pub type SE_R = crate::R<bool, SE_A>;
impl SE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::DISABLED,
            true => SE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SE_A::ENABLED
    }
}
#[doc = "Write proxy for field `SE`"]
pub struct SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SE_A::DISABLED)
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SE_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Synchronization event type selector Defines the synchronization event on the selected synchronization input:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPOL_A {
    #[doc = "0: No event, i.e. no synchronization nor detection"]
    NOEDGE = 0,
    #[doc = "1: Rising edge"]
    RISINGEDGE = 1,
    #[doc = "2: Falling edge"]
    FALLINGEDGE = 2,
    #[doc = "3: Rising and falling edges"]
    BOTHEDGES = 3,
}
impl From<SPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPOL`"]
pub type SPOL_R = crate::R<u8, SPOL_A>;
impl SPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            0 => SPOL_A::NOEDGE,
            1 => SPOL_A::RISINGEDGE,
            2 => SPOL_A::FALLINGEDGE,
            3 => SPOL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOEDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == SPOL_A::NOEDGE
    }
    #[doc = "Checks if the value of the field is `RISINGEDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SPOL_A::RISINGEDGE
    }
    #[doc = "Checks if the value of the field is `FALLINGEDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SPOL_A::FALLINGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == SPOL_A::BOTHEDGES
    }
}
#[doc = "Write proxy for field `SPOL`"]
pub struct SPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No event, i.e. no synchronization nor detection"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(SPOL_A::NOEDGE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(SPOL_A::RISINGEDGE)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(SPOL_A::FALLINGEDGE)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(SPOL_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `NBREQ`"]
pub type NBREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBREQ`"]
pub struct NBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NBREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Synchronization input selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNC_ID_A {
    #[doc = "0: Signal `dmamux1_evt0` selected as synchronization input"]
    DMAMUX1_EVT0 = 0,
    #[doc = "1: Signal `dmamux1_evt1` selected as synchronization input"]
    DMAMUX1_EVT1 = 1,
    #[doc = "2: Signal `dmamux1_evt2` selected as synchronization input"]
    DMAMUX1_EVT2 = 2,
    #[doc = "3: Signal `lptim1_out` selected as synchronization input"]
    LPTIM1_OUT = 3,
    #[doc = "4: Signal `lptim2_out` selected as synchronization input"]
    LPTIM2_OUT = 4,
    #[doc = "5: Signal `lptim3_out` selected as synchronization input"]
    LPTIM3_OUT = 5,
    #[doc = "6: Signal `extit0` selected as synchronization input"]
    EXTIT0 = 6,
    #[doc = "7: Signal `tim12_trgo` selected as synchronization input"]
    TIM12_TRGO = 7,
}
impl From<SYNC_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_ID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SYNC_ID`"]
pub type SYNC_ID_R = crate::R<u8, SYNC_ID_A>;
impl SYNC_ID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SYNC_ID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SYNC_ID_A::DMAMUX1_EVT0),
            1 => Val(SYNC_ID_A::DMAMUX1_EVT1),
            2 => Val(SYNC_ID_A::DMAMUX1_EVT2),
            3 => Val(SYNC_ID_A::LPTIM1_OUT),
            4 => Val(SYNC_ID_A::LPTIM2_OUT),
            5 => Val(SYNC_ID_A::LPTIM3_OUT),
            6 => Val(SYNC_ID_A::EXTIT0),
            7 => Val(SYNC_ID_A::TIM12_TRGO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_EVT0`"]
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        *self == SYNC_ID_A::DMAMUX1_EVT0
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_EVT1`"]
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        *self == SYNC_ID_A::DMAMUX1_EVT1
    }
    #[doc = "Checks if the value of the field is `DMAMUX1_EVT2`"]
    #[inline(always)]
    pub fn is_dmamux1_evt2(&self) -> bool {
        *self == SYNC_ID_A::DMAMUX1_EVT2
    }
    #[doc = "Checks if the value of the field is `LPTIM1_OUT`"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == SYNC_ID_A::LPTIM1_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM2_OUT`"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SYNC_ID_A::LPTIM2_OUT
    }
    #[doc = "Checks if the value of the field is `LPTIM3_OUT`"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SYNC_ID_A::LPTIM3_OUT
    }
    #[doc = "Checks if the value of the field is `EXTIT0`"]
    #[inline(always)]
    pub fn is_extit0(&self) -> bool {
        *self == SYNC_ID_A::EXTIT0
    }
    #[doc = "Checks if the value of the field is `TIM12_TRGO`"]
    #[inline(always)]
    pub fn is_tim12_trgo(&self) -> bool {
        *self == SYNC_ID_A::TIM12_TRGO
    }
}
#[doc = "Write proxy for field `SYNC_ID`"]
pub struct SYNC_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_ID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNC_ID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal `dmamux1_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX1_EVT0)
    }
    #[doc = "Signal `dmamux1_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX1_EVT1)
    }
    #[doc = "Signal `dmamux1_evt2` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt2(self) -> &'a mut W {
        self.variant(SYNC_ID_A::DMAMUX1_EVT2)
    }
    #[doc = "Signal `lptim1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::LPTIM1_OUT)
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::LPTIM2_OUT)
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SYNC_ID_A::LPTIM3_OUT)
    }
    #[doc = "Signal `extit0` selected as synchronization input"]
    #[inline(always)]
    pub fn extit0(self) -> &'a mut W {
        self.variant(SYNC_ID_A::EXTIT0)
    }
    #[doc = "Signal `tim12_trgo` selected as synchronization input"]
    #[inline(always)]
    pub fn tim12_trgo(self) -> &'a mut W {
        self.variant(SYNC_ID_A::TIM12_TRGO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Input DMA request line selected"]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input DMA request line selected"]
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W {
        DMAREQ_ID_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W {
        SOIE_W { w: self }
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W {
        EGE_W { w: self }
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&mut self) -> SE_W {
        SE_W { w: self }
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W {
        SPOL_W { w: self }
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W {
        NBREQ_W { w: self }
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W {
        SYNC_ID_W { w: self }
    }
}
