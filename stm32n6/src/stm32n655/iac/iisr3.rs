///Register `IISR3` reader
pub type R = crate::R<IISR3rs>;
///Field `ILACIN96` reader - illegal access input 96
pub type ILACIN96_R = crate::BitReader;
///Field `ILACIN97` reader - illegal access input 97
pub type ILACIN97_R = crate::BitReader;
///Field `ILACIN98` reader - illegal access input 98
pub type ILACIN98_R = crate::BitReader;
///Field `ILACIN99` reader - illegal access input 99
pub type ILACIN99_R = crate::BitReader;
///Field `ILACIN100` reader - illegal access input 100
pub type ILACIN100_R = crate::BitReader;
///Field `ILACIN101` reader - illegal access input 101
pub type ILACIN101_R = crate::BitReader;
///Field `ILACIN102` reader - illegal access input 102
pub type ILACIN102_R = crate::BitReader;
///Field `ILACIN103` reader - illegal access input 103
pub type ILACIN103_R = crate::BitReader;
///Field `ILACIN104` reader - illegal access input 104
pub type ILACIN104_R = crate::BitReader;
///Field `ILACIN105` reader - illegal access input 105
pub type ILACIN105_R = crate::BitReader;
///Field `ILACIN106` reader - illegal access input 106
pub type ILACIN106_R = crate::BitReader;
///Field `ILACIN107` reader - illegal access input 107
pub type ILACIN107_R = crate::BitReader;
///Field `ILACIN108` reader - illegal access input 108
pub type ILACIN108_R = crate::BitReader;
///Field `ILACIN109` reader - illegal access input 109
pub type ILACIN109_R = crate::BitReader;
///Field `ILACIN110` reader - illegal access input 110
pub type ILACIN110_R = crate::BitReader;
///Field `ILACIN111` reader - illegal access input 111
pub type ILACIN111_R = crate::BitReader;
///Field `ILACIN112` reader - illegal access input 112
pub type ILACIN112_R = crate::BitReader;
///Field `ILACIN113` reader - illegal access input 113
pub type ILACIN113_R = crate::BitReader;
///Field `ILACIN114` reader - illegal access input 114
pub type ILACIN114_R = crate::BitReader;
///Field `ILACIN115` reader - illegal access input 115
pub type ILACIN115_R = crate::BitReader;
///Field `ILACIN116` reader - illegal access input 116
pub type ILACIN116_R = crate::BitReader;
///Field `ILACIN117` reader - illegal access input 117
pub type ILACIN117_R = crate::BitReader;
///Field `ILACIN118` reader - illegal access input 118
pub type ILACIN118_R = crate::BitReader;
///Field `ILACIN119` reader - illegal access input 119
pub type ILACIN119_R = crate::BitReader;
///Field `ILACIN120` reader - illegal access input 120
pub type ILACIN120_R = crate::BitReader;
///Field `ILACIN121` reader - illegal access input 121
pub type ILACIN121_R = crate::BitReader;
///Field `ILACIN122` reader - illegal access input 122
pub type ILACIN122_R = crate::BitReader;
///Field `ILACIN123` reader - illegal access input 123
pub type ILACIN123_R = crate::BitReader;
///Field `ILACIN124` reader - illegal access input 124
pub type ILACIN124_R = crate::BitReader;
///Field `ILACIN125` reader - illegal access input 125
pub type ILACIN125_R = crate::BitReader;
///Field `ILACIN126` reader - illegal access input 126
pub type ILACIN126_R = crate::BitReader;
///Field `ILACIN127` reader - illegal access input 127
pub type ILACIN127_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access input 96
    #[inline(always)]
    pub fn ilacin96(&self) -> ILACIN96_R {
        ILACIN96_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access input 97
    #[inline(always)]
    pub fn ilacin97(&self) -> ILACIN97_R {
        ILACIN97_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access input 98
    #[inline(always)]
    pub fn ilacin98(&self) -> ILACIN98_R {
        ILACIN98_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access input 99
    #[inline(always)]
    pub fn ilacin99(&self) -> ILACIN99_R {
        ILACIN99_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access input 100
    #[inline(always)]
    pub fn ilacin100(&self) -> ILACIN100_R {
        ILACIN100_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access input 101
    #[inline(always)]
    pub fn ilacin101(&self) -> ILACIN101_R {
        ILACIN101_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access input 102
    #[inline(always)]
    pub fn ilacin102(&self) -> ILACIN102_R {
        ILACIN102_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access input 103
    #[inline(always)]
    pub fn ilacin103(&self) -> ILACIN103_R {
        ILACIN103_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access input 104
    #[inline(always)]
    pub fn ilacin104(&self) -> ILACIN104_R {
        ILACIN104_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access input 105
    #[inline(always)]
    pub fn ilacin105(&self) -> ILACIN105_R {
        ILACIN105_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access input 106
    #[inline(always)]
    pub fn ilacin106(&self) -> ILACIN106_R {
        ILACIN106_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access input 107
    #[inline(always)]
    pub fn ilacin107(&self) -> ILACIN107_R {
        ILACIN107_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access input 108
    #[inline(always)]
    pub fn ilacin108(&self) -> ILACIN108_R {
        ILACIN108_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access input 109
    #[inline(always)]
    pub fn ilacin109(&self) -> ILACIN109_R {
        ILACIN109_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access input 110
    #[inline(always)]
    pub fn ilacin110(&self) -> ILACIN110_R {
        ILACIN110_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access input 111
    #[inline(always)]
    pub fn ilacin111(&self) -> ILACIN111_R {
        ILACIN111_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access input 112
    #[inline(always)]
    pub fn ilacin112(&self) -> ILACIN112_R {
        ILACIN112_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access input 113
    #[inline(always)]
    pub fn ilacin113(&self) -> ILACIN113_R {
        ILACIN113_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access input 114
    #[inline(always)]
    pub fn ilacin114(&self) -> ILACIN114_R {
        ILACIN114_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access input 115
    #[inline(always)]
    pub fn ilacin115(&self) -> ILACIN115_R {
        ILACIN115_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access input 116
    #[inline(always)]
    pub fn ilacin116(&self) -> ILACIN116_R {
        ILACIN116_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access input 117
    #[inline(always)]
    pub fn ilacin117(&self) -> ILACIN117_R {
        ILACIN117_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access input 118
    #[inline(always)]
    pub fn ilacin118(&self) -> ILACIN118_R {
        ILACIN118_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access input 119
    #[inline(always)]
    pub fn ilacin119(&self) -> ILACIN119_R {
        ILACIN119_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access input 120
    #[inline(always)]
    pub fn ilacin120(&self) -> ILACIN120_R {
        ILACIN120_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access input 121
    #[inline(always)]
    pub fn ilacin121(&self) -> ILACIN121_R {
        ILACIN121_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access input 122
    #[inline(always)]
    pub fn ilacin122(&self) -> ILACIN122_R {
        ILACIN122_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access input 123
    #[inline(always)]
    pub fn ilacin123(&self) -> ILACIN123_R {
        ILACIN123_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access input 124
    #[inline(always)]
    pub fn ilacin124(&self) -> ILACIN124_R {
        ILACIN124_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access input 125
    #[inline(always)]
    pub fn ilacin125(&self) -> ILACIN125_R {
        ILACIN125_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access input 126
    #[inline(always)]
    pub fn ilacin126(&self) -> ILACIN126_R {
        ILACIN126_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access input 127
    #[inline(always)]
    pub fn ilacin127(&self) -> ILACIN127_R {
        ILACIN127_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IISR3")
            .field("ilacin96", &self.ilacin96())
            .field("ilacin97", &self.ilacin97())
            .field("ilacin98", &self.ilacin98())
            .field("ilacin99", &self.ilacin99())
            .field("ilacin100", &self.ilacin100())
            .field("ilacin101", &self.ilacin101())
            .field("ilacin102", &self.ilacin102())
            .field("ilacin103", &self.ilacin103())
            .field("ilacin104", &self.ilacin104())
            .field("ilacin105", &self.ilacin105())
            .field("ilacin106", &self.ilacin106())
            .field("ilacin107", &self.ilacin107())
            .field("ilacin108", &self.ilacin108())
            .field("ilacin109", &self.ilacin109())
            .field("ilacin110", &self.ilacin110())
            .field("ilacin111", &self.ilacin111())
            .field("ilacin112", &self.ilacin112())
            .field("ilacin113", &self.ilacin113())
            .field("ilacin114", &self.ilacin114())
            .field("ilacin115", &self.ilacin115())
            .field("ilacin116", &self.ilacin116())
            .field("ilacin117", &self.ilacin117())
            .field("ilacin118", &self.ilacin118())
            .field("ilacin119", &self.ilacin119())
            .field("ilacin120", &self.ilacin120())
            .field("ilacin121", &self.ilacin121())
            .field("ilacin122", &self.ilacin122())
            .field("ilacin123", &self.ilacin123())
            .field("ilacin124", &self.ilacin124())
            .field("ilacin125", &self.ilacin125())
            .field("ilacin126", &self.ilacin126())
            .field("ilacin127", &self.ilacin127())
            .finish()
    }
}
/**IAC ILAC input status register 3

You can [`read`](crate::Reg::read) this register and get [`iisr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#IAC:IISR3)*/
pub struct IISR3rs;
impl crate::RegisterSpec for IISR3rs {
    type Ux = u32;
}
///`read()` method returns [`iisr3::R`](R) reader structure
impl crate::Readable for IISR3rs {}
///`reset()` method sets IISR3 to value 0x05ff
impl crate::Resettable for IISR3rs {
    const RESET_VALUE: u32 = 0x05ff;
}
