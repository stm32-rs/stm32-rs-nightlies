///Register `TR2` reader
pub type R = crate::R<TR2rs>;
///Register `TR2` writer
pub type W = crate::W<TR2rs>;
/**GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\[7:0\] value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REQSEL {
    ///0: adc1_dma selected
    Adc1Dma = 0,
    ///2: dac1_ch1_dma selected
    Dac1Ch1Dma = 2,
    ///3: dac1_ch2_dma selected
    Dac1Ch2Dma = 3,
    ///4: tim6_upd_dma selected
    Tim6UpdDma = 4,
    ///5: tim7_upd_dma selected
    Tim7UpdDma = 5,
    ///6: spi1_rx_dma selected
    Spi1RxDma = 6,
    ///7: spi1_tx_dma selected
    Spi1TxDma = 7,
    ///8: spi2_rx_dma selected
    Spi2RxDma = 8,
    ///9: spi2_tx_dma selected
    Spi2TxDma = 9,
    ///10: spi3_rx_dma selected
    Spi3RxDma = 10,
    ///11: spi3_tx_dma selected
    Spi3TxDma = 11,
    ///12: i2c1_rx_dma selected
    I2c1RxDma = 12,
    ///13: i2c1_tx_dma selected
    I2c1TxDma = 13,
    ///15: i2c2_rx_dma selected
    I2c2RxDma = 15,
    ///16: i2c2_tx_dma selected
    I2c2TxDma = 16,
    ///18: i2c3_rx_dma selected
    I2c3RxDma = 18,
    ///19: i2c3_tx_dma selected
    I2c3TxDma = 19,
    ///21: usart1_rx_dma selected
    Usart1RxDma = 21,
    ///22: usart1_tx_dma selected
    Usart1TxDma = 22,
    ///23: usart2_rx_dma selected
    Usart2RxDma = 23,
    ///24: usart2_tx_dma selected
    Usart2TxDma = 24,
    ///25: usart3_rx_dma selected
    Usart3RxDma = 25,
    ///26: usart3_tx_dma selected
    Usart3TxDma = 26,
    ///27: uart4_rx_dma selected
    Uart4RxDma = 27,
    ///28: uart4_tx_dma selected
    Uart4TxDma = 28,
    ///29: uart5_rx_dma selected
    Uart5RxDma = 29,
    ///30: uart5_tx_dma selected
    Uart5TxDma = 30,
    ///31: usart6_rx_dma selected
    Usart6RxDma = 31,
    ///32: usart6_tx_dma selected
    Usart6TxDma = 32,
    ///33: uart7_rx_dma selected
    Uart7RxDma = 33,
    ///34: uart7_tx_dma selected
    Uart7TxDma = 34,
    ///35: uart8_rx_dma selected
    Uart8RxDma = 35,
    ///36: uart8_tx_dma selected
    Uart8TxDma = 36,
    ///37: uart9_rx_dma selected
    Uart9RxDma = 37,
    ///38: uart9_tx_dma selected
    Uart9TxDma = 38,
    ///39: uart10_rx_dma selected
    Uart10RxDma = 39,
    ///40: uart10_tx_dma selected
    Uart10TxDma = 40,
    ///41: uart11_rx_dma selected
    Uart11RxDma = 41,
    ///42: uart11_tx_dma selected
    Uart11TxDma = 42,
    ///43: uart12_rx_dma selected
    Uart12RxDma = 43,
    ///44: uart12_tx_dma selected
    Uart12TxDma = 44,
    ///45: lpuart1_rx_dma selected
    Lpuart1RxDma = 45,
    ///46: lpuart1_tx_dma selected
    Lpuart1TxDma = 46,
    ///47: spi4_rx_dma selected
    Spi4RxDma = 47,
    ///48: spi4_tx_dma selected
    Spi4TxDma = 48,
    ///49: spi5_rx_dma selected
    Spi5RxDma = 49,
    ///50: spi5_tx_dma selected
    Spi5TxDma = 50,
    ///51: spi6_rx_dma selected
    Spi6RxDma = 51,
    ///52: spi6_tx_dma selected
    Spi6TxDma = 52,
    ///53: sai1_a_dma selected
    Sai1ADma = 53,
    ///54: sai1_b_dma selected
    Sai1BDma = 54,
    ///55: sai2_a_dma selected
    Sai2ADma = 55,
    ///56: sai2_b_dma selected
    Sai2BDma = 56,
    ///57: ospi1_dma selected
    Ospi1Dma = 57,
    ///58: tim1_cc1_dma selected
    Tim1Cc1Dma = 58,
    ///59: tim1_cc2_dma selected
    Tim1Cc2Dma = 59,
    ///60: tim1_cc3_dma selected
    Tim1Cc3Dma = 60,
    ///61: tim1_cc4_dma selected
    Tim1Cc4Dma = 61,
    ///62: tim1_upd_dma selected
    Tim1UpdDma = 62,
    ///63: tim1_trg_dma selected
    Tim1TrgDma = 63,
    ///64: tim1_com_dma selected
    Tim1ComDma = 64,
    ///65: tim8_cc1_dma selected
    Tim8Cc1Dma = 65,
    ///66: tim8_cc2_dma selected
    Tim8Cc2Dma = 66,
    ///67: tim8_cc3_dma selected
    Tim8Cc3Dma = 67,
    ///68: tim8_cc4_dma selected
    Tim8Cc4Dma = 68,
    ///69: tim8_upd_dma selected
    Tim8UpdDma = 69,
    ///70: tim8_tig_dma selected
    Tim8TigDma = 70,
    ///71: tim8_com_dma selected
    Tim8ComDma = 71,
    ///72: tim2_cc1_dma selected
    Tim2Cc1Dma = 72,
    ///73: tim2_cc2_dma selected
    Tim2Cc2Dma = 73,
    ///74: tim2_cc3_dma selected
    Tim2Cc3Dma = 74,
    ///75: tim2_cc4_dma selected
    Tim2Cc4Dma = 75,
    ///76: tim2_upd_dma selected
    Tim2UpdDma = 76,
    ///77: tim3_cc1_dma selected
    Tim3Cc1Dma = 77,
    ///78: tim3_cc2_dma selected
    Tim3Cc2Dma = 78,
    ///79: tim3_cc3_dma selected
    Tim3Cc3Dma = 79,
    ///80: tim3_cc4_dma selected
    Tim3Cc4Dma = 80,
    ///81: tim3_upd_dma selected
    Tim3UpdDma = 81,
    ///82: tim3_trg_dma selected
    Tim3TrgDma = 82,
    ///83: tim4_cc1_dma selected
    Tim4Cc1Dma = 83,
    ///84: tim4_cc2_dma selected
    Tim4Cc2Dma = 84,
    ///85: tim4_cc3_dma selected
    Tim4Cc3Dma = 85,
    ///86: tim4_cc4_dma selected
    Tim4Cc4Dma = 86,
    ///87: tim4_upd_dma selected
    Tim4UpdDma = 87,
    ///88: tim5_cc1_dma selected
    Tim5Cc1Dma = 88,
    ///89: tim5_cc2_dma selected
    Tim5Cc2Dma = 89,
    ///90: tim5_cc3_dma selected
    Tim5Cc3Dma = 90,
    ///91: tim5_cc4_dma selected
    Tim5Cc4Dma = 91,
    ///92: tim5_upd_dma selected
    Tim5UpdDma = 92,
    ///93: tim5_trg_dma selected
    Tim5TrgDma = 93,
    ///94: tim15_cc1_dma selected
    Tim15Cc1Dma = 94,
    ///95: tim15_upd_dma selected
    Tim15UpdDma = 95,
    ///96: tim15_trg_dma selected
    Tim15TrgDma = 96,
    ///97: tim15_com_dma selected
    Tim15ComDma = 97,
    ///98: tim16_cc1_dma selected
    Tim16Cc1Dma = 98,
    ///99: tim16_upd_dma selected
    Tim16UpdDma = 99,
    ///100: tim17_cc1_dma selected
    Tim17Cc1Dma = 100,
    ///101: tim17_upd_dma selected
    Tim17UpdDma = 101,
    ///102: lptim1_ic1_dma selected
    Lptim1Ic1Dma = 102,
    ///103: lptim1_ic2_dma selected
    Lptim1Ic2Dma = 103,
    ///104: lptim1_ue_dma selected
    Lptim1UeDma = 104,
    ///105: lptim2_ic1_dma selected
    Lptim2Ic1Dma = 105,
    ///106: lptim2_ic2_dma selected
    Lptim2Ic2Dma = 106,
    ///107: lptim2_ue_dma selected
    Lptim2UeDma = 107,
    ///108: dcmi_dma or pssi_dma(1) selected
    DcmiPssiDma = 108,
    ///109: aes_out_dma selected
    AesOutDma = 109,
    ///110: aes_in_dma selected
    AesInDma = 110,
    ///111: hash_in_dma selected
    HashInDma = 111,
    ///112: ucpd1_rx_dma selected
    Ucpd1RxDma = 112,
    ///113: ucpd1_tx_dma selected
    Ucpd1TxDma = 113,
    ///114: cordic_read_dma selected
    CordicReadDma = 114,
    ///115: cordic_write_dma selected
    CordicWriteDma = 115,
    ///116: fmac_read_dma selected
    FmacReadDma = 116,
    ///117: fmac_write_dma selected
    FmacWriteDma = 117,
    ///118: saes_out_dma selected
    SaesOutDma = 118,
    ///119: saes_in_dma selected
    SaesInDma = 119,
    ///120: i3c1_rx_dma selected
    I3c1RxDma = 120,
    ///121: i3c1_tx_dma selected
    I3c1TxDma = 121,
    ///122: i3c1_tc_dma selected
    I3c1TcDma = 122,
    ///123: i3c1_rs_dma selected
    I3c1RsDma = 123,
    ///124: i2c4_rx_dma selected
    I2c4RxDma = 124,
    ///125: i2c4_tx_dma selected
    I2c4TxDma = 125,
    ///127: lptim3_ic1_dma selected
    Lptim3Ic1Dma = 127,
    ///128: lptim3_ic2_dma selected
    Lptim3Ic2Dma = 128,
    ///129: lptim3_ue_dma selected
    Lptim3UeDma = 129,
    ///130: lptim5_ic1_dma selected
    Lptim5Ic1Dma = 130,
    ///131: lptim5_ic2_dma selected
    Lptim5Ic2Dma = 131,
    ///132: lptim5_ue_dma selected
    Lptim5UeDma = 132,
    ///133: lptim6_ic1_dma selected
    Lptim6Ic1Dma = 133,
    ///134: lptim6_ic2_dma selected
    Lptim6Ic2Dma = 134,
    ///135: lptim6_ue_dma selected
    Lptim6UeDma = 135,
    ///136: i3c2_rx selected
    I3c2Rx = 136,
    ///137: i3c2_tx selected
    I3c2Tx = 137,
    ///138: i3c2_tc selected
    I3c2Tc = 138,
    ///139: i3c2_rs selected
    I3c2Rs = 139,
}
impl From<REQSEL> for u8 {
    #[inline(always)]
    fn from(variant: REQSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REQSEL {
    type Ux = u8;
}
impl crate::IsEnum for REQSEL {}
///Field `REQSEL` reader - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\[7:0\] value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
pub type REQSEL_R = crate::FieldReader<REQSEL>;
impl REQSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<REQSEL> {
        match self.bits {
            0 => Some(REQSEL::Adc1Dma),
            2 => Some(REQSEL::Dac1Ch1Dma),
            3 => Some(REQSEL::Dac1Ch2Dma),
            4 => Some(REQSEL::Tim6UpdDma),
            5 => Some(REQSEL::Tim7UpdDma),
            6 => Some(REQSEL::Spi1RxDma),
            7 => Some(REQSEL::Spi1TxDma),
            8 => Some(REQSEL::Spi2RxDma),
            9 => Some(REQSEL::Spi2TxDma),
            10 => Some(REQSEL::Spi3RxDma),
            11 => Some(REQSEL::Spi3TxDma),
            12 => Some(REQSEL::I2c1RxDma),
            13 => Some(REQSEL::I2c1TxDma),
            15 => Some(REQSEL::I2c2RxDma),
            16 => Some(REQSEL::I2c2TxDma),
            18 => Some(REQSEL::I2c3RxDma),
            19 => Some(REQSEL::I2c3TxDma),
            21 => Some(REQSEL::Usart1RxDma),
            22 => Some(REQSEL::Usart1TxDma),
            23 => Some(REQSEL::Usart2RxDma),
            24 => Some(REQSEL::Usart2TxDma),
            25 => Some(REQSEL::Usart3RxDma),
            26 => Some(REQSEL::Usart3TxDma),
            27 => Some(REQSEL::Uart4RxDma),
            28 => Some(REQSEL::Uart4TxDma),
            29 => Some(REQSEL::Uart5RxDma),
            30 => Some(REQSEL::Uart5TxDma),
            31 => Some(REQSEL::Usart6RxDma),
            32 => Some(REQSEL::Usart6TxDma),
            33 => Some(REQSEL::Uart7RxDma),
            34 => Some(REQSEL::Uart7TxDma),
            35 => Some(REQSEL::Uart8RxDma),
            36 => Some(REQSEL::Uart8TxDma),
            37 => Some(REQSEL::Uart9RxDma),
            38 => Some(REQSEL::Uart9TxDma),
            39 => Some(REQSEL::Uart10RxDma),
            40 => Some(REQSEL::Uart10TxDma),
            41 => Some(REQSEL::Uart11RxDma),
            42 => Some(REQSEL::Uart11TxDma),
            43 => Some(REQSEL::Uart12RxDma),
            44 => Some(REQSEL::Uart12TxDma),
            45 => Some(REQSEL::Lpuart1RxDma),
            46 => Some(REQSEL::Lpuart1TxDma),
            47 => Some(REQSEL::Spi4RxDma),
            48 => Some(REQSEL::Spi4TxDma),
            49 => Some(REQSEL::Spi5RxDma),
            50 => Some(REQSEL::Spi5TxDma),
            51 => Some(REQSEL::Spi6RxDma),
            52 => Some(REQSEL::Spi6TxDma),
            53 => Some(REQSEL::Sai1ADma),
            54 => Some(REQSEL::Sai1BDma),
            55 => Some(REQSEL::Sai2ADma),
            56 => Some(REQSEL::Sai2BDma),
            57 => Some(REQSEL::Ospi1Dma),
            58 => Some(REQSEL::Tim1Cc1Dma),
            59 => Some(REQSEL::Tim1Cc2Dma),
            60 => Some(REQSEL::Tim1Cc3Dma),
            61 => Some(REQSEL::Tim1Cc4Dma),
            62 => Some(REQSEL::Tim1UpdDma),
            63 => Some(REQSEL::Tim1TrgDma),
            64 => Some(REQSEL::Tim1ComDma),
            65 => Some(REQSEL::Tim8Cc1Dma),
            66 => Some(REQSEL::Tim8Cc2Dma),
            67 => Some(REQSEL::Tim8Cc3Dma),
            68 => Some(REQSEL::Tim8Cc4Dma),
            69 => Some(REQSEL::Tim8UpdDma),
            70 => Some(REQSEL::Tim8TigDma),
            71 => Some(REQSEL::Tim8ComDma),
            72 => Some(REQSEL::Tim2Cc1Dma),
            73 => Some(REQSEL::Tim2Cc2Dma),
            74 => Some(REQSEL::Tim2Cc3Dma),
            75 => Some(REQSEL::Tim2Cc4Dma),
            76 => Some(REQSEL::Tim2UpdDma),
            77 => Some(REQSEL::Tim3Cc1Dma),
            78 => Some(REQSEL::Tim3Cc2Dma),
            79 => Some(REQSEL::Tim3Cc3Dma),
            80 => Some(REQSEL::Tim3Cc4Dma),
            81 => Some(REQSEL::Tim3UpdDma),
            82 => Some(REQSEL::Tim3TrgDma),
            83 => Some(REQSEL::Tim4Cc1Dma),
            84 => Some(REQSEL::Tim4Cc2Dma),
            85 => Some(REQSEL::Tim4Cc3Dma),
            86 => Some(REQSEL::Tim4Cc4Dma),
            87 => Some(REQSEL::Tim4UpdDma),
            88 => Some(REQSEL::Tim5Cc1Dma),
            89 => Some(REQSEL::Tim5Cc2Dma),
            90 => Some(REQSEL::Tim5Cc3Dma),
            91 => Some(REQSEL::Tim5Cc4Dma),
            92 => Some(REQSEL::Tim5UpdDma),
            93 => Some(REQSEL::Tim5TrgDma),
            94 => Some(REQSEL::Tim15Cc1Dma),
            95 => Some(REQSEL::Tim15UpdDma),
            96 => Some(REQSEL::Tim15TrgDma),
            97 => Some(REQSEL::Tim15ComDma),
            98 => Some(REQSEL::Tim16Cc1Dma),
            99 => Some(REQSEL::Tim16UpdDma),
            100 => Some(REQSEL::Tim17Cc1Dma),
            101 => Some(REQSEL::Tim17UpdDma),
            102 => Some(REQSEL::Lptim1Ic1Dma),
            103 => Some(REQSEL::Lptim1Ic2Dma),
            104 => Some(REQSEL::Lptim1UeDma),
            105 => Some(REQSEL::Lptim2Ic1Dma),
            106 => Some(REQSEL::Lptim2Ic2Dma),
            107 => Some(REQSEL::Lptim2UeDma),
            108 => Some(REQSEL::DcmiPssiDma),
            109 => Some(REQSEL::AesOutDma),
            110 => Some(REQSEL::AesInDma),
            111 => Some(REQSEL::HashInDma),
            112 => Some(REQSEL::Ucpd1RxDma),
            113 => Some(REQSEL::Ucpd1TxDma),
            114 => Some(REQSEL::CordicReadDma),
            115 => Some(REQSEL::CordicWriteDma),
            116 => Some(REQSEL::FmacReadDma),
            117 => Some(REQSEL::FmacWriteDma),
            118 => Some(REQSEL::SaesOutDma),
            119 => Some(REQSEL::SaesInDma),
            120 => Some(REQSEL::I3c1RxDma),
            121 => Some(REQSEL::I3c1TxDma),
            122 => Some(REQSEL::I3c1TcDma),
            123 => Some(REQSEL::I3c1RsDma),
            124 => Some(REQSEL::I2c4RxDma),
            125 => Some(REQSEL::I2c4TxDma),
            127 => Some(REQSEL::Lptim3Ic1Dma),
            128 => Some(REQSEL::Lptim3Ic2Dma),
            129 => Some(REQSEL::Lptim3UeDma),
            130 => Some(REQSEL::Lptim5Ic1Dma),
            131 => Some(REQSEL::Lptim5Ic2Dma),
            132 => Some(REQSEL::Lptim5UeDma),
            133 => Some(REQSEL::Lptim6Ic1Dma),
            134 => Some(REQSEL::Lptim6Ic2Dma),
            135 => Some(REQSEL::Lptim6UeDma),
            136 => Some(REQSEL::I3c2Rx),
            137 => Some(REQSEL::I3c2Tx),
            138 => Some(REQSEL::I3c2Tc),
            139 => Some(REQSEL::I3c2Rs),
            _ => None,
        }
    }
    ///adc1_dma selected
    #[inline(always)]
    pub fn is_adc1_dma(&self) -> bool {
        *self == REQSEL::Adc1Dma
    }
    ///dac1_ch1_dma selected
    #[inline(always)]
    pub fn is_dac1_ch1_dma(&self) -> bool {
        *self == REQSEL::Dac1Ch1Dma
    }
    ///dac1_ch2_dma selected
    #[inline(always)]
    pub fn is_dac1_ch2_dma(&self) -> bool {
        *self == REQSEL::Dac1Ch2Dma
    }
    ///tim6_upd_dma selected
    #[inline(always)]
    pub fn is_tim6_upd_dma(&self) -> bool {
        *self == REQSEL::Tim6UpdDma
    }
    ///tim7_upd_dma selected
    #[inline(always)]
    pub fn is_tim7_upd_dma(&self) -> bool {
        *self == REQSEL::Tim7UpdDma
    }
    ///spi1_rx_dma selected
    #[inline(always)]
    pub fn is_spi1_rx_dma(&self) -> bool {
        *self == REQSEL::Spi1RxDma
    }
    ///spi1_tx_dma selected
    #[inline(always)]
    pub fn is_spi1_tx_dma(&self) -> bool {
        *self == REQSEL::Spi1TxDma
    }
    ///spi2_rx_dma selected
    #[inline(always)]
    pub fn is_spi2_rx_dma(&self) -> bool {
        *self == REQSEL::Spi2RxDma
    }
    ///spi2_tx_dma selected
    #[inline(always)]
    pub fn is_spi2_tx_dma(&self) -> bool {
        *self == REQSEL::Spi2TxDma
    }
    ///spi3_rx_dma selected
    #[inline(always)]
    pub fn is_spi3_rx_dma(&self) -> bool {
        *self == REQSEL::Spi3RxDma
    }
    ///spi3_tx_dma selected
    #[inline(always)]
    pub fn is_spi3_tx_dma(&self) -> bool {
        *self == REQSEL::Spi3TxDma
    }
    ///i2c1_rx_dma selected
    #[inline(always)]
    pub fn is_i2c1_rx_dma(&self) -> bool {
        *self == REQSEL::I2c1RxDma
    }
    ///i2c1_tx_dma selected
    #[inline(always)]
    pub fn is_i2c1_tx_dma(&self) -> bool {
        *self == REQSEL::I2c1TxDma
    }
    ///i2c2_rx_dma selected
    #[inline(always)]
    pub fn is_i2c2_rx_dma(&self) -> bool {
        *self == REQSEL::I2c2RxDma
    }
    ///i2c2_tx_dma selected
    #[inline(always)]
    pub fn is_i2c2_tx_dma(&self) -> bool {
        *self == REQSEL::I2c2TxDma
    }
    ///i2c3_rx_dma selected
    #[inline(always)]
    pub fn is_i2c3_rx_dma(&self) -> bool {
        *self == REQSEL::I2c3RxDma
    }
    ///i2c3_tx_dma selected
    #[inline(always)]
    pub fn is_i2c3_tx_dma(&self) -> bool {
        *self == REQSEL::I2c3TxDma
    }
    ///usart1_rx_dma selected
    #[inline(always)]
    pub fn is_usart1_rx_dma(&self) -> bool {
        *self == REQSEL::Usart1RxDma
    }
    ///usart1_tx_dma selected
    #[inline(always)]
    pub fn is_usart1_tx_dma(&self) -> bool {
        *self == REQSEL::Usart1TxDma
    }
    ///usart2_rx_dma selected
    #[inline(always)]
    pub fn is_usart2_rx_dma(&self) -> bool {
        *self == REQSEL::Usart2RxDma
    }
    ///usart2_tx_dma selected
    #[inline(always)]
    pub fn is_usart2_tx_dma(&self) -> bool {
        *self == REQSEL::Usart2TxDma
    }
    ///usart3_rx_dma selected
    #[inline(always)]
    pub fn is_usart3_rx_dma(&self) -> bool {
        *self == REQSEL::Usart3RxDma
    }
    ///usart3_tx_dma selected
    #[inline(always)]
    pub fn is_usart3_tx_dma(&self) -> bool {
        *self == REQSEL::Usart3TxDma
    }
    ///uart4_rx_dma selected
    #[inline(always)]
    pub fn is_uart4_rx_dma(&self) -> bool {
        *self == REQSEL::Uart4RxDma
    }
    ///uart4_tx_dma selected
    #[inline(always)]
    pub fn is_uart4_tx_dma(&self) -> bool {
        *self == REQSEL::Uart4TxDma
    }
    ///uart5_rx_dma selected
    #[inline(always)]
    pub fn is_uart5_rx_dma(&self) -> bool {
        *self == REQSEL::Uart5RxDma
    }
    ///uart5_tx_dma selected
    #[inline(always)]
    pub fn is_uart5_tx_dma(&self) -> bool {
        *self == REQSEL::Uart5TxDma
    }
    ///usart6_rx_dma selected
    #[inline(always)]
    pub fn is_usart6_rx_dma(&self) -> bool {
        *self == REQSEL::Usart6RxDma
    }
    ///usart6_tx_dma selected
    #[inline(always)]
    pub fn is_usart6_tx_dma(&self) -> bool {
        *self == REQSEL::Usart6TxDma
    }
    ///uart7_rx_dma selected
    #[inline(always)]
    pub fn is_uart7_rx_dma(&self) -> bool {
        *self == REQSEL::Uart7RxDma
    }
    ///uart7_tx_dma selected
    #[inline(always)]
    pub fn is_uart7_tx_dma(&self) -> bool {
        *self == REQSEL::Uart7TxDma
    }
    ///uart8_rx_dma selected
    #[inline(always)]
    pub fn is_uart8_rx_dma(&self) -> bool {
        *self == REQSEL::Uart8RxDma
    }
    ///uart8_tx_dma selected
    #[inline(always)]
    pub fn is_uart8_tx_dma(&self) -> bool {
        *self == REQSEL::Uart8TxDma
    }
    ///uart9_rx_dma selected
    #[inline(always)]
    pub fn is_uart9_rx_dma(&self) -> bool {
        *self == REQSEL::Uart9RxDma
    }
    ///uart9_tx_dma selected
    #[inline(always)]
    pub fn is_uart9_tx_dma(&self) -> bool {
        *self == REQSEL::Uart9TxDma
    }
    ///uart10_rx_dma selected
    #[inline(always)]
    pub fn is_uart10_rx_dma(&self) -> bool {
        *self == REQSEL::Uart10RxDma
    }
    ///uart10_tx_dma selected
    #[inline(always)]
    pub fn is_uart10_tx_dma(&self) -> bool {
        *self == REQSEL::Uart10TxDma
    }
    ///uart11_rx_dma selected
    #[inline(always)]
    pub fn is_uart11_rx_dma(&self) -> bool {
        *self == REQSEL::Uart11RxDma
    }
    ///uart11_tx_dma selected
    #[inline(always)]
    pub fn is_uart11_tx_dma(&self) -> bool {
        *self == REQSEL::Uart11TxDma
    }
    ///uart12_rx_dma selected
    #[inline(always)]
    pub fn is_uart12_rx_dma(&self) -> bool {
        *self == REQSEL::Uart12RxDma
    }
    ///uart12_tx_dma selected
    #[inline(always)]
    pub fn is_uart12_tx_dma(&self) -> bool {
        *self == REQSEL::Uart12TxDma
    }
    ///lpuart1_rx_dma selected
    #[inline(always)]
    pub fn is_lpuart1_rx_dma(&self) -> bool {
        *self == REQSEL::Lpuart1RxDma
    }
    ///lpuart1_tx_dma selected
    #[inline(always)]
    pub fn is_lpuart1_tx_dma(&self) -> bool {
        *self == REQSEL::Lpuart1TxDma
    }
    ///spi4_rx_dma selected
    #[inline(always)]
    pub fn is_spi4_rx_dma(&self) -> bool {
        *self == REQSEL::Spi4RxDma
    }
    ///spi4_tx_dma selected
    #[inline(always)]
    pub fn is_spi4_tx_dma(&self) -> bool {
        *self == REQSEL::Spi4TxDma
    }
    ///spi5_rx_dma selected
    #[inline(always)]
    pub fn is_spi5_rx_dma(&self) -> bool {
        *self == REQSEL::Spi5RxDma
    }
    ///spi5_tx_dma selected
    #[inline(always)]
    pub fn is_spi5_tx_dma(&self) -> bool {
        *self == REQSEL::Spi5TxDma
    }
    ///spi6_rx_dma selected
    #[inline(always)]
    pub fn is_spi6_rx_dma(&self) -> bool {
        *self == REQSEL::Spi6RxDma
    }
    ///spi6_tx_dma selected
    #[inline(always)]
    pub fn is_spi6_tx_dma(&self) -> bool {
        *self == REQSEL::Spi6TxDma
    }
    ///sai1_a_dma selected
    #[inline(always)]
    pub fn is_sai1_a_dma(&self) -> bool {
        *self == REQSEL::Sai1ADma
    }
    ///sai1_b_dma selected
    #[inline(always)]
    pub fn is_sai1_b_dma(&self) -> bool {
        *self == REQSEL::Sai1BDma
    }
    ///sai2_a_dma selected
    #[inline(always)]
    pub fn is_sai2_a_dma(&self) -> bool {
        *self == REQSEL::Sai2ADma
    }
    ///sai2_b_dma selected
    #[inline(always)]
    pub fn is_sai2_b_dma(&self) -> bool {
        *self == REQSEL::Sai2BDma
    }
    ///ospi1_dma selected
    #[inline(always)]
    pub fn is_ospi1_dma(&self) -> bool {
        *self == REQSEL::Ospi1Dma
    }
    ///tim1_cc1_dma selected
    #[inline(always)]
    pub fn is_tim1_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim1Cc1Dma
    }
    ///tim1_cc2_dma selected
    #[inline(always)]
    pub fn is_tim1_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim1Cc2Dma
    }
    ///tim1_cc3_dma selected
    #[inline(always)]
    pub fn is_tim1_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim1Cc3Dma
    }
    ///tim1_cc4_dma selected
    #[inline(always)]
    pub fn is_tim1_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim1Cc4Dma
    }
    ///tim1_upd_dma selected
    #[inline(always)]
    pub fn is_tim1_upd_dma(&self) -> bool {
        *self == REQSEL::Tim1UpdDma
    }
    ///tim1_trg_dma selected
    #[inline(always)]
    pub fn is_tim1_trg_dma(&self) -> bool {
        *self == REQSEL::Tim1TrgDma
    }
    ///tim1_com_dma selected
    #[inline(always)]
    pub fn is_tim1_com_dma(&self) -> bool {
        *self == REQSEL::Tim1ComDma
    }
    ///tim8_cc1_dma selected
    #[inline(always)]
    pub fn is_tim8_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim8Cc1Dma
    }
    ///tim8_cc2_dma selected
    #[inline(always)]
    pub fn is_tim8_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim8Cc2Dma
    }
    ///tim8_cc3_dma selected
    #[inline(always)]
    pub fn is_tim8_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim8Cc3Dma
    }
    ///tim8_cc4_dma selected
    #[inline(always)]
    pub fn is_tim8_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim8Cc4Dma
    }
    ///tim8_upd_dma selected
    #[inline(always)]
    pub fn is_tim8_upd_dma(&self) -> bool {
        *self == REQSEL::Tim8UpdDma
    }
    ///tim8_tig_dma selected
    #[inline(always)]
    pub fn is_tim8_tig_dma(&self) -> bool {
        *self == REQSEL::Tim8TigDma
    }
    ///tim8_com_dma selected
    #[inline(always)]
    pub fn is_tim8_com_dma(&self) -> bool {
        *self == REQSEL::Tim8ComDma
    }
    ///tim2_cc1_dma selected
    #[inline(always)]
    pub fn is_tim2_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim2Cc1Dma
    }
    ///tim2_cc2_dma selected
    #[inline(always)]
    pub fn is_tim2_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim2Cc2Dma
    }
    ///tim2_cc3_dma selected
    #[inline(always)]
    pub fn is_tim2_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim2Cc3Dma
    }
    ///tim2_cc4_dma selected
    #[inline(always)]
    pub fn is_tim2_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim2Cc4Dma
    }
    ///tim2_upd_dma selected
    #[inline(always)]
    pub fn is_tim2_upd_dma(&self) -> bool {
        *self == REQSEL::Tim2UpdDma
    }
    ///tim3_cc1_dma selected
    #[inline(always)]
    pub fn is_tim3_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim3Cc1Dma
    }
    ///tim3_cc2_dma selected
    #[inline(always)]
    pub fn is_tim3_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim3Cc2Dma
    }
    ///tim3_cc3_dma selected
    #[inline(always)]
    pub fn is_tim3_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim3Cc3Dma
    }
    ///tim3_cc4_dma selected
    #[inline(always)]
    pub fn is_tim3_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim3Cc4Dma
    }
    ///tim3_upd_dma selected
    #[inline(always)]
    pub fn is_tim3_upd_dma(&self) -> bool {
        *self == REQSEL::Tim3UpdDma
    }
    ///tim3_trg_dma selected
    #[inline(always)]
    pub fn is_tim3_trg_dma(&self) -> bool {
        *self == REQSEL::Tim3TrgDma
    }
    ///tim4_cc1_dma selected
    #[inline(always)]
    pub fn is_tim4_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim4Cc1Dma
    }
    ///tim4_cc2_dma selected
    #[inline(always)]
    pub fn is_tim4_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim4Cc2Dma
    }
    ///tim4_cc3_dma selected
    #[inline(always)]
    pub fn is_tim4_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim4Cc3Dma
    }
    ///tim4_cc4_dma selected
    #[inline(always)]
    pub fn is_tim4_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim4Cc4Dma
    }
    ///tim4_upd_dma selected
    #[inline(always)]
    pub fn is_tim4_upd_dma(&self) -> bool {
        *self == REQSEL::Tim4UpdDma
    }
    ///tim5_cc1_dma selected
    #[inline(always)]
    pub fn is_tim5_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim5Cc1Dma
    }
    ///tim5_cc2_dma selected
    #[inline(always)]
    pub fn is_tim5_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim5Cc2Dma
    }
    ///tim5_cc3_dma selected
    #[inline(always)]
    pub fn is_tim5_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim5Cc3Dma
    }
    ///tim5_cc4_dma selected
    #[inline(always)]
    pub fn is_tim5_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim5Cc4Dma
    }
    ///tim5_upd_dma selected
    #[inline(always)]
    pub fn is_tim5_upd_dma(&self) -> bool {
        *self == REQSEL::Tim5UpdDma
    }
    ///tim5_trg_dma selected
    #[inline(always)]
    pub fn is_tim5_trg_dma(&self) -> bool {
        *self == REQSEL::Tim5TrgDma
    }
    ///tim15_cc1_dma selected
    #[inline(always)]
    pub fn is_tim15_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim15Cc1Dma
    }
    ///tim15_upd_dma selected
    #[inline(always)]
    pub fn is_tim15_upd_dma(&self) -> bool {
        *self == REQSEL::Tim15UpdDma
    }
    ///tim15_trg_dma selected
    #[inline(always)]
    pub fn is_tim15_trg_dma(&self) -> bool {
        *self == REQSEL::Tim15TrgDma
    }
    ///tim15_com_dma selected
    #[inline(always)]
    pub fn is_tim15_com_dma(&self) -> bool {
        *self == REQSEL::Tim15ComDma
    }
    ///tim16_cc1_dma selected
    #[inline(always)]
    pub fn is_tim16_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim16Cc1Dma
    }
    ///tim16_upd_dma selected
    #[inline(always)]
    pub fn is_tim16_upd_dma(&self) -> bool {
        *self == REQSEL::Tim16UpdDma
    }
    ///tim17_cc1_dma selected
    #[inline(always)]
    pub fn is_tim17_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim17Cc1Dma
    }
    ///tim17_upd_dma selected
    #[inline(always)]
    pub fn is_tim17_upd_dma(&self) -> bool {
        *self == REQSEL::Tim17UpdDma
    }
    ///lptim1_ic1_dma selected
    #[inline(always)]
    pub fn is_lptim1_ic1_dma(&self) -> bool {
        *self == REQSEL::Lptim1Ic1Dma
    }
    ///lptim1_ic2_dma selected
    #[inline(always)]
    pub fn is_lptim1_ic2_dma(&self) -> bool {
        *self == REQSEL::Lptim1Ic2Dma
    }
    ///lptim1_ue_dma selected
    #[inline(always)]
    pub fn is_lptim1_ue_dma(&self) -> bool {
        *self == REQSEL::Lptim1UeDma
    }
    ///lptim2_ic1_dma selected
    #[inline(always)]
    pub fn is_lptim2_ic1_dma(&self) -> bool {
        *self == REQSEL::Lptim2Ic1Dma
    }
    ///lptim2_ic2_dma selected
    #[inline(always)]
    pub fn is_lptim2_ic2_dma(&self) -> bool {
        *self == REQSEL::Lptim2Ic2Dma
    }
    ///lptim2_ue_dma selected
    #[inline(always)]
    pub fn is_lptim2_ue_dma(&self) -> bool {
        *self == REQSEL::Lptim2UeDma
    }
    ///dcmi_dma or pssi_dma(1) selected
    #[inline(always)]
    pub fn is_dcmi_pssi_dma(&self) -> bool {
        *self == REQSEL::DcmiPssiDma
    }
    ///aes_out_dma selected
    #[inline(always)]
    pub fn is_aes_out_dma(&self) -> bool {
        *self == REQSEL::AesOutDma
    }
    ///aes_in_dma selected
    #[inline(always)]
    pub fn is_aes_in_dma(&self) -> bool {
        *self == REQSEL::AesInDma
    }
    ///hash_in_dma selected
    #[inline(always)]
    pub fn is_hash_in_dma(&self) -> bool {
        *self == REQSEL::HashInDma
    }
    ///ucpd1_rx_dma selected
    #[inline(always)]
    pub fn is_ucpd1_rx_dma(&self) -> bool {
        *self == REQSEL::Ucpd1RxDma
    }
    ///ucpd1_tx_dma selected
    #[inline(always)]
    pub fn is_ucpd1_tx_dma(&self) -> bool {
        *self == REQSEL::Ucpd1TxDma
    }
    ///cordic_read_dma selected
    #[inline(always)]
    pub fn is_cordic_read_dma(&self) -> bool {
        *self == REQSEL::CordicReadDma
    }
    ///cordic_write_dma selected
    #[inline(always)]
    pub fn is_cordic_write_dma(&self) -> bool {
        *self == REQSEL::CordicWriteDma
    }
    ///fmac_read_dma selected
    #[inline(always)]
    pub fn is_fmac_read_dma(&self) -> bool {
        *self == REQSEL::FmacReadDma
    }
    ///fmac_write_dma selected
    #[inline(always)]
    pub fn is_fmac_write_dma(&self) -> bool {
        *self == REQSEL::FmacWriteDma
    }
    ///saes_out_dma selected
    #[inline(always)]
    pub fn is_saes_out_dma(&self) -> bool {
        *self == REQSEL::SaesOutDma
    }
    ///saes_in_dma selected
    #[inline(always)]
    pub fn is_saes_in_dma(&self) -> bool {
        *self == REQSEL::SaesInDma
    }
    ///i3c1_rx_dma selected
    #[inline(always)]
    pub fn is_i3c1_rx_dma(&self) -> bool {
        *self == REQSEL::I3c1RxDma
    }
    ///i3c1_tx_dma selected
    #[inline(always)]
    pub fn is_i3c1_tx_dma(&self) -> bool {
        *self == REQSEL::I3c1TxDma
    }
    ///i3c1_tc_dma selected
    #[inline(always)]
    pub fn is_i3c1_tc_dma(&self) -> bool {
        *self == REQSEL::I3c1TcDma
    }
    ///i3c1_rs_dma selected
    #[inline(always)]
    pub fn is_i3c1_rs_dma(&self) -> bool {
        *self == REQSEL::I3c1RsDma
    }
    ///i2c4_rx_dma selected
    #[inline(always)]
    pub fn is_i2c4_rx_dma(&self) -> bool {
        *self == REQSEL::I2c4RxDma
    }
    ///i2c4_tx_dma selected
    #[inline(always)]
    pub fn is_i2c4_tx_dma(&self) -> bool {
        *self == REQSEL::I2c4TxDma
    }
    ///lptim3_ic1_dma selected
    #[inline(always)]
    pub fn is_lptim3_ic1_dma(&self) -> bool {
        *self == REQSEL::Lptim3Ic1Dma
    }
    ///lptim3_ic2_dma selected
    #[inline(always)]
    pub fn is_lptim3_ic2_dma(&self) -> bool {
        *self == REQSEL::Lptim3Ic2Dma
    }
    ///lptim3_ue_dma selected
    #[inline(always)]
    pub fn is_lptim3_ue_dma(&self) -> bool {
        *self == REQSEL::Lptim3UeDma
    }
    ///lptim5_ic1_dma selected
    #[inline(always)]
    pub fn is_lptim5_ic1_dma(&self) -> bool {
        *self == REQSEL::Lptim5Ic1Dma
    }
    ///lptim5_ic2_dma selected
    #[inline(always)]
    pub fn is_lptim5_ic2_dma(&self) -> bool {
        *self == REQSEL::Lptim5Ic2Dma
    }
    ///lptim5_ue_dma selected
    #[inline(always)]
    pub fn is_lptim5_ue_dma(&self) -> bool {
        *self == REQSEL::Lptim5UeDma
    }
    ///lptim6_ic1_dma selected
    #[inline(always)]
    pub fn is_lptim6_ic1_dma(&self) -> bool {
        *self == REQSEL::Lptim6Ic1Dma
    }
    ///lptim6_ic2_dma selected
    #[inline(always)]
    pub fn is_lptim6_ic2_dma(&self) -> bool {
        *self == REQSEL::Lptim6Ic2Dma
    }
    ///lptim6_ue_dma selected
    #[inline(always)]
    pub fn is_lptim6_ue_dma(&self) -> bool {
        *self == REQSEL::Lptim6UeDma
    }
    ///i3c2_rx selected
    #[inline(always)]
    pub fn is_i3c2_rx(&self) -> bool {
        *self == REQSEL::I3c2Rx
    }
    ///i3c2_tx selected
    #[inline(always)]
    pub fn is_i3c2_tx(&self) -> bool {
        *self == REQSEL::I3c2Tx
    }
    ///i3c2_tc selected
    #[inline(always)]
    pub fn is_i3c2_tc(&self) -> bool {
        *self == REQSEL::I3c2Tc
    }
    ///i3c2_rs selected
    #[inline(always)]
    pub fn is_i3c2_rs(&self) -> bool {
        *self == REQSEL::I3c2Rs
    }
}
///Field `REQSEL` writer - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\[7:0\] value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, REQSEL>;
impl<'a, REG> REQSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///adc1_dma selected
    #[inline(always)]
    pub fn adc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Adc1Dma)
    }
    ///dac1_ch1_dma selected
    #[inline(always)]
    pub fn dac1_ch1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Dac1Ch1Dma)
    }
    ///dac1_ch2_dma selected
    #[inline(always)]
    pub fn dac1_ch2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Dac1Ch2Dma)
    }
    ///tim6_upd_dma selected
    #[inline(always)]
    pub fn tim6_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim6UpdDma)
    }
    ///tim7_upd_dma selected
    #[inline(always)]
    pub fn tim7_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim7UpdDma)
    }
    ///spi1_rx_dma selected
    #[inline(always)]
    pub fn spi1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi1RxDma)
    }
    ///spi1_tx_dma selected
    #[inline(always)]
    pub fn spi1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi1TxDma)
    }
    ///spi2_rx_dma selected
    #[inline(always)]
    pub fn spi2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi2RxDma)
    }
    ///spi2_tx_dma selected
    #[inline(always)]
    pub fn spi2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi2TxDma)
    }
    ///spi3_rx_dma selected
    #[inline(always)]
    pub fn spi3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi3RxDma)
    }
    ///spi3_tx_dma selected
    #[inline(always)]
    pub fn spi3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi3TxDma)
    }
    ///i2c1_rx_dma selected
    #[inline(always)]
    pub fn i2c1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c1RxDma)
    }
    ///i2c1_tx_dma selected
    #[inline(always)]
    pub fn i2c1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c1TxDma)
    }
    ///i2c2_rx_dma selected
    #[inline(always)]
    pub fn i2c2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c2RxDma)
    }
    ///i2c2_tx_dma selected
    #[inline(always)]
    pub fn i2c2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c2TxDma)
    }
    ///i2c3_rx_dma selected
    #[inline(always)]
    pub fn i2c3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c3RxDma)
    }
    ///i2c3_tx_dma selected
    #[inline(always)]
    pub fn i2c3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c3TxDma)
    }
    ///usart1_rx_dma selected
    #[inline(always)]
    pub fn usart1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart1RxDma)
    }
    ///usart1_tx_dma selected
    #[inline(always)]
    pub fn usart1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart1TxDma)
    }
    ///usart2_rx_dma selected
    #[inline(always)]
    pub fn usart2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart2RxDma)
    }
    ///usart2_tx_dma selected
    #[inline(always)]
    pub fn usart2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart2TxDma)
    }
    ///usart3_rx_dma selected
    #[inline(always)]
    pub fn usart3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart3RxDma)
    }
    ///usart3_tx_dma selected
    #[inline(always)]
    pub fn usart3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart3TxDma)
    }
    ///uart4_rx_dma selected
    #[inline(always)]
    pub fn uart4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart4RxDma)
    }
    ///uart4_tx_dma selected
    #[inline(always)]
    pub fn uart4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart4TxDma)
    }
    ///uart5_rx_dma selected
    #[inline(always)]
    pub fn uart5_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart5RxDma)
    }
    ///uart5_tx_dma selected
    #[inline(always)]
    pub fn uart5_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart5TxDma)
    }
    ///usart6_rx_dma selected
    #[inline(always)]
    pub fn usart6_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart6RxDma)
    }
    ///usart6_tx_dma selected
    #[inline(always)]
    pub fn usart6_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart6TxDma)
    }
    ///uart7_rx_dma selected
    #[inline(always)]
    pub fn uart7_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart7RxDma)
    }
    ///uart7_tx_dma selected
    #[inline(always)]
    pub fn uart7_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart7TxDma)
    }
    ///uart8_rx_dma selected
    #[inline(always)]
    pub fn uart8_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart8RxDma)
    }
    ///uart8_tx_dma selected
    #[inline(always)]
    pub fn uart8_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart8TxDma)
    }
    ///uart9_rx_dma selected
    #[inline(always)]
    pub fn uart9_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart9RxDma)
    }
    ///uart9_tx_dma selected
    #[inline(always)]
    pub fn uart9_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart9TxDma)
    }
    ///uart10_rx_dma selected
    #[inline(always)]
    pub fn uart10_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart10RxDma)
    }
    ///uart10_tx_dma selected
    #[inline(always)]
    pub fn uart10_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart10TxDma)
    }
    ///uart11_rx_dma selected
    #[inline(always)]
    pub fn uart11_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart11RxDma)
    }
    ///uart11_tx_dma selected
    #[inline(always)]
    pub fn uart11_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart11TxDma)
    }
    ///uart12_rx_dma selected
    #[inline(always)]
    pub fn uart12_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart12RxDma)
    }
    ///uart12_tx_dma selected
    #[inline(always)]
    pub fn uart12_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart12TxDma)
    }
    ///lpuart1_rx_dma selected
    #[inline(always)]
    pub fn lpuart1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lpuart1RxDma)
    }
    ///lpuart1_tx_dma selected
    #[inline(always)]
    pub fn lpuart1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lpuart1TxDma)
    }
    ///spi4_rx_dma selected
    #[inline(always)]
    pub fn spi4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi4RxDma)
    }
    ///spi4_tx_dma selected
    #[inline(always)]
    pub fn spi4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi4TxDma)
    }
    ///spi5_rx_dma selected
    #[inline(always)]
    pub fn spi5_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi5RxDma)
    }
    ///spi5_tx_dma selected
    #[inline(always)]
    pub fn spi5_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi5TxDma)
    }
    ///spi6_rx_dma selected
    #[inline(always)]
    pub fn spi6_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi6RxDma)
    }
    ///spi6_tx_dma selected
    #[inline(always)]
    pub fn spi6_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi6TxDma)
    }
    ///sai1_a_dma selected
    #[inline(always)]
    pub fn sai1_a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Sai1ADma)
    }
    ///sai1_b_dma selected
    #[inline(always)]
    pub fn sai1_b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Sai1BDma)
    }
    ///sai2_a_dma selected
    #[inline(always)]
    pub fn sai2_a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Sai2ADma)
    }
    ///sai2_b_dma selected
    #[inline(always)]
    pub fn sai2_b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Sai2BDma)
    }
    ///ospi1_dma selected
    #[inline(always)]
    pub fn ospi1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Ospi1Dma)
    }
    ///tim1_cc1_dma selected
    #[inline(always)]
    pub fn tim1_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1Cc1Dma)
    }
    ///tim1_cc2_dma selected
    #[inline(always)]
    pub fn tim1_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1Cc2Dma)
    }
    ///tim1_cc3_dma selected
    #[inline(always)]
    pub fn tim1_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1Cc3Dma)
    }
    ///tim1_cc4_dma selected
    #[inline(always)]
    pub fn tim1_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1Cc4Dma)
    }
    ///tim1_upd_dma selected
    #[inline(always)]
    pub fn tim1_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1UpdDma)
    }
    ///tim1_trg_dma selected
    #[inline(always)]
    pub fn tim1_trg_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1TrgDma)
    }
    ///tim1_com_dma selected
    #[inline(always)]
    pub fn tim1_com_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1ComDma)
    }
    ///tim8_cc1_dma selected
    #[inline(always)]
    pub fn tim8_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8Cc1Dma)
    }
    ///tim8_cc2_dma selected
    #[inline(always)]
    pub fn tim8_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8Cc2Dma)
    }
    ///tim8_cc3_dma selected
    #[inline(always)]
    pub fn tim8_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8Cc3Dma)
    }
    ///tim8_cc4_dma selected
    #[inline(always)]
    pub fn tim8_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8Cc4Dma)
    }
    ///tim8_upd_dma selected
    #[inline(always)]
    pub fn tim8_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8UpdDma)
    }
    ///tim8_tig_dma selected
    #[inline(always)]
    pub fn tim8_tig_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8TigDma)
    }
    ///tim8_com_dma selected
    #[inline(always)]
    pub fn tim8_com_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8ComDma)
    }
    ///tim2_cc1_dma selected
    #[inline(always)]
    pub fn tim2_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim2Cc1Dma)
    }
    ///tim2_cc2_dma selected
    #[inline(always)]
    pub fn tim2_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim2Cc2Dma)
    }
    ///tim2_cc3_dma selected
    #[inline(always)]
    pub fn tim2_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim2Cc3Dma)
    }
    ///tim2_cc4_dma selected
    #[inline(always)]
    pub fn tim2_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim2Cc4Dma)
    }
    ///tim2_upd_dma selected
    #[inline(always)]
    pub fn tim2_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim2UpdDma)
    }
    ///tim3_cc1_dma selected
    #[inline(always)]
    pub fn tim3_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3Cc1Dma)
    }
    ///tim3_cc2_dma selected
    #[inline(always)]
    pub fn tim3_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3Cc2Dma)
    }
    ///tim3_cc3_dma selected
    #[inline(always)]
    pub fn tim3_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3Cc3Dma)
    }
    ///tim3_cc4_dma selected
    #[inline(always)]
    pub fn tim3_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3Cc4Dma)
    }
    ///tim3_upd_dma selected
    #[inline(always)]
    pub fn tim3_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3UpdDma)
    }
    ///tim3_trg_dma selected
    #[inline(always)]
    pub fn tim3_trg_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3TrgDma)
    }
    ///tim4_cc1_dma selected
    #[inline(always)]
    pub fn tim4_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim4Cc1Dma)
    }
    ///tim4_cc2_dma selected
    #[inline(always)]
    pub fn tim4_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim4Cc2Dma)
    }
    ///tim4_cc3_dma selected
    #[inline(always)]
    pub fn tim4_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim4Cc3Dma)
    }
    ///tim4_cc4_dma selected
    #[inline(always)]
    pub fn tim4_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim4Cc4Dma)
    }
    ///tim4_upd_dma selected
    #[inline(always)]
    pub fn tim4_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim4UpdDma)
    }
    ///tim5_cc1_dma selected
    #[inline(always)]
    pub fn tim5_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5Cc1Dma)
    }
    ///tim5_cc2_dma selected
    #[inline(always)]
    pub fn tim5_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5Cc2Dma)
    }
    ///tim5_cc3_dma selected
    #[inline(always)]
    pub fn tim5_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5Cc3Dma)
    }
    ///tim5_cc4_dma selected
    #[inline(always)]
    pub fn tim5_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5Cc4Dma)
    }
    ///tim5_upd_dma selected
    #[inline(always)]
    pub fn tim5_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5UpdDma)
    }
    ///tim5_trg_dma selected
    #[inline(always)]
    pub fn tim5_trg_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5TrgDma)
    }
    ///tim15_cc1_dma selected
    #[inline(always)]
    pub fn tim15_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim15Cc1Dma)
    }
    ///tim15_upd_dma selected
    #[inline(always)]
    pub fn tim15_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim15UpdDma)
    }
    ///tim15_trg_dma selected
    #[inline(always)]
    pub fn tim15_trg_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim15TrgDma)
    }
    ///tim15_com_dma selected
    #[inline(always)]
    pub fn tim15_com_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim15ComDma)
    }
    ///tim16_cc1_dma selected
    #[inline(always)]
    pub fn tim16_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim16Cc1Dma)
    }
    ///tim16_upd_dma selected
    #[inline(always)]
    pub fn tim16_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim16UpdDma)
    }
    ///tim17_cc1_dma selected
    #[inline(always)]
    pub fn tim17_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim17Cc1Dma)
    }
    ///tim17_upd_dma selected
    #[inline(always)]
    pub fn tim17_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim17UpdDma)
    }
    ///lptim1_ic1_dma selected
    #[inline(always)]
    pub fn lptim1_ic1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim1Ic1Dma)
    }
    ///lptim1_ic2_dma selected
    #[inline(always)]
    pub fn lptim1_ic2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim1Ic2Dma)
    }
    ///lptim1_ue_dma selected
    #[inline(always)]
    pub fn lptim1_ue_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim1UeDma)
    }
    ///lptim2_ic1_dma selected
    #[inline(always)]
    pub fn lptim2_ic1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim2Ic1Dma)
    }
    ///lptim2_ic2_dma selected
    #[inline(always)]
    pub fn lptim2_ic2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim2Ic2Dma)
    }
    ///lptim2_ue_dma selected
    #[inline(always)]
    pub fn lptim2_ue_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim2UeDma)
    }
    ///dcmi_dma or pssi_dma(1) selected
    #[inline(always)]
    pub fn dcmi_pssi_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::DcmiPssiDma)
    }
    ///aes_out_dma selected
    #[inline(always)]
    pub fn aes_out_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::AesOutDma)
    }
    ///aes_in_dma selected
    #[inline(always)]
    pub fn aes_in_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::AesInDma)
    }
    ///hash_in_dma selected
    #[inline(always)]
    pub fn hash_in_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::HashInDma)
    }
    ///ucpd1_rx_dma selected
    #[inline(always)]
    pub fn ucpd1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Ucpd1RxDma)
    }
    ///ucpd1_tx_dma selected
    #[inline(always)]
    pub fn ucpd1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Ucpd1TxDma)
    }
    ///cordic_read_dma selected
    #[inline(always)]
    pub fn cordic_read_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::CordicReadDma)
    }
    ///cordic_write_dma selected
    #[inline(always)]
    pub fn cordic_write_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::CordicWriteDma)
    }
    ///fmac_read_dma selected
    #[inline(always)]
    pub fn fmac_read_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::FmacReadDma)
    }
    ///fmac_write_dma selected
    #[inline(always)]
    pub fn fmac_write_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::FmacWriteDma)
    }
    ///saes_out_dma selected
    #[inline(always)]
    pub fn saes_out_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::SaesOutDma)
    }
    ///saes_in_dma selected
    #[inline(always)]
    pub fn saes_in_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::SaesInDma)
    }
    ///i3c1_rx_dma selected
    #[inline(always)]
    pub fn i3c1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c1RxDma)
    }
    ///i3c1_tx_dma selected
    #[inline(always)]
    pub fn i3c1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c1TxDma)
    }
    ///i3c1_tc_dma selected
    #[inline(always)]
    pub fn i3c1_tc_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c1TcDma)
    }
    ///i3c1_rs_dma selected
    #[inline(always)]
    pub fn i3c1_rs_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c1RsDma)
    }
    ///i2c4_rx_dma selected
    #[inline(always)]
    pub fn i2c4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c4RxDma)
    }
    ///i2c4_tx_dma selected
    #[inline(always)]
    pub fn i2c4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c4TxDma)
    }
    ///lptim3_ic1_dma selected
    #[inline(always)]
    pub fn lptim3_ic1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim3Ic1Dma)
    }
    ///lptim3_ic2_dma selected
    #[inline(always)]
    pub fn lptim3_ic2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim3Ic2Dma)
    }
    ///lptim3_ue_dma selected
    #[inline(always)]
    pub fn lptim3_ue_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim3UeDma)
    }
    ///lptim5_ic1_dma selected
    #[inline(always)]
    pub fn lptim5_ic1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim5Ic1Dma)
    }
    ///lptim5_ic2_dma selected
    #[inline(always)]
    pub fn lptim5_ic2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim5Ic2Dma)
    }
    ///lptim5_ue_dma selected
    #[inline(always)]
    pub fn lptim5_ue_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim5UeDma)
    }
    ///lptim6_ic1_dma selected
    #[inline(always)]
    pub fn lptim6_ic1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim6Ic1Dma)
    }
    ///lptim6_ic2_dma selected
    #[inline(always)]
    pub fn lptim6_ic2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim6Ic2Dma)
    }
    ///lptim6_ue_dma selected
    #[inline(always)]
    pub fn lptim6_ue_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim6UeDma)
    }
    ///i3c2_rx selected
    #[inline(always)]
    pub fn i3c2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c2Rx)
    }
    ///i3c2_tx selected
    #[inline(always)]
    pub fn i3c2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c2Tx)
    }
    ///i3c2_tc selected
    #[inline(always)]
    pub fn i3c2_tc(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c2Tc)
    }
    ///i3c2_rs selected
    #[inline(always)]
    pub fn i3c2_rs(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c2Rs)
    }
}
/**software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWREQ {
    ///0: No software request. The selected hardware request REQSEL\[7:0\] is taken into account
    Hardware = 0,
    ///1: Software request for memory-to-memory transfer
    Software = 1,
}
impl From<SWREQ> for bool {
    #[inline(always)]
    fn from(variant: SWREQ) -> Self {
        variant as u8 != 0
    }
}
///Field `SWREQ` reader - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted.
pub type SWREQ_R = crate::BitReader<SWREQ>;
impl SWREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWREQ {
        match self.bits {
            false => SWREQ::Hardware,
            true => SWREQ::Software,
        }
    }
    ///No software request. The selected hardware request REQSEL\[7:0\] is taken into account
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == SWREQ::Hardware
    }
    ///Software request for memory-to-memory transfer
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == SWREQ::Software
    }
}
///Field `SWREQ` writer - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted.
pub type SWREQ_W<'a, REG> = crate::BitWriter<'a, REG, SWREQ>;
impl<'a, REG> SWREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No software request. The selected hardware request REQSEL\[7:0\] is taken into account
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQ::Hardware)
    }
    ///Software request for memory-to-memory transfer
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQ::Software)
    }
}
/**destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DREQ {
    ///0: Selected hardware request driven by a source peripheral
    Source = 0,
    ///1: Selected hardware request driven by a destination peripheral
    Destination = 1,
}
impl From<DREQ> for bool {
    #[inline(always)]
    fn from(variant: DREQ) -> Self {
        variant as u8 != 0
    }
}
///Field `DREQ` reader - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.
pub type DREQ_R = crate::BitReader<DREQ>;
impl DREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DREQ {
        match self.bits {
            false => DREQ::Source,
            true => DREQ::Destination,
        }
    }
    ///Selected hardware request driven by a source peripheral
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == DREQ::Source
    }
    ///Selected hardware request driven by a destination peripheral
    #[inline(always)]
    pub fn is_destination(&self) -> bool {
        *self == DREQ::Destination
    }
}
///Field `DREQ` writer - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.
pub type DREQ_W<'a, REG> = crate::BitWriter<'a, REG, DREQ>;
impl<'a, REG> DREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Selected hardware request driven by a source peripheral
    #[inline(always)]
    pub fn source(self) -> &'a mut crate::W<REG> {
        self.variant(DREQ::Source)
    }
    ///Selected hardware request driven by a destination peripheral
    #[inline(always)]
    pub fn destination(self) -> &'a mut crate::W<REG> {
        self.variant(DREQ::Destination)
    }
}
/**Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREQ {
    ///0: The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level
    Burst = 0,
    ///1: The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level
    Block = 1,
}
impl From<BREQ> for bool {
    #[inline(always)]
    fn from(variant: BREQ) -> Self {
        variant as u8 != 0
    }
}
///Field `BREQ` reader - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
pub type BREQ_R = crate::BitReader<BREQ>;
impl BREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BREQ {
        match self.bits {
            false => BREQ::Burst,
            true => BREQ::Block,
        }
    }
    ///The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == BREQ::Burst
    }
    ///The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == BREQ::Block
    }
}
///Field `BREQ` writer - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
pub type BREQ_W<'a, REG> = crate::BitWriter<'a, REG, BREQ>;
impl<'a, REG> BREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(BREQ::Burst)
    }
    ///The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(BREQ::Block)
    }
}
/**Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: - no 2D/repeated block support (GPDMA_CxBR1.BRC\[10:0\] must be set to 0 if present) - the peripheral must be set as the source of the transfer (DREQ = 0). - data packing to a wider destination width is not supported (if destination width source data width, GPDMA_CxTR1.PAM\[1\] must be set to 0). - GPDMA_CxBR1.BNDT\[15:0\] must be programmed as a multiple of the source (peripheral) burst size.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFREQ {
    ///0: The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in GPDMA control mode
    GpdmaControlMode = 0,
    ///1: The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in peripheral control mode.
    PeripheralControlMode = 1,
}
impl From<PFREQ> for bool {
    #[inline(always)]
    fn from(variant: PFREQ) -> Self {
        variant as u8 != 0
    }
}
///Field `PFREQ` reader - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: - no 2D/repeated block support (GPDMA_CxBR1.BRC\[10:0\] must be set to 0 if present) - the peripheral must be set as the source of the transfer (DREQ = 0). - data packing to a wider destination width is not supported (if destination width source data width, GPDMA_CxTR1.PAM\[1\] must be set to 0). - GPDMA_CxBR1.BNDT\[15:0\] must be programmed as a multiple of the source (peripheral) burst size.
pub type PFREQ_R = crate::BitReader<PFREQ>;
impl PFREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PFREQ {
        match self.bits {
            false => PFREQ::GpdmaControlMode,
            true => PFREQ::PeripheralControlMode,
        }
    }
    ///The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in GPDMA control mode
    #[inline(always)]
    pub fn is_gpdma_control_mode(&self) -> bool {
        *self == PFREQ::GpdmaControlMode
    }
    ///The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in peripheral control mode.
    #[inline(always)]
    pub fn is_peripheral_control_mode(&self) -> bool {
        *self == PFREQ::PeripheralControlMode
    }
}
///Field `PFREQ` writer - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: - no 2D/repeated block support (GPDMA_CxBR1.BRC\[10:0\] must be set to 0 if present) - the peripheral must be set as the source of the transfer (DREQ = 0). - data packing to a wider destination width is not supported (if destination width source data width, GPDMA_CxTR1.PAM\[1\] must be set to 0). - GPDMA_CxBR1.BNDT\[15:0\] must be programmed as a multiple of the source (peripheral) burst size.
pub type PFREQ_W<'a, REG> = crate::BitWriter<'a, REG, PFREQ>;
impl<'a, REG> PFREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in GPDMA control mode
    #[inline(always)]
    pub fn gpdma_control_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PFREQ::GpdmaControlMode)
    }
    ///The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in peripheral control mode.
    #[inline(always)]
    pub fn peripheral_control_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PFREQ::PeripheralControlMode)
    }
}
/**trigger mode These bits define the transfer granularity for its conditioning by the trigger..

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGM {
    ///0: At block level: the first burst read of each block transfer is conditioned by one hit trigger
    BlockLevel = 0,
    ///2: At link level: a LLI link transfer is conditioned by one hit trigger
    LinkLevel = 2,
    ///3: At programmed burst level: programmed burst read is conditioned by one hit trigger.
    ProgrammedBurstLevel = 3,
}
impl From<TRIGM> for u8 {
    #[inline(always)]
    fn from(variant: TRIGM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGM {
    type Ux = u8;
}
impl crate::IsEnum for TRIGM {}
///Field `TRIGM` reader - trigger mode These bits define the transfer granularity for its conditioning by the trigger..
pub type TRIGM_R = crate::FieldReader<TRIGM>;
impl TRIGM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRIGM> {
        match self.bits {
            0 => Some(TRIGM::BlockLevel),
            2 => Some(TRIGM::LinkLevel),
            3 => Some(TRIGM::ProgrammedBurstLevel),
            _ => None,
        }
    }
    ///At block level: the first burst read of each block transfer is conditioned by one hit trigger
    #[inline(always)]
    pub fn is_block_level(&self) -> bool {
        *self == TRIGM::BlockLevel
    }
    ///At link level: a LLI link transfer is conditioned by one hit trigger
    #[inline(always)]
    pub fn is_link_level(&self) -> bool {
        *self == TRIGM::LinkLevel
    }
    ///At programmed burst level: programmed burst read is conditioned by one hit trigger.
    #[inline(always)]
    pub fn is_programmed_burst_level(&self) -> bool {
        *self == TRIGM::ProgrammedBurstLevel
    }
}
///Field `TRIGM` writer - trigger mode These bits define the transfer granularity for its conditioning by the trigger..
pub type TRIGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRIGM>;
impl<'a, REG> TRIGM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///At block level: the first burst read of each block transfer is conditioned by one hit trigger
    #[inline(always)]
    pub fn block_level(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGM::BlockLevel)
    }
    ///At link level: a LLI link transfer is conditioned by one hit trigger
    #[inline(always)]
    pub fn link_level(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGM::LinkLevel)
    }
    ///At programmed burst level: programmed burst read is conditioned by one hit trigger.
    #[inline(always)]
    pub fn programmed_burst_level(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGM::ProgrammedBurstLevel)
    }
}
/**trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\[1:0\] different 00.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSEL {
    ///0: exti0 is trigger input
    Exti0 = 0,
    ///1: exti1 is trigger input
    Exti1 = 1,
    ///2: exti2 is trigger input
    Exti2 = 2,
    ///3: exti3 is trigger input
    Exti3 = 3,
    ///4: exti4 is trigger input
    Exti4 = 4,
    ///5: exti5 is trigger input
    Exti5 = 5,
    ///6: exti6 is trigger input
    Exti6 = 6,
    ///7: exti7 is trigger input
    Exti7 = 7,
    ///8: tamp_trg1 is trigger input
    TampTrg1 = 8,
    ///9: tamp_trg2 is trigger input
    TampTrg2 = 9,
    ///11: lptim1_ch1 is trigger input
    Lptim1Ch1 = 11,
    ///12: lptim1_ch2 is trigger input
    Lptim1Ch2 = 12,
    ///13: lptim2_ch1 is trigger input
    Lptim2Ch1 = 13,
    ///14: lptim2_ch2 is trigger input
    Lptim2Ch2 = 14,
    ///15: rtc_alra_trg is trigger input
    RtcAlraTrg = 15,
    ///16: rtc_alrb_trg is trigger input
    RtcAlrbTrg = 16,
    ///17: rtc_wut_trg is trigger input
    RtcWutTrg = 17,
    ///18: gpdma1_ch0_tc is trigger input
    Gpdma1Ch0Tc = 18,
    ///19: gpdma1_ch1_tc is trigger input
    Gpdma1Ch1Tc = 19,
    ///20: gpdma1_ch2_tc is trigger input
    Gpdma1Ch2Tc = 20,
    ///21: gpdma1_ch3_tc is trigger input
    Gpdma1Ch3Tc = 21,
    ///22: gpdma1_ch4_tc is trigger input
    Gpdma1Ch4Tc = 22,
    ///23: gpdma1_ch5_tc is trigger input
    Gpdma1Ch5Tc = 23,
    ///24: gpdma1_ch6_tc is trigger input
    Gpdma1Ch6Tc = 24,
    ///25: gpdma1_ch7_tc is trigger input
    Gpdma1Ch7Tc = 25,
    ///26: gpdma2_ch0_tc is trigger input
    Gpdma2Ch0Tc = 26,
    ///27: gpdma2_ch1_tc is trigger input
    Gpdma2Ch1Tc = 27,
    ///28: gpdma2_ch2_tc is trigger input
    Gpdma2Ch2Tc = 28,
    ///29: gpdma2_ch3_tc is trigger input
    Gpdma2Ch3Tc = 29,
    ///30: gpdma2_ch4_tc is trigger input
    Gpdma2Ch4Tc = 30,
    ///31: gpdma2_ch5_tc is trigger input
    Gpdma2Ch5Tc = 31,
    ///32: gpdma2_ch6_tc is trigger input
    Gpdma2Ch6Tc = 32,
    ///33: gpdma2_ch7_tc is trigger input
    Gpdma2Ch7Tc = 33,
    ///34: tim2_trgo is trigger input
    Tim2Trg0 = 34,
    ///44: comp1_out is trigger input
    Comp1Out = 44,
}
impl From<TRIGSEL> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGSEL {
    type Ux = u8;
}
impl crate::IsEnum for TRIGSEL {}
///Field `TRIGSEL` reader - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\[1:0\] different 00.
pub type TRIGSEL_R = crate::FieldReader<TRIGSEL>;
impl TRIGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRIGSEL> {
        match self.bits {
            0 => Some(TRIGSEL::Exti0),
            1 => Some(TRIGSEL::Exti1),
            2 => Some(TRIGSEL::Exti2),
            3 => Some(TRIGSEL::Exti3),
            4 => Some(TRIGSEL::Exti4),
            5 => Some(TRIGSEL::Exti5),
            6 => Some(TRIGSEL::Exti6),
            7 => Some(TRIGSEL::Exti7),
            8 => Some(TRIGSEL::TampTrg1),
            9 => Some(TRIGSEL::TampTrg2),
            11 => Some(TRIGSEL::Lptim1Ch1),
            12 => Some(TRIGSEL::Lptim1Ch2),
            13 => Some(TRIGSEL::Lptim2Ch1),
            14 => Some(TRIGSEL::Lptim2Ch2),
            15 => Some(TRIGSEL::RtcAlraTrg),
            16 => Some(TRIGSEL::RtcAlrbTrg),
            17 => Some(TRIGSEL::RtcWutTrg),
            18 => Some(TRIGSEL::Gpdma1Ch0Tc),
            19 => Some(TRIGSEL::Gpdma1Ch1Tc),
            20 => Some(TRIGSEL::Gpdma1Ch2Tc),
            21 => Some(TRIGSEL::Gpdma1Ch3Tc),
            22 => Some(TRIGSEL::Gpdma1Ch4Tc),
            23 => Some(TRIGSEL::Gpdma1Ch5Tc),
            24 => Some(TRIGSEL::Gpdma1Ch6Tc),
            25 => Some(TRIGSEL::Gpdma1Ch7Tc),
            26 => Some(TRIGSEL::Gpdma2Ch0Tc),
            27 => Some(TRIGSEL::Gpdma2Ch1Tc),
            28 => Some(TRIGSEL::Gpdma2Ch2Tc),
            29 => Some(TRIGSEL::Gpdma2Ch3Tc),
            30 => Some(TRIGSEL::Gpdma2Ch4Tc),
            31 => Some(TRIGSEL::Gpdma2Ch5Tc),
            32 => Some(TRIGSEL::Gpdma2Ch6Tc),
            33 => Some(TRIGSEL::Gpdma2Ch7Tc),
            34 => Some(TRIGSEL::Tim2Trg0),
            44 => Some(TRIGSEL::Comp1Out),
            _ => None,
        }
    }
    ///exti0 is trigger input
    #[inline(always)]
    pub fn is_exti0(&self) -> bool {
        *self == TRIGSEL::Exti0
    }
    ///exti1 is trigger input
    #[inline(always)]
    pub fn is_exti1(&self) -> bool {
        *self == TRIGSEL::Exti1
    }
    ///exti2 is trigger input
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == TRIGSEL::Exti2
    }
    ///exti3 is trigger input
    #[inline(always)]
    pub fn is_exti3(&self) -> bool {
        *self == TRIGSEL::Exti3
    }
    ///exti4 is trigger input
    #[inline(always)]
    pub fn is_exti4(&self) -> bool {
        *self == TRIGSEL::Exti4
    }
    ///exti5 is trigger input
    #[inline(always)]
    pub fn is_exti5(&self) -> bool {
        *self == TRIGSEL::Exti5
    }
    ///exti6 is trigger input
    #[inline(always)]
    pub fn is_exti6(&self) -> bool {
        *self == TRIGSEL::Exti6
    }
    ///exti7 is trigger input
    #[inline(always)]
    pub fn is_exti7(&self) -> bool {
        *self == TRIGSEL::Exti7
    }
    ///tamp_trg1 is trigger input
    #[inline(always)]
    pub fn is_tamp_trg1(&self) -> bool {
        *self == TRIGSEL::TampTrg1
    }
    ///tamp_trg2 is trigger input
    #[inline(always)]
    pub fn is_tamp_trg2(&self) -> bool {
        *self == TRIGSEL::TampTrg2
    }
    ///lptim1_ch1 is trigger input
    #[inline(always)]
    pub fn is_lptim1_ch1(&self) -> bool {
        *self == TRIGSEL::Lptim1Ch1
    }
    ///lptim1_ch2 is trigger input
    #[inline(always)]
    pub fn is_lptim1_ch2(&self) -> bool {
        *self == TRIGSEL::Lptim1Ch2
    }
    ///lptim2_ch1 is trigger input
    #[inline(always)]
    pub fn is_lptim2_ch1(&self) -> bool {
        *self == TRIGSEL::Lptim2Ch1
    }
    ///lptim2_ch2 is trigger input
    #[inline(always)]
    pub fn is_lptim2_ch2(&self) -> bool {
        *self == TRIGSEL::Lptim2Ch2
    }
    ///rtc_alra_trg is trigger input
    #[inline(always)]
    pub fn is_rtc_alra_trg(&self) -> bool {
        *self == TRIGSEL::RtcAlraTrg
    }
    ///rtc_alrb_trg is trigger input
    #[inline(always)]
    pub fn is_rtc_alrb_trg(&self) -> bool {
        *self == TRIGSEL::RtcAlrbTrg
    }
    ///rtc_wut_trg is trigger input
    #[inline(always)]
    pub fn is_rtc_wut_trg(&self) -> bool {
        *self == TRIGSEL::RtcWutTrg
    }
    ///gpdma1_ch0_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma1_ch0_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch0Tc
    }
    ///gpdma1_ch1_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma1_ch1_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch1Tc
    }
    ///gpdma1_ch2_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma1_ch2_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch2Tc
    }
    ///gpdma1_ch3_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma1_ch3_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch3Tc
    }
    ///gpdma1_ch4_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma1_ch4_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch4Tc
    }
    ///gpdma1_ch5_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma1_ch5_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch5Tc
    }
    ///gpdma1_ch6_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma1_ch6_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch6Tc
    }
    ///gpdma1_ch7_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma1_ch7_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch7Tc
    }
    ///gpdma2_ch0_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma2_ch0_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch0Tc
    }
    ///gpdma2_ch1_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma2_ch1_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch1Tc
    }
    ///gpdma2_ch2_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma2_ch2_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch2Tc
    }
    ///gpdma2_ch3_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma2_ch3_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch3Tc
    }
    ///gpdma2_ch4_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma2_ch4_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch4Tc
    }
    ///gpdma2_ch5_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma2_ch5_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch5Tc
    }
    ///gpdma2_ch6_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma2_ch6_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch6Tc
    }
    ///gpdma2_ch7_tc is trigger input
    #[inline(always)]
    pub fn is_gpdma2_ch7_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch7Tc
    }
    ///tim2_trgo is trigger input
    #[inline(always)]
    pub fn is_tim2_trg0(&self) -> bool {
        *self == TRIGSEL::Tim2Trg0
    }
    ///comp1_out is trigger input
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == TRIGSEL::Comp1Out
    }
}
///Field `TRIGSEL` writer - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\[1:0\] different 00.
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, TRIGSEL>;
impl<'a, REG> TRIGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///exti0 is trigger input
    #[inline(always)]
    pub fn exti0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti0)
    }
    ///exti1 is trigger input
    #[inline(always)]
    pub fn exti1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti1)
    }
    ///exti2 is trigger input
    #[inline(always)]
    pub fn exti2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti2)
    }
    ///exti3 is trigger input
    #[inline(always)]
    pub fn exti3(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti3)
    }
    ///exti4 is trigger input
    #[inline(always)]
    pub fn exti4(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti4)
    }
    ///exti5 is trigger input
    #[inline(always)]
    pub fn exti5(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti5)
    }
    ///exti6 is trigger input
    #[inline(always)]
    pub fn exti6(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti6)
    }
    ///exti7 is trigger input
    #[inline(always)]
    pub fn exti7(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti7)
    }
    ///tamp_trg1 is trigger input
    #[inline(always)]
    pub fn tamp_trg1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::TampTrg1)
    }
    ///tamp_trg2 is trigger input
    #[inline(always)]
    pub fn tamp_trg2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::TampTrg2)
    }
    ///lptim1_ch1 is trigger input
    #[inline(always)]
    pub fn lptim1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Lptim1Ch1)
    }
    ///lptim1_ch2 is trigger input
    #[inline(always)]
    pub fn lptim1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Lptim1Ch2)
    }
    ///lptim2_ch1 is trigger input
    #[inline(always)]
    pub fn lptim2_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Lptim2Ch1)
    }
    ///lptim2_ch2 is trigger input
    #[inline(always)]
    pub fn lptim2_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Lptim2Ch2)
    }
    ///rtc_alra_trg is trigger input
    #[inline(always)]
    pub fn rtc_alra_trg(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::RtcAlraTrg)
    }
    ///rtc_alrb_trg is trigger input
    #[inline(always)]
    pub fn rtc_alrb_trg(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::RtcAlrbTrg)
    }
    ///rtc_wut_trg is trigger input
    #[inline(always)]
    pub fn rtc_wut_trg(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::RtcWutTrg)
    }
    ///gpdma1_ch0_tc is trigger input
    #[inline(always)]
    pub fn gpdma1_ch0_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch0Tc)
    }
    ///gpdma1_ch1_tc is trigger input
    #[inline(always)]
    pub fn gpdma1_ch1_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch1Tc)
    }
    ///gpdma1_ch2_tc is trigger input
    #[inline(always)]
    pub fn gpdma1_ch2_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch2Tc)
    }
    ///gpdma1_ch3_tc is trigger input
    #[inline(always)]
    pub fn gpdma1_ch3_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch3Tc)
    }
    ///gpdma1_ch4_tc is trigger input
    #[inline(always)]
    pub fn gpdma1_ch4_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch4Tc)
    }
    ///gpdma1_ch5_tc is trigger input
    #[inline(always)]
    pub fn gpdma1_ch5_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch5Tc)
    }
    ///gpdma1_ch6_tc is trigger input
    #[inline(always)]
    pub fn gpdma1_ch6_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch6Tc)
    }
    ///gpdma1_ch7_tc is trigger input
    #[inline(always)]
    pub fn gpdma1_ch7_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch7Tc)
    }
    ///gpdma2_ch0_tc is trigger input
    #[inline(always)]
    pub fn gpdma2_ch0_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch0Tc)
    }
    ///gpdma2_ch1_tc is trigger input
    #[inline(always)]
    pub fn gpdma2_ch1_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch1Tc)
    }
    ///gpdma2_ch2_tc is trigger input
    #[inline(always)]
    pub fn gpdma2_ch2_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch2Tc)
    }
    ///gpdma2_ch3_tc is trigger input
    #[inline(always)]
    pub fn gpdma2_ch3_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch3Tc)
    }
    ///gpdma2_ch4_tc is trigger input
    #[inline(always)]
    pub fn gpdma2_ch4_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch4Tc)
    }
    ///gpdma2_ch5_tc is trigger input
    #[inline(always)]
    pub fn gpdma2_ch5_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch5Tc)
    }
    ///gpdma2_ch6_tc is trigger input
    #[inline(always)]
    pub fn gpdma2_ch6_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch6Tc)
    }
    ///gpdma2_ch7_tc is trigger input
    #[inline(always)]
    pub fn gpdma2_ch7_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch7Tc)
    }
    ///tim2_trgo is trigger input
    #[inline(always)]
    pub fn tim2_trg0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Tim2Trg0)
    }
    ///comp1_out is trigger input
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Comp1Out)
    }
}
/**trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[5:0\].

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGPOL {
    ///0: No trigger
    NoTrigger = 0,
    ///1: Trigger on rising edge
    RisingEdge = 1,
    ///2: Trigger on falling edge
    FallingEdge = 2,
}
impl From<TRIGPOL> for u8 {
    #[inline(always)]
    fn from(variant: TRIGPOL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGPOL {
    type Ux = u8;
}
impl crate::IsEnum for TRIGPOL {}
///Field `TRIGPOL` reader - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[5:0\].
pub type TRIGPOL_R = crate::FieldReader<TRIGPOL>;
impl TRIGPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRIGPOL> {
        match self.bits {
            0 => Some(TRIGPOL::NoTrigger),
            1 => Some(TRIGPOL::RisingEdge),
            2 => Some(TRIGPOL::FallingEdge),
            _ => None,
        }
    }
    ///No trigger
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == TRIGPOL::NoTrigger
    }
    ///Trigger on rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TRIGPOL::RisingEdge
    }
    ///Trigger on falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TRIGPOL::FallingEdge
    }
}
///Field `TRIGPOL` writer - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[5:0\].
pub type TRIGPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRIGPOL>;
impl<'a, REG> TRIGPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No trigger
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL::NoTrigger)
    }
    ///Trigger on rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL::RisingEdge)
    }
    ///Trigger on falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL::FallingEdge)
    }
}
/**transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCEM {
    ///0: At block level: the complete (and the half) transfer event is generated at the (respectively half of the) end of a block
    BlockLevel = 0,
    ///2: At LLI level: the complete transfer event is generated at the end of the LLI transfer. The half transfer event is generated at the half of the LLI data transfer
    LliLevel = 2,
    ///3: At channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI
    ChannelLevel = 3,
}
impl From<TCEM> for u8 {
    #[inline(always)]
    fn from(variant: TCEM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TCEM {
    type Ux = u8;
}
impl crate::IsEnum for TCEM {}
///Field `TCEM` reader - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
pub type TCEM_R = crate::FieldReader<TCEM>;
impl TCEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCEM> {
        match self.bits {
            0 => Some(TCEM::BlockLevel),
            2 => Some(TCEM::LliLevel),
            3 => Some(TCEM::ChannelLevel),
            _ => None,
        }
    }
    ///At block level: the complete (and the half) transfer event is generated at the (respectively half of the) end of a block
    #[inline(always)]
    pub fn is_block_level(&self) -> bool {
        *self == TCEM::BlockLevel
    }
    ///At LLI level: the complete transfer event is generated at the end of the LLI transfer. The half transfer event is generated at the half of the LLI data transfer
    #[inline(always)]
    pub fn is_lli_level(&self) -> bool {
        *self == TCEM::LliLevel
    }
    ///At channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI
    #[inline(always)]
    pub fn is_channel_level(&self) -> bool {
        *self == TCEM::ChannelLevel
    }
}
///Field `TCEM` writer - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
pub type TCEM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCEM>;
impl<'a, REG> TCEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///At block level: the complete (and the half) transfer event is generated at the (respectively half of the) end of a block
    #[inline(always)]
    pub fn block_level(self) -> &'a mut crate::W<REG> {
        self.variant(TCEM::BlockLevel)
    }
    ///At LLI level: the complete transfer event is generated at the end of the LLI transfer. The half transfer event is generated at the half of the LLI data transfer
    #[inline(always)]
    pub fn lli_level(self) -> &'a mut crate::W<REG> {
        self.variant(TCEM::LliLevel)
    }
    ///At channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI
    #[inline(always)]
    pub fn channel_level(self) -> &'a mut crate::W<REG> {
        self.variant(TCEM::ChannelLevel)
    }
}
impl R {
    ///Bits 0:7 - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\[7:0\] value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 9 - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted.
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
    #[inline(always)]
    pub fn breq(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: - no 2D/repeated block support (GPDMA_CxBR1.BRC\[10:0\] must be set to 0 if present) - the peripheral must be set as the source of the transfer (DREQ = 0). - data packing to a wider destination width is not supported (if destination width source data width, GPDMA_CxTR1.PAM\[1\] must be set to 0). - GPDMA_CxBR1.BNDT\[15:0\] must be programmed as a multiple of the source (peripheral) burst size.
    #[inline(always)]
    pub fn pfreq(&self) -> PFREQ_R {
        PFREQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger..
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:21 - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\[1:0\] different 00.
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[5:0\].
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
    #[inline(always)]
    pub fn tcem(&self) -> TCEM_R {
        TCEM_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR2")
            .field("reqsel", &self.reqsel())
            .field("swreq", &self.swreq())
            .field("dreq", &self.dreq())
            .field("breq", &self.breq())
            .field("pfreq", &self.pfreq())
            .field("trigm", &self.trigm())
            .field("trigsel", &self.trigsel())
            .field("trigpol", &self.trigpol())
            .field("tcem", &self.tcem())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\[7:0\] value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.
    #[inline(always)]
    pub fn reqsel(&mut self) -> REQSEL_W<'_, TR2rs> {
        REQSEL_W::new(self, 0)
    }
    ///Bit 9 - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted.
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W<'_, TR2rs> {
        SWREQ_W::new(self, 9)
    }
    ///Bit 10 - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.
    #[inline(always)]
    pub fn dreq(&mut self) -> DREQ_W<'_, TR2rs> {
        DREQ_W::new(self, 10)
    }
    ///Bit 11 - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:
    #[inline(always)]
    pub fn breq(&mut self) -> BREQ_W<'_, TR2rs> {
        BREQ_W::new(self, 11)
    }
    ///Bit 12 - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: - no 2D/repeated block support (GPDMA_CxBR1.BRC\[10:0\] must be set to 0 if present) - the peripheral must be set as the source of the transfer (DREQ = 0). - data packing to a wider destination width is not supported (if destination width source data width, GPDMA_CxTR1.PAM\[1\] must be set to 0). - GPDMA_CxBR1.BNDT\[15:0\] must be programmed as a multiple of the source (peripheral) burst size.
    #[inline(always)]
    pub fn pfreq(&mut self) -> PFREQ_W<'_, TR2rs> {
        PFREQ_W::new(self, 12)
    }
    ///Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger..
    #[inline(always)]
    pub fn trigm(&mut self) -> TRIGM_W<'_, TR2rs> {
        TRIGM_W::new(self, 14)
    }
    ///Bits 16:21 - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\[1:0\] different 00.
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<'_, TR2rs> {
        TRIGSEL_W::new(self, 16)
    }
    ///Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\[5:0\].
    #[inline(always)]
    pub fn trigpol(&mut self) -> TRIGPOL_W<'_, TR2rs> {
        TRIGPOL_W::new(self, 24)
    }
    ///Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] = 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\[15:0\] =0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1.
    #[inline(always)]
    pub fn tcem(&mut self) -> TCEM_W<'_, TR2rs> {
        TCEM_W::new(self, 30)
    }
}
/**GPDMA channel 0 transfer register 2

You can [`read`](crate::Reg::read) this register and get [`tr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TR2rs;
impl crate::RegisterSpec for TR2rs {
    type Ux = u32;
}
///`read()` method returns [`tr2::R`](R) reader structure
impl crate::Readable for TR2rs {}
///`write(|w| ..)` method takes [`tr2::W`](W) writer structure
impl crate::Writable for TR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TR2 to value 0
impl crate::Resettable for TR2rs {}
