///Register `ISR3` reader
pub type R = crate::R<ISR3rs>;
///Field `IAF96` reader - illegal access interrupt enable for peripheral 96
pub type IAF96_R = crate::BitReader;
///Field `IAF97` reader - illegal access interrupt enable for peripheral 97
pub type IAF97_R = crate::BitReader;
///Field `IAF98` reader - illegal access interrupt enable for peripheral 98
pub type IAF98_R = crate::BitReader;
///Field `IAF99` reader - illegal access interrupt enable for peripheral 99
pub type IAF99_R = crate::BitReader;
///Field `IAF100` reader - illegal access interrupt enable for peripheral 100
pub type IAF100_R = crate::BitReader;
///Field `IAF101` reader - illegal access interrupt enable for peripheral 101
pub type IAF101_R = crate::BitReader;
///Field `IAF102` reader - illegal access interrupt enable for peripheral 102
pub type IAF102_R = crate::BitReader;
///Field `IAF103` reader - illegal access interrupt enable for peripheral 103
pub type IAF103_R = crate::BitReader;
///Field `IAF104` reader - illegal access interrupt enable for peripheral 104
pub type IAF104_R = crate::BitReader;
///Field `IAF105` reader - illegal access interrupt enable for peripheral 105
pub type IAF105_R = crate::BitReader;
///Field `IAF106` reader - illegal access interrupt enable for peripheral 106
pub type IAF106_R = crate::BitReader;
///Field `IAF107` reader - illegal access interrupt enable for peripheral 107
pub type IAF107_R = crate::BitReader;
///Field `IAF108` reader - illegal access interrupt enable for peripheral 108
pub type IAF108_R = crate::BitReader;
///Field `IAF109` reader - illegal access interrupt enable for peripheral 109
pub type IAF109_R = crate::BitReader;
///Field `IAF110` reader - illegal access interrupt enable for peripheral 110
pub type IAF110_R = crate::BitReader;
///Field `IAF111` reader - illegal access interrupt enable for peripheral 111
pub type IAF111_R = crate::BitReader;
///Field `IAF112` reader - illegal access interrupt enable for peripheral 112
pub type IAF112_R = crate::BitReader;
///Field `IAF113` reader - illegal access interrupt enable for peripheral 113
pub type IAF113_R = crate::BitReader;
///Field `IAF114` reader - illegal access interrupt enable for peripheral 114
pub type IAF114_R = crate::BitReader;
///Field `IAF115` reader - illegal access interrupt enable for peripheral 115
pub type IAF115_R = crate::BitReader;
///Field `IAF116` reader - illegal access interrupt enable for peripheral 116
pub type IAF116_R = crate::BitReader;
///Field `IAF117` reader - illegal access interrupt enable for peripheral 117
pub type IAF117_R = crate::BitReader;
///Field `IAF118` reader - illegal access interrupt enable for peripheral 118
pub type IAF118_R = crate::BitReader;
///Field `IAF119` reader - illegal access interrupt enable for peripheral 119
pub type IAF119_R = crate::BitReader;
///Field `IAF120` reader - illegal access interrupt enable for peripheral 120
pub type IAF120_R = crate::BitReader;
///Field `IAF121` reader - illegal access interrupt enable for peripheral 121
pub type IAF121_R = crate::BitReader;
///Field `IAF122` reader - illegal access interrupt enable for peripheral 122
pub type IAF122_R = crate::BitReader;
///Field `IAF123` reader - illegal access interrupt enable for peripheral 123
pub type IAF123_R = crate::BitReader;
///Field `IAF124` reader - illegal access interrupt enable for peripheral 124
pub type IAF124_R = crate::BitReader;
///Field `IAF125` reader - illegal access interrupt enable for peripheral 125
pub type IAF125_R = crate::BitReader;
///Field `IAF126` reader - illegal access interrupt enable for peripheral 126
pub type IAF126_R = crate::BitReader;
///Field `IAF127` reader - illegal access interrupt enable for peripheral 127
pub type IAF127_R = crate::BitReader;
impl R {
    ///Bit 0 - illegal access interrupt enable for peripheral 96
    #[inline(always)]
    pub fn iaf96(&self) -> IAF96_R {
        IAF96_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access interrupt enable for peripheral 97
    #[inline(always)]
    pub fn iaf97(&self) -> IAF97_R {
        IAF97_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access interrupt enable for peripheral 98
    #[inline(always)]
    pub fn iaf98(&self) -> IAF98_R {
        IAF98_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access interrupt enable for peripheral 99
    #[inline(always)]
    pub fn iaf99(&self) -> IAF99_R {
        IAF99_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for peripheral 100
    #[inline(always)]
    pub fn iaf100(&self) -> IAF100_R {
        IAF100_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access interrupt enable for peripheral 101
    #[inline(always)]
    pub fn iaf101(&self) -> IAF101_R {
        IAF101_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for peripheral 102
    #[inline(always)]
    pub fn iaf102(&self) -> IAF102_R {
        IAF102_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - illegal access interrupt enable for peripheral 103
    #[inline(always)]
    pub fn iaf103(&self) -> IAF103_R {
        IAF103_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - illegal access interrupt enable for peripheral 104
    #[inline(always)]
    pub fn iaf104(&self) -> IAF104_R {
        IAF104_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access interrupt enable for peripheral 105
    #[inline(always)]
    pub fn iaf105(&self) -> IAF105_R {
        IAF105_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access interrupt enable for peripheral 106
    #[inline(always)]
    pub fn iaf106(&self) -> IAF106_R {
        IAF106_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for peripheral 107
    #[inline(always)]
    pub fn iaf107(&self) -> IAF107_R {
        IAF107_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for peripheral 108
    #[inline(always)]
    pub fn iaf108(&self) -> IAF108_R {
        IAF108_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for peripheral 109
    #[inline(always)]
    pub fn iaf109(&self) -> IAF109_R {
        IAF109_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for peripheral 110
    #[inline(always)]
    pub fn iaf110(&self) -> IAF110_R {
        IAF110_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for peripheral 111
    #[inline(always)]
    pub fn iaf111(&self) -> IAF111_R {
        IAF111_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for peripheral 112
    #[inline(always)]
    pub fn iaf112(&self) -> IAF112_R {
        IAF112_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access interrupt enable for peripheral 113
    #[inline(always)]
    pub fn iaf113(&self) -> IAF113_R {
        IAF113_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access interrupt enable for peripheral 114
    #[inline(always)]
    pub fn iaf114(&self) -> IAF114_R {
        IAF114_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access interrupt enable for peripheral 115
    #[inline(always)]
    pub fn iaf115(&self) -> IAF115_R {
        IAF115_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - illegal access interrupt enable for peripheral 116
    #[inline(always)]
    pub fn iaf116(&self) -> IAF116_R {
        IAF116_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - illegal access interrupt enable for peripheral 117
    #[inline(always)]
    pub fn iaf117(&self) -> IAF117_R {
        IAF117_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for peripheral 118
    #[inline(always)]
    pub fn iaf118(&self) -> IAF118_R {
        IAF118_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for peripheral 119
    #[inline(always)]
    pub fn iaf119(&self) -> IAF119_R {
        IAF119_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - illegal access interrupt enable for peripheral 120
    #[inline(always)]
    pub fn iaf120(&self) -> IAF120_R {
        IAF120_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access interrupt enable for peripheral 121
    #[inline(always)]
    pub fn iaf121(&self) -> IAF121_R {
        IAF121_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access interrupt enable for peripheral 122
    #[inline(always)]
    pub fn iaf122(&self) -> IAF122_R {
        IAF122_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access interrupt enable for peripheral 123
    #[inline(always)]
    pub fn iaf123(&self) -> IAF123_R {
        IAF123_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access interrupt enable for peripheral 124
    #[inline(always)]
    pub fn iaf124(&self) -> IAF124_R {
        IAF124_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access interrupt enable for peripheral 125
    #[inline(always)]
    pub fn iaf125(&self) -> IAF125_R {
        IAF125_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access interrupt enable for peripheral 126
    #[inline(always)]
    pub fn iaf126(&self) -> IAF126_R {
        IAF126_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access interrupt enable for peripheral 127
    #[inline(always)]
    pub fn iaf127(&self) -> IAF127_R {
        IAF127_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR3")
            .field("iaf96", &self.iaf96())
            .field("iaf97", &self.iaf97())
            .field("iaf98", &self.iaf98())
            .field("iaf99", &self.iaf99())
            .field("iaf100", &self.iaf100())
            .field("iaf101", &self.iaf101())
            .field("iaf102", &self.iaf102())
            .field("iaf103", &self.iaf103())
            .field("iaf104", &self.iaf104())
            .field("iaf105", &self.iaf105())
            .field("iaf106", &self.iaf106())
            .field("iaf107", &self.iaf107())
            .field("iaf108", &self.iaf108())
            .field("iaf109", &self.iaf109())
            .field("iaf110", &self.iaf110())
            .field("iaf111", &self.iaf111())
            .field("iaf112", &self.iaf112())
            .field("iaf113", &self.iaf113())
            .field("iaf114", &self.iaf114())
            .field("iaf115", &self.iaf115())
            .field("iaf116", &self.iaf116())
            .field("iaf117", &self.iaf117())
            .field("iaf118", &self.iaf118())
            .field("iaf119", &self.iaf119())
            .field("iaf120", &self.iaf120())
            .field("iaf121", &self.iaf121())
            .field("iaf122", &self.iaf122())
            .field("iaf123", &self.iaf123())
            .field("iaf124", &self.iaf124())
            .field("iaf125", &self.iaf125())
            .field("iaf126", &self.iaf126())
            .field("iaf127", &self.iaf127())
            .finish()
    }
}
/**IAC interrupt status register 3

You can [`read`](crate::Reg::read) this register and get [`isr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#IAC:ISR3)*/
pub struct ISR3rs;
impl crate::RegisterSpec for ISR3rs {
    type Ux = u32;
}
///`read()` method returns [`isr3::R`](R) reader structure
impl crate::Readable for ISR3rs {}
///`reset()` method sets ISR3 to value 0
impl crate::Resettable for ISR3rs {}
