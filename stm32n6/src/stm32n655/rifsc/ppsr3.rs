///Register `PPSR3` reader
pub type R = crate::R<PPSR3rs>;
///Field `PPEN96` reader - peripheral protection enable 96
pub type PPEN96_R = crate::BitReader;
///Field `PPEN97` reader - peripheral protection enable 97
pub type PPEN97_R = crate::BitReader;
///Field `PPEN98` reader - peripheral protection enable 98
pub type PPEN98_R = crate::BitReader;
///Field `PPEN99` reader - peripheral protection enable 99
pub type PPEN99_R = crate::BitReader;
///Field `PPEN100` reader - peripheral protection enable 100
pub type PPEN100_R = crate::BitReader;
///Field `PPEN101` reader - peripheral protection enable 101
pub type PPEN101_R = crate::BitReader;
///Field `PPEN102` reader - peripheral protection enable 102
pub type PPEN102_R = crate::BitReader;
///Field `PPEN103` reader - peripheral protection enable 103
pub type PPEN103_R = crate::BitReader;
///Field `PPEN104` reader - peripheral protection enable 104
pub type PPEN104_R = crate::BitReader;
///Field `PPEN105` reader - peripheral protection enable 105
pub type PPEN105_R = crate::BitReader;
///Field `PPEN106` reader - peripheral protection enable 106
pub type PPEN106_R = crate::BitReader;
///Field `PPEN107` reader - peripheral protection enable 107
pub type PPEN107_R = crate::BitReader;
///Field `PPEN108` reader - peripheral protection enable 108
pub type PPEN108_R = crate::BitReader;
///Field `PPEN109` reader - peripheral protection enable 109
pub type PPEN109_R = crate::BitReader;
///Field `PPEN110` reader - peripheral protection enable 110
pub type PPEN110_R = crate::BitReader;
///Field `PPEN111` reader - peripheral protection enable 111
pub type PPEN111_R = crate::BitReader;
///Field `PPEN112` reader - peripheral protection enable 112
pub type PPEN112_R = crate::BitReader;
///Field `PPEN113` reader - peripheral protection enable 113
pub type PPEN113_R = crate::BitReader;
///Field `PPEN114` reader - peripheral protection enable 114
pub type PPEN114_R = crate::BitReader;
///Field `PPEN115` reader - peripheral protection enable 115
pub type PPEN115_R = crate::BitReader;
///Field `PPEN116` reader - peripheral protection enable 116
pub type PPEN116_R = crate::BitReader;
///Field `PPEN117` reader - peripheral protection enable 117
pub type PPEN117_R = crate::BitReader;
///Field `PPEN118` reader - peripheral protection enable 118
pub type PPEN118_R = crate::BitReader;
///Field `PPEN119` reader - peripheral protection enable 119
pub type PPEN119_R = crate::BitReader;
///Field `PPEN120` reader - peripheral protection enable 120
pub type PPEN120_R = crate::BitReader;
///Field `PPEN121` reader - peripheral protection enable 121
pub type PPEN121_R = crate::BitReader;
///Field `PPEN122` reader - peripheral protection enable 122
pub type PPEN122_R = crate::BitReader;
///Field `PPEN123` reader - peripheral protection enable 123
pub type PPEN123_R = crate::BitReader;
///Field `PPEN124` reader - peripheral protection enable 124
pub type PPEN124_R = crate::BitReader;
///Field `PPEN125` reader - peripheral protection enable 125
pub type PPEN125_R = crate::BitReader;
///Field `PPEN126` reader - peripheral protection enable 126
pub type PPEN126_R = crate::BitReader;
///Field `PPEN127` reader - peripheral protection enable 127
pub type PPEN127_R = crate::BitReader;
impl R {
    ///Bit 0 - peripheral protection enable 96
    #[inline(always)]
    pub fn ppen96(&self) -> PPEN96_R {
        PPEN96_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - peripheral protection enable 97
    #[inline(always)]
    pub fn ppen97(&self) -> PPEN97_R {
        PPEN97_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - peripheral protection enable 98
    #[inline(always)]
    pub fn ppen98(&self) -> PPEN98_R {
        PPEN98_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - peripheral protection enable 99
    #[inline(always)]
    pub fn ppen99(&self) -> PPEN99_R {
        PPEN99_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - peripheral protection enable 100
    #[inline(always)]
    pub fn ppen100(&self) -> PPEN100_R {
        PPEN100_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - peripheral protection enable 101
    #[inline(always)]
    pub fn ppen101(&self) -> PPEN101_R {
        PPEN101_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - peripheral protection enable 102
    #[inline(always)]
    pub fn ppen102(&self) -> PPEN102_R {
        PPEN102_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - peripheral protection enable 103
    #[inline(always)]
    pub fn ppen103(&self) -> PPEN103_R {
        PPEN103_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - peripheral protection enable 104
    #[inline(always)]
    pub fn ppen104(&self) -> PPEN104_R {
        PPEN104_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - peripheral protection enable 105
    #[inline(always)]
    pub fn ppen105(&self) -> PPEN105_R {
        PPEN105_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - peripheral protection enable 106
    #[inline(always)]
    pub fn ppen106(&self) -> PPEN106_R {
        PPEN106_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - peripheral protection enable 107
    #[inline(always)]
    pub fn ppen107(&self) -> PPEN107_R {
        PPEN107_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - peripheral protection enable 108
    #[inline(always)]
    pub fn ppen108(&self) -> PPEN108_R {
        PPEN108_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - peripheral protection enable 109
    #[inline(always)]
    pub fn ppen109(&self) -> PPEN109_R {
        PPEN109_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - peripheral protection enable 110
    #[inline(always)]
    pub fn ppen110(&self) -> PPEN110_R {
        PPEN110_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - peripheral protection enable 111
    #[inline(always)]
    pub fn ppen111(&self) -> PPEN111_R {
        PPEN111_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - peripheral protection enable 112
    #[inline(always)]
    pub fn ppen112(&self) -> PPEN112_R {
        PPEN112_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - peripheral protection enable 113
    #[inline(always)]
    pub fn ppen113(&self) -> PPEN113_R {
        PPEN113_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - peripheral protection enable 114
    #[inline(always)]
    pub fn ppen114(&self) -> PPEN114_R {
        PPEN114_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - peripheral protection enable 115
    #[inline(always)]
    pub fn ppen115(&self) -> PPEN115_R {
        PPEN115_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - peripheral protection enable 116
    #[inline(always)]
    pub fn ppen116(&self) -> PPEN116_R {
        PPEN116_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - peripheral protection enable 117
    #[inline(always)]
    pub fn ppen117(&self) -> PPEN117_R {
        PPEN117_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - peripheral protection enable 118
    #[inline(always)]
    pub fn ppen118(&self) -> PPEN118_R {
        PPEN118_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - peripheral protection enable 119
    #[inline(always)]
    pub fn ppen119(&self) -> PPEN119_R {
        PPEN119_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - peripheral protection enable 120
    #[inline(always)]
    pub fn ppen120(&self) -> PPEN120_R {
        PPEN120_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - peripheral protection enable 121
    #[inline(always)]
    pub fn ppen121(&self) -> PPEN121_R {
        PPEN121_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - peripheral protection enable 122
    #[inline(always)]
    pub fn ppen122(&self) -> PPEN122_R {
        PPEN122_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - peripheral protection enable 123
    #[inline(always)]
    pub fn ppen123(&self) -> PPEN123_R {
        PPEN123_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - peripheral protection enable 124
    #[inline(always)]
    pub fn ppen124(&self) -> PPEN124_R {
        PPEN124_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - peripheral protection enable 125
    #[inline(always)]
    pub fn ppen125(&self) -> PPEN125_R {
        PPEN125_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - peripheral protection enable 126
    #[inline(always)]
    pub fn ppen126(&self) -> PPEN126_R {
        PPEN126_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - peripheral protection enable 127
    #[inline(always)]
    pub fn ppen127(&self) -> PPEN127_R {
        PPEN127_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PPSR3")
            .field("ppen96", &self.ppen96())
            .field("ppen97", &self.ppen97())
            .field("ppen98", &self.ppen98())
            .field("ppen99", &self.ppen99())
            .field("ppen100", &self.ppen100())
            .field("ppen101", &self.ppen101())
            .field("ppen102", &self.ppen102())
            .field("ppen103", &self.ppen103())
            .field("ppen104", &self.ppen104())
            .field("ppen105", &self.ppen105())
            .field("ppen106", &self.ppen106())
            .field("ppen107", &self.ppen107())
            .field("ppen108", &self.ppen108())
            .field("ppen109", &self.ppen109())
            .field("ppen110", &self.ppen110())
            .field("ppen111", &self.ppen111())
            .field("ppen112", &self.ppen112())
            .field("ppen113", &self.ppen113())
            .field("ppen114", &self.ppen114())
            .field("ppen115", &self.ppen115())
            .field("ppen116", &self.ppen116())
            .field("ppen117", &self.ppen117())
            .field("ppen118", &self.ppen118())
            .field("ppen119", &self.ppen119())
            .field("ppen120", &self.ppen120())
            .field("ppen121", &self.ppen121())
            .field("ppen122", &self.ppen122())
            .field("ppen123", &self.ppen123())
            .field("ppen124", &self.ppen124())
            .field("ppen125", &self.ppen125())
            .field("ppen126", &self.ppen126())
            .field("ppen127", &self.ppen127())
            .finish()
    }
}
/**RIFSC peripheral protection status register 3

You can [`read`](crate::Reg::read) this register and get [`ppsr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RIFSC:PPSR3)*/
pub struct PPSR3rs;
impl crate::RegisterSpec for PPSR3rs {
    type Ux = u32;
}
///`read()` method returns [`ppsr3::R`](R) reader structure
impl crate::Readable for PPSR3rs {}
///`reset()` method sets PPSR3 to value 0x05ff
impl crate::Resettable for PPSR3rs {
    const RESET_VALUE: u32 = 0x05ff;
}
