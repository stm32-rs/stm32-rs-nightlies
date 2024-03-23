#[doc = "Register `C1TR2` reader"]
pub type R = crate::R<C1TR2rs>;
#[doc = "Register `C1TR2` writer"]
pub type W = crate::W<C1TR2rs>;
#[doc = "GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 14.3.4. The user must not assign a same input hardware request (same REQSEL\\[7:0\\]
value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REQSEL {
    #[doc = "0: adc1_dma selected"]
    Adc1Dma = 0,
    #[doc = "2: dac1_ch1_dma selected"]
    Dac1Ch1Dma = 2,
    #[doc = "3: dac1_ch2_dma selected"]
    Dac1Ch2Dma = 3,
    #[doc = "4: tim6_upd_dma selected"]
    Tim6UpdDma = 4,
    #[doc = "5: tim7_upd_dma selected"]
    Tim7UpdDma = 5,
    #[doc = "6: spi1_rx_dma selected"]
    Spi1RxDma = 6,
    #[doc = "7: spi1_tx_dma selected"]
    Spi1TxDma = 7,
    #[doc = "8: spi2_rx_dma selected"]
    Spi2RxDma = 8,
    #[doc = "9: spi2_tx_dma selected"]
    Spi2TxDma = 9,
    #[doc = "10: spi3_rx_dma selected"]
    Spi3RxDma = 10,
    #[doc = "11: spi3_tx_dma selected"]
    Spi3TxDma = 11,
    #[doc = "12: i2c1_rx_dma selected"]
    I2c1RxDma = 12,
    #[doc = "13: i2c1_tx_dma selected"]
    I2c1TxDma = 13,
    #[doc = "15: i2c2_rx_dma selected"]
    I2c2RxDma = 15,
    #[doc = "16: i2c2_tx_dma selected"]
    I2c2TxDma = 16,
    #[doc = "18: i2c3_rx_dma selected"]
    I2c3RxDma = 18,
    #[doc = "19: i2c3_tx_dma selected"]
    I2c3TxDma = 19,
    #[doc = "21: usart1_rx_dma selected"]
    Usart1RxDma = 21,
    #[doc = "22: usart1_tx_dma selected"]
    Usart1TxDma = 22,
    #[doc = "23: usart2_rx_dma selected"]
    Usart2RxDma = 23,
    #[doc = "24: usart2_tx_dma selected"]
    Usart2TxDma = 24,
    #[doc = "25: usart3_rx_dma selected"]
    Usart3RxDma = 25,
    #[doc = "26: usart3_tx_dma selected"]
    Usart3TxDma = 26,
    #[doc = "27: uart4_rx_dma selected"]
    Uart4RxDma = 27,
    #[doc = "28: uart4_tx_dma selected"]
    Uart4TxDma = 28,
    #[doc = "29: uart5_rx_dma selected"]
    Uart5RxDma = 29,
    #[doc = "30: uart5_tx_dma selected"]
    Uart5TxDma = 30,
    #[doc = "31: usart6_rx_dma selected"]
    Usart6RxDma = 31,
    #[doc = "32: usart6_tx_dma selected"]
    Usart6TxDma = 32,
    #[doc = "33: uart7_rx_dma selected"]
    Uart7RxDma = 33,
    #[doc = "34: uart7_tx_dma selected"]
    Uart7TxDma = 34,
    #[doc = "35: uart8_rx_dma selected"]
    Uart8RxDma = 35,
    #[doc = "36: uart8_tx_dma selected"]
    Uart8TxDma = 36,
    #[doc = "37: uart9_rx_dma selected"]
    Uart9RxDma = 37,
    #[doc = "38: uart9_tx_dma selected"]
    Uart9TxDma = 38,
    #[doc = "39: uart10_rx_dma selected"]
    Uart10RxDma = 39,
    #[doc = "40: uart10_tx_dma selected"]
    Uart10TxDma = 40,
    #[doc = "41: uart11_rx_dma selected"]
    Uart11RxDma = 41,
    #[doc = "42: uart11_tx_dma selected"]
    Uart11TxDma = 42,
    #[doc = "43: uart12_rx_dma selected"]
    Uart12RxDma = 43,
    #[doc = "44: uart12_tx_dma selected"]
    Uart12TxDma = 44,
    #[doc = "45: lpuart1_rx_dma selected"]
    Lpuart1RxDma = 45,
    #[doc = "46: lpuart1_tx_dma selected"]
    Lpuart1TxDma = 46,
    #[doc = "47: spi4_rx_dma selected"]
    Spi4RxDma = 47,
    #[doc = "48: spi4_tx_dma selected"]
    Spi4TxDma = 48,
    #[doc = "49: spi5_rx_dma selected"]
    Spi5RxDma = 49,
    #[doc = "50: spi5_tx_dma selected"]
    Spi5TxDma = 50,
    #[doc = "51: spi6_rx_dma selected"]
    Spi6RxDma = 51,
    #[doc = "52: spi6_tx_dma selected"]
    Spi6TxDma = 52,
    #[doc = "53: sai1_a_dma selected"]
    Sai1ADma = 53,
    #[doc = "54: sai1_b_dma selected"]
    Sai1BDma = 54,
    #[doc = "55: sai2_a_dma selected"]
    Sai2ADma = 55,
    #[doc = "56: sai2_b_dma selected"]
    Sai2BDma = 56,
    #[doc = "57: ospi1_dma selected"]
    Ospi1Dma = 57,
    #[doc = "58: tim1_cc1_dma selected"]
    Tim1Cc1Dma = 58,
    #[doc = "59: tim1_cc2_dma selected"]
    Tim1Cc2Dma = 59,
    #[doc = "60: tim1_cc3_dma selected"]
    Tim1Cc3Dma = 60,
    #[doc = "61: tim1_cc4_dma selected"]
    Tim1Cc4Dma = 61,
    #[doc = "62: tim1_upd_dma selected"]
    Tim1UpdDma = 62,
    #[doc = "63: tim1_trg_dma selected"]
    Tim1TrgDma = 63,
    #[doc = "64: tim1_com_dma selected"]
    Tim1ComDma = 64,
    #[doc = "65: tim8_cc1_dma selected"]
    Tim8Cc1Dma = 65,
    #[doc = "66: tim8_cc2_dma selected"]
    Tim8Cc2Dma = 66,
    #[doc = "67: tim8_cc3_dma selected"]
    Tim8Cc3Dma = 67,
    #[doc = "68: tim8_cc4_dma selected"]
    Tim8Cc4Dma = 68,
    #[doc = "69: tim8_upd_dma selected"]
    Tim8UpdDma = 69,
    #[doc = "70: tim8_tig_dma selected"]
    Tim8TigDma = 70,
    #[doc = "71: tim8_com_dma selected"]
    Tim8ComDma = 71,
    #[doc = "72: tim2_cc1_dma selected"]
    Tim2Cc1Dma = 72,
    #[doc = "73: tim2_cc2_dma selected"]
    Tim2Cc2Dma = 73,
    #[doc = "74: tim2_cc3_dma selected"]
    Tim2Cc3Dma = 74,
    #[doc = "75: tim2_cc4_dma selected"]
    Tim2Cc4Dma = 75,
    #[doc = "76: tim2_upd_dma selected"]
    Tim2UpdDma = 76,
    #[doc = "77: tim3_cc1_dma selected"]
    Tim3Cc1Dma = 77,
    #[doc = "78: tim3_cc2_dma selected"]
    Tim3Cc2Dma = 78,
    #[doc = "79: tim3_cc3_dma selected"]
    Tim3Cc3Dma = 79,
    #[doc = "80: tim3_cc4_dma selected"]
    Tim3Cc4Dma = 80,
    #[doc = "81: tim3_upd_dma selected"]
    Tim3UpdDma = 81,
    #[doc = "82: tim3_trg_dma selected"]
    Tim3TrgDma = 82,
    #[doc = "83: tim4_cc1_dma selected"]
    Tim4Cc1Dma = 83,
    #[doc = "84: tim4_cc2_dma selected"]
    Tim4Cc2Dma = 84,
    #[doc = "85: tim4_cc3_dma selected"]
    Tim4Cc3Dma = 85,
    #[doc = "86: tim4_cc4_dma selected"]
    Tim4Cc4Dma = 86,
    #[doc = "87: tim4_upd_dma selected"]
    Tim4UpdDma = 87,
    #[doc = "88: tim5_cc1_dma selected"]
    Tim5Cc1Dma = 88,
    #[doc = "89: tim5_cc2_dma selected"]
    Tim5Cc2Dma = 89,
    #[doc = "90: tim5_cc3_dma selected"]
    Tim5Cc3Dma = 90,
    #[doc = "91: tim5_cc4_dma selected"]
    Tim5Cc4Dma = 91,
    #[doc = "92: tim5_upd_dma selected"]
    Tim5UpdDma = 92,
    #[doc = "93: tim5_trg_dma selected"]
    Tim5TrgDma = 93,
    #[doc = "94: tim15_cc1_dma selected"]
    Tim15Cc1Dma = 94,
    #[doc = "95: tim15_upd_dma selected"]
    Tim15UpdDma = 95,
    #[doc = "96: tim15_trg_dma selected"]
    Tim15TrgDma = 96,
    #[doc = "97: tim15_com_dma selected"]
    Tim15ComDma = 97,
    #[doc = "98: tim16_cc1_dma selected"]
    Tim16Cc1Dma = 98,
    #[doc = "99: tim16_upd_dma selected"]
    Tim16UpdDma = 99,
    #[doc = "100: tim17_cc1_dma selected"]
    Tim17Cc1Dma = 100,
    #[doc = "101: tim17_upd_dma selected"]
    Tim17UpdDma = 101,
    #[doc = "102: lptim1_ic1_dma selected"]
    Lptim1Ic1Dma = 102,
    #[doc = "103: lptim1_ic2_dma selected"]
    Lptim1Ic2Dma = 103,
    #[doc = "104: lptim1_ue_dma selected"]
    Lptim1UeDma = 104,
    #[doc = "105: lptim2_ic1_dma selected"]
    Lptim2Ic1Dma = 105,
    #[doc = "106: lptim2_ic2_dma selected"]
    Lptim2Ic2Dma = 106,
    #[doc = "107: lptim2_ue_dma selected"]
    Lptim2UeDma = 107,
    #[doc = "108: dcmi_dma or pssi_dma(1) selected"]
    DcmiPssiDma = 108,
    #[doc = "109: aes_out_dma selected"]
    AesOutDma = 109,
    #[doc = "110: aes_in_dma selected"]
    AesInDma = 110,
    #[doc = "111: hash_in_dma selected"]
    HashInDma = 111,
    #[doc = "112: ucpd1_rx_dma selected"]
    Ucpd1RxDma = 112,
    #[doc = "113: ucpd1_tx_dma selected"]
    Ucpd1TxDma = 113,
    #[doc = "114: cordic_read_dma selected"]
    CordicReadDma = 114,
    #[doc = "115: cordic_write_dma selected"]
    CordicWriteDma = 115,
    #[doc = "116: fmac_read_dma selected"]
    FmacReadDma = 116,
    #[doc = "117: fmac_write_dma selected"]
    FmacWriteDma = 117,
    #[doc = "118: saes_out_dma selected"]
    SaesOutDma = 118,
    #[doc = "119: saes_in_dma selected"]
    SaesInDma = 119,
    #[doc = "120: i3c1_rx_dma selected"]
    I3c1RxDma = 120,
    #[doc = "121: i3c1_tx_dma selected"]
    I3c1TxDma = 121,
    #[doc = "122: i3c1_tc_dma selected"]
    I3c1TcDma = 122,
    #[doc = "123: i3c1_rs_dma selected"]
    I3c1RsDma = 123,
    #[doc = "124: i2c4_rx_dma selected"]
    I2c4RxDma = 124,
    #[doc = "125: i2c4_tx_dma selected"]
    I2c4TxDma = 125,
    #[doc = "127: lptim3_ic1_dma selected"]
    Lptim3Ic1Dma = 127,
    #[doc = "128: lptim3_ic2_dma selected"]
    Lptim3Ic2Dma = 128,
    #[doc = "129: lptim3_ue_dma selected"]
    Lptim3UeDma = 129,
    #[doc = "130: lptim5_ic1_dma selected"]
    Lptim5Ic1Dma = 130,
    #[doc = "131: lptim5_ic2_dma selected"]
    Lptim5Ic2Dma = 131,
    #[doc = "132: lptim5_ue_dma selected"]
    Lptim5UeDma = 132,
    #[doc = "133: lptim6_ic1_dma selected"]
    Lptim6Ic1Dma = 133,
    #[doc = "134: lptim6_ic2_dma selected"]
    Lptim6Ic2Dma = 134,
    #[doc = "135: lptim6_ue_dma selected"]
    Lptim6UeDma = 135,
    #[doc = "136: i3c2_rx selected"]
    I3c2Rx = 136,
    #[doc = "137: i3c2_tx selected"]
    I3c2Tx = 137,
    #[doc = "138: i3c2_tc selected"]
    I3c2Tc = 138,
    #[doc = "139: i3c2_rs selected"]
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
#[doc = "Field `REQSEL` reader - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 14.3.4. The user must not assign a same input hardware request (same REQSEL\\[7:0\\]
value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
pub type REQSEL_R = crate::FieldReader<REQSEL>;
impl REQSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "adc1_dma selected"]
    #[inline(always)]
    pub fn is_adc1_dma(&self) -> bool {
        *self == REQSEL::Adc1Dma
    }
    #[doc = "dac1_ch1_dma selected"]
    #[inline(always)]
    pub fn is_dac1_ch1_dma(&self) -> bool {
        *self == REQSEL::Dac1Ch1Dma
    }
    #[doc = "dac1_ch2_dma selected"]
    #[inline(always)]
    pub fn is_dac1_ch2_dma(&self) -> bool {
        *self == REQSEL::Dac1Ch2Dma
    }
    #[doc = "tim6_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim6_upd_dma(&self) -> bool {
        *self == REQSEL::Tim6UpdDma
    }
    #[doc = "tim7_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim7_upd_dma(&self) -> bool {
        *self == REQSEL::Tim7UpdDma
    }
    #[doc = "spi1_rx_dma selected"]
    #[inline(always)]
    pub fn is_spi1_rx_dma(&self) -> bool {
        *self == REQSEL::Spi1RxDma
    }
    #[doc = "spi1_tx_dma selected"]
    #[inline(always)]
    pub fn is_spi1_tx_dma(&self) -> bool {
        *self == REQSEL::Spi1TxDma
    }
    #[doc = "spi2_rx_dma selected"]
    #[inline(always)]
    pub fn is_spi2_rx_dma(&self) -> bool {
        *self == REQSEL::Spi2RxDma
    }
    #[doc = "spi2_tx_dma selected"]
    #[inline(always)]
    pub fn is_spi2_tx_dma(&self) -> bool {
        *self == REQSEL::Spi2TxDma
    }
    #[doc = "spi3_rx_dma selected"]
    #[inline(always)]
    pub fn is_spi3_rx_dma(&self) -> bool {
        *self == REQSEL::Spi3RxDma
    }
    #[doc = "spi3_tx_dma selected"]
    #[inline(always)]
    pub fn is_spi3_tx_dma(&self) -> bool {
        *self == REQSEL::Spi3TxDma
    }
    #[doc = "i2c1_rx_dma selected"]
    #[inline(always)]
    pub fn is_i2c1_rx_dma(&self) -> bool {
        *self == REQSEL::I2c1RxDma
    }
    #[doc = "i2c1_tx_dma selected"]
    #[inline(always)]
    pub fn is_i2c1_tx_dma(&self) -> bool {
        *self == REQSEL::I2c1TxDma
    }
    #[doc = "i2c2_rx_dma selected"]
    #[inline(always)]
    pub fn is_i2c2_rx_dma(&self) -> bool {
        *self == REQSEL::I2c2RxDma
    }
    #[doc = "i2c2_tx_dma selected"]
    #[inline(always)]
    pub fn is_i2c2_tx_dma(&self) -> bool {
        *self == REQSEL::I2c2TxDma
    }
    #[doc = "i2c3_rx_dma selected"]
    #[inline(always)]
    pub fn is_i2c3_rx_dma(&self) -> bool {
        *self == REQSEL::I2c3RxDma
    }
    #[doc = "i2c3_tx_dma selected"]
    #[inline(always)]
    pub fn is_i2c3_tx_dma(&self) -> bool {
        *self == REQSEL::I2c3TxDma
    }
    #[doc = "usart1_rx_dma selected"]
    #[inline(always)]
    pub fn is_usart1_rx_dma(&self) -> bool {
        *self == REQSEL::Usart1RxDma
    }
    #[doc = "usart1_tx_dma selected"]
    #[inline(always)]
    pub fn is_usart1_tx_dma(&self) -> bool {
        *self == REQSEL::Usart1TxDma
    }
    #[doc = "usart2_rx_dma selected"]
    #[inline(always)]
    pub fn is_usart2_rx_dma(&self) -> bool {
        *self == REQSEL::Usart2RxDma
    }
    #[doc = "usart2_tx_dma selected"]
    #[inline(always)]
    pub fn is_usart2_tx_dma(&self) -> bool {
        *self == REQSEL::Usart2TxDma
    }
    #[doc = "usart3_rx_dma selected"]
    #[inline(always)]
    pub fn is_usart3_rx_dma(&self) -> bool {
        *self == REQSEL::Usart3RxDma
    }
    #[doc = "usart3_tx_dma selected"]
    #[inline(always)]
    pub fn is_usart3_tx_dma(&self) -> bool {
        *self == REQSEL::Usart3TxDma
    }
    #[doc = "uart4_rx_dma selected"]
    #[inline(always)]
    pub fn is_uart4_rx_dma(&self) -> bool {
        *self == REQSEL::Uart4RxDma
    }
    #[doc = "uart4_tx_dma selected"]
    #[inline(always)]
    pub fn is_uart4_tx_dma(&self) -> bool {
        *self == REQSEL::Uart4TxDma
    }
    #[doc = "uart5_rx_dma selected"]
    #[inline(always)]
    pub fn is_uart5_rx_dma(&self) -> bool {
        *self == REQSEL::Uart5RxDma
    }
    #[doc = "uart5_tx_dma selected"]
    #[inline(always)]
    pub fn is_uart5_tx_dma(&self) -> bool {
        *self == REQSEL::Uart5TxDma
    }
    #[doc = "usart6_rx_dma selected"]
    #[inline(always)]
    pub fn is_usart6_rx_dma(&self) -> bool {
        *self == REQSEL::Usart6RxDma
    }
    #[doc = "usart6_tx_dma selected"]
    #[inline(always)]
    pub fn is_usart6_tx_dma(&self) -> bool {
        *self == REQSEL::Usart6TxDma
    }
    #[doc = "uart7_rx_dma selected"]
    #[inline(always)]
    pub fn is_uart7_rx_dma(&self) -> bool {
        *self == REQSEL::Uart7RxDma
    }
    #[doc = "uart7_tx_dma selected"]
    #[inline(always)]
    pub fn is_uart7_tx_dma(&self) -> bool {
        *self == REQSEL::Uart7TxDma
    }
    #[doc = "uart8_rx_dma selected"]
    #[inline(always)]
    pub fn is_uart8_rx_dma(&self) -> bool {
        *self == REQSEL::Uart8RxDma
    }
    #[doc = "uart8_tx_dma selected"]
    #[inline(always)]
    pub fn is_uart8_tx_dma(&self) -> bool {
        *self == REQSEL::Uart8TxDma
    }
    #[doc = "uart9_rx_dma selected"]
    #[inline(always)]
    pub fn is_uart9_rx_dma(&self) -> bool {
        *self == REQSEL::Uart9RxDma
    }
    #[doc = "uart9_tx_dma selected"]
    #[inline(always)]
    pub fn is_uart9_tx_dma(&self) -> bool {
        *self == REQSEL::Uart9TxDma
    }
    #[doc = "uart10_rx_dma selected"]
    #[inline(always)]
    pub fn is_uart10_rx_dma(&self) -> bool {
        *self == REQSEL::Uart10RxDma
    }
    #[doc = "uart10_tx_dma selected"]
    #[inline(always)]
    pub fn is_uart10_tx_dma(&self) -> bool {
        *self == REQSEL::Uart10TxDma
    }
    #[doc = "uart11_rx_dma selected"]
    #[inline(always)]
    pub fn is_uart11_rx_dma(&self) -> bool {
        *self == REQSEL::Uart11RxDma
    }
    #[doc = "uart11_tx_dma selected"]
    #[inline(always)]
    pub fn is_uart11_tx_dma(&self) -> bool {
        *self == REQSEL::Uart11TxDma
    }
    #[doc = "uart12_rx_dma selected"]
    #[inline(always)]
    pub fn is_uart12_rx_dma(&self) -> bool {
        *self == REQSEL::Uart12RxDma
    }
    #[doc = "uart12_tx_dma selected"]
    #[inline(always)]
    pub fn is_uart12_tx_dma(&self) -> bool {
        *self == REQSEL::Uart12TxDma
    }
    #[doc = "lpuart1_rx_dma selected"]
    #[inline(always)]
    pub fn is_lpuart1_rx_dma(&self) -> bool {
        *self == REQSEL::Lpuart1RxDma
    }
    #[doc = "lpuart1_tx_dma selected"]
    #[inline(always)]
    pub fn is_lpuart1_tx_dma(&self) -> bool {
        *self == REQSEL::Lpuart1TxDma
    }
    #[doc = "spi4_rx_dma selected"]
    #[inline(always)]
    pub fn is_spi4_rx_dma(&self) -> bool {
        *self == REQSEL::Spi4RxDma
    }
    #[doc = "spi4_tx_dma selected"]
    #[inline(always)]
    pub fn is_spi4_tx_dma(&self) -> bool {
        *self == REQSEL::Spi4TxDma
    }
    #[doc = "spi5_rx_dma selected"]
    #[inline(always)]
    pub fn is_spi5_rx_dma(&self) -> bool {
        *self == REQSEL::Spi5RxDma
    }
    #[doc = "spi5_tx_dma selected"]
    #[inline(always)]
    pub fn is_spi5_tx_dma(&self) -> bool {
        *self == REQSEL::Spi5TxDma
    }
    #[doc = "spi6_rx_dma selected"]
    #[inline(always)]
    pub fn is_spi6_rx_dma(&self) -> bool {
        *self == REQSEL::Spi6RxDma
    }
    #[doc = "spi6_tx_dma selected"]
    #[inline(always)]
    pub fn is_spi6_tx_dma(&self) -> bool {
        *self == REQSEL::Spi6TxDma
    }
    #[doc = "sai1_a_dma selected"]
    #[inline(always)]
    pub fn is_sai1_a_dma(&self) -> bool {
        *self == REQSEL::Sai1ADma
    }
    #[doc = "sai1_b_dma selected"]
    #[inline(always)]
    pub fn is_sai1_b_dma(&self) -> bool {
        *self == REQSEL::Sai1BDma
    }
    #[doc = "sai2_a_dma selected"]
    #[inline(always)]
    pub fn is_sai2_a_dma(&self) -> bool {
        *self == REQSEL::Sai2ADma
    }
    #[doc = "sai2_b_dma selected"]
    #[inline(always)]
    pub fn is_sai2_b_dma(&self) -> bool {
        *self == REQSEL::Sai2BDma
    }
    #[doc = "ospi1_dma selected"]
    #[inline(always)]
    pub fn is_ospi1_dma(&self) -> bool {
        *self == REQSEL::Ospi1Dma
    }
    #[doc = "tim1_cc1_dma selected"]
    #[inline(always)]
    pub fn is_tim1_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim1Cc1Dma
    }
    #[doc = "tim1_cc2_dma selected"]
    #[inline(always)]
    pub fn is_tim1_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim1Cc2Dma
    }
    #[doc = "tim1_cc3_dma selected"]
    #[inline(always)]
    pub fn is_tim1_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim1Cc3Dma
    }
    #[doc = "tim1_cc4_dma selected"]
    #[inline(always)]
    pub fn is_tim1_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim1Cc4Dma
    }
    #[doc = "tim1_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim1_upd_dma(&self) -> bool {
        *self == REQSEL::Tim1UpdDma
    }
    #[doc = "tim1_trg_dma selected"]
    #[inline(always)]
    pub fn is_tim1_trg_dma(&self) -> bool {
        *self == REQSEL::Tim1TrgDma
    }
    #[doc = "tim1_com_dma selected"]
    #[inline(always)]
    pub fn is_tim1_com_dma(&self) -> bool {
        *self == REQSEL::Tim1ComDma
    }
    #[doc = "tim8_cc1_dma selected"]
    #[inline(always)]
    pub fn is_tim8_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim8Cc1Dma
    }
    #[doc = "tim8_cc2_dma selected"]
    #[inline(always)]
    pub fn is_tim8_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim8Cc2Dma
    }
    #[doc = "tim8_cc3_dma selected"]
    #[inline(always)]
    pub fn is_tim8_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim8Cc3Dma
    }
    #[doc = "tim8_cc4_dma selected"]
    #[inline(always)]
    pub fn is_tim8_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim8Cc4Dma
    }
    #[doc = "tim8_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim8_upd_dma(&self) -> bool {
        *self == REQSEL::Tim8UpdDma
    }
    #[doc = "tim8_tig_dma selected"]
    #[inline(always)]
    pub fn is_tim8_tig_dma(&self) -> bool {
        *self == REQSEL::Tim8TigDma
    }
    #[doc = "tim8_com_dma selected"]
    #[inline(always)]
    pub fn is_tim8_com_dma(&self) -> bool {
        *self == REQSEL::Tim8ComDma
    }
    #[doc = "tim2_cc1_dma selected"]
    #[inline(always)]
    pub fn is_tim2_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim2Cc1Dma
    }
    #[doc = "tim2_cc2_dma selected"]
    #[inline(always)]
    pub fn is_tim2_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim2Cc2Dma
    }
    #[doc = "tim2_cc3_dma selected"]
    #[inline(always)]
    pub fn is_tim2_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim2Cc3Dma
    }
    #[doc = "tim2_cc4_dma selected"]
    #[inline(always)]
    pub fn is_tim2_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim2Cc4Dma
    }
    #[doc = "tim2_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim2_upd_dma(&self) -> bool {
        *self == REQSEL::Tim2UpdDma
    }
    #[doc = "tim3_cc1_dma selected"]
    #[inline(always)]
    pub fn is_tim3_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim3Cc1Dma
    }
    #[doc = "tim3_cc2_dma selected"]
    #[inline(always)]
    pub fn is_tim3_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim3Cc2Dma
    }
    #[doc = "tim3_cc3_dma selected"]
    #[inline(always)]
    pub fn is_tim3_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim3Cc3Dma
    }
    #[doc = "tim3_cc4_dma selected"]
    #[inline(always)]
    pub fn is_tim3_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim3Cc4Dma
    }
    #[doc = "tim3_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim3_upd_dma(&self) -> bool {
        *self == REQSEL::Tim3UpdDma
    }
    #[doc = "tim3_trg_dma selected"]
    #[inline(always)]
    pub fn is_tim3_trg_dma(&self) -> bool {
        *self == REQSEL::Tim3TrgDma
    }
    #[doc = "tim4_cc1_dma selected"]
    #[inline(always)]
    pub fn is_tim4_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim4Cc1Dma
    }
    #[doc = "tim4_cc2_dma selected"]
    #[inline(always)]
    pub fn is_tim4_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim4Cc2Dma
    }
    #[doc = "tim4_cc3_dma selected"]
    #[inline(always)]
    pub fn is_tim4_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim4Cc3Dma
    }
    #[doc = "tim4_cc4_dma selected"]
    #[inline(always)]
    pub fn is_tim4_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim4Cc4Dma
    }
    #[doc = "tim4_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim4_upd_dma(&self) -> bool {
        *self == REQSEL::Tim4UpdDma
    }
    #[doc = "tim5_cc1_dma selected"]
    #[inline(always)]
    pub fn is_tim5_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim5Cc1Dma
    }
    #[doc = "tim5_cc2_dma selected"]
    #[inline(always)]
    pub fn is_tim5_cc2_dma(&self) -> bool {
        *self == REQSEL::Tim5Cc2Dma
    }
    #[doc = "tim5_cc3_dma selected"]
    #[inline(always)]
    pub fn is_tim5_cc3_dma(&self) -> bool {
        *self == REQSEL::Tim5Cc3Dma
    }
    #[doc = "tim5_cc4_dma selected"]
    #[inline(always)]
    pub fn is_tim5_cc4_dma(&self) -> bool {
        *self == REQSEL::Tim5Cc4Dma
    }
    #[doc = "tim5_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim5_upd_dma(&self) -> bool {
        *self == REQSEL::Tim5UpdDma
    }
    #[doc = "tim5_trg_dma selected"]
    #[inline(always)]
    pub fn is_tim5_trg_dma(&self) -> bool {
        *self == REQSEL::Tim5TrgDma
    }
    #[doc = "tim15_cc1_dma selected"]
    #[inline(always)]
    pub fn is_tim15_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim15Cc1Dma
    }
    #[doc = "tim15_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim15_upd_dma(&self) -> bool {
        *self == REQSEL::Tim15UpdDma
    }
    #[doc = "tim15_trg_dma selected"]
    #[inline(always)]
    pub fn is_tim15_trg_dma(&self) -> bool {
        *self == REQSEL::Tim15TrgDma
    }
    #[doc = "tim15_com_dma selected"]
    #[inline(always)]
    pub fn is_tim15_com_dma(&self) -> bool {
        *self == REQSEL::Tim15ComDma
    }
    #[doc = "tim16_cc1_dma selected"]
    #[inline(always)]
    pub fn is_tim16_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim16Cc1Dma
    }
    #[doc = "tim16_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim16_upd_dma(&self) -> bool {
        *self == REQSEL::Tim16UpdDma
    }
    #[doc = "tim17_cc1_dma selected"]
    #[inline(always)]
    pub fn is_tim17_cc1_dma(&self) -> bool {
        *self == REQSEL::Tim17Cc1Dma
    }
    #[doc = "tim17_upd_dma selected"]
    #[inline(always)]
    pub fn is_tim17_upd_dma(&self) -> bool {
        *self == REQSEL::Tim17UpdDma
    }
    #[doc = "lptim1_ic1_dma selected"]
    #[inline(always)]
    pub fn is_lptim1_ic1_dma(&self) -> bool {
        *self == REQSEL::Lptim1Ic1Dma
    }
    #[doc = "lptim1_ic2_dma selected"]
    #[inline(always)]
    pub fn is_lptim1_ic2_dma(&self) -> bool {
        *self == REQSEL::Lptim1Ic2Dma
    }
    #[doc = "lptim1_ue_dma selected"]
    #[inline(always)]
    pub fn is_lptim1_ue_dma(&self) -> bool {
        *self == REQSEL::Lptim1UeDma
    }
    #[doc = "lptim2_ic1_dma selected"]
    #[inline(always)]
    pub fn is_lptim2_ic1_dma(&self) -> bool {
        *self == REQSEL::Lptim2Ic1Dma
    }
    #[doc = "lptim2_ic2_dma selected"]
    #[inline(always)]
    pub fn is_lptim2_ic2_dma(&self) -> bool {
        *self == REQSEL::Lptim2Ic2Dma
    }
    #[doc = "lptim2_ue_dma selected"]
    #[inline(always)]
    pub fn is_lptim2_ue_dma(&self) -> bool {
        *self == REQSEL::Lptim2UeDma
    }
    #[doc = "dcmi_dma or pssi_dma(1) selected"]
    #[inline(always)]
    pub fn is_dcmi_pssi_dma(&self) -> bool {
        *self == REQSEL::DcmiPssiDma
    }
    #[doc = "aes_out_dma selected"]
    #[inline(always)]
    pub fn is_aes_out_dma(&self) -> bool {
        *self == REQSEL::AesOutDma
    }
    #[doc = "aes_in_dma selected"]
    #[inline(always)]
    pub fn is_aes_in_dma(&self) -> bool {
        *self == REQSEL::AesInDma
    }
    #[doc = "hash_in_dma selected"]
    #[inline(always)]
    pub fn is_hash_in_dma(&self) -> bool {
        *self == REQSEL::HashInDma
    }
    #[doc = "ucpd1_rx_dma selected"]
    #[inline(always)]
    pub fn is_ucpd1_rx_dma(&self) -> bool {
        *self == REQSEL::Ucpd1RxDma
    }
    #[doc = "ucpd1_tx_dma selected"]
    #[inline(always)]
    pub fn is_ucpd1_tx_dma(&self) -> bool {
        *self == REQSEL::Ucpd1TxDma
    }
    #[doc = "cordic_read_dma selected"]
    #[inline(always)]
    pub fn is_cordic_read_dma(&self) -> bool {
        *self == REQSEL::CordicReadDma
    }
    #[doc = "cordic_write_dma selected"]
    #[inline(always)]
    pub fn is_cordic_write_dma(&self) -> bool {
        *self == REQSEL::CordicWriteDma
    }
    #[doc = "fmac_read_dma selected"]
    #[inline(always)]
    pub fn is_fmac_read_dma(&self) -> bool {
        *self == REQSEL::FmacReadDma
    }
    #[doc = "fmac_write_dma selected"]
    #[inline(always)]
    pub fn is_fmac_write_dma(&self) -> bool {
        *self == REQSEL::FmacWriteDma
    }
    #[doc = "saes_out_dma selected"]
    #[inline(always)]
    pub fn is_saes_out_dma(&self) -> bool {
        *self == REQSEL::SaesOutDma
    }
    #[doc = "saes_in_dma selected"]
    #[inline(always)]
    pub fn is_saes_in_dma(&self) -> bool {
        *self == REQSEL::SaesInDma
    }
    #[doc = "i3c1_rx_dma selected"]
    #[inline(always)]
    pub fn is_i3c1_rx_dma(&self) -> bool {
        *self == REQSEL::I3c1RxDma
    }
    #[doc = "i3c1_tx_dma selected"]
    #[inline(always)]
    pub fn is_i3c1_tx_dma(&self) -> bool {
        *self == REQSEL::I3c1TxDma
    }
    #[doc = "i3c1_tc_dma selected"]
    #[inline(always)]
    pub fn is_i3c1_tc_dma(&self) -> bool {
        *self == REQSEL::I3c1TcDma
    }
    #[doc = "i3c1_rs_dma selected"]
    #[inline(always)]
    pub fn is_i3c1_rs_dma(&self) -> bool {
        *self == REQSEL::I3c1RsDma
    }
    #[doc = "i2c4_rx_dma selected"]
    #[inline(always)]
    pub fn is_i2c4_rx_dma(&self) -> bool {
        *self == REQSEL::I2c4RxDma
    }
    #[doc = "i2c4_tx_dma selected"]
    #[inline(always)]
    pub fn is_i2c4_tx_dma(&self) -> bool {
        *self == REQSEL::I2c4TxDma
    }
    #[doc = "lptim3_ic1_dma selected"]
    #[inline(always)]
    pub fn is_lptim3_ic1_dma(&self) -> bool {
        *self == REQSEL::Lptim3Ic1Dma
    }
    #[doc = "lptim3_ic2_dma selected"]
    #[inline(always)]
    pub fn is_lptim3_ic2_dma(&self) -> bool {
        *self == REQSEL::Lptim3Ic2Dma
    }
    #[doc = "lptim3_ue_dma selected"]
    #[inline(always)]
    pub fn is_lptim3_ue_dma(&self) -> bool {
        *self == REQSEL::Lptim3UeDma
    }
    #[doc = "lptim5_ic1_dma selected"]
    #[inline(always)]
    pub fn is_lptim5_ic1_dma(&self) -> bool {
        *self == REQSEL::Lptim5Ic1Dma
    }
    #[doc = "lptim5_ic2_dma selected"]
    #[inline(always)]
    pub fn is_lptim5_ic2_dma(&self) -> bool {
        *self == REQSEL::Lptim5Ic2Dma
    }
    #[doc = "lptim5_ue_dma selected"]
    #[inline(always)]
    pub fn is_lptim5_ue_dma(&self) -> bool {
        *self == REQSEL::Lptim5UeDma
    }
    #[doc = "lptim6_ic1_dma selected"]
    #[inline(always)]
    pub fn is_lptim6_ic1_dma(&self) -> bool {
        *self == REQSEL::Lptim6Ic1Dma
    }
    #[doc = "lptim6_ic2_dma selected"]
    #[inline(always)]
    pub fn is_lptim6_ic2_dma(&self) -> bool {
        *self == REQSEL::Lptim6Ic2Dma
    }
    #[doc = "lptim6_ue_dma selected"]
    #[inline(always)]
    pub fn is_lptim6_ue_dma(&self) -> bool {
        *self == REQSEL::Lptim6UeDma
    }
    #[doc = "i3c2_rx selected"]
    #[inline(always)]
    pub fn is_i3c2_rx(&self) -> bool {
        *self == REQSEL::I3c2Rx
    }
    #[doc = "i3c2_tx selected"]
    #[inline(always)]
    pub fn is_i3c2_tx(&self) -> bool {
        *self == REQSEL::I3c2Tx
    }
    #[doc = "i3c2_tc selected"]
    #[inline(always)]
    pub fn is_i3c2_tc(&self) -> bool {
        *self == REQSEL::I3c2Tc
    }
    #[doc = "i3c2_rs selected"]
    #[inline(always)]
    pub fn is_i3c2_rs(&self) -> bool {
        *self == REQSEL::I3c2Rs
    }
}
#[doc = "Field `REQSEL` writer - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 14.3.4. The user must not assign a same input hardware request (same REQSEL\\[7:0\\]
value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
pub type REQSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8, REQSEL>;
impl<'a, REG> REQSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "adc1_dma selected"]
    #[inline(always)]
    pub fn adc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Adc1Dma)
    }
    #[doc = "dac1_ch1_dma selected"]
    #[inline(always)]
    pub fn dac1_ch1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Dac1Ch1Dma)
    }
    #[doc = "dac1_ch2_dma selected"]
    #[inline(always)]
    pub fn dac1_ch2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Dac1Ch2Dma)
    }
    #[doc = "tim6_upd_dma selected"]
    #[inline(always)]
    pub fn tim6_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim6UpdDma)
    }
    #[doc = "tim7_upd_dma selected"]
    #[inline(always)]
    pub fn tim7_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim7UpdDma)
    }
    #[doc = "spi1_rx_dma selected"]
    #[inline(always)]
    pub fn spi1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi1RxDma)
    }
    #[doc = "spi1_tx_dma selected"]
    #[inline(always)]
    pub fn spi1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi1TxDma)
    }
    #[doc = "spi2_rx_dma selected"]
    #[inline(always)]
    pub fn spi2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi2RxDma)
    }
    #[doc = "spi2_tx_dma selected"]
    #[inline(always)]
    pub fn spi2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi2TxDma)
    }
    #[doc = "spi3_rx_dma selected"]
    #[inline(always)]
    pub fn spi3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi3RxDma)
    }
    #[doc = "spi3_tx_dma selected"]
    #[inline(always)]
    pub fn spi3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi3TxDma)
    }
    #[doc = "i2c1_rx_dma selected"]
    #[inline(always)]
    pub fn i2c1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c1RxDma)
    }
    #[doc = "i2c1_tx_dma selected"]
    #[inline(always)]
    pub fn i2c1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c1TxDma)
    }
    #[doc = "i2c2_rx_dma selected"]
    #[inline(always)]
    pub fn i2c2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c2RxDma)
    }
    #[doc = "i2c2_tx_dma selected"]
    #[inline(always)]
    pub fn i2c2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c2TxDma)
    }
    #[doc = "i2c3_rx_dma selected"]
    #[inline(always)]
    pub fn i2c3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c3RxDma)
    }
    #[doc = "i2c3_tx_dma selected"]
    #[inline(always)]
    pub fn i2c3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c3TxDma)
    }
    #[doc = "usart1_rx_dma selected"]
    #[inline(always)]
    pub fn usart1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart1RxDma)
    }
    #[doc = "usart1_tx_dma selected"]
    #[inline(always)]
    pub fn usart1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart1TxDma)
    }
    #[doc = "usart2_rx_dma selected"]
    #[inline(always)]
    pub fn usart2_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart2RxDma)
    }
    #[doc = "usart2_tx_dma selected"]
    #[inline(always)]
    pub fn usart2_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart2TxDma)
    }
    #[doc = "usart3_rx_dma selected"]
    #[inline(always)]
    pub fn usart3_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart3RxDma)
    }
    #[doc = "usart3_tx_dma selected"]
    #[inline(always)]
    pub fn usart3_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart3TxDma)
    }
    #[doc = "uart4_rx_dma selected"]
    #[inline(always)]
    pub fn uart4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart4RxDma)
    }
    #[doc = "uart4_tx_dma selected"]
    #[inline(always)]
    pub fn uart4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart4TxDma)
    }
    #[doc = "uart5_rx_dma selected"]
    #[inline(always)]
    pub fn uart5_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart5RxDma)
    }
    #[doc = "uart5_tx_dma selected"]
    #[inline(always)]
    pub fn uart5_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart5TxDma)
    }
    #[doc = "usart6_rx_dma selected"]
    #[inline(always)]
    pub fn usart6_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart6RxDma)
    }
    #[doc = "usart6_tx_dma selected"]
    #[inline(always)]
    pub fn usart6_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Usart6TxDma)
    }
    #[doc = "uart7_rx_dma selected"]
    #[inline(always)]
    pub fn uart7_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart7RxDma)
    }
    #[doc = "uart7_tx_dma selected"]
    #[inline(always)]
    pub fn uart7_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart7TxDma)
    }
    #[doc = "uart8_rx_dma selected"]
    #[inline(always)]
    pub fn uart8_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart8RxDma)
    }
    #[doc = "uart8_tx_dma selected"]
    #[inline(always)]
    pub fn uart8_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart8TxDma)
    }
    #[doc = "uart9_rx_dma selected"]
    #[inline(always)]
    pub fn uart9_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart9RxDma)
    }
    #[doc = "uart9_tx_dma selected"]
    #[inline(always)]
    pub fn uart9_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart9TxDma)
    }
    #[doc = "uart10_rx_dma selected"]
    #[inline(always)]
    pub fn uart10_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart10RxDma)
    }
    #[doc = "uart10_tx_dma selected"]
    #[inline(always)]
    pub fn uart10_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart10TxDma)
    }
    #[doc = "uart11_rx_dma selected"]
    #[inline(always)]
    pub fn uart11_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart11RxDma)
    }
    #[doc = "uart11_tx_dma selected"]
    #[inline(always)]
    pub fn uart11_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart11TxDma)
    }
    #[doc = "uart12_rx_dma selected"]
    #[inline(always)]
    pub fn uart12_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart12RxDma)
    }
    #[doc = "uart12_tx_dma selected"]
    #[inline(always)]
    pub fn uart12_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Uart12TxDma)
    }
    #[doc = "lpuart1_rx_dma selected"]
    #[inline(always)]
    pub fn lpuart1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lpuart1RxDma)
    }
    #[doc = "lpuart1_tx_dma selected"]
    #[inline(always)]
    pub fn lpuart1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lpuart1TxDma)
    }
    #[doc = "spi4_rx_dma selected"]
    #[inline(always)]
    pub fn spi4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi4RxDma)
    }
    #[doc = "spi4_tx_dma selected"]
    #[inline(always)]
    pub fn spi4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi4TxDma)
    }
    #[doc = "spi5_rx_dma selected"]
    #[inline(always)]
    pub fn spi5_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi5RxDma)
    }
    #[doc = "spi5_tx_dma selected"]
    #[inline(always)]
    pub fn spi5_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi5TxDma)
    }
    #[doc = "spi6_rx_dma selected"]
    #[inline(always)]
    pub fn spi6_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi6RxDma)
    }
    #[doc = "spi6_tx_dma selected"]
    #[inline(always)]
    pub fn spi6_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Spi6TxDma)
    }
    #[doc = "sai1_a_dma selected"]
    #[inline(always)]
    pub fn sai1_a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Sai1ADma)
    }
    #[doc = "sai1_b_dma selected"]
    #[inline(always)]
    pub fn sai1_b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Sai1BDma)
    }
    #[doc = "sai2_a_dma selected"]
    #[inline(always)]
    pub fn sai2_a_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Sai2ADma)
    }
    #[doc = "sai2_b_dma selected"]
    #[inline(always)]
    pub fn sai2_b_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Sai2BDma)
    }
    #[doc = "ospi1_dma selected"]
    #[inline(always)]
    pub fn ospi1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Ospi1Dma)
    }
    #[doc = "tim1_cc1_dma selected"]
    #[inline(always)]
    pub fn tim1_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1Cc1Dma)
    }
    #[doc = "tim1_cc2_dma selected"]
    #[inline(always)]
    pub fn tim1_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1Cc2Dma)
    }
    #[doc = "tim1_cc3_dma selected"]
    #[inline(always)]
    pub fn tim1_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1Cc3Dma)
    }
    #[doc = "tim1_cc4_dma selected"]
    #[inline(always)]
    pub fn tim1_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1Cc4Dma)
    }
    #[doc = "tim1_upd_dma selected"]
    #[inline(always)]
    pub fn tim1_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1UpdDma)
    }
    #[doc = "tim1_trg_dma selected"]
    #[inline(always)]
    pub fn tim1_trg_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1TrgDma)
    }
    #[doc = "tim1_com_dma selected"]
    #[inline(always)]
    pub fn tim1_com_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim1ComDma)
    }
    #[doc = "tim8_cc1_dma selected"]
    #[inline(always)]
    pub fn tim8_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8Cc1Dma)
    }
    #[doc = "tim8_cc2_dma selected"]
    #[inline(always)]
    pub fn tim8_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8Cc2Dma)
    }
    #[doc = "tim8_cc3_dma selected"]
    #[inline(always)]
    pub fn tim8_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8Cc3Dma)
    }
    #[doc = "tim8_cc4_dma selected"]
    #[inline(always)]
    pub fn tim8_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8Cc4Dma)
    }
    #[doc = "tim8_upd_dma selected"]
    #[inline(always)]
    pub fn tim8_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8UpdDma)
    }
    #[doc = "tim8_tig_dma selected"]
    #[inline(always)]
    pub fn tim8_tig_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8TigDma)
    }
    #[doc = "tim8_com_dma selected"]
    #[inline(always)]
    pub fn tim8_com_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim8ComDma)
    }
    #[doc = "tim2_cc1_dma selected"]
    #[inline(always)]
    pub fn tim2_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim2Cc1Dma)
    }
    #[doc = "tim2_cc2_dma selected"]
    #[inline(always)]
    pub fn tim2_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim2Cc2Dma)
    }
    #[doc = "tim2_cc3_dma selected"]
    #[inline(always)]
    pub fn tim2_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim2Cc3Dma)
    }
    #[doc = "tim2_cc4_dma selected"]
    #[inline(always)]
    pub fn tim2_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim2Cc4Dma)
    }
    #[doc = "tim2_upd_dma selected"]
    #[inline(always)]
    pub fn tim2_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim2UpdDma)
    }
    #[doc = "tim3_cc1_dma selected"]
    #[inline(always)]
    pub fn tim3_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3Cc1Dma)
    }
    #[doc = "tim3_cc2_dma selected"]
    #[inline(always)]
    pub fn tim3_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3Cc2Dma)
    }
    #[doc = "tim3_cc3_dma selected"]
    #[inline(always)]
    pub fn tim3_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3Cc3Dma)
    }
    #[doc = "tim3_cc4_dma selected"]
    #[inline(always)]
    pub fn tim3_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3Cc4Dma)
    }
    #[doc = "tim3_upd_dma selected"]
    #[inline(always)]
    pub fn tim3_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3UpdDma)
    }
    #[doc = "tim3_trg_dma selected"]
    #[inline(always)]
    pub fn tim3_trg_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim3TrgDma)
    }
    #[doc = "tim4_cc1_dma selected"]
    #[inline(always)]
    pub fn tim4_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim4Cc1Dma)
    }
    #[doc = "tim4_cc2_dma selected"]
    #[inline(always)]
    pub fn tim4_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim4Cc2Dma)
    }
    #[doc = "tim4_cc3_dma selected"]
    #[inline(always)]
    pub fn tim4_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim4Cc3Dma)
    }
    #[doc = "tim4_cc4_dma selected"]
    #[inline(always)]
    pub fn tim4_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim4Cc4Dma)
    }
    #[doc = "tim4_upd_dma selected"]
    #[inline(always)]
    pub fn tim4_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim4UpdDma)
    }
    #[doc = "tim5_cc1_dma selected"]
    #[inline(always)]
    pub fn tim5_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5Cc1Dma)
    }
    #[doc = "tim5_cc2_dma selected"]
    #[inline(always)]
    pub fn tim5_cc2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5Cc2Dma)
    }
    #[doc = "tim5_cc3_dma selected"]
    #[inline(always)]
    pub fn tim5_cc3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5Cc3Dma)
    }
    #[doc = "tim5_cc4_dma selected"]
    #[inline(always)]
    pub fn tim5_cc4_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5Cc4Dma)
    }
    #[doc = "tim5_upd_dma selected"]
    #[inline(always)]
    pub fn tim5_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5UpdDma)
    }
    #[doc = "tim5_trg_dma selected"]
    #[inline(always)]
    pub fn tim5_trg_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim5TrgDma)
    }
    #[doc = "tim15_cc1_dma selected"]
    #[inline(always)]
    pub fn tim15_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim15Cc1Dma)
    }
    #[doc = "tim15_upd_dma selected"]
    #[inline(always)]
    pub fn tim15_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim15UpdDma)
    }
    #[doc = "tim15_trg_dma selected"]
    #[inline(always)]
    pub fn tim15_trg_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim15TrgDma)
    }
    #[doc = "tim15_com_dma selected"]
    #[inline(always)]
    pub fn tim15_com_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim15ComDma)
    }
    #[doc = "tim16_cc1_dma selected"]
    #[inline(always)]
    pub fn tim16_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim16Cc1Dma)
    }
    #[doc = "tim16_upd_dma selected"]
    #[inline(always)]
    pub fn tim16_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim16UpdDma)
    }
    #[doc = "tim17_cc1_dma selected"]
    #[inline(always)]
    pub fn tim17_cc1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim17Cc1Dma)
    }
    #[doc = "tim17_upd_dma selected"]
    #[inline(always)]
    pub fn tim17_upd_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Tim17UpdDma)
    }
    #[doc = "lptim1_ic1_dma selected"]
    #[inline(always)]
    pub fn lptim1_ic1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim1Ic1Dma)
    }
    #[doc = "lptim1_ic2_dma selected"]
    #[inline(always)]
    pub fn lptim1_ic2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim1Ic2Dma)
    }
    #[doc = "lptim1_ue_dma selected"]
    #[inline(always)]
    pub fn lptim1_ue_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim1UeDma)
    }
    #[doc = "lptim2_ic1_dma selected"]
    #[inline(always)]
    pub fn lptim2_ic1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim2Ic1Dma)
    }
    #[doc = "lptim2_ic2_dma selected"]
    #[inline(always)]
    pub fn lptim2_ic2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim2Ic2Dma)
    }
    #[doc = "lptim2_ue_dma selected"]
    #[inline(always)]
    pub fn lptim2_ue_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim2UeDma)
    }
    #[doc = "dcmi_dma or pssi_dma(1) selected"]
    #[inline(always)]
    pub fn dcmi_pssi_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::DcmiPssiDma)
    }
    #[doc = "aes_out_dma selected"]
    #[inline(always)]
    pub fn aes_out_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::AesOutDma)
    }
    #[doc = "aes_in_dma selected"]
    #[inline(always)]
    pub fn aes_in_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::AesInDma)
    }
    #[doc = "hash_in_dma selected"]
    #[inline(always)]
    pub fn hash_in_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::HashInDma)
    }
    #[doc = "ucpd1_rx_dma selected"]
    #[inline(always)]
    pub fn ucpd1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Ucpd1RxDma)
    }
    #[doc = "ucpd1_tx_dma selected"]
    #[inline(always)]
    pub fn ucpd1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Ucpd1TxDma)
    }
    #[doc = "cordic_read_dma selected"]
    #[inline(always)]
    pub fn cordic_read_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::CordicReadDma)
    }
    #[doc = "cordic_write_dma selected"]
    #[inline(always)]
    pub fn cordic_write_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::CordicWriteDma)
    }
    #[doc = "fmac_read_dma selected"]
    #[inline(always)]
    pub fn fmac_read_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::FmacReadDma)
    }
    #[doc = "fmac_write_dma selected"]
    #[inline(always)]
    pub fn fmac_write_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::FmacWriteDma)
    }
    #[doc = "saes_out_dma selected"]
    #[inline(always)]
    pub fn saes_out_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::SaesOutDma)
    }
    #[doc = "saes_in_dma selected"]
    #[inline(always)]
    pub fn saes_in_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::SaesInDma)
    }
    #[doc = "i3c1_rx_dma selected"]
    #[inline(always)]
    pub fn i3c1_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c1RxDma)
    }
    #[doc = "i3c1_tx_dma selected"]
    #[inline(always)]
    pub fn i3c1_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c1TxDma)
    }
    #[doc = "i3c1_tc_dma selected"]
    #[inline(always)]
    pub fn i3c1_tc_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c1TcDma)
    }
    #[doc = "i3c1_rs_dma selected"]
    #[inline(always)]
    pub fn i3c1_rs_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c1RsDma)
    }
    #[doc = "i2c4_rx_dma selected"]
    #[inline(always)]
    pub fn i2c4_rx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c4RxDma)
    }
    #[doc = "i2c4_tx_dma selected"]
    #[inline(always)]
    pub fn i2c4_tx_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I2c4TxDma)
    }
    #[doc = "lptim3_ic1_dma selected"]
    #[inline(always)]
    pub fn lptim3_ic1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim3Ic1Dma)
    }
    #[doc = "lptim3_ic2_dma selected"]
    #[inline(always)]
    pub fn lptim3_ic2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim3Ic2Dma)
    }
    #[doc = "lptim3_ue_dma selected"]
    #[inline(always)]
    pub fn lptim3_ue_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim3UeDma)
    }
    #[doc = "lptim5_ic1_dma selected"]
    #[inline(always)]
    pub fn lptim5_ic1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim5Ic1Dma)
    }
    #[doc = "lptim5_ic2_dma selected"]
    #[inline(always)]
    pub fn lptim5_ic2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim5Ic2Dma)
    }
    #[doc = "lptim5_ue_dma selected"]
    #[inline(always)]
    pub fn lptim5_ue_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim5UeDma)
    }
    #[doc = "lptim6_ic1_dma selected"]
    #[inline(always)]
    pub fn lptim6_ic1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim6Ic1Dma)
    }
    #[doc = "lptim6_ic2_dma selected"]
    #[inline(always)]
    pub fn lptim6_ic2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim6Ic2Dma)
    }
    #[doc = "lptim6_ue_dma selected"]
    #[inline(always)]
    pub fn lptim6_ue_dma(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::Lptim6UeDma)
    }
    #[doc = "i3c2_rx selected"]
    #[inline(always)]
    pub fn i3c2_rx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c2Rx)
    }
    #[doc = "i3c2_tx selected"]
    #[inline(always)]
    pub fn i3c2_tx(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c2Tx)
    }
    #[doc = "i3c2_tc selected"]
    #[inline(always)]
    pub fn i3c2_tc(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c2Tc)
    }
    #[doc = "i3c2_rs selected"]
    #[inline(always)]
    pub fn i3c2_rs(self) -> &'a mut crate::W<REG> {
        self.variant(REQSEL::I3c2Rs)
    }
}
#[doc = "software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWREQ {
    #[doc = "0: No software request. The selected hardware request REQSEL\\[7:0\\]
