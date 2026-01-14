///Register `C%sCR` reader
pub type R = crate::R<CCRrs>;
///Register `C%sCR` writer
pub type W = crate::W<CCRrs>;
/**DMA request identification

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMAREQ_ID {
    ///0: No signal selected as request input
    None = 0,
    ///1: Signal `dmamux1_req_gen0` selected as request input
    Dmamux1ReqGen0 = 1,
    ///2: Signal `dmamux1_req_gen1` selected as request input
    Dmamux1ReqGen1 = 2,
    ///3: Signal `dmamux1_req_gen2` selected as request input
    Dmamux1ReqGen2 = 3,
    ///4: Signal `dmamux1_req_gen3` selected as request input
    Dmamux1ReqGen3 = 4,
    ///5: Signal `dmamux1_req_gen4` selected as request input
    Dmamux1ReqGen4 = 5,
    ///6: Signal `dmamux1_req_gen5` selected as request input
    Dmamux1ReqGen5 = 6,
    ///7: Signal `dmamux1_req_gen6` selected as request input
    Dmamux1ReqGen6 = 7,
    ///8: Signal `dmamux1_req_gen7` selected as request input
    Dmamux1ReqGen7 = 8,
    ///9: Signal `adc1_dma` selected as request input
    Adc1Dma = 9,
    ///10: Signal `adc2_dma` selected as request input
    Adc2Dma = 10,
    ///11: Signal `tim1_ch1` selected as request input
    Tim1Ch1 = 11,
    ///12: Signal `tim1_ch2` selected as request input
    Tim1Ch2 = 12,
    ///13: Signal `tim1_ch3` selected as request input
    Tim1Ch3 = 13,
    ///14: Signal `tim1_ch4` selected as request input
    Tim1Ch4 = 14,
    ///15: Signal `tim1_up` selected as request input
    Tim1Up = 15,
    ///16: Signal `tim1_trig` selected as request input
    Tim1Trig = 16,
    ///17: Signal `tim1_com` selected as request input
    Tim1Com = 17,
    ///18: Signal `tim2_ch1` selected as request input
    Tim2Ch1 = 18,
    ///19: Signal `tim2_ch2` selected as request input
    Tim2Ch2 = 19,
    ///20: Signal `tim2_ch3` selected as request input
    Tim2Ch3 = 20,
    ///21: Signal `tim2_ch4` selected as request input
    Tim2Ch4 = 21,
    ///22: Signal `tim2_up` selected as request input
    Tim2Up = 22,
    ///23: Signal `tim3_ch1` selected as request input
    Tim3Ch1 = 23,
    ///24: Signal `tim3_ch2` selected as request input
    Tim3Ch2 = 24,
    ///25: Signal `tim3_ch3` selected as request input
    Tim3Ch3 = 25,
    ///26: Signal `tim3_ch4` selected as request input
    Tim3Ch4 = 26,
    ///27: Signal `tim3_up` selected as request input
    Tim3Up = 27,
    ///28: Signal `tim3_trig` selected as request input
    Tim3Trig = 28,
    ///29: Signal `tim4_ch1` selected as request input
    Tim4Ch1 = 29,
    ///30: Signal `tim4_ch2` selected as request input
    Tim4Ch2 = 30,
    ///31: Signal `tim4_ch3` selected as request input
    Tim4Ch3 = 31,
    ///32: Signal `tim4_up` selected as request input
    Tim4Up = 32,
    ///33: Signal `i2c1_rx_dma` selected as request input
    I2c1RxDma = 33,
    ///34: Signal `i2c1_tx_dma` selected as request input
    I2c1TxDma = 34,
    ///35: Signal `i2c2_rx_dma` selected as request input
    I2c2RxDma = 35,
    ///36: Signal `i2c2_tx_dma` selected as request input
    I2c2TxDma = 36,
    ///37: Signal `spi1_rx_dma` selected as request input
    Spi1RxDma = 37,
    ///38: Signal `spi1_tx_dma` selected as request input
    Spi1TxDma = 38,
    ///39: Signal `spi2_rx_dma` selected as request input
    Spi2RxDma = 39,
    ///40: Signal `spi2_tx_dma` selected as request input
    Spi2TxDma = 40,
    ///41: Signal `usart1_rx_dma` selected as request input
    Usart1RxDma = 41,
    ///42: Signal `usart1_tx_dma` selected as request input
    Usart1TxDma = 42,
    ///43: Signal `usart2_rx_dma` selected as request input
    Usart2RxDma = 43,
    ///44: Signal `usart2_tx_dma` selected as request input
    Usart2TxDma = 44,
    ///45: Signal `usart3_rx_dma` selected as request input
    Usart3RxDma = 45,
    ///46: Signal `usart3_tx_dma` selected as request input
    Usart3TxDma = 46,
    ///47: Signal `tim8_ch1` selected as request input
    Tim8Ch1 = 47,
    ///48: Signal `tim8_ch2` selected as request input
    Tim8Ch2 = 48,
    ///49: Signal `tim8_ch3` selected as request input
    Tim8Ch3 = 49,
    ///50: Signal `tim8_ch4` selected as request input
    Tim8Ch4 = 50,
    ///51: Signal `tim8_up` selected as request input
    Tim8Up = 51,
    ///52: Signal `tim8_trig` selected as request input
    Tim8Trig = 52,
    ///53: Signal `tim8_com` selected as request input
    Tim8Com = 53,
    ///55: Signal `tim5_ch1` selected as request input
    Tim5Ch1 = 55,
    ///56: Signal `tim5_ch2` selected as request input
    Tim5Ch2 = 56,
    ///57: Signal `tim5_ch3` selected as request input
    Tim5Ch3 = 57,
    ///58: Signal `tim5_ch4` selected as request input
    Tim5Ch4 = 58,
    ///59: Signal `tim5_up` selected as request input
    Tim5Up = 59,
    ///60: Signal `tim5_trig` selected as request input
    Tim5Trig = 60,
    ///61: Signal `spi3_rx_dma` selected as request input
    Spi3RxDma = 61,
    ///62: Signal `spi3_tx_dma` selected as request input
    Spi3TxDma = 62,
    ///63: Signal `uart4_rx_dma` selected as request input
    Uart4RxDma = 63,
    ///64: Signal `uart4_tx_dma` selected as request input
    Uart4TxDma = 64,
    ///65: Signal `uart5_rx_dma` selected as request input
    Uart5RxDma = 65,
    ///66: Signal `uart5_tx_dma` selected as request input
    Uart5TxDma = 66,
    ///67: Signal `dac_ch1_dma` selected as request input
    DacCh1Dma = 67,
    ///68: Signal `dac_ch2_dma` selected as request input
    DacCh2Dma = 68,
    ///69: Signal `tim6_up` selected as request input
    Tim6Up = 69,
    ///70: Signal `tim7_up` selected as request input
    Tim7Up = 70,
    ///71: Signal `usart6_rx_dma` selected as request input
    Usart6RxDma = 71,
    ///72: Signal `usart6_tx_dma` selected as request input
    Usart6TxDma = 72,
    ///73: Signal `i2c3_rx_dma` selected as request input
    I2c3RxDma = 73,
    ///74: Signal `i2c3_tx_dma` selected as request input
    I2c3TxDma = 74,
    ///75: Signal `dcmi_dma` selected as request input
    DcmiDma = 75,
    ///76: Signal `cryp_in_dma` selected as request input
    CrypInDma = 76,
    ///77: Signal `cryp_out_dma` selected as request input
    CrypOutDma = 77,
    ///78: Signal `hash_in_dma` selected as request input
    HashInDma = 78,
    ///79: Signal `uart7_rx_dma` selected as request input
    Uart7RxDma = 79,
    ///80: Signal `uart7_tx_dma` selected as request input
    Uart7TxDma = 80,
    ///81: Signal `uart8_rx_dma` selected as request input
    Uart8RxDma = 81,
    ///82: Signal `uart8_tx_dma` selected as request input
    Uart8TxDma = 82,
    ///83: Signal `spi4_rx_dma` selected as request input
    Spi4RxDma = 83,
    ///84: Signal `spi4_tx_dma` selected as request input
    Spi4TxDma = 84,
    ///85: Signal `spi5_rx_dma` selected as request input
    Spi5RxDma = 85,
    ///86: Signal `spi5_tx_dma` selected as request input
    Spi5TxDma = 86,
    ///87: Signal `sai1a_dma` selected as request input
    Sai1aDma = 87,
    ///88: Signal `sai1b_dma` selected as request input
    Sai1bDma = 88,
    ///89: Signal `sai2a_dma` selected as request input
    Sai2aDma = 89,
    ///90: Signal `sai2b_dma` selected as request input
    Sai2bDma = 90,
    ///91: Signal `swpmi_rx_dma` selected as request input
    SwpmiRxDma = 91,
    ///92: Signal `swpmi_tx_dma` selected as request input
    SwpmiTxDma = 92,
    ///93: Signal `spdifrx_dat_dma` selected as request input
    SpdifrxDatDma = 93,
    ///94: Signal `spdifrx_ctrl_dma` selected as request input
    SpdifrxCtrlDma = 94,
    ///95: Signal `hr_req(1)` selected as request input
    HrReq1 = 95,
    ///96: Signal `hr_req(2)` selected as request input
    HrReq2 = 96,
    ///97: Signal `hr_req(3)` selected as request input
    HrReq3 = 97,
    ///98: Signal `hr_req(4)` selected as request input
    HrReq4 = 98,
    ///99: Signal `hr_req(5)` selected as request input
    HrReq5 = 99,
    ///100: Signal `hr_req(6)` selected as request input
    HrReq6 = 100,
    ///101: Signal `dfsdm1_dma0` selected as request input
    Dfsdm1Dma0 = 101,
    ///102: Signal `dfsdm1_dma1` selected as request input
    Dfsdm1Dma1 = 102,
    ///103: Signal `dfsdm1_dma2` selected as request input
    Dfsdm1Dma2 = 103,
    ///104: Signal `dfsdm1_dma3` selected as request input
    Dfsdm1Dma3 = 104,
    ///105: Signal `tim15_ch1` selected as request input
    Tim15Ch1 = 105,
    ///106: Signal `tim15_up` selected as request input
    Tim15Up = 106,
    ///107: Signal `tim15_trig` selected as request input
    Tim15Trig = 107,
    ///108: Signal `tim15_com` selected as request input
    Tim15Com = 108,
    ///109: Signal `tim16_ch1` selected as request input
    Tim16Ch1 = 109,
    ///110: Signal `tim16_up` selected as request input
    Tim16Up = 110,
    ///111: Signal `tim17_ch1` selected as request input
    Tim17Ch1 = 111,
    ///112: Signal `tim17_up` selected as request input
    Tim17Up = 112,
    ///113: Signal `sai3_a_dma` selected as request input
    Sai3ADma = 113,
    ///114: Signal `sai3_b_dma` selected as request input
    Sai3BDma = 114,
    ///115: Signal `adc3_dma` selected as request input
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
impl crate::IsEnum for DMAREQ_ID {}
///Field `DMAREQ_ID` reader - DMA request identification
pub type DMAREQ_ID_R = crate::FieldReader<DMAREQ_ID>;
impl DMAREQ_ID_R {
    ///Get enumerated values variant
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
    ///No signal selected as request input
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DMAREQ_ID::None
    }
    ///Signal `dmamux1_req_gen0` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen0(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen0
    }
    ///Signal `dmamux1_req_gen1` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen1(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen1
    }
    ///Signal `dmamux1_req_gen2` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen2(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen2
    }
    ///Signal `dmamux1_req_gen3` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen3(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen3
    }
    ///Signal `dmamux1_req_gen4` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen4(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen4
    }
    ///Signal `dmamux1_req_gen5` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen5(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen5
    }
    ///Signal `dmamux1_req_gen6` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen6(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen6
    }
    ///Signal `dmamux1_req_gen7` selected as request input
    #[inline(always)]
    pub fn is_dmamux1_req_gen7(&self) -> bool {
        *self == DMAREQ_ID::Dmamux1ReqGen7
    }
    ///Signal `adc1_dma` selected as request input
    #[inline(always)]
    pub fn is_adc1_dma(&self) -> bool {
        *self == DMAREQ_ID::Adc1Dma
    }
    ///Signal `adc2_dma` selected as request input
    #[inline(always)]
    pub fn is_adc2_dma(&self) -> bool {
        *self == DMAREQ_ID::Adc2Dma
    }
    ///Signal `tim1_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim1_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch1
    }
    ///Signal `tim1_ch2` selected as request input
    #[inline(always)]
    pub fn is_tim1_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch2
    }
    ///Signal `tim1_ch3` selected as request input
    #[inline(always)]
    pub fn is_tim1_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch3
    }
    ///Signal `tim1_ch4` selected as request input
    #[inline(always)]
    pub fn is_tim1_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim1Ch4
    }
    ///Signal `tim1_up` selected as request input
    #[inline(always)]
    pub fn is_tim1_up(&self) -> bool {
        *self == DMAREQ_ID::Tim1Up
    }
    ///Signal `tim1_trig` selected as request input
    #[inline(always)]
    pub fn is_tim1_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim1Trig
    }
    ///Signal `tim1_com` selected as request input
    #[inline(always)]
    pub fn is_tim1_com(&self) -> bool {
        *self == DMAREQ_ID::Tim1Com
    }
    ///Signal `tim2_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim2_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch1
    }
    ///Signal `tim2_ch2` selected as request input
    #[inline(always)]
    pub fn is_tim2_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch2
    }
    ///Signal `tim2_ch3` selected as request input
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch3
    }
    ///Signal `tim2_ch4` selected as request input
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim2Ch4
    }
    ///Signal `tim2_up` selected as request input
    #[inline(always)]
    pub fn is_tim2_up(&self) -> bool {
        *self == DMAREQ_ID::Tim2Up
    }
    ///Signal `tim3_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim3_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim3Ch1
    }
    ///Signal `tim3_ch2` selected as request input
    #[inline(always)]
    pub fn is_tim3_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim3Ch2
    }
    ///Signal `tim3_ch3` selected as request input
    #[inline(always)]
    pub fn is_tim3_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim3Ch3
    }
    ///Signal `tim3_ch4` selected as request input
    #[inline(always)]
    pub fn is_tim3_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim3Ch4
    }
    ///Signal `tim3_up` selected as request input
    #[inline(always)]
    pub fn is_tim3_up(&self) -> bool {
        *self == DMAREQ_ID::Tim3Up
    }
    ///Signal `tim3_trig` selected as request input
    #[inline(always)]
    pub fn is_tim3_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim3Trig
    }
    ///Signal `tim4_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim4_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim4Ch1
    }
    ///Signal `tim4_ch2` selected as request input
    #[inline(always)]
    pub fn is_tim4_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim4Ch2
    }
    ///Signal `tim4_ch3` selected as request input
    #[inline(always)]
    pub fn is_tim4_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim4Ch3
    }
    ///Signal `tim4_up` selected as request input
    #[inline(always)]
    pub fn is_tim4_up(&self) -> bool {
        *self == DMAREQ_ID::Tim4Up
    }
    ///Signal `i2c1_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c1RxDma
    }
    ///Signal `i2c1_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c1TxDma
    }
    ///Signal `i2c2_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c2RxDma
    }
    ///Signal `i2c2_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c2TxDma
    }
    ///Signal `spi1_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi1RxDma
    }
    ///Signal `spi1_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi1TxDma
    }
    ///Signal `spi2_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi2RxDma
    }
    ///Signal `spi2_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi2TxDma
    }
    ///Signal `usart1_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart1_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart1RxDma
    }
    ///Signal `usart1_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart1_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart1TxDma
    }
    ///Signal `usart2_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart2_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart2RxDma
    }
    ///Signal `usart2_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart2_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart2TxDma
    }
    ///Signal `usart3_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart3RxDma
    }
    ///Signal `usart3_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart3TxDma
    }
    ///Signal `tim8_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim8_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim8Ch1
    }
    ///Signal `tim8_ch2` selected as request input
    #[inline(always)]
    pub fn is_tim8_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim8Ch2
    }
    ///Signal `tim8_ch3` selected as request input
    #[inline(always)]
    pub fn is_tim8_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim8Ch3
    }
    ///Signal `tim8_ch4` selected as request input
    #[inline(always)]
    pub fn is_tim8_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim8Ch4
    }
    ///Signal `tim8_up` selected as request input
    #[inline(always)]
    pub fn is_tim8_up(&self) -> bool {
        *self == DMAREQ_ID::Tim8Up
    }
    ///Signal `tim8_trig` selected as request input
    #[inline(always)]
    pub fn is_tim8_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim8Trig
    }
    ///Signal `tim8_com` selected as request input
    #[inline(always)]
    pub fn is_tim8_com(&self) -> bool {
        *self == DMAREQ_ID::Tim8Com
    }
    ///Signal `tim5_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim5_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim5Ch1
    }
    ///Signal `tim5_ch2` selected as request input
    #[inline(always)]
    pub fn is_tim5_ch2(&self) -> bool {
        *self == DMAREQ_ID::Tim5Ch2
    }
    ///Signal `tim5_ch3` selected as request input
    #[inline(always)]
    pub fn is_tim5_ch3(&self) -> bool {
        *self == DMAREQ_ID::Tim5Ch3
    }
    ///Signal `tim5_ch4` selected as request input
    #[inline(always)]
    pub fn is_tim5_ch4(&self) -> bool {
        *self == DMAREQ_ID::Tim5Ch4
    }
    ///Signal `tim5_up` selected as request input
    #[inline(always)]
    pub fn is_tim5_up(&self) -> bool {
        *self == DMAREQ_ID::Tim5Up
    }
    ///Signal `tim5_trig` selected as request input
    #[inline(always)]
    pub fn is_tim5_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim5Trig
    }
    ///Signal `spi3_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi3RxDma
    }
    ///Signal `spi3_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi3TxDma
    }
    ///Signal `uart4_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_uart4_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart4RxDma
    }
    ///Signal `uart4_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_uart4_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart4TxDma
    }
    ///Signal `uart5_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_uart5_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart5RxDma
    }
    ///Signal `uart5_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_uart5_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart5TxDma
    }
    ///Signal `dac_ch1_dma` selected as request input
    #[inline(always)]
    pub fn is_dac_ch1_dma(&self) -> bool {
        *self == DMAREQ_ID::DacCh1Dma
    }
    ///Signal `dac_ch2_dma` selected as request input
    #[inline(always)]
    pub fn is_dac_ch2_dma(&self) -> bool {
        *self == DMAREQ_ID::DacCh2Dma
    }
    ///Signal `tim6_up` selected as request input
    #[inline(always)]
    pub fn is_tim6_up(&self) -> bool {
        *self == DMAREQ_ID::Tim6Up
    }
    ///Signal `tim7_up` selected as request input
    #[inline(always)]
    pub fn is_tim7_up(&self) -> bool {
        *self == DMAREQ_ID::Tim7Up
    }
    ///Signal `usart6_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart6_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart6RxDma
    }
    ///Signal `usart6_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_usart6_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Usart6TxDma
    }
    ///Signal `i2c3_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c3_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c3RxDma
    }
    ///Signal `i2c3_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_i2c3_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::I2c3TxDma
    }
    ///Signal `dcmi_dma` selected as request input
    #[inline(always)]
    pub fn is_dcmi_dma(&self) -> bool {
        *self == DMAREQ_ID::DcmiDma
    }
    ///Signal `cryp_in_dma` selected as request input
    #[inline(always)]
    pub fn is_cryp_in_dma(&self) -> bool {
        *self == DMAREQ_ID::CrypInDma
    }
    ///Signal `cryp_out_dma` selected as request input
    #[inline(always)]
    pub fn is_cryp_out_dma(&self) -> bool {
        *self == DMAREQ_ID::CrypOutDma
    }
    ///Signal `hash_in_dma` selected as request input
    #[inline(always)]
    pub fn is_hash_in_dma(&self) -> bool {
        *self == DMAREQ_ID::HashInDma
    }
    ///Signal `uart7_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_uart7_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart7RxDma
    }
    ///Signal `uart7_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_uart7_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart7TxDma
    }
    ///Signal `uart8_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_uart8_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart8RxDma
    }
    ///Signal `uart8_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_uart8_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Uart8TxDma
    }
    ///Signal `spi4_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi4_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi4RxDma
    }
    ///Signal `spi4_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi4_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi4TxDma
    }
    ///Signal `spi5_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi5_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi5RxDma
    }
    ///Signal `spi5_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_spi5_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::Spi5TxDma
    }
    ///Signal `sai1a_dma` selected as request input
    #[inline(always)]
    pub fn is_sai1a_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai1aDma
    }
    ///Signal `sai1b_dma` selected as request input
    #[inline(always)]
    pub fn is_sai1b_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai1bDma
    }
    ///Signal `sai2a_dma` selected as request input
    #[inline(always)]
    pub fn is_sai2a_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai2aDma
    }
    ///Signal `sai2b_dma` selected as request input
    #[inline(always)]
    pub fn is_sai2b_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai2bDma
    }
    ///Signal `swpmi_rx_dma` selected as request input
    #[inline(always)]
    pub fn is_swpmi_rx_dma(&self) -> bool {
        *self == DMAREQ_ID::SwpmiRxDma
    }
    ///Signal `swpmi_tx_dma` selected as request input
    #[inline(always)]
    pub fn is_swpmi_tx_dma(&self) -> bool {
        *self == DMAREQ_ID::SwpmiTxDma
    }
    ///Signal `spdifrx_dat_dma` selected as request input
    #[inline(always)]
    pub fn is_spdifrx_dat_dma(&self) -> bool {
        *self == DMAREQ_ID::SpdifrxDatDma
    }
    ///Signal `spdifrx_ctrl_dma` selected as request input
    #[inline(always)]
    pub fn is_spdifrx_ctrl_dma(&self) -> bool {
        *self == DMAREQ_ID::SpdifrxCtrlDma
    }
    ///Signal `hr_req(1)` selected as request input
    #[inline(always)]
    pub fn is_hr_req1(&self) -> bool {
        *self == DMAREQ_ID::HrReq1
    }
    ///Signal `hr_req(2)` selected as request input
    #[inline(always)]
    pub fn is_hr_req2(&self) -> bool {
        *self == DMAREQ_ID::HrReq2
    }
    ///Signal `hr_req(3)` selected as request input
    #[inline(always)]
    pub fn is_hr_req3(&self) -> bool {
        *self == DMAREQ_ID::HrReq3
    }
    ///Signal `hr_req(4)` selected as request input
    #[inline(always)]
    pub fn is_hr_req4(&self) -> bool {
        *self == DMAREQ_ID::HrReq4
    }
    ///Signal `hr_req(5)` selected as request input
    #[inline(always)]
    pub fn is_hr_req5(&self) -> bool {
        *self == DMAREQ_ID::HrReq5
    }
    ///Signal `hr_req(6)` selected as request input
    #[inline(always)]
    pub fn is_hr_req6(&self) -> bool {
        *self == DMAREQ_ID::HrReq6
    }
    ///Signal `dfsdm1_dma0` selected as request input
    #[inline(always)]
    pub fn is_dfsdm1_dma0(&self) -> bool {
        *self == DMAREQ_ID::Dfsdm1Dma0
    }
    ///Signal `dfsdm1_dma1` selected as request input
    #[inline(always)]
    pub fn is_dfsdm1_dma1(&self) -> bool {
        *self == DMAREQ_ID::Dfsdm1Dma1
    }
    ///Signal `dfsdm1_dma2` selected as request input
    #[inline(always)]
    pub fn is_dfsdm1_dma2(&self) -> bool {
        *self == DMAREQ_ID::Dfsdm1Dma2
    }
    ///Signal `dfsdm1_dma3` selected as request input
    #[inline(always)]
    pub fn is_dfsdm1_dma3(&self) -> bool {
        *self == DMAREQ_ID::Dfsdm1Dma3
    }
    ///Signal `tim15_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim15_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim15Ch1
    }
    ///Signal `tim15_up` selected as request input
    #[inline(always)]
    pub fn is_tim15_up(&self) -> bool {
        *self == DMAREQ_ID::Tim15Up
    }
    ///Signal `tim15_trig` selected as request input
    #[inline(always)]
    pub fn is_tim15_trig(&self) -> bool {
        *self == DMAREQ_ID::Tim15Trig
    }
    ///Signal `tim15_com` selected as request input
    #[inline(always)]
    pub fn is_tim15_com(&self) -> bool {
        *self == DMAREQ_ID::Tim15Com
    }
    ///Signal `tim16_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim16_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim16Ch1
    }
    ///Signal `tim16_up` selected as request input
    #[inline(always)]
    pub fn is_tim16_up(&self) -> bool {
        *self == DMAREQ_ID::Tim16Up
    }
    ///Signal `tim17_ch1` selected as request input
    #[inline(always)]
    pub fn is_tim17_ch1(&self) -> bool {
        *self == DMAREQ_ID::Tim17Ch1
    }
    ///Signal `tim17_up` selected as request input
    #[inline(always)]
    pub fn is_tim17_up(&self) -> bool {
        *self == DMAREQ_ID::Tim17Up
    }
    ///Signal `sai3_a_dma` selected as request input
    #[inline(always)]
    pub fn is_sai3_a_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai3ADma
    }
    ///Signal `sai3_b_dma` selected as request input
    #[inline(always)]
    pub fn is_sai3_b_dma(&self) -> bool {
        *self == DMAREQ_ID::Sai3BDma
    }
    ///Signal `adc3_dma` selected as request input
    #[inline(always)]
    pub fn is_adc3_dma(&self) -> bool {
        *self == DMAREQ_ID::Adc3Dma
    }
}
///Field `DMAREQ_ID` writer - DMA request identification
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7, DMAREQ_ID>;
impl<'a, REG> DMAREQ_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No signal selected as request input
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::None)
    }
    ///Signal `dmamux1_req_gen0` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen0)
    }
    ///Signal `dmamux1_req_gen1` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen1)
    }
    ///Signal `dmamux1_req_gen2` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen2)
    }
    ///Signal `dmamux1_req_gen3` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen3)
    }
    ///Signal `dmamux1_req_gen4` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen4)
    }
    ///Signal `dmamux1_req_gen5` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen5(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen5)
    }
    ///Signal `dmamux1_req_gen6` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen6(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen6)
    }
    ///Signal `dmamux1_req_gen7` selected as request input
    #[inline(always)]
    pub fn dmamux1_req_gen7(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dmamux1ReqGen7)
    }
    ///Signal `adc1_dma` selected as request input
    #[inline(always)]
    pub fn adc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Adc1Dma)
    }
    ///Signal `adc2_dma` selected as request input
    #[inline(always)]
    pub fn adc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Adc2Dma)
    }
    ///Signal `tim1_ch1` selected as request input
    #[inline(always)]
    pub fn tim1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch1)
    }
    ///Signal `tim1_ch2` selected as request input
    #[inline(always)]
    pub fn tim1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch2)
    }
    ///Signal `tim1_ch3` selected as request input
    #[inline(always)]
    pub fn tim1_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch3)
    }
    ///Signal `tim1_ch4` selected as request input
    #[inline(always)]
    pub fn tim1_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Ch4)
    }
    ///Signal `tim1_up` selected as request input
    #[inline(always)]
    pub fn tim1_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Up)
    }
    ///Signal `tim1_trig` selected as request input
    #[inline(always)]
    pub fn tim1_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Trig)
    }
    ///Signal `tim1_com` selected as request input
    #[inline(always)]
    pub fn tim1_com(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim1Com)
    }
    ///Signal `tim2_ch1` selected as request input
    #[inline(always)]
    pub fn tim2_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch1)
    }
    ///Signal `tim2_ch2` selected as request input
    #[inline(always)]
    pub fn tim2_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch2)
    }
    ///Signal `tim2_ch3` selected as request input
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch3)
    }
    ///Signal `tim2_ch4` selected as request input
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Ch4)
    }
    ///Signal `tim2_up` selected as request input
    #[inline(always)]
    pub fn tim2_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim2Up)
    }
    ///Signal `tim3_ch1` selected as request input
    #[inline(always)]
    pub fn tim3_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Ch1)
    }
    ///Signal `tim3_ch2` selected as request input
    #[inline(always)]
    pub fn tim3_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Ch2)
    }
    ///Signal `tim3_ch3` selected as request input
    #[inline(always)]
    pub fn tim3_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Ch3)
    }
    ///Signal `tim3_ch4` selected as request input
    #[inline(always)]
    pub fn tim3_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Ch4)
    }
    ///Signal `tim3_up` selected as request input
    #[inline(always)]
    pub fn tim3_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Up)
    }
    ///Signal `tim3_trig` selected as request input
    #[inline(always)]
    pub fn tim3_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim3Trig)
    }
    ///Signal `tim4_ch1` selected as request input
    #[inline(always)]
    pub fn tim4_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim4Ch1)
    }
    ///Signal `tim4_ch2` selected as request input
    #[inline(always)]
    pub fn tim4_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim4Ch2)
    }
    ///Signal `tim4_ch3` selected as request input
    #[inline(always)]
    pub fn tim4_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim4Ch3)
    }
    ///Signal `tim4_up` selected as request input
    #[inline(always)]
    pub fn tim4_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim4Up)
    }
    ///Signal `i2c1_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c1RxDma)
    }
    ///Signal `i2c1_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c1TxDma)
    }
    ///Signal `i2c2_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c2RxDma)
    }
    ///Signal `i2c2_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c2TxDma)
    }
    ///Signal `spi1_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi1RxDma)
    }
    ///Signal `spi1_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi1TxDma)
    }
    ///Signal `spi2_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi2RxDma)
    }
    ///Signal `spi2_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi2TxDma)
    }
    ///Signal `usart1_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart1RxDma)
    }
    ///Signal `usart1_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart1TxDma)
    }
    ///Signal `usart2_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart2RxDma)
    }
    ///Signal `usart2_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart2TxDma)
    }
    ///Signal `usart3_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart3RxDma)
    }
    ///Signal `usart3_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart3TxDma)
    }
    ///Signal `tim8_ch1` selected as request input
    #[inline(always)]
    pub fn tim8_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Ch1)
    }
    ///Signal `tim8_ch2` selected as request input
    #[inline(always)]
    pub fn tim8_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Ch2)
    }
    ///Signal `tim8_ch3` selected as request input
    #[inline(always)]
    pub fn tim8_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Ch3)
    }
    ///Signal `tim8_ch4` selected as request input
    #[inline(always)]
    pub fn tim8_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Ch4)
    }
    ///Signal `tim8_up` selected as request input
    #[inline(always)]
    pub fn tim8_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Up)
    }
    ///Signal `tim8_trig` selected as request input
    #[inline(always)]
    pub fn tim8_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Trig)
    }
    ///Signal `tim8_com` selected as request input
    #[inline(always)]
    pub fn tim8_com(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim8Com)
    }
    ///Signal `tim5_ch1` selected as request input
    #[inline(always)]
    pub fn tim5_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Ch1)
    }
    ///Signal `tim5_ch2` selected as request input
    #[inline(always)]
    pub fn tim5_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Ch2)
    }
    ///Signal `tim5_ch3` selected as request input
    #[inline(always)]
    pub fn tim5_ch3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Ch3)
    }
    ///Signal `tim5_ch4` selected as request input
    #[inline(always)]
    pub fn tim5_ch4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Ch4)
    }
    ///Signal `tim5_up` selected as request input
    #[inline(always)]
    pub fn tim5_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Up)
    }
    ///Signal `tim5_trig` selected as request input
    #[inline(always)]
    pub fn tim5_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim5Trig)
    }
    ///Signal `spi3_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi3RxDma)
    }
    ///Signal `spi3_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi3TxDma)
    }
    ///Signal `uart4_rx_dma` selected as request input
    #[inline(always)]
    pub fn uart4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart4RxDma)
    }
    ///Signal `uart4_tx_dma` selected as request input
    #[inline(always)]
    pub fn uart4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart4TxDma)
    }
    ///Signal `uart5_rx_dma` selected as request input
    #[inline(always)]
    pub fn uart5_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart5RxDma)
    }
    ///Signal `uart5_tx_dma` selected as request input
    #[inline(always)]
    pub fn uart5_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart5TxDma)
    }
    ///Signal `dac_ch1_dma` selected as request input
    #[inline(always)]
    pub fn dac_ch1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::DacCh1Dma)
    }
    ///Signal `dac_ch2_dma` selected as request input
    #[inline(always)]
    pub fn dac_ch2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::DacCh2Dma)
    }
    ///Signal `tim6_up` selected as request input
    #[inline(always)]
    pub fn tim6_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim6Up)
    }
    ///Signal `tim7_up` selected as request input
    #[inline(always)]
    pub fn tim7_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim7Up)
    }
    ///Signal `usart6_rx_dma` selected as request input
    #[inline(always)]
    pub fn usart6_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart6RxDma)
    }
    ///Signal `usart6_tx_dma` selected as request input
    #[inline(always)]
    pub fn usart6_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Usart6TxDma)
    }
    ///Signal `i2c3_rx_dma` selected as request input
    #[inline(always)]
    pub fn i2c3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c3RxDma)
    }
    ///Signal `i2c3_tx_dma` selected as request input
    #[inline(always)]
    pub fn i2c3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::I2c3TxDma)
    }
    ///Signal `dcmi_dma` selected as request input
    #[inline(always)]
    pub fn dcmi_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::DcmiDma)
    }
    ///Signal `cryp_in_dma` selected as request input
    #[inline(always)]
    pub fn cryp_in_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::CrypInDma)
    }
    ///Signal `cryp_out_dma` selected as request input
    #[inline(always)]
    pub fn cryp_out_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::CrypOutDma)
    }
    ///Signal `hash_in_dma` selected as request input
    #[inline(always)]
    pub fn hash_in_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HashInDma)
    }
    ///Signal `uart7_rx_dma` selected as request input
    #[inline(always)]
    pub fn uart7_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart7RxDma)
    }
    ///Signal `uart7_tx_dma` selected as request input
    #[inline(always)]
    pub fn uart7_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart7TxDma)
    }
    ///Signal `uart8_rx_dma` selected as request input
    #[inline(always)]
    pub fn uart8_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart8RxDma)
    }
    ///Signal `uart8_tx_dma` selected as request input
    #[inline(always)]
    pub fn uart8_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Uart8TxDma)
    }
    ///Signal `spi4_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi4RxDma)
    }
    ///Signal `spi4_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi4TxDma)
    }
    ///Signal `spi5_rx_dma` selected as request input
    #[inline(always)]
    pub fn spi5_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi5RxDma)
    }
    ///Signal `spi5_tx_dma` selected as request input
    #[inline(always)]
    pub fn spi5_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Spi5TxDma)
    }
    ///Signal `sai1a_dma` selected as request input
    #[inline(always)]
    pub fn sai1a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai1aDma)
    }
    ///Signal `sai1b_dma` selected as request input
    #[inline(always)]
    pub fn sai1b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai1bDma)
    }
    ///Signal `sai2a_dma` selected as request input
    #[inline(always)]
    pub fn sai2a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai2aDma)
    }
    ///Signal `sai2b_dma` selected as request input
    #[inline(always)]
    pub fn sai2b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai2bDma)
    }
    ///Signal `swpmi_rx_dma` selected as request input
    #[inline(always)]
    pub fn swpmi_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::SwpmiRxDma)
    }
    ///Signal `swpmi_tx_dma` selected as request input
    #[inline(always)]
    pub fn swpmi_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::SwpmiTxDma)
    }
    ///Signal `spdifrx_dat_dma` selected as request input
    #[inline(always)]
    pub fn spdifrx_dat_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::SpdifrxDatDma)
    }
    ///Signal `spdifrx_ctrl_dma` selected as request input
    #[inline(always)]
    pub fn spdifrx_ctrl_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::SpdifrxCtrlDma)
    }
    ///Signal `hr_req(1)` selected as request input
    #[inline(always)]
    pub fn hr_req1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq1)
    }
    ///Signal `hr_req(2)` selected as request input
    #[inline(always)]
    pub fn hr_req2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq2)
    }
    ///Signal `hr_req(3)` selected as request input
    #[inline(always)]
    pub fn hr_req3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq3)
    }
    ///Signal `hr_req(4)` selected as request input
    #[inline(always)]
    pub fn hr_req4(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq4)
    }
    ///Signal `hr_req(5)` selected as request input
    #[inline(always)]
    pub fn hr_req5(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq5)
    }
    ///Signal `hr_req(6)` selected as request input
    #[inline(always)]
    pub fn hr_req6(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::HrReq6)
    }
    ///Signal `dfsdm1_dma0` selected as request input
    #[inline(always)]
    pub fn dfsdm1_dma0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dfsdm1Dma0)
    }
    ///Signal `dfsdm1_dma1` selected as request input
    #[inline(always)]
    pub fn dfsdm1_dma1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dfsdm1Dma1)
    }
    ///Signal `dfsdm1_dma2` selected as request input
    #[inline(always)]
    pub fn dfsdm1_dma2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dfsdm1Dma2)
    }
    ///Signal `dfsdm1_dma3` selected as request input
    #[inline(always)]
    pub fn dfsdm1_dma3(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Dfsdm1Dma3)
    }
    ///Signal `tim15_ch1` selected as request input
    #[inline(always)]
    pub fn tim15_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim15Ch1)
    }
    ///Signal `tim15_up` selected as request input
    #[inline(always)]
    pub fn tim15_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim15Up)
    }
    ///Signal `tim15_trig` selected as request input
    #[inline(always)]
    pub fn tim15_trig(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim15Trig)
    }
    ///Signal `tim15_com` selected as request input
    #[inline(always)]
    pub fn tim15_com(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim15Com)
    }
    ///Signal `tim16_ch1` selected as request input
    #[inline(always)]
    pub fn tim16_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim16Ch1)
    }
    ///Signal `tim16_up` selected as request input
    #[inline(always)]
    pub fn tim16_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim16Up)
    }
    ///Signal `tim17_ch1` selected as request input
    #[inline(always)]
    pub fn tim17_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim17Ch1)
    }
    ///Signal `tim17_up` selected as request input
    #[inline(always)]
    pub fn tim17_up(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Tim17Up)
    }
    ///Signal `sai3_a_dma` selected as request input
    #[inline(always)]
    pub fn sai3_a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai3ADma)
    }
    ///Signal `sai3_b_dma` selected as request input
    #[inline(always)]
    pub fn sai3_b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Sai3BDma)
    }
    ///Signal `adc3_dma` selected as request input
    #[inline(always)]
    pub fn adc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(DMAREQ_ID::Adc3Dma)
    }
}
/**Synchronization overrun interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOIE {
    ///0: Synchronization overrun interrupt disabled
    Disabled = 0,
    ///1: Synchronization overrun interrupt enabled
    Enabled = 1,
}
impl From<SOIE> for bool {
    #[inline(always)]
    fn from(variant: SOIE) -> Self {
        variant as u8 != 0
    }
}
///Field `SOIE` reader - Synchronization overrun interrupt enable
pub type SOIE_R = crate::BitReader<SOIE>;
impl SOIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SOIE {
        match self.bits {
            false => SOIE::Disabled,
            true => SOIE::Enabled,
        }
    }
    ///Synchronization overrun interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOIE::Disabled
    }
    ///Synchronization overrun interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOIE::Enabled
    }
}
///Field `SOIE` writer - Synchronization overrun interrupt enable
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG, SOIE>;
impl<'a, REG> SOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Synchronization overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::Disabled)
    }
    ///Synchronization overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SOIE::Enabled)
    }
}
/**Event generation enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EGE {
    ///0: Event generation disabled
    Disabled = 0,
    ///1: Event generation enabled
    Enabled = 1,
}
impl From<EGE> for bool {
    #[inline(always)]
    fn from(variant: EGE) -> Self {
        variant as u8 != 0
    }
}
///Field `EGE` reader - Event generation enable
pub type EGE_R = crate::BitReader<EGE>;
impl EGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EGE {
        match self.bits {
            false => EGE::Disabled,
            true => EGE::Enabled,
        }
    }
    ///Event generation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EGE::Disabled
    }
    ///Event generation enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EGE::Enabled
    }
}
///Field `EGE` writer - Event generation enable
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG, EGE>;
impl<'a, REG> EGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::Disabled)
    }
    ///Event generation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EGE::Enabled)
    }
}
/**Synchronization enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE {
    ///0: Synchronization disabled
    Disabled = 0,
    ///1: Synchronization enabled
    Enabled = 1,
}
impl From<SE> for bool {
    #[inline(always)]
    fn from(variant: SE) -> Self {
        variant as u8 != 0
    }
}
///Field `SE` reader - Synchronization enable
pub type SE_R = crate::BitReader<SE>;
impl SE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SE {
        match self.bits {
            false => SE::Disabled,
            true => SE::Enabled,
        }
    }
    ///Synchronization disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SE::Disabled
    }
    ///Synchronization enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SE::Enabled
    }
}
///Field `SE` writer - Synchronization enable
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG, SE>;
impl<'a, REG> SE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Synchronization disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SE::Disabled)
    }
    ///Synchronization enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SE::Enabled)
    }
}
/**Synchronization polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPOL {
    ///0: No event, i.e. no synchronization nor detection
    NoEdge = 0,
    ///1: Rising edge
    RisingEdge = 1,
    ///2: Falling edge
    FallingEdge = 2,
    ///3: Rising and falling edges
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
impl crate::IsEnum for SPOL {}
///Field `SPOL` reader - Synchronization polarity
pub type SPOL_R = crate::FieldReader<SPOL>;
impl SPOL_R {
    ///Get enumerated values variant
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
    ///No event, i.e. no synchronization nor detection
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == SPOL::NoEdge
    }
    ///Rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == SPOL::RisingEdge
    }
    ///Falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == SPOL::FallingEdge
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == SPOL::BothEdges
    }
}
///Field `SPOL` writer - Synchronization polarity
pub type SPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPOL, crate::Safe>;
impl<'a, REG> SPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No event, i.e. no synchronization nor detection
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::NoEdge)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::RisingEdge)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::FallingEdge)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(SPOL::BothEdges)
    }
}
///Field `NBREQ` reader - Number of DMA requests minus 1 to forward
pub type NBREQ_R = crate::FieldReader;
///Field `NBREQ` writer - Number of DMA requests minus 1 to forward
pub type NBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Synchronization identification

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNC_ID {
    ///0: Signal `dmamux1_evt0` selected as synchronization input
    Dmamux1Evt0 = 0,
    ///1: Signal `dmamux1_evt1` selected as synchronization input
    Dmamux1Evt1 = 1,
    ///2: Signal `dmamux1_evt2` selected as synchronization input
    Dmamux1Evt2 = 2,
    ///3: Signal `lptim1_out` selected as synchronization input
    Lptim1Out = 3,
    ///4: Signal `lptim2_out` selected as synchronization input
    Lptim2Out = 4,
    ///5: Signal `lptim3_out` selected as synchronization input
    Lptim3Out = 5,
    ///6: Signal `extit0` selected as synchronization input
    Extit0 = 6,
    ///7: Signal `tim12_trgo` selected as synchronization input
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
impl crate::IsEnum for SYNC_ID {}
///Field `SYNC_ID` reader - Synchronization identification
pub type SYNC_ID_R = crate::FieldReader<SYNC_ID>;
impl SYNC_ID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNC_ID {
        match self.bits {
            0 => SYNC_ID::Dmamux1Evt0,
            1 => SYNC_ID::Dmamux1Evt1,
            2 => SYNC_ID::Dmamux1Evt2,
            3 => SYNC_ID::Lptim1Out,
            4 => SYNC_ID::Lptim2Out,
            5 => SYNC_ID::Lptim3Out,
            6 => SYNC_ID::Extit0,
            7 => SYNC_ID::Tim12Trgo,
            _ => unreachable!(),
        }
    }
    ///Signal `dmamux1_evt0` selected as synchronization input
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        *self == SYNC_ID::Dmamux1Evt0
    }
    ///Signal `dmamux1_evt1` selected as synchronization input
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        *self == SYNC_ID::Dmamux1Evt1
    }
    ///Signal `dmamux1_evt2` selected as synchronization input
    #[inline(always)]
    pub fn is_dmamux1_evt2(&self) -> bool {
        *self == SYNC_ID::Dmamux1Evt2
    }
    ///Signal `lptim1_out` selected as synchronization input
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == SYNC_ID::Lptim1Out
    }
    ///Signal `lptim2_out` selected as synchronization input
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SYNC_ID::Lptim2Out
    }
    ///Signal `lptim3_out` selected as synchronization input
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SYNC_ID::Lptim3Out
    }
    ///Signal `extit0` selected as synchronization input
    #[inline(always)]
    pub fn is_extit0(&self) -> bool {
        *self == SYNC_ID::Extit0
    }
    ///Signal `tim12_trgo` selected as synchronization input
    #[inline(always)]
    pub fn is_tim12_trgo(&self) -> bool {
        *self == SYNC_ID::Tim12Trgo
    }
}
///Field `SYNC_ID` writer - Synchronization identification
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SYNC_ID, crate::Safe>;
impl<'a, REG> SYNC_ID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Signal `dmamux1_evt0` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux1Evt0)
    }
    ///Signal `dmamux1_evt1` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux1Evt1)
    }
    ///Signal `dmamux1_evt2` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt2(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Dmamux1Evt2)
    }
    ///Signal `lptim1_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim1Out)
    }
    ///Signal `lptim2_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim2Out)
    }
    ///Signal `lptim3_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Lptim3Out)
    }
    ///Signal `extit0` selected as synchronization input
    #[inline(always)]
    pub fn extit0(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Extit0)
    }
    ///Signal `tim12_trgo` selected as synchronization input
    #[inline(always)]
    pub fn tim12_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(SYNC_ID::Tim12Trgo)
    }
}
impl R {
    ///Bits 0:6 - DMA request identification
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Synchronization polarity
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:26 - Synchronization identification
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("dmareq_id", &self.dmareq_id())
            .field("soie", &self.soie())
            .field("ege", &self.ege())
            .field("se", &self.se())
            .field("spol", &self.spol())
            .field("nbreq", &self.nbreq())
            .field("sync_id", &self.sync_id())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - DMA request identification
    #[inline(always)]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<'_, CCRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
    ///Bit 8 - Synchronization overrun interrupt enable
    #[inline(always)]
    pub fn soie(&mut self) -> SOIE_W<'_, CCRrs> {
        SOIE_W::new(self, 8)
    }
    ///Bit 9 - Event generation enable
    #[inline(always)]
    pub fn ege(&mut self) -> EGE_W<'_, CCRrs> {
        EGE_W::new(self, 9)
    }
    ///Bit 16 - Synchronization enable
    #[inline(always)]
    pub fn se(&mut self) -> SE_W<'_, CCRrs> {
        SE_W::new(self, 16)
    }
    ///Bits 17:18 - Synchronization polarity
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W<'_, CCRrs> {
        SPOL_W::new(self, 17)
    }
    ///Bits 19:23 - Number of DMA requests minus 1 to forward
    #[inline(always)]
    pub fn nbreq(&mut self) -> NBREQ_W<'_, CCRrs> {
        NBREQ_W::new(self, 19)
    }
    ///Bits 24:26 - Synchronization identification
    #[inline(always)]
    pub fn sync_id(&mut self) -> SYNC_ID_W<'_, CCRrs> {
        SYNC_ID_W::new(self, 24)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#DMAMUX1:C[0]CR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C%sCR to value 0
impl crate::Resettable for CCRrs {}
