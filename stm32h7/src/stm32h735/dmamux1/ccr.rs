#[doc = "Register `C%sCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `C%sCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Input DMA request line selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAREQ_ID {
    #[doc = "0: No signal selected as request input"]
    None = 0,
    #[doc = "1: Signal `dmamux1_req_gen0` selected as request input"]
    Dmamux1ReqGen0 = 1,
    #[doc = "2: Signal `dmamux1_req_gen1` selected as request input"]
    Dmamux1ReqGen1 = 2,
    #[doc = "3: Signal `dmamux1_req_gen2` selected as request input"]
    Dmamux1ReqGen2 = 3,
    #[doc = "4: Signal `dmamux1_req_gen3` selected as request input"]
    Dmamux1ReqGen3 = 4,
    #[doc = "5: Signal `dmamux1_req_gen4` selected as request input"]
    Dmamux1ReqGen4 = 5,
    #[doc = "6: Signal `dmamux1_req_gen5` selected as request input"]
    Dmamux1ReqGen5 = 6,
    #[doc = "7: Signal `dmamux1_req_gen6` selected as request input"]
    Dmamux1ReqGen6 = 7,
    #[doc = "8: Signal `dmamux1_req_gen7` selected as request input"]
    Dmamux1ReqGen7 = 8,
    #[doc = "9: Signal `adc1_dma` selected as request input"]
    Adc1Dma = 9,
    #[doc = "10: Signal `adc2_dma` selected as request input"]
    Adc2Dma = 10,
    #[doc = "11: Signal `tim1_ch1` selected as request input"]
    Tim1Ch1 = 11,
    #[doc = "12: Signal `tim1_ch2` selected as request input"]
    Tim1Ch2 = 12,
    #[doc = "13: Signal `tim1_ch3` selected as request input"]
    Tim1Ch3 = 13,
    #[doc = "14: Signal `tim1_ch4` selected as request input"]
    Tim1Ch4 = 14,
    #[doc = "15: Signal `tim1_up` selected as request input"]
    Tim1Up = 15,
    #[doc = "16: Signal `tim1_trig` selected as request input"]
    Tim1Trig = 16,
    #[doc = "17: Signal `tim1_com` selected as request input"]
    Tim1Com = 17,
    #[doc = "18: Signal `tim2_ch1` selected as request input"]
    Tim2Ch1 = 18,
    #[doc = "19: Signal `tim2_ch2` selected as request input"]
    Tim2Ch2 = 19,
    #[doc = "20: Signal `tim2_ch3` selected as request input"]
    Tim2Ch3 = 20,
    #[doc = "21: Signal `tim2_ch4` selected as request input"]
    Tim2Ch4 = 21,
    #[doc = "22: Signal `tim2_up` selected as request input"]
    Tim2Up = 22,
    #[doc = "23: Signal `tim3_ch1` selected as request input"]
    Tim3Ch1 = 23,
    #[doc = "24: Signal `tim3_ch2` selected as request input"]
    Tim3Ch2 = 24,
    #[doc = "25: Signal `tim3_ch3` selected as request input"]
    Tim3Ch3 = 25,
    #[doc = "26: Signal `tim3_ch4` selected as request input"]
    Tim3Ch4 = 26,
    #[doc = "27: Signal `tim3_up` selected as request input"]
    Tim3Up = 27,
    #[doc = "28: Signal `tim3_trig` selected as request input"]
    Tim3Trig = 28,
    #[doc = "29: Signal `tim4_ch1` selected as request input"]
    Tim4Ch1 = 29,
    #[doc = "30: Signal `tim4_ch2` selected as request input"]
    Tim4Ch2 = 30,
    #[doc = "31: Signal `tim4_ch3` selected as request input"]
    Tim4Ch3 = 31,
    #[doc = "32: Signal `tim4_up` selected as request input"]
    Tim4Up = 32,
    #[doc = "33: Signal `i2c1_rx_dma` selected as request input"]
    I2c1RxDma = 33,
    #[doc = "34: Signal `i2c1_tx_dma` selected as request input"]
    I2c1TxDma = 34,
    #[doc = "35: Signal `i2c2_rx_dma` selected as request input"]
    I2c2RxDma = 35,
    #[doc = "36: Signal `i2c2_tx_dma` selected as request input"]
    I2c2TxDma = 36,
    #[doc = "37: Signal `spi1_rx_dma` selected as request input"]
    Spi1RxDma = 37,
    #[doc = "38: Signal `spi1_tx_dma` selected as request input"]
    Spi1TxDma = 38,
    #[doc = "39: Signal `spi2_rx_dma` selected as request input"]
    Spi2RxDma = 39,
    #[doc = "40: Signal `spi2_tx_dma` selected as request input"]
    Spi2TxDma = 40,
    #[doc = "41: Signal `usart1_rx_dma` selected as request input"]
    Usart1RxDma = 41,
    #[doc = "42: Signal `usart1_tx_dma` selected as request input"]
    Usart1TxDma = 42,
    #[doc = "43: Signal `usart2_rx_dma` selected as request input"]
    Usart2RxDma = 43,
    #[doc = "44: Signal `usart2_tx_dma` selected as request input"]
    Usart2TxDma = 44,
    #[doc = "45: Signal `usart3_rx_dma` selected as request input"]
    Usart3RxDma = 45,
    #[doc = "46: Signal `usart3_tx_dma` selected as request input"]
    Usart3TxDma = 46,
    #[doc = "47: Signal `tim8_ch1` selected as request input"]
    Tim8Ch1 = 47,
    #[doc = "48: Signal `tim8_ch2` selected as request input"]
    Tim8Ch2 = 48,
    #[doc = "49: Signal `tim8_ch3` selected as request input"]
    Tim8Ch3 = 49,
    #[doc = "50: Signal `tim8_ch4` selected as request input"]
    Tim8Ch4 = 50,
    #[doc = "51: Signal `tim8_up` selected as request input"]
    Tim8Up = 51,
    #[doc = "52: Signal `tim8_trig` selected as request input"]
    Tim8Trig = 52,
    #[doc = "53: Signal `tim8_com` selected as request input"]
    Tim8Com = 53,
    #[doc = "55: Signal `tim5_ch1` selected as request input"]
    Tim5Ch1 = 55,
    #[doc = "56: Signal `tim5_ch2` selected as request input"]
    Tim5Ch2 = 56,
    #[doc = "57: Signal `tim5_ch3` selected as request input"]
    Tim5Ch3 = 57,
    #[doc = "58: Signal `tim5_ch4` selected as request input"]
    Tim5Ch4 = 58,
    #[doc = "59: Signal `tim5_up` selected as request input"]
    Tim5Up = 59,
    #[doc = "60: Signal `tim5_trig` selected as request input"]
    Tim5Trig = 60,
    #[doc = "61: Signal `spi3_rx_dma` selected as request input"]
    Spi3RxDma = 61,
    #[doc = "62: Signal `spi3_tx_dma` selected as request input"]
    Spi3TxDma = 62,
    #[doc = "63: Signal `uart4_rx_dma` selected as request input"]
    Uart4RxDma = 63,
    #[doc = "64: Signal `uart4_tx_dma` selected as request input"]
    Uart4TxDma = 64,
    #[doc = "65: Signal `uart5_rx_dma` selected as request input"]
    Uart5RxDma = 65,
    #[doc = "66: Signal `uart5_tx_dma` selected as request input"]
    Uart5TxDma = 66,
    #[doc = "67: Signal `dac_ch1_dma` selected as request input"]
    DacCh1Dma = 67,
    #[doc = "68: Signal `dac_ch2_dma` selected as request input"]
    DacCh2Dma = 68,
    #[doc = "69: Signal `tim6_up` selected as request input"]
    Tim6Up = 69,
    #[doc = "70: Signal `tim7_up` selected as request input"]
    Tim7Up = 70,
    #[doc = "71: Signal `usart6_rx_dma` selected as request input"]
    Usart6RxDma = 71,
    #[doc = "72: Signal `usart6_tx_dma` selected as request input"]
    Usart6TxDma = 72,
    #[doc = "73: Signal `i2c3_rx_dma` selected as request input"]
    I2c3RxDma = 73,
    #[doc = "74: Signal `i2c3_tx_dma` selected as request input"]
    I2c3TxDma = 74,
    #[doc = "75: Signal `dcmi_dma` selected as request input"]
    DcmiDma = 75,
    #[doc = "76: Signal `cryp_in_dma` selected as request input"]
    CrypInDma = 76,
    #[doc = "77: Signal `cryp_out_dma` selected as request input"]
    CrypOutDma = 77,
    #[doc = "78: Signal `hash_in_dma` selected as request input"]
    HashInDma = 78,
    #[doc = "79: Signal `uart7_rx_dma` selected as request input"]
    Uart7RxDma = 79,
    #[doc = "80: Signal `uart7_tx_dma` selected as request input"]
    Uart7TxDma = 80,
    #[doc = "81: Signal `uart8_rx_dma` selected as request input"]
    Uart8RxDma = 81,
    #[doc = "82: Signal `uart8_tx_dma` selected as request input"]
    Uart8TxDma = 82,
    #[doc = "83: Signal `spi4_rx_dma` selected as request input"]
    Spi4RxDma = 83,
    #[doc = "84: Signal `spi4_tx_dma` selected as request input"]
    Spi4TxDma = 84,
    #[doc = "85: Signal `spi5_rx_dma` selected as request input"]
    Spi5RxDma = 85,
    #[doc = "86: Signal `spi5_tx_dma` selected as request input"]
    Spi5TxDma = 86,
    #[doc = "87: Signal `sai1a_dma` selected as request input"]
    Sai1aDma = 87,
    #[doc = "88: Signal `sai1b_dma` selected as request input"]
    Sai1bDma = 88,
    #[doc = "89: Signal `sai2a_dma` selected as request input"]
    Sai2aDma = 89,
    #[doc = "90: Signal `sai2b_dma` selected as request input"]
    Sai2bDma = 90,
    #[doc = "91: Signal `swpmi_rx_dma` selected as request input"]
    SwpmiRxDma = 91,
    #[doc = "92: Signal `swpmi_tx_dma` selected as request input"]
    SwpmiTxDma = 92,
    #[doc = "93: Signal `spdifrx_dat_dma` selected as request input"]
    SpdifrxDatDma = 93,
    #[doc = "94: Signal `spdifrx_ctrl_dma` selected as request input"]
    SpdifrxCtrlDma = 94,
    #[doc = "95: Signal `hr_req(1)` selected as request input"]
    HrReq1 = 95,
    #[doc = "96: Signal `hr_req(2)` selected as request input"]
    HrReq2 = 96,
    #[doc = "97: Signal `hr_req(3)` selected as request input"]
    HrReq3 = 97,
    #[doc = "98: Signal `hr_req(4)` selected as request input"]
    HrReq4 = 98,
    #[doc = "99: Signal `hr_req(5)` selected as request input"]
    HrReq5 = 99,
    #[doc = "100: Signal `hr_req(6)` selected as request input"]
    HrReq6 = 100,
    #[doc = "101: Signal `dfsdm1_dma0` selected as request input"]
    Dfsdm1Dma0 = 101,
    #[doc = "102: Signal `dfsdm1_dma1` selected as request input"]
    Dfsdm1Dma1 = 102,
    #[doc = "103: Signal `dfsdm1_dma2` selected as request input"]
    Dfsdm1Dma2 = 103,
    #[doc = "104: Signal `dfsdm1_dma3` selected as request input"]
    Dfsdm1Dma3 = 104,
    #[doc = "105: Signal `tim15_ch1` selected as request input"]
    Tim15Ch1 = 105,
    #[doc = "106: Signal `tim15_up` selected as request input"]
    Tim15Up = 106,
    #[doc = "107: Signal `tim15_trig` selected as request input"]
    Tim15Trig = 107,
    #[doc = "108: Signal `tim15_com` selected as request input"]
    Tim15Com = 108,
    #[doc = "109: Signal `tim16_ch1` selected as request input"]
    Tim16Ch1 = 109,
    #[doc = "110: Signal `tim16_up` selected as request input"]
    Tim16Up = 110,
    #[doc = "111: Signal `tim17_ch1` selected as request input"]
    Tim17Ch1 = 111,
    #[doc = "112: Signal `tim17_up` selected as request input"]
    Tim17Up = 112,
    #[doc = "113: Signal `sai3_a_dma` selected as request input"]
    Sai3ADma = 113,
    #[doc = "114: Signal `sai3_b_dma` selected as request input"]
    Sai3BDma = 114,
    #[doc = "115: Signal `adc3_dma` selected as request input"]
    Adc3Dma = 115,
}
impl From<DMAREQ_ID> for u8 {
    #[inline(always)]
    fn from(variant: DMAREQ_ID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMAREQ_ID {
    type Ux = u8;
}
#[doc = "Field `DMAREQ_ID` reader - Input DMA request line selected"]
pub type DMAREQ_ID_R = crate::FieldReader<DMAREQ_ID>;
impl DMAREQ_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DMAREQ_ID> {
        match self.bits {
            0 => Some(DMAREQ_ID::None),
            1 => Some(DMAREQ_ID::Dmamux1ReqGen0),
            2 => Some(DMAREQ_ID::Dmamux1ReqGen1),
            3 => Some(DMAREQ_ID::Dmamux1ReqGen2),
            4 => Some(DMAREQ_ID::Dmamux1ReqGen3),
            5 => Some(DMAREQ_ID::Dmamux1ReqGen4),
            6 => Some(DMAREQ_ID::Dmamux1ReqGen5),
            7 => Some(DMAREQ_ID::Dmamux1ReqGen6),
            8 => Some(DMAREQ_ID::Dmamux1ReqGen7),
            9 => Some(DMAREQ_ID::Adc1Dma),
            10 => Some(DMAREQ_ID::Adc2Dma),
            11 => Some(DMAREQ_ID::Tim1Ch1),
            12 => Some(DMAREQ_ID::Tim1Ch2),
            13 => Some(DMAREQ_ID::Tim1Ch3),
            14 => Some(DMAREQ_ID::Tim1Ch4),
            15 => Some(DMAREQ_ID::Tim1Up),
            16 => Some(DMAREQ_ID::Tim1Trig),
            17 => Some(DMAREQ_ID::Tim1Com),
            18 => Some(DMAREQ_ID::Tim2Ch1),
            19 => Some(DMAREQ_ID::Tim2Ch2),
            20 => Some(DMAREQ_ID::Tim2Ch3),
            21 => Some(DMAREQ_ID::Tim2Ch4),
            22 => Some(DMAREQ_ID::Tim2Up),
            23 => Some(DMAREQ_ID::Tim3Ch1),
            24 => Some(DMAREQ_ID::Tim3Ch2),
            25 => Some(DMAREQ_ID::Tim3Ch3),
            26 => Some(DMAREQ_ID::Tim3Ch4),
            27 => Some(DMAREQ_ID::Tim3Up),
            28 => Some(DMAREQ_ID::Tim3Trig),
            29 => Some(DMAREQ_ID::Tim4Ch1),
            30 => Some(DMAREQ_ID::Tim4Ch2),
            31 => Some(DMAREQ_ID::Tim4Ch3),
            32 => Some(DMAREQ_ID::Tim4Up),
            33 => Some(DMAREQ_ID::I2c1RxDma),
            34 => Some(DMAREQ_ID::I2c1TxDma),
            35 => Some(DMAREQ_ID::I2c2RxDma),
            36 => Some(DMAREQ_ID::I2c2TxDma),
            37 => Some(DMAREQ_ID::Spi1RxDma),
            38 => Some(DMAREQ_ID::Spi1TxDma),
            39 => Some(DMAREQ_ID::Spi2RxDma),
            40 => Some(DMAREQ_ID::Spi2TxDma),
            41 => Some(DMAREQ_ID::Usart1RxDma),
            42 => Some(DMAREQ_ID::Usart1TxDma),
            43 => Some(DMAREQ_ID::Usart2RxDma),
            44 => Some(DMAREQ_ID::Usart2TxDma),
            45 => Some(DMAREQ_ID::Usart3RxDma),
            46 => Some(DMAREQ_ID::Usart3TxDma),
            47 => Some(DMAREQ_ID::Tim8Ch1),
            48 => Some(DMAREQ_ID::Tim8Ch2),
            49 => Some(DMAREQ_ID::Tim8Ch3),
            50 => Some(DMAREQ_ID::Tim8Ch4),
            51 => Some(DMAREQ_ID::Tim8Up),
            52 => Some(DMAREQ_ID::Tim8Trig),
            53 => Some(DMAREQ_ID::Tim8Com),
            55 => Some(DMAREQ_ID::Tim5Ch1),
            56 => Some(DMAREQ_ID::Tim5Ch2),
            57 => Some(DMAREQ_ID::Tim5Ch3),
            58 => Some(DMAREQ_ID::Tim5Ch4),
            59 => Some(DMAREQ_ID::Tim5Up),
            60 => Some(DMAREQ_ID::Tim5Trig),
            61 => Some(DMAREQ_ID::Spi3RxDma),
            62 => Some(DMAREQ_ID::Spi3TxDma),
            63 => Some(DMAREQ_ID::Uart4RxDma),
            64 => Some(DMAREQ_ID::Uart4TxDma),
            65 => Some(DMAREQ_ID::Uart5RxDma),
            66 => Some(DMAREQ_ID::Uart5TxDma),
            67 => Some(DMAREQ_ID::DacCh1Dma),
            68 => Some(DMAREQ_ID::DacCh2Dma),
            69 => Some(DMAREQ_ID::Tim6Up),
            70 => Some(DMAREQ_ID::Tim7Up),
            71 => Some(DMAREQ_ID::Usart6RxDma),
            72 => Some(DMAREQ_ID::Usart6TxDma),
            73 => Some(DMAREQ_ID::I2c3RxDma),
            74 => Some(DMAREQ_ID::I2c3TxDma),
            75 => Some(DMAREQ_ID::DcmiDma),
            76 => Some(DMAREQ_ID::CrypInDma),
            77 => Some(DMAREQ_ID::CrypOutDma),
            78 => Some(DMAREQ_ID::HashInDma),
            79 => Some(DMAREQ_ID::Uart7RxDma),
            80 => Some(DMAREQ_ID::Uart7TxDma),
            81 => Some(DMAREQ_ID::Uart8RxDma),
            82 => Some(DMAREQ_ID::Uart8TxDma),
            83 => Some(DMAREQ_ID::Spi4RxDma),
            84 => Some(DMAREQ_ID::Spi4TxDma),
            85 => Some(DMAREQ_ID::Spi5RxDma),
            86 => Some(DMAREQ_ID::Spi5TxDma),
            87 => Some(DMAREQ_ID::Sai1aDma),
            88 => Some(DMAREQ_ID::Sai1bDma),
            89 => Some(DMAREQ_ID::Sai2aDma),
            90 => Some(DMAREQ_ID::Sai2bDma),
            91 => Some(DMAREQ_ID::SwpmiRxDma),
            92 => Some(DMAREQ_ID::SwpmiTxDma),
            93 => Some(DMAREQ_ID::SpdifrxDatDma),
            94 => Some(DMAREQ_ID::SpdifrxCtrlDma),
            95 => Some(DMAREQ_ID::HrReq1),
            96 => Some(DMAREQ_ID::HrReq2),
            97 => Some(DMAREQ_ID::HrReq3),
            98 => Some(DMAREQ_ID::HrReq4),
            99 => Some(DMAREQ_ID::HrReq5),
            100 => Some(DMAREQ_ID::HrReq6),
            101 => Some(DMAREQ_ID::Dfsdm1Dma0),
            102 => Some(DMAREQ_ID::Dfsdm1Dma1),
            103 => Some(DMAREQ_ID::Dfsdm1Dma2),
            104 => Some(DMAREQ_ID::Dfsdm1Dma3),
            105 => Some(DMAREQ_ID::Tim15Ch1),
            106 => Some(DMAREQ_ID::Tim15Up),
            107 => Some(DMAREQ_ID::Tim15Trig),
            108 => Some(DMAREQ_ID::Tim15Com),
            109 => Some(DMAREQ_ID::Tim16Ch1),
            110 => Some(DMAREQ_ID::Tim16Up),
            111 => Some(DMAREQ_ID::Tim17Ch1),
            112 => Some(DMAREQ_ID::Tim17Up),
            113 => Some(DMAREQ_ID::Sai3ADma),
            114 => Some(DMAREQ_ID::Sai3BDma),
            115 => Some(DMAREQ_ID::Adc3Dma),
            _ => None,
        }
    }
    #[doc = "No signal selected as request input"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DMAREQ_ID::None
    }
    #[doc = "Signal `dmamux1_req_gen0` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen0(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen0
    }
    #[doc = "Signal `dmamux1_req_gen1` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen1(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen1
    }
    #[doc = "Signal `dmamux1_req_gen2` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen2(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen2
    }
    #[doc = "Signal `dmamux1_req_gen3` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen3(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen3
    }
    #[doc = "Signal `dmamux1_req_gen4` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen4(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen4
    }
    #[doc = "Signal `dmamux1_req_gen5` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen5(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen5
    }
    #[doc = "Signal `dmamux1_req_gen6` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen6(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen6
    }
    #[doc = "Signal `dmamux1_req_gen7` selected as request input"]
    #[inline(always)]
    pub fn is_dmamux1_req_gen7(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen7
    }
    #[doc = "Signal `adc1_dma` selected as request input"]
    #[inline(always)]
    pub fn is_adc1_dma(&self) -> bool {
        *self == DMAREQ_ID::Adc1Dma
    }
    #[doc = "Signal `adc2_dma` selected as request input"]
    #[inline(always)]
    pub fn is_adc2_dma(&self) -> bool {
        *self == DMAREQ_ID::Adc2Dma
    }
    #[doc = "Signal `tim1_ch1` selected as request input"]
    #[inline(always)]
    pub fn is_tim1_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch1
    }
    #[doc = "Signal `tim1_ch2` selected as request input"]
    #[inline(always)]
    pub fn is_tim1_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch2
    }
    #[doc = "Signal `tim1_ch3` selected as request input"]
    #[inline(always)]
    pub fn is_tim1_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch3
    }
    #[doc = "Signal `tim1_ch4` selected as request input"]
    #[inline(always)]
    pub fn is_tim1_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch4
    }
    #[doc = "Signal `tim1_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim1_up(&self) -> bool {
        *self == DMAREQ_ID::Tim1Up
    }
    #[doc = "Signal `tim1_trig` selected as request input"]
    #[inline(always)]
    pub fn is_tim1_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim1Trig
    }
    #[doc = "Signal `tim1_com` selected as request input"]
    #[inline(always)]
    pub fn is_tim1_com(&self) -> bool {
        *self == DMAREQ_ID::Tim1Com
    }
    #[doc = "Signal `tim2_ch1` selected as request input"]
    #[inline(always)]
    pub fn is_tim2_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch1
    }
    #[doc = "Signal `tim2_ch2` selected as request input"]
    #[inline(always)]
    pub fn is_tim2_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch2
    }
    #[doc = "Signal `tim2_ch3` selected as request input"]
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch3
    }
    #[doc = "Signal `tim2_ch4` selected as request input"]
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch4
    }
    #[doc = "Signal `tim2_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim2_up(&self) -> bool {
        *self == DMAREQ_ID::Tim2Up
    }
    #[doc = "Signal `tim3_ch1` selected as request input"]
    #[inline(always)]
    pub fn is_tim3_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim3Ch1
    }
    #[doc = "Signal `tim3_ch2` selected as request input"]
    #[inline(always)]
    pub fn is_tim3_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim3Ch2
    }
    #[doc = "Signal `tim3_ch3` selected as request input"]
    #[inline(always)]
    pub fn is_tim3_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim3Ch3
    }
    #[doc = "Signal `tim3_ch4` selected as request input"]
    #[inline(always)]
    pub fn is_tim3_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim3Ch4
    }
    #[doc = "Signal `tim3_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim3_up(&self) -> bool {
        *self == DMAREQ_ID::Tim3Up
    }
    #[doc = "Signal `tim3_trig` selected as request input"]
    #[inline(always)]
    pub fn is_tim3_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim3Trig
    }
    #[doc = "Signal `tim4_ch1` selected as request input"]
    #[inline(always)]
    pub fn is_tim4_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim4Ch1
    }
    #[doc = "Signal `tim4_ch2` selected as request input"]
    #[inline(always)]
    pub fn is_tim4_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim4Ch2
    }
    #[doc = "Signal `tim4_ch3` selected as request input"]
    #[inline(always)]
    pub fn is_tim4_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim4Ch3
    }
    #[doc = "Signal `tim4_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim4_up(&self) -> bool {
        *self == DMAREQ_ID::Tim4Up
    }
    #[doc = "Signal `i2c1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_i2c1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c1RxDma
    }
    #[doc = "Signal `i2c1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_i2c1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c1TxDma
    }
    #[doc = "Signal `i2c2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_i2c2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c2RxDma
    }
    #[doc = "Signal `i2c2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_i2c2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c2TxDma
    }
    #[doc = "Signal `spi1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi1RxDma
    }
    #[doc = "Signal `spi1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi1TxDma
    }
    #[doc = "Signal `spi2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi2RxDma
    }
    #[doc = "Signal `spi2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi2TxDma
    }
    #[doc = "Signal `usart1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_usart1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart1RxDma
    }
    #[doc = "Signal `usart1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_usart1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart1TxDma
    }
    #[doc = "Signal `usart2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_usart2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart2RxDma
    }
    #[doc = "Signal `usart2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_usart2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart2TxDma
    }
    #[doc = "Signal `usart3_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_usart3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart3RxDma
    }
    #[doc = "Signal `usart3_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_usart3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart3TxDma
    }
    #[doc = "Signal `tim8_ch1` selected as request input"]
    #[inline(always)]
    pub fn is_tim8_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim8Ch1
    }
    #[doc = "Signal `tim8_ch2` selected as request input"]
    #[inline(always)]
    pub fn is_tim8_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim8Ch2
    }
    #[doc = "Signal `tim8_ch3` selected as request input"]
    #[inline(always)]
    pub fn is_tim8_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim8Ch3
    }
    #[doc = "Signal `tim8_ch4` selected as request input"]
    #[inline(always)]
    pub fn is_tim8_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim8Ch4
    }
    #[doc = "Signal `tim8_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim8_up(&self) -> bool {
        *self == DMAREQ_ID::Tim8Up
    }
    #[doc = "Signal `tim8_trig` selected as request input"]
    #[inline(always)]
    pub fn is_tim8_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim8Trig
    }
    #[doc = "Signal `tim8_com` selected as request input"]
    #[inline(always)]
    pub fn is_tim8_com(&self) -> bool {
        *self == DMAREQ_ID::Tim8Com
    }
    #[doc = "Signal `tim5_ch1` selected as request input"]
    #[inline(always)]
    pub fn is_tim5_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim5Ch1
    }
    #[doc = "Signal `tim5_ch2` selected as request input"]
    #[inline(always)]
    pub fn is_tim5_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim5Ch2
    }
    #[doc = "Signal `tim5_ch3` selected as request input"]
    #[inline(always)]
    pub fn is_tim5_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim5Ch3
    }
    #[doc = "Signal `tim5_ch4` selected as request input"]
    #[inline(always)]
    pub fn is_tim5_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim5Ch4
    }
    #[doc = "Signal `tim5_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim5_up(&self) -> bool {
        *self == DMAREQ_ID::Tim5Up
    }
    #[doc = "Signal `tim5_trig` selected as request input"]
    #[inline(always)]
    pub fn is_tim5_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim5Trig
    }
    #[doc = "Signal `spi3_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi3RxDma
    }
    #[doc = "Signal `spi3_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi3TxDma
    }
    #[doc = "Signal `uart4_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_uart4_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart4RxDma
    }
    #[doc = "Signal `uart4_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_uart4_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart4TxDma
    }
    #[doc = "Signal `uart5_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_uart5_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart5RxDma
    }
    #[doc = "Signal `uart5_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_uart5_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart5TxDma
    }
    #[doc = "Signal `dac_ch1_dma` selected as request input"]
    #[inline(always)]
    pub fn is_dac_ch1_dma(&self) -> bool {
        *self == DMAREQ_ID::DacCh1Dma
    }
    #[doc = "Signal `dac_ch2_dma` selected as request input"]
    #[inline(always)]
    pub fn is_dac_ch2_dma(&self) -> bool {
        *self == DMAREQ_ID::DacCh2Dma
    }
    #[doc = "Signal `tim6_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim6_up(&self) -> bool {
        *self == DMAREQ_ID::Tim6Up
    }
    #[doc = "Signal `tim7_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim7_up(&self) -> bool {
        *self == DMAREQ_ID::Tim7Up
    }
    #[doc = "Signal `usart6_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_usart6_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart6RxDma
    }
    #[doc = "Signal `usart6_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_usart6_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart6TxDma
    }
    #[doc = "Signal `i2c3_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_i2c3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c3RxDma
    }
    #[doc = "Signal `i2c3_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_i2c3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c3TxDma
    }
    #[doc = "Signal `dcmi_dma` selected as request input"]
    #[inline(always)]
    pub fn is_dcmi_dma(&self) -> bool {
        *self == DMAREQ_ID::DcmiDma
    }
    #[doc = "Signal `cryp_in_dma` selected as request input"]
    #[inline(always)]
    pub fn is_cryp_in_dma(&self) -> bool {
        *self == DMAREQ_ID::CrypInDma
    }
    #[doc = "Signal `cryp_out_dma` selected as request input"]
    #[inline(always)]
    pub fn is_cryp_out_dma(&self) -> bool {
        *self == DMAREQ_ID::CrypOutDma
    }
    #[doc = "Signal `hash_in_dma` selected as request input"]
    #[inline(always)]
    pub fn is_hash_in_dma(&self) -> bool {
        *self == DMAREQ_ID::HashInDma
    }
    #[doc = "Signal `uart7_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_uart7_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart7RxDma
    }
    #[doc = "Signal `uart7_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_uart7_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart7TxDma
    }
    #[doc = "Signal `uart8_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_uart8_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart8RxDma
    }
    #[doc = "Signal `uart8_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_uart8_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart8TxDma
    }
    #[doc = "Signal `spi4_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi4_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi4RxDma
    }
    #[doc = "Signal `spi4_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi4_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi4TxDma
    }
    #[doc = "Signal `spi5_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi5_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi5RxDma
    }
    #[doc = "Signal `spi5_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spi5_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi5TxDma
    }
    #[doc = "Signal `sai1a_dma` selected as request input"]
    #[inline(always)]
    pub fn is_sai1a_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai1aDma
    }
    #[doc = "Signal `sai1b_dma` selected as request input"]
    #[inline(always)]
    pub fn is_sai1b_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai1bDma
    }
    #[doc = "Signal `sai2a_dma` selected as request input"]
    #[inline(always)]
    pub fn is_sai2a_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai2aDma
    }
    #[doc = "Signal `sai2b_dma` selected as request input"]
    #[inline(always)]
    pub fn is_sai2b_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai2bDma
    }
    #[doc = "Signal `swpmi_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_swpmi_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::SwpmiRxDma
    }
    #[doc = "Signal `swpmi_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn is_swpmi_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::SwpmiTxDma
    }
    #[doc = "Signal `spdifrx_dat_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spdifrx_dat_dma(&self) -> bool {
        *self == DMAREQ_ID::SpdifrxDatDma
    }
    #[doc = "Signal `spdifrx_ctrl_dma` selected as request input"]
    #[inline(always)]
    pub fn is_spdifrx_ctrl_dma(&self) -> bool {
        *self == DMAREQ_ID::SpdifrxCtrlDma
    }
    #[doc = "Signal `hr_req(1)` selected as request input"]
    #[inline(always)]
    pub fn is_hr_req1(&self) -> bool {
        *self == DMAREQ_ID::HrReq1
    }
    #[doc = "Signal `hr_req(2)` selected as request input"]
    #[inline(always)]
    pub fn is_hr_req2(&self) -> bool {
        *self == DMAREQ_ID::HrReq2
    }
    #[doc = "Signal `hr_req(3)` selected as request input"]
    #[inline(always)]
    pub fn is_hr_req3(&self) -> bool {
        *self == DMAREQ_ID::HrReq3
    }
    #[doc = "Signal `hr_req(4)` selected as request input"]
    #[inline(always)]
    pub fn is_hr_req4(&self) -> bool {
        *self == DMAREQ_ID::HrReq4
    }
    #[doc = "Signal `hr_req(5)` selected as request input"]
    #[inline(always)]
    pub fn is_hr_req5(&self) -> bool {
        *self == DMAREQ_ID::HrReq5
    }
    #[doc = "Signal `hr_req(6)` selected as request input"]
    #[inline(always)]
    pub fn is_hr_req6(&self) -> bool {
        *self == DMAREQ_ID::HrReq6
    }
    #[doc = "Signal `dfsdm1_dma0` selected as request input"]
    #[inline(always)]
    pub fn is_dfsdm1_dma0(&self) -> bool {
        *self == DMAREQ_ID::Dfsdm1Dma0
    }
    #[doc = "Signal `dfsdm1_dma1` selected as request input"]
    #[inline(always)]
    pub fn is_dfsdm1_dma1(&self) -> bool {
        *self == DMAREQ_ID::Dfsdm1Dma1
    }
    #[doc = "Signal `dfsdm1_dma2` selected as request input"]
    #[inline(always)]
    pub fn is_dfsdm1_dma2(&self) -> bool {
        *self == DMAREQ_ID::Dfsdm1Dma2
    }
    #[doc = "Signal `dfsdm1_dma3` selected as request input"]
    #[inline(always)]
    pub fn is_dfsdm1_dma3(&self) -> bool {
        *self == DMAREQ_ID::Dfsdm1Dma3
    }
    #[doc = "Signal `tim15_ch1` selected as request input"]
    #[inline(always)]
    pub fn is_tim15_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim15Ch1
    }
    #[doc = "Signal `tim15_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim15_up(&self) -> bool {
        *self == DMAREQ_ID::Tim15Up
    }
    #[doc = "Signal `tim15_trig` selected as request input"]
    #[inline(always)]
    pub fn is_tim15_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim15Trig
    }
    #[doc = "Signal `tim15_com` selected as request input"]
    #[inline(always)]
    pub fn is_tim15_com(&self) -> bool {
        *self == DMAREQ_ID::Tim15Com
    }
    #[doc = "Signal `tim16_ch1` selected as request input"]
    #[inline(always)]
    pub fn is_tim16_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim16Ch1
    }
    #[doc = "Signal `tim16_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim16_up(&self) -> bool {
        *self == DMAREQ_ID::Tim16Up
    }
    #[doc = "Signal `tim17_ch1` selected as request input"]
    #[inline(always)]
    pub fn is_tim17_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim17Ch1
    }
    #[doc = "Signal `tim17_up` selected as request input"]
    #[inline(always)]
    pub fn is_tim17_up(&self) -> bool {
        *self == DMAREQ_ID::Tim17Up
    }
    #[doc = "Signal `sai3_a_dma` selected as request input"]
    #[inline(always)]
    pub fn is_sai3_a_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai3ADma
    }
    #[doc = "Signal `sai3_b_dma` selected as request input"]
    #[inline(always)]
    pub fn is_sai3_b_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai3BDma
    }
    #[doc = "Signal `adc3_dma` selected as request input"]
    #[inline(always)]
    pub fn is_adc3_dma(&self) -> bool {
        *self == DMAREQ_ID::Adc3Dma
    }
}
#[doc = "Field `DMAREQ_ID` writer - Input DMA request line selected"]
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8, DMAREQ_ID>;
impl<'a, REG> DMAREQ_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No signal selected as request input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::None)
    }
    #[doc = "Signal `dmamux1_req_gen0` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen0)
    }
    #[doc = "Signal `dmamux1_req_gen1` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen1)
    }
    #[doc = "Signal `dmamux1_req_gen2` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen2)
    }
    #[doc = "Signal `dmamux1_req_gen3` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen3)
    }
    #[doc = "Signal `dmamux1_req_gen4` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen4)
    }
    #[doc = "Signal `dmamux1_req_gen5` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen5(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen5)
    }
    #[doc = "Signal `dmamux1_req_gen6` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen6(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen6)
    }
    #[doc = "Signal `dmamux1_req_gen7` selected as request input"]
    #[inline(always)]
    pub fn dmamux1_req_gen7(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen7)
    }
    #[doc = "Signal `adc1_dma` selected as request input"]
    #[inline(always)]
    pub fn adc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Adc1Dma)
    }
    #[doc = "Signal `adc2_dma` selected as request input"]
    #[inline(always)]
    pub fn adc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Adc2Dma)
    }
    #[doc = "Signal `tim1_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch1)
    }
    #[doc = "Signal `tim1_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch2)
    }
    #[doc = "Signal `tim1_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch3)
    }
    #[doc = "Signal `tim1_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim1_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch4)
    }
    #[doc = "Signal `tim1_up` selected as request input"]
    #[inline(always)]
    pub fn tim1_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Up)
    }
    #[doc = "Signal `tim1_trig` selected as request input"]
    #[inline(always)]
    pub fn tim1_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Trig)
    }
    #[doc = "Signal `tim1_com` selected as request input"]
    #[inline(always)]
    pub fn tim1_com(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Com)
    }
    #[doc = "Signal `tim2_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch1)
    }
    #[doc = "Signal `tim2_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch2)
    }
    #[doc = "Signal `tim2_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch3)
    }
    #[doc = "Signal `tim2_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch4)
    }
    #[doc = "Signal `tim2_up` selected as request input"]
    #[inline(always)]
    pub fn tim2_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Up)
    }
    #[doc = "Signal `tim3_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim3_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Ch1)
    }
    #[doc = "Signal `tim3_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim3_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Ch2)
    }
    #[doc = "Signal `tim3_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim3_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Ch3)
    }
    #[doc = "Signal `tim3_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim3_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Ch4)
    }
    #[doc = "Signal `tim3_up` selected as request input"]
    #[inline(always)]
    pub fn tim3_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Up)
    }
    #[doc = "Signal `tim3_trig` selected as request input"]
    #[inline(always)]
    pub fn tim3_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Trig)
    }
    #[doc = "Signal `tim4_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim4_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim4Ch1)
    }
    #[doc = "Signal `tim4_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim4_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim4Ch2)
    }
    #[doc = "Signal `tim4_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim4_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim4Ch3)
    }
    #[doc = "Signal `tim4_up` selected as request input"]
    #[inline(always)]
    pub fn tim4_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim4Up)
    }
    #[doc = "Signal `i2c1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c1RxDma)
    }
    #[doc = "Signal `i2c1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c1TxDma)
    }
    #[doc = "Signal `i2c2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c2RxDma)
    }
    #[doc = "Signal `i2c2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c2TxDma)
    }
    #[doc = "Signal `spi1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi1RxDma)
    }
    #[doc = "Signal `spi1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi1TxDma)
    }
    #[doc = "Signal `spi2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi2RxDma)
    }
    #[doc = "Signal `spi2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi2TxDma)
    }
    #[doc = "Signal `usart1_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart1RxDma)
    }
    #[doc = "Signal `usart1_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart1TxDma)
    }
    #[doc = "Signal `usart2_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart2RxDma)
    }
    #[doc = "Signal `usart2_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart2TxDma)
    }
    #[doc = "Signal `usart3_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart3RxDma)
    }
    #[doc = "Signal `usart3_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart3TxDma)
    }
    #[doc = "Signal `tim8_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim8_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Ch1)
    }
    #[doc = "Signal `tim8_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim8_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Ch2)
    }
    #[doc = "Signal `tim8_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim8_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Ch3)
    }
    #[doc = "Signal `tim8_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim8_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Ch4)
    }
    #[doc = "Signal `tim8_up` selected as request input"]
    #[inline(always)]
    pub fn tim8_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Up)
    }
    #[doc = "Signal `tim8_trig` selected as request input"]
    #[inline(always)]
    pub fn tim8_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Trig)
    }
    #[doc = "Signal `tim8_com` selected as request input"]
    #[inline(always)]
    pub fn tim8_com(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Com)
    }
    #[doc = "Signal `tim5_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim5_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Ch1)
    }
    #[doc = "Signal `tim5_ch2` selected as request input"]
    #[inline(always)]
    pub fn tim5_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Ch2)
    }
    #[doc = "Signal `tim5_ch3` selected as request input"]
    #[inline(always)]
    pub fn tim5_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Ch3)
    }
    #[doc = "Signal `tim5_ch4` selected as request input"]
    #[inline(always)]
    pub fn tim5_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Ch4)
    }
    #[doc = "Signal `tim5_up` selected as request input"]
    #[inline(always)]
    pub fn tim5_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Up)
    }
    #[doc = "Signal `tim5_trig` selected as request input"]
    #[inline(always)]
    pub fn tim5_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Trig)
    }
    #[doc = "Signal `spi3_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi3RxDma)
    }
    #[doc = "Signal `spi3_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi3TxDma)
    }
    #[doc = "Signal `uart4_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart4RxDma)
    }
    #[doc = "Signal `uart4_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart4TxDma)
    }
    #[doc = "Signal `uart5_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart5_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart5RxDma)
    }
    #[doc = "Signal `uart5_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart5_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart5TxDma)
    }
    #[doc = "Signal `dac_ch1_dma` selected as request input"]
    #[inline(always)]
    pub fn dac_ch1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::DacCh1Dma)
    }
    #[doc = "Signal `dac_ch2_dma` selected as request input"]
    #[inline(always)]
    pub fn dac_ch2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::DacCh2Dma)
    }
    #[doc = "Signal `tim6_up` selected as request input"]
    #[inline(always)]
    pub fn tim6_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim6Up)
    }
    #[doc = "Signal `tim7_up` selected as request input"]
    #[inline(always)]
    pub fn tim7_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim7Up)
    }
    #[doc = "Signal `usart6_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart6_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart6RxDma)
    }
    #[doc = "Signal `usart6_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn usart6_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart6TxDma)
    }
    #[doc = "Signal `i2c3_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c3RxDma)
    }
    #[doc = "Signal `i2c3_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn i2c3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c3TxDma)
    }
    #[doc = "Signal `dcmi_dma` selected as request input"]
    #[inline(always)]
    pub fn dcmi_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::DcmiDma)
    }
    #[doc = "Signal `cryp_in_dma` selected as request input"]
    #[inline(always)]
    pub fn cryp_in_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::CrypInDma)
    }
    #[doc = "Signal `cryp_out_dma` selected as request input"]
    #[inline(always)]
    pub fn cryp_out_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::CrypOutDma)
    }
    #[doc = "Signal `hash_in_dma` selected as request input"]
    #[inline(always)]
    pub fn hash_in_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HashInDma)
    }
    #[doc = "Signal `uart7_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart7_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart7RxDma)
    }
    #[doc = "Signal `uart7_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart7_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart7TxDma)
    }
    #[doc = "Signal `uart8_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart8_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart8RxDma)
    }
    #[doc = "Signal `uart8_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn uart8_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart8TxDma)
    }
    #[doc = "Signal `spi4_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi4RxDma)
    }
    #[doc = "Signal `spi4_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi4TxDma)
    }
    #[doc = "Signal `spi5_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi5_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi5RxDma)
    }
    #[doc = "Signal `spi5_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn spi5_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi5TxDma)
    }
    #[doc = "Signal `sai1a_dma` selected as request input"]
    #[inline(always)]
    pub fn sai1a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai1aDma)
    }
    #[doc = "Signal `sai1b_dma` selected as request input"]
    #[inline(always)]
    pub fn sai1b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai1bDma)
    }
    #[doc = "Signal `sai2a_dma` selected as request input"]
    #[inline(always)]
    pub fn sai2a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai2aDma)
    }
    #[doc = "Signal `sai2b_dma` selected as request input"]
    #[inline(always)]
    pub fn sai2b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai2bDma)
    }
    #[doc = "Signal `swpmi_rx_dma` selected as request input"]
    #[inline(always)]
    pub fn swpmi_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::SwpmiRxDma)
    }
    #[doc = "Signal `swpmi_tx_dma` selected as request input"]
    #[inline(always)]
    pub fn swpmi_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::SwpmiTxDma)
    }
    #[doc = "Signal `spdifrx_dat_dma` selected as request input"]
    #[inline(always)]
    pub fn spdifrx_dat_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::SpdifrxDatDma)
    }
    #[doc = "Signal `spdifrx_ctrl_dma` selected as request input"]
    #[inline(always)]
    pub fn spdifrx_ctrl_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::SpdifrxCtrlDma)
    }
    #[doc = "Signal `hr_req(1)` selected as request input"]
    #[inline(always)]
    pub fn hr_req1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq1)
    }
    #[doc = "Signal `hr_req(2)` selected as request input"]
    #[inline(always)]
    pub fn hr_req2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq2)
    }
    #[doc = "Signal `hr_req(3)` selected as request input"]
    #[inline(always)]
    pub fn hr_req3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq3)
    }
    #[doc = "Signal `hr_req(4)` selected as request input"]
    #[inline(always)]
    pub fn hr_req4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq4)
    }
    #[doc = "Signal `hr_req(5)` selected as request input"]
    #[inline(always)]
    pub fn hr_req5(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq5)
    }
    #[doc = "Signal `hr_req(6)` selected as request input"]
    #[inline(always)]
    pub fn hr_req6(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq6)
    }
    #[doc = "Signal `dfsdm1_dma0` selected as request input"]
    #[inline(always)]
    pub fn dfsdm1_dma0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dfsdm1Dma0)
    }
    #[doc = "Signal `dfsdm1_dma1` selected as request input"]
    #[inline(always)]
    pub fn dfsdm1_dma1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dfsdm1Dma1)
    }
    #[doc = "Signal `dfsdm1_dma2` selected as request input"]
    #[inline(always)]
    pub fn dfsdm1_dma2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dfsdm1Dma2)
    }
    #[doc = "Signal `dfsdm1_dma3` selected as request input"]
    #[inline(always)]
    pub fn dfsdm1_dma3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dfsdm1Dma3)
    }
    #[doc = "Signal `tim15_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim15_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim15Ch1)
    }
    #[doc = "Signal `tim15_up` selected as request input"]
    #[inline(always)]
    pub fn tim15_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim15Up)
    }
    #[doc = "Signal `tim15_trig` selected as request input"]
    #[inline(always)]
    pub fn tim15_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim15Trig)
    }
    #[doc = "Signal `tim15_com` selected as request input"]
    #[inline(always)]
    pub fn tim15_com(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim15Com)
    }
    #[doc = "Signal `tim16_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim16_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim16Ch1)
    }
    #[doc = "Signal `tim16_up` selected as request input"]
    #[inline(always)]
    pub fn tim16_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim16Up)
    }
    #[doc = "Signal `tim17_ch1` selected as request input"]
    #[inline(always)]
    pub fn tim17_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim17Ch1)
    }
    #[doc = "Signal `tim17_up` selected as request input"]
    #[inline(always)]
    pub fn tim17_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim17Up)
    }
    #[doc = "Signal `sai3_a_dma` selected as request input"]
    #[inline(always)]
    pub fn sai3_a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai3ADma)
    }
    #[doc = "Signal `sai3_b_dma` selected as request input"]
    #[inline(always)]
    pub fn sai3_b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai3BDma)
    }
    #[doc = "Signal `adc3_dma` selected as request input"]
    #[inline(always)]
    pub fn adc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Adc3Dma)
    }
}
#[doc = "Interrupt enable at synchronization event overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOIE {
    #[doc = "0: Synchronization overrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Synchronization overrun interrupt enabled"]
    Enabled = 1,
}
impl From<SOIE> for bool {
    #[inline(always)]
    fn from(variant: SOIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOIE` reader - Interrupt enable at synchronization event overrun"]
pub type SOIE_R = crate::BitReader<SOIE>;
impl SOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SOIE {
        match self.bits {
            false => SOIE::Disabled,
            true => SOIE::Enabled,
        }
    }
    #[doc = "Synchronization overrun interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOIE::Disabled
    }
    #[doc = "Synchronization overrun interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOIE::Enabled
    }
}
#[doc = "Field `SOIE` writer - Interrupt enable at synchronization event overrun"]
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG, SOIE>;
impl<'a, REG> SOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronization overrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::Disabled)
    }
    #[doc = "Synchronization overrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::Enabled)
    }
}
#[doc = "Event generation enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EGE {
    #[doc = "0: Event generation disabled"]
    Disabled = 0,
    #[doc = "1: Event generation enabled"]
    Enabled = 1,
}
impl From<EGE> for bool {
    #[inline(always)]
    fn from(variant: EGE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EGE` reader - Event generation enable/disable"]
pub type EGE_R = crate::BitReader<EGE>;
impl EGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EGE {
        match self.bits {
            false => EGE::Disabled,
            true => EGE::Enabled,
        }
    }
    #[doc = "Event generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EGE::Disabled
    }
    #[doc = "Event generation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EGE::Enabled
    }
}
#[doc = "Field `EGE` writer - Event generation enable/disable"]
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG, EGE>;
impl<'a, REG> EGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event generation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::Disabled)
    }
    #[doc = "Event generation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::Enabled)
    }
}
#[doc = "Synchronous operating mode enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE {
    #[doc = "0: Synchronization disabled"]
    Disabled = 0,
    #[doc = "1: Synchronization enabled"]
    Enabled = 1,
}
impl From<SE> for bool {
    #[inline(always)]
    fn from(variant: SE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SE` reader - Synchronous operating mode enable/disable"]
pub type SE_R = crate::BitReader<SE>;
impl SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SE {
        match self.bits {
            false => SE::Disabled,
            true => SE::Enabled,
        }
    }
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SE::Disabled
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SE::Enabled
    }
}
#[doc = "Field `SE` writer - Synchronous operating mode enable/disable"]
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG, SE>;
impl<'a, REG> SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronization disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SE::Disabled)
    }
    #[doc = "Synchronization enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SE::Enabled)
    }
}
#[doc = "Synchronization event type selector Defines the synchronization event on the selected synchronization input:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPOL {
    #[doc = "0: No event, i.e. no synchronization nor detection"]
    NoEdge = 0,
    #[doc = "1: Rising edge"]
    RisingEdge = 1,
    #[doc = "2: Falling edge"]
    FallingEdge = 2,
    #[doc = "3: Rising and falling edges"]
    BothEdges = 3,
}
impl From<SPOL> for u8 {
    #[inline(always)]
    fn from(variant: SPOL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPOL {
    type Ux = u8;
}
#[doc = "Field `SPOL` reader - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SPOL_R = crate::FieldReader<SPOL>;
impl SPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SPOL {
        match self.bits {
            0 => SPOL::NoEdge,
            1 => SPOL::RisingEdge,
            2 => SPOL::FallingEdge,
            3 => SPOL::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "No event, i.e. no synchronization nor detection"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == SPOL::NoEdge
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SPOL::RisingEdge
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SPOL::FallingEdge
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == SPOL::BothEdges
    }
}
#[doc = "Field `SPOL` writer - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SPOL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SPOL>;
impl<'a, REG> SPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No event, i.e. no synchronization nor detection"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::NoEdge)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::RisingEdge)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::FallingEdge)
    }
    #[doc = "Rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::BothEdges)
    }
}
#[doc = "Field `NBREQ` reader - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NBREQ_R = crate::FieldReader;
#[doc = "Field `NBREQ` writer - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NBREQ_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Synchronization input selected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNC_ID {
    #[doc = "0: Signal `dmamux1_evt0` selected as synchronization input"]
    Dmamux1Evt0 = 0,
    #[doc = "1: Signal `dmamux1_evt1` selected as synchronization input"]
    Dmamux1Evt1 = 1,
    #[doc = "2: Signal `dmamux1_evt2` selected as synchronization input"]
    Dmamux1Evt2 = 2,
    #[doc = "3: Signal `lptim1_out` selected as synchronization input"]
    Lptim1Out = 3,
    #[doc = "4: Signal `lptim2_out` selected as synchronization input"]
    Lptim2Out = 4,
    #[doc = "5: Signal `lptim3_out` selected as synchronization input"]
    Lptim3Out = 5,
    #[doc = "6: Signal `extit0` selected as synchronization input"]
    Extit0 = 6,
    #[doc = "7: Signal `tim12_trgo` selected as synchronization input"]
    Tim12Trgo = 7,
}
impl From<SYNC_ID> for u8 {
    #[inline(always)]
    fn from(variant: SYNC_ID) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNC_ID {
    type Ux = u8;
}
#[doc = "Field `SYNC_ID` reader - Synchronization input selected"]
pub type SYNC_ID_R = crate::FieldReader<SYNC_ID>;
impl SYNC_ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNC_ID> {
        match self.bits {
            0 => Some(SYNC_ID::Dmamux1Evt0),
            1 => Some(SYNC_ID::Dmamux1Evt1),
            2 => Some(SYNC_ID::Dmamux1Evt2),
            3 => Some(SYNC_ID::Lptim1Out),
            4 => Some(SYNC_ID::Lptim2Out),
            5 => Some(SYNC_ID::Lptim3Out),
            6 => Some(SYNC_ID::Extit0),
            7 => Some(SYNC_ID::Tim12Trgo),
            _ => None,
        }
    }
    #[doc = "Signal `dmamux1_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        *self == SYNC_ID::Dmamux1Evt0
    }
    #[doc = "Signal `dmamux1_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        *self == SYNC_ID::Dmamux1Evt1
    }
    #[doc = "Signal `dmamux1_evt2` selected as synchronization input"]
    #[inline(always)]
    pub fn is_dmamux1_evt2(&self) -> bool {
        *self == SYNC_ID::Dmamux1Evt2
    }
    #[doc = "Signal `lptim1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == SYNC_ID::Lptim1Out
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SYNC_ID::Lptim2Out
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SYNC_ID::Lptim3Out
    }
    #[doc = "Signal `extit0` selected as synchronization input"]
    #[inline(always)]
    pub fn is_extit0(&self) -> bool {
        *self == SYNC_ID::Extit0
    }
    #[doc = "Signal `tim12_trgo` selected as synchronization input"]
    #[inline(always)]
    pub fn is_tim12_trgo(&self) -> bool {
        *self == SYNC_ID::Tim12Trgo
    }
}
#[doc = "Field `SYNC_ID` writer - Synchronization input selected"]
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5, SYNC_ID>;
impl<'a, REG> SYNC_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Signal `dmamux1_evt0` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux1Evt0)
    }
    #[doc = "Signal `dmamux1_evt1` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux1Evt1)
    }
    #[doc = "Signal `dmamux1_evt2` selected as synchronization input"]
    #[inline(always)]
    pub fn dmamux1_evt2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux1Evt2)
    }
    #[doc = "Signal `lptim1_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim1Out)
    }
    #[doc = "Signal `lptim2_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim2Out)
    }
    #[doc = "Signal `lptim3_out` selected as synchronization input"]
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim3Out)
    }
    #[doc = "Signal `extit0` selected as synchronization input"]
    #[inline(always)]
    pub fn extit0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Extit0)
    }
    #[doc = "Signal `tim12_trgo` selected as synchronization input"]
    #[inline(always)]
    pub fn tim12_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Tim12Trgo)
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
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
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
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<CCRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<CCRrs> {
        SOIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<CCRrs> {
        EGE_W::new(self, 9)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<CCRrs> {
        SE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<CCRrs> {
        SPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<CCRrs> {
        NBREQ_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<CCRrs> {
        SYNC_ID_W::new(self, 24)
    }
}
#[doc = "DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C%sCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