is taken into account"]
    Hardware = 0,
    #[doc = "1: Software request for memory-to-memory transfer"]
    Software = 1,
}
impl From<SWREQ> for bool {
    #[inline(always)]
    fn from(variant: SWREQ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWREQ` reader - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
pub type SWREQ_R = crate::BitReader<SWREQ>;
impl SWREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWREQ {
        match self.bits {
            false => SWREQ::Hardware,
            true => SWREQ::Software,
        }
    }
    #[doc = "No software request. The selected hardware request REQSEL\\[7:0\\]
is taken into account"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == SWREQ::Hardware
    }
    #[doc = "Software request for memory-to-memory transfer"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == SWREQ::Software
    }
}
#[doc = "Field `SWREQ` writer - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
pub type SWREQ_W<'a, REG> = crate::BitWriter<'a, REG, SWREQ>;
impl<'a, REG> SWREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No software request. The selected hardware request REQSEL\\[7:0\\]
is taken into account"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQ::Hardware)
    }
    #[doc = "Software request for memory-to-memory transfer"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(SWREQ::Software)
    }
}
#[doc = "destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DREQ {
    #[doc = "0: Selected hardware request driven by a source peripheral"]
    Source = 0,
    #[doc = "1: Selected hardware request driven by a destination peripheral"]
    Destination = 1,
}
impl From<DREQ> for bool {
    #[inline(always)]
    fn from(variant: DREQ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DREQ` reader - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
pub type DREQ_R = crate::BitReader<DREQ>;
impl DREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DREQ {
        match self.bits {
            false => DREQ::Source,
            true => DREQ::Destination,
        }
    }
    #[doc = "Selected hardware request driven by a source peripheral"]
    #[inline(always)]
    pub fn is_source(&self) -> bool {
        *self == DREQ::Source
    }
    #[doc = "Selected hardware request driven by a destination peripheral"]
    #[inline(always)]
    pub fn is_destination(&self) -> bool {
        *self == DREQ::Destination
    }
}
#[doc = "Field `DREQ` writer - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
pub type DREQ_W<'a, REG> = crate::BitWriter<'a, REG, DREQ>;
impl<'a, REG> DREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selected hardware request driven by a source peripheral"]
    #[inline(always)]
    pub fn source(self) -> &'a mut crate::W<REG> {
        self.variant(DREQ::Source)
    }
    #[doc = "Selected hardware request driven by a destination peripheral"]
    #[inline(always)]
    pub fn destination(self) -> &'a mut crate::W<REG> {
        self.variant(DREQ::Destination)
    }
}
#[doc = "Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREQ {
    #[doc = "0: The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level"]
    Burst = 0,
    #[doc = "1: The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level"]
    Block = 1,
}
impl From<BREQ> for bool {
    #[inline(always)]
    fn from(variant: BREQ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BREQ` reader - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
pub type BREQ_R = crate::BitReader<BREQ>;
impl BREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BREQ {
        match self.bits {
            false => BREQ::Burst,
            true => BREQ::Block,
        }
    }
    #[doc = "The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == BREQ::Burst
    }
    #[doc = "The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == BREQ::Block
    }
}
#[doc = "Field `BREQ` writer - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
pub type BREQ_W<'a, REG> = crate::BitWriter<'a, REG, BREQ>;
impl<'a, REG> BREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(BREQ::Burst)
    }
    #[doc = "The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(BREQ::Block)
    }
}
#[doc = "Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\]
must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width > source data width, GPDMA_CxTR1.PAM\\[1\\]
must be set to 0). Note: - GPDMA_CxBR1.BNDT\\[15:0\\]
must be programmed as a multiple of the source (peripheral) burst size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFREQ {
    #[doc = "0: The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in GPDMA control mode"]
    GpdmaControlMode = 0,
    #[doc = "1: The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in peripheral control mode."]
    PeripheralControlMode = 1,
}
impl From<PFREQ> for bool {
    #[inline(always)]
    fn from(variant: PFREQ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFREQ` reader - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\]
must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width > source data width, GPDMA_CxTR1.PAM\\[1\\]
must be set to 0). Note: - GPDMA_CxBR1.BNDT\\[15:0\\]
must be programmed as a multiple of the source (peripheral) burst size."]
pub type PFREQ_R = crate::BitReader<PFREQ>;
impl PFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PFREQ {
        match self.bits {
            false => PFREQ::GpdmaControlMode,
            true => PFREQ::PeripheralControlMode,
        }
    }
    #[doc = "The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in GPDMA control mode"]
    #[inline(always)]
    pub fn is_gpdma_control_mode(&self) -> bool {
        *self == PFREQ::GpdmaControlMode
    }
    #[doc = "The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in peripheral control mode."]
    #[inline(always)]
    pub fn is_peripheral_control_mode(&self) -> bool {
        *self == PFREQ::PeripheralControlMode
    }
}
#[doc = "Field `PFREQ` writer - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\]
must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width > source data width, GPDMA_CxTR1.PAM\\[1\\]
must be set to 0). Note: - GPDMA_CxBR1.BNDT\\[15:0\\]
must be programmed as a multiple of the source (peripheral) burst size."]
pub type PFREQ_W<'a, REG> = crate::BitWriter<'a, REG, PFREQ>;
impl<'a, REG> PFREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in GPDMA control mode"]
    #[inline(always)]
    pub fn gpdma_control_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PFREQ::GpdmaControlMode)
    }
    #[doc = "The selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol in peripheral control mode."]
    #[inline(always)]
    pub fn peripheral_control_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PFREQ::PeripheralControlMode)
    }
}
#[doc = "trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLI&lt;sub>n+1&lt;/sub> that updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLI&lt;sub>n &lt;/sub>trigger. After a first new trigger hit&lt;sub>n+1&lt;/sub> is memorized, if another second trigger hit&lt;sub>n+2&lt;/sub> is detected and if the hit&lt;sub>n&lt;/sub> triggered transfer is still not completed, hit&lt;sub>n+2 &lt;/sub>is lost and not memorized.memorized. A trigger overrun flag is reported (GPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGM {
    #[doc = "0: At block level: the first burst read of each block transfer is conditioned by one hit trigger"]
    BlockLevel = 0,
    #[doc = "2: At link level: a LLI link transfer is conditioned by one hit trigger"]
    LinkLevel = 2,
    #[doc = "3: At programmed burst level: programmed burst read is conditioned by one hit trigger."]
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
#[doc = "Field `TRIGM` reader - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLI&lt;sub>n+1&lt;/sub> that updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLI&lt;sub>n &lt;/sub>trigger. After a first new trigger hit&lt;sub>n+1&lt;/sub> is memorized, if another second trigger hit&lt;sub>n+2&lt;/sub> is detected and if the hit&lt;sub>n&lt;/sub> triggered transfer is still not completed, hit&lt;sub>n+2 &lt;/sub>is lost and not memorized.memorized. A trigger overrun flag is reported (GPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger."]
pub type TRIGM_R = crate::FieldReader<TRIGM>;
impl TRIGM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRIGM> {
        match self.bits {
            0 => Some(TRIGM::BlockLevel),
            2 => Some(TRIGM::LinkLevel),
            3 => Some(TRIGM::ProgrammedBurstLevel),
            _ => None,
        }
    }
    #[doc = "At block level: the first burst read of each block transfer is conditioned by one hit trigger"]
    #[inline(always)]
    pub fn is_block_level(&self) -> bool {
        *self == TRIGM::BlockLevel
    }
    #[doc = "At link level: a LLI link transfer is conditioned by one hit trigger"]
    #[inline(always)]
    pub fn is_link_level(&self) -> bool {
        *self == TRIGM::LinkLevel
    }
    #[doc = "At programmed burst level: programmed burst read is conditioned by one hit trigger."]
    #[inline(always)]
    pub fn is_programmed_burst_level(&self) -> bool {
        *self == TRIGM::ProgrammedBurstLevel
    }
}
#[doc = "Field `TRIGM` writer - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLI&lt;sub>n+1&lt;/sub> that updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLI&lt;sub>n &lt;/sub>trigger. After a first new trigger hit&lt;sub>n+1&lt;/sub> is memorized, if another second trigger hit&lt;sub>n+2&lt;/sub> is detected and if the hit&lt;sub>n&lt;/sub> triggered transfer is still not completed, hit&lt;sub>n+2 &lt;/sub>is lost and not memorized.memorized. A trigger overrun flag is reported (GPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger."]
pub type TRIGM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRIGM>;
impl<'a, REG> TRIGM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "At block level: the first burst read of each block transfer is conditioned by one hit trigger"]
    #[inline(always)]
    pub fn block_level(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGM::BlockLevel)
    }
    #[doc = "At link level: a LLI link transfer is conditioned by one hit trigger"]
    #[inline(always)]
    pub fn link_level(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGM::LinkLevel)
    }
    #[doc = "At programmed burst level: programmed burst read is conditioned by one hit trigger."]
    #[inline(always)]
    pub fn programmed_burst_level(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGM::ProgrammedBurstLevel)
    }
}
#[doc = "trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per Section 14.3.7), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSEL {
    #[doc = "0: exti0 is trigger input"]
    Exti0 = 0,
    #[doc = "1: exti1 is trigger input"]
    Exti1 = 1,
    #[doc = "2: exti2 is trigger input"]
    Exti2 = 2,
    #[doc = "3: exti3 is trigger input"]
    Exti3 = 3,
    #[doc = "4: exti4 is trigger input"]
    Exti4 = 4,
    #[doc = "5: exti5 is trigger input"]
    Exti5 = 5,
    #[doc = "6: exti6 is trigger input"]
    Exti6 = 6,
    #[doc = "7: exti7 is trigger input"]
    Exti7 = 7,
    #[doc = "8: tamp_trg1 is trigger input"]
    TampTrg1 = 8,
    #[doc = "9: tamp_trg2 is trigger input"]
    TampTrg2 = 9,
    #[doc = "11: lptim1_ch1 is trigger input"]
    Lptim1Ch1 = 11,
    #[doc = "12: lptim1_ch2 is trigger input"]
    Lptim1Ch2 = 12,
    #[doc = "13: lptim2_ch1 is trigger input"]
    Lptim2Ch1 = 13,
    #[doc = "14: lptim2_ch2 is trigger input"]
    Lptim2Ch2 = 14,
    #[doc = "15: rtc_alra_trg is trigger input"]
    RtcAlraTrg = 15,
    #[doc = "16: rtc_alrb_trg is trigger input"]
    RtcAlrbTrg = 16,
    #[doc = "17: rtc_wut_trg is trigger input"]
    RtcWutTrg = 17,
    #[doc = "18: gpdma1_ch0_tc is trigger input"]
    Gpdma1Ch0Tc = 18,
    #[doc = "19: gpdma1_ch1_tc is trigger input"]
    Gpdma1Ch1Tc = 19,
    #[doc = "20: gpdma1_ch2_tc is trigger input"]
    Gpdma1Ch2Tc = 20,
    #[doc = "21: gpdma1_ch3_tc is trigger input"]
    Gpdma1Ch3Tc = 21,
    #[doc = "22: gpdma1_ch4_tc is trigger input"]
    Gpdma1Ch4Tc = 22,
    #[doc = "23: gpdma1_ch5_tc is trigger input"]
    Gpdma1Ch5Tc = 23,
    #[doc = "24: gpdma1_ch6_tc is trigger input"]
    Gpdma1Ch6Tc = 24,
    #[doc = "25: gpdma1_ch7_tc is trigger input"]
    Gpdma1Ch7Tc = 25,
    #[doc = "26: gpdma2_ch0_tc is trigger input"]
    Gpdma2Ch0Tc = 26,
    #[doc = "27: gpdma2_ch1_tc is trigger input"]
    Gpdma2Ch1Tc = 27,
    #[doc = "28: gpdma2_ch2_tc is trigger input"]
    Gpdma2Ch2Tc = 28,
    #[doc = "29: gpdma2_ch3_tc is trigger input"]
    Gpdma2Ch3Tc = 29,
    #[doc = "30: gpdma2_ch4_tc is trigger input"]
    Gpdma2Ch4Tc = 30,
    #[doc = "31: gpdma2_ch5_tc is trigger input"]
    Gpdma2Ch5Tc = 31,
    #[doc = "32: gpdma2_ch6_tc is trigger input"]
    Gpdma2Ch6Tc = 32,
    #[doc = "33: gpdma2_ch7_tc is trigger input"]
    Gpdma2Ch7Tc = 33,
    #[doc = "34: tim2_trgo is trigger input"]
    Tim2Trg0 = 34,
    #[doc = "44: comp1_out is trigger input"]
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
#[doc = "Field `TRIGSEL` reader - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per Section 14.3.7), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00."]
pub type TRIGSEL_R = crate::FieldReader<TRIGSEL>;
impl TRIGSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "exti0 is trigger input"]
    #[inline(always)]
    pub fn is_exti0(&self) -> bool {
        *self == TRIGSEL::Exti0
    }
    #[doc = "exti1 is trigger input"]
    #[inline(always)]
    pub fn is_exti1(&self) -> bool {
        *self == TRIGSEL::Exti1
    }
    #[doc = "exti2 is trigger input"]
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == TRIGSEL::Exti2
    }
    #[doc = "exti3 is trigger input"]
    #[inline(always)]
    pub fn is_exti3(&self) -> bool {
        *self == TRIGSEL::Exti3
    }
    #[doc = "exti4 is trigger input"]
    #[inline(always)]
    pub fn is_exti4(&self) -> bool {
        *self == TRIGSEL::Exti4
    }
    #[doc = "exti5 is trigger input"]
    #[inline(always)]
    pub fn is_exti5(&self) -> bool {
        *self == TRIGSEL::Exti5
    }
    #[doc = "exti6 is trigger input"]
    #[inline(always)]
    pub fn is_exti6(&self) -> bool {
        *self == TRIGSEL::Exti6
    }
    #[doc = "exti7 is trigger input"]
    #[inline(always)]
    pub fn is_exti7(&self) -> bool {
        *self == TRIGSEL::Exti7
    }
    #[doc = "tamp_trg1 is trigger input"]
    #[inline(always)]
    pub fn is_tamp_trg1(&self) -> bool {
        *self == TRIGSEL::TampTrg1
    }
    #[doc = "tamp_trg2 is trigger input"]
    #[inline(always)]
    pub fn is_tamp_trg2(&self) -> bool {
        *self == TRIGSEL::TampTrg2
    }
    #[doc = "lptim1_ch1 is trigger input"]
    #[inline(always)]
    pub fn is_lptim1_ch1(&self) -> bool {
        *self == TRIGSEL::Lptim1Ch1
    }
    #[doc = "lptim1_ch2 is trigger input"]
    #[inline(always)]
    pub fn is_lptim1_ch2(&self) -> bool {
        *self == TRIGSEL::Lptim1Ch2
    }
    #[doc = "lptim2_ch1 is trigger input"]
    #[inline(always)]
    pub fn is_lptim2_ch1(&self) -> bool {
        *self == TRIGSEL::Lptim2Ch1
    }
    #[doc = "lptim2_ch2 is trigger input"]
    #[inline(always)]
    pub fn is_lptim2_ch2(&self) -> bool {
        *self == TRIGSEL::Lptim2Ch2
    }
    #[doc = "rtc_alra_trg is trigger input"]
    #[inline(always)]
    pub fn is_rtc_alra_trg(&self) -> bool {
        *self == TRIGSEL::RtcAlraTrg
    }
    #[doc = "rtc_alrb_trg is trigger input"]
    #[inline(always)]
    pub fn is_rtc_alrb_trg(&self) -> bool {
        *self == TRIGSEL::RtcAlrbTrg
    }
    #[doc = "rtc_wut_trg is trigger input"]
    #[inline(always)]
    pub fn is_rtc_wut_trg(&self) -> bool {
        *self == TRIGSEL::RtcWutTrg
    }
    #[doc = "gpdma1_ch0_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma1_ch0_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch0Tc
    }
    #[doc = "gpdma1_ch1_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma1_ch1_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch1Tc
    }
    #[doc = "gpdma1_ch2_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma1_ch2_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch2Tc
    }
    #[doc = "gpdma1_ch3_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma1_ch3_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch3Tc
    }
    #[doc = "gpdma1_ch4_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma1_ch4_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch4Tc
    }
    #[doc = "gpdma1_ch5_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma1_ch5_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch5Tc
    }
    #[doc = "gpdma1_ch6_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma1_ch6_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch6Tc
    }
    #[doc = "gpdma1_ch7_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma1_ch7_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma1Ch7Tc
    }
    #[doc = "gpdma2_ch0_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma2_ch0_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch0Tc
    }
    #[doc = "gpdma2_ch1_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma2_ch1_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch1Tc
    }
    #[doc = "gpdma2_ch2_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma2_ch2_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch2Tc
    }
    #[doc = "gpdma2_ch3_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma2_ch3_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch3Tc
    }
    #[doc = "gpdma2_ch4_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma2_ch4_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch4Tc
    }
    #[doc = "gpdma2_ch5_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma2_ch5_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch5Tc
    }
    #[doc = "gpdma2_ch6_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma2_ch6_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch6Tc
    }
    #[doc = "gpdma2_ch7_tc is trigger input"]
    #[inline(always)]
    pub fn is_gpdma2_ch7_tc(&self) -> bool {
        *self == TRIGSEL::Gpdma2Ch7Tc
    }
    #[doc = "tim2_trgo is trigger input"]
    #[inline(always)]
    pub fn is_tim2_trg0(&self) -> bool {
        *self == TRIGSEL::Tim2Trg0
    }
    #[doc = "comp1_out is trigger input"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == TRIGSEL::Comp1Out
    }
}
#[doc = "Field `TRIGSEL` writer - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per Section 14.3.7), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00."]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6, TRIGSEL>;
impl<'a, REG> TRIGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "exti0 is trigger input"]
    #[inline(always)]
    pub fn exti0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti0)
    }
    #[doc = "exti1 is trigger input"]
    #[inline(always)]
    pub fn exti1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti1)
    }
    #[doc = "exti2 is trigger input"]
    #[inline(always)]
    pub fn exti2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti2)
    }
    #[doc = "exti3 is trigger input"]
    #[inline(always)]
    pub fn exti3(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti3)
    }
    #[doc = "exti4 is trigger input"]
    #[inline(always)]
    pub fn exti4(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti4)
    }
    #[doc = "exti5 is trigger input"]
    #[inline(always)]
    pub fn exti5(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti5)
    }
    #[doc = "exti6 is trigger input"]
    #[inline(always)]
    pub fn exti6(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti6)
    }
    #[doc = "exti7 is trigger input"]
    #[inline(always)]
    pub fn exti7(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Exti7)
    }
    #[doc = "tamp_trg1 is trigger input"]
    #[inline(always)]
    pub fn tamp_trg1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::TampTrg1)
    }
    #[doc = "tamp_trg2 is trigger input"]
    #[inline(always)]
    pub fn tamp_trg2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::TampTrg2)
    }
    #[doc = "lptim1_ch1 is trigger input"]
    #[inline(always)]
    pub fn lptim1_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Lptim1Ch1)
    }
    #[doc = "lptim1_ch2 is trigger input"]
    #[inline(always)]
    pub fn lptim1_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Lptim1Ch2)
    }
    #[doc = "lptim2_ch1 is trigger input"]
    #[inline(always)]
    pub fn lptim2_ch1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Lptim2Ch1)
    }
    #[doc = "lptim2_ch2 is trigger input"]
    #[inline(always)]
    pub fn lptim2_ch2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Lptim2Ch2)
    }
    #[doc = "rtc_alra_trg is trigger input"]
    #[inline(always)]
    pub fn rtc_alra_trg(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::RtcAlraTrg)
    }
    #[doc = "rtc_alrb_trg is trigger input"]
    #[inline(always)]
    pub fn rtc_alrb_trg(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::RtcAlrbTrg)
    }
    #[doc = "rtc_wut_trg is trigger input"]
    #[inline(always)]
    pub fn rtc_wut_trg(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::RtcWutTrg)
    }
    #[doc = "gpdma1_ch0_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma1_ch0_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch0Tc)
    }
    #[doc = "gpdma1_ch1_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma1_ch1_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch1Tc)
    }
    #[doc = "gpdma1_ch2_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma1_ch2_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch2Tc)
    }
    #[doc = "gpdma1_ch3_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma1_ch3_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch3Tc)
    }
    #[doc = "gpdma1_ch4_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma1_ch4_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch4Tc)
    }
    #[doc = "gpdma1_ch5_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma1_ch5_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch5Tc)
    }
    #[doc = "gpdma1_ch6_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma1_ch6_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch6Tc)
    }
    #[doc = "gpdma1_ch7_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma1_ch7_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma1Ch7Tc)
    }
    #[doc = "gpdma2_ch0_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma2_ch0_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch0Tc)
    }
    #[doc = "gpdma2_ch1_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma2_ch1_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch1Tc)
    }
    #[doc = "gpdma2_ch2_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma2_ch2_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch2Tc)
    }
    #[doc = "gpdma2_ch3_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma2_ch3_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch3Tc)
    }
    #[doc = "gpdma2_ch4_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma2_ch4_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch4Tc)
    }
    #[doc = "gpdma2_ch5_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma2_ch5_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch5Tc)
    }
    #[doc = "gpdma2_ch6_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma2_ch6_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch6Tc)
    }
    #[doc = "gpdma2_ch7_tc is trigger input"]
    #[inline(always)]
    pub fn gpdma2_ch7_tc(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Gpdma2Ch7Tc)
    }
    #[doc = "tim2_trgo is trigger input"]
    #[inline(always)]
    pub fn tim2_trg0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Tim2Trg0)
    }
    #[doc = "comp1_out is trigger input"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Comp1Out)
    }
}
#[doc = "trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGPOL {
    #[doc = "0: No trigger"]
    NoTrigger = 0,
    #[doc = "1: Trigger on rising edge"]
    RisingEdge = 1,
    #[doc = "2: Trigger on falling edge"]
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
#[doc = "Field `TRIGPOL` reader - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
pub type TRIGPOL_R = crate::FieldReader<TRIGPOL>;
impl TRIGPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRIGPOL> {
        match self.bits {
            0 => Some(TRIGPOL::NoTrigger),
            1 => Some(TRIGPOL::RisingEdge),
            2 => Some(TRIGPOL::FallingEdge),
            _ => None,
        }
    }
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == TRIGPOL::NoTrigger
    }
    #[doc = "Trigger on rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TRIGPOL::RisingEdge
    }
    #[doc = "Trigger on falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TRIGPOL::FallingEdge
    }
}
#[doc = "Field `TRIGPOL` writer - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
pub type TRIGPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRIGPOL>;
impl<'a, REG> TRIGPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL::NoTrigger)
    }
    #[doc = "Trigger on rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL::RisingEdge)
    }
    #[doc = "Trigger on falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGPOL::FallingEdge)
    }
}
#[doc = "transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI&lt;sub>1&lt;/sub>.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCEM {
    #[doc = "0: At block level: the complete (and the half) transfer event is generated at the (respectively half of the) end of a block"]
    BlockLevel = 0,
    #[doc = "2: At LLI level: the complete transfer event is generated at the end of the LLI transfer. The half transfer event is generated at the half of the LLI data transfer"]
    LliLevel = 2,
    #[doc = "3: At channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI"]
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
#[doc = "Field `TCEM` reader - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI&lt;sub>1&lt;/sub>."]
pub type TCEM_R = crate::FieldReader<TCEM>;
impl TCEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TCEM> {
        match self.bits {
            0 => Some(TCEM::BlockLevel),
            2 => Some(TCEM::LliLevel),
            3 => Some(TCEM::ChannelLevel),
            _ => None,
        }
    }
    #[doc = "At block level: the complete (and the half) transfer event is generated at the (respectively half of the) end of a block"]
    #[inline(always)]
    pub fn is_block_level(&self) -> bool {
        *self == TCEM::BlockLevel
    }
    #[doc = "At LLI level: the complete transfer event is generated at the end of the LLI transfer. The half transfer event is generated at the half of the LLI data transfer"]
    #[inline(always)]
    pub fn is_lli_level(&self) -> bool {
        *self == TCEM::LliLevel
    }
    #[doc = "At channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI"]
    #[inline(always)]
    pub fn is_channel_level(&self) -> bool {
        *self == TCEM::ChannelLevel
    }
}
#[doc = "Field `TCEM` writer - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI&lt;sub>1&lt;/sub>."]
pub type TCEM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TCEM>;
impl<'a, REG> TCEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "At block level: the complete (and the half) transfer event is generated at the (respectively half of the) end of a block"]
    #[inline(always)]
    pub fn block_level(self) -> &'a mut crate::W<REG> {
        self.variant(TCEM::BlockLevel)
    }
    #[doc = "At LLI level: the complete transfer event is generated at the end of the LLI transfer. The half transfer event is generated at the half of the LLI data transfer"]
    #[inline(always)]
    pub fn lli_level(self) -> &'a mut crate::W<REG> {
        self.variant(TCEM::LliLevel)
    }
    #[doc = "At channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI"]
    #[inline(always)]
    pub fn channel_level(self) -> &'a mut crate::W<REG> {
        self.variant(TCEM::ChannelLevel)
    }
}
impl R {
    #[doc = "Bits 0:7 - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 14.3.4. The user must not assign a same input hardware request (same REQSEL\\[7:0\\]
value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
    #[inline(always)]
    pub fn reqsel(&self) -> REQSEL_R {
        REQSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 9 - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
    #[inline(always)]
    pub fn breq(&self) -> BREQ_R {
        BREQ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\]
must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width > source data width, GPDMA_CxTR1.PAM\\[1\\]
must be set to 0). Note: - GPDMA_CxBR1.BNDT\\[15:0\\]
must be programmed as a multiple of the source (peripheral) burst size."]
    #[inline(always)]
    pub fn pfreq(&self) -> PFREQ_R {
        PFREQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLI&lt;sub>n+1&lt;/sub> that updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLI&lt;sub>n &lt;/sub>trigger. After a first new trigger hit&lt;sub>n+1&lt;/sub> is memorized, if another second trigger hit&lt;sub>n+2&lt;/sub> is detected and if the hit&lt;sub>n&lt;/sub> triggered transfer is still not completed, hit&lt;sub>n+2 &lt;/sub>is lost and not memorized.memorized. A trigger overrun flag is reported (GPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger."]
    #[inline(always)]
    pub fn trigm(&self) -> TRIGM_R {
        TRIGM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per Section 14.3.7), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00."]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI&lt;sub>1&lt;/sub>."]
    #[inline(always)]
    pub fn tcem(&self) -> TCEM_R {
        TCEM_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPDMA hardware request selection These bits are ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per Section 14.3.4. The user must not assign a same input hardware request (same REQSEL\\[7:0\\]
value) to different active GPDMA channels (GPDMA_CxCR.EN = 1 and GPDMA_CxTR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
    #[inline(always)]
    #[must_use]
    pub fn reqsel(&mut self) -> REQSEL_W<C1TR2rs> {
        REQSEL_W::new(self, 0)
    }
    #[doc = "Bit 9 - software request This bit is internally taken into account when GPDMA_CxCR.EN is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn swreq(&mut self) -> SWREQ_W<C1TR2rs> {
        SWREQ_W::new(self, 9)
    }
    #[doc = "Bit 10 - destination hardware request This bit is ignored if channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note: If the channel x is activated (GPDMA_CxCR.EN is asserted) with SWREQ = 0 and PFREQ = 1 (peripheral hardware request with peripheral flow-control mode), any software assertion to this DREQ bit is ignored: in peripheral flow-control mode, only a peripheral-to-memory transfer is supported."]
    #[inline(always)]
    #[must_use]
    pub fn dreq(&mut self) -> DREQ_W<C1TR2rs> {
        DREQ_W::new(self, 10)
    }
    #[doc = "Bit 11 - Block hardware request If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
    #[inline(always)]
    #[must_use]
    pub fn breq(&mut self) -> BREQ_W<C1TR2rs> {
        BREQ_W::new(self, 11)
    }
    #[doc = "Bit 12 - Hardware request in peripheral flow control mode Important: If a given channel x is not implemented with this feature, this bit is reserved and PFREQ is not present (see Section 14.3.2 for the list of the implemented channels with this feature. If the channel x is activated (GPDMA_CxCR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else: Note: In peripheral flow control mode, there are the following restrictions: Note: - no 2D/repeated block support (GPDMA_CxBR1.BRC\\[10:0\\]
must be set to 0) Note: - the peripheral must be set as the source of the transfer (DREQ = 0). Note: - data packing to a wider destination width is not supported (if destination width > source data width, GPDMA_CxTR1.PAM\\[1\\]
must be set to 0). Note: - GPDMA_CxBR1.BNDT\\[15:0\\]
must be programmed as a multiple of the source (peripheral) burst size."]
    #[inline(always)]
    #[must_use]
    pub fn pfreq(&mut self) -> PFREQ_W<C1TR2rs> {
        PFREQ_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - trigger mode These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (GPDMA_CxCR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLI&lt;sub>n+1&lt;/sub> that updates the GPDMA_CxTR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLI&lt;sub>n &lt;/sub>trigger. After a first new trigger hit&lt;sub>n+1&lt;/sub> is memorized, if another second trigger hit&lt;sub>n+2&lt;/sub> is detected and if the hit&lt;sub>n&lt;/sub> triggered transfer is still not completed, hit&lt;sub>n+2 &lt;/sub>is lost and not memorized.memorized. A trigger overrun flag is reported (GPDMA_CxSR.TOF =1 ), and an interrupt is generated if enabled (GPDMA_CxCR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. Note: When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger."]
    #[inline(always)]
    #[must_use]
    pub fn trigm(&mut self) -> TRIGM_W<C1TR2rs> {
        TRIGM_W::new(self, 14)
    }
    #[doc = "Bits 16:21 - trigger event input selection These bits select the trigger event input of the GPDMA transfer (as per Section 14.3.7), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00."]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<C1TR2rs> {
        TRIGSEL_W::new(self, 16)
    }
    #[doc = "Bits 24:25 - trigger event polarity These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<C1TR2rs> {
        TRIGPOL_W::new(self, 24)
    }
    #[doc = "Bits 30:31 - transfer complete event mode These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI&lt;sub>0 &lt;/sub>data transfer is null/void (i.e. directly programmed by the internal register file with GPDMA_CxBR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI&lt;sub>1&lt;/sub>."]
    #[inline(always)]
    #[must_use]
    pub fn tcem(&mut self) -> TCEM_W<C1TR2rs> {
        TCEM_W::new(self, 30)
    }
}
#[doc = "GPDMA channel 1 transfer register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1tr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1tr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1TR2rs;
impl crate::RegisterSpec for C1TR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1tr2::R`](R) reader structure"]
impl crate::Readable for C1TR2rs {}
#[doc = "`write(|w| ..)` method takes [`c1tr2::W`](W) writer structure"]
impl crate::Writable for C1TR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1TR2 to value 0"]
impl crate::Resettable for C1TR2rs {
    const RESET_VALUE: u32 = 0;
}
