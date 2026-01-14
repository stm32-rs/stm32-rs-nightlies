///Register `FMKEYR3` writer
pub type W = crate::W<FMKEYR3rs>;
///Field `FMKEY96` writer - Fast master key bit 96 (i = 31 to 0)
pub type FMKEY96_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY97` writer - Fast master key bit 97 (i = 31 to 0)
pub type FMKEY97_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY98` writer - Fast master key bit 98 (i = 31 to 0)
pub type FMKEY98_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY99` writer - Fast master key bit 99 (i = 31 to 0)
pub type FMKEY99_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY100` writer - Fast master key bit 100 (i = 31 to 0)
pub type FMKEY100_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY101` writer - Fast master key bit 101 (i = 31 to 0)
pub type FMKEY101_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY102` writer - Fast master key bit 102 (i = 31 to 0)
pub type FMKEY102_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY103` writer - Fast master key bit 103 (i = 31 to 0)
pub type FMKEY103_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY104` writer - Fast master key bit 104 (i = 31 to 0)
pub type FMKEY104_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY105` writer - Fast master key bit 105 (i = 31 to 0)
pub type FMKEY105_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY106` writer - Fast master key bit 106 (i = 31 to 0)
pub type FMKEY106_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY107` writer - Fast master key bit 107 (i = 31 to 0)
pub type FMKEY107_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY108` writer - Fast master key bit 108 (i = 31 to 0)
pub type FMKEY108_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY109` writer - Fast master key bit 109 (i = 31 to 0)
pub type FMKEY109_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY110` writer - Fast master key bit 110 (i = 31 to 0)
pub type FMKEY110_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY111` writer - Fast master key bit 111 (i = 31 to 0)
pub type FMKEY111_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY112` writer - Fast master key bit 112 (i = 31 to 0)
pub type FMKEY112_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY113` writer - Fast master key bit 113 (i = 31 to 0)
pub type FMKEY113_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY114` writer - Fast master key bit 114 (i = 31 to 0)
pub type FMKEY114_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY115` writer - Fast master key bit 115 (i = 31 to 0)
pub type FMKEY115_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY116` writer - Fast master key bit 116 (i = 31 to 0)
pub type FMKEY116_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY117` writer - Fast master key bit 117 (i = 31 to 0)
pub type FMKEY117_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY118` writer - Fast master key bit 118 (i = 31 to 0)
pub type FMKEY118_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY119` writer - Fast master key bit 119 (i = 31 to 0)
pub type FMKEY119_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY120` writer - Fast master key bit 120 (i = 31 to 0)
pub type FMKEY120_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY121` writer - Fast master key bit 121 (i = 31 to 0)
pub type FMKEY121_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY122` writer - Fast master key bit 122 (i = 31 to 0)
pub type FMKEY122_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY123` writer - Fast master key bit 123 (i = 31 to 0)
pub type FMKEY123_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY124` writer - Fast master key bit 124 (i = 31 to 0)
pub type FMKEY124_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY125` writer - Fast master key bit 125 (i = 31 to 0)
pub type FMKEY125_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY126` writer - Fast master key bit 126 (i = 31 to 0)
pub type FMKEY126_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY127` writer - Fast master key bit 127 (i = 31 to 0)
pub type FMKEY127_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FMKEYR3rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fast master key bit 96 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey96(&mut self) -> FMKEY96_W<'_, FMKEYR3rs> {
        FMKEY96_W::new(self, 0)
    }
    ///Bit 1 - Fast master key bit 97 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey97(&mut self) -> FMKEY97_W<'_, FMKEYR3rs> {
        FMKEY97_W::new(self, 1)
    }
    ///Bit 2 - Fast master key bit 98 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey98(&mut self) -> FMKEY98_W<'_, FMKEYR3rs> {
        FMKEY98_W::new(self, 2)
    }
    ///Bit 3 - Fast master key bit 99 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey99(&mut self) -> FMKEY99_W<'_, FMKEYR3rs> {
        FMKEY99_W::new(self, 3)
    }
    ///Bit 4 - Fast master key bit 100 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey100(&mut self) -> FMKEY100_W<'_, FMKEYR3rs> {
        FMKEY100_W::new(self, 4)
    }
    ///Bit 5 - Fast master key bit 101 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey101(&mut self) -> FMKEY101_W<'_, FMKEYR3rs> {
        FMKEY101_W::new(self, 5)
    }
    ///Bit 6 - Fast master key bit 102 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey102(&mut self) -> FMKEY102_W<'_, FMKEYR3rs> {
        FMKEY102_W::new(self, 6)
    }
    ///Bit 7 - Fast master key bit 103 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey103(&mut self) -> FMKEY103_W<'_, FMKEYR3rs> {
        FMKEY103_W::new(self, 7)
    }
    ///Bit 8 - Fast master key bit 104 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey104(&mut self) -> FMKEY104_W<'_, FMKEYR3rs> {
        FMKEY104_W::new(self, 8)
    }
    ///Bit 9 - Fast master key bit 105 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey105(&mut self) -> FMKEY105_W<'_, FMKEYR3rs> {
        FMKEY105_W::new(self, 9)
    }
    ///Bit 10 - Fast master key bit 106 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey106(&mut self) -> FMKEY106_W<'_, FMKEYR3rs> {
        FMKEY106_W::new(self, 10)
    }
    ///Bit 11 - Fast master key bit 107 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey107(&mut self) -> FMKEY107_W<'_, FMKEYR3rs> {
        FMKEY107_W::new(self, 11)
    }
    ///Bit 12 - Fast master key bit 108 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey108(&mut self) -> FMKEY108_W<'_, FMKEYR3rs> {
        FMKEY108_W::new(self, 12)
    }
    ///Bit 13 - Fast master key bit 109 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey109(&mut self) -> FMKEY109_W<'_, FMKEYR3rs> {
        FMKEY109_W::new(self, 13)
    }
    ///Bit 14 - Fast master key bit 110 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey110(&mut self) -> FMKEY110_W<'_, FMKEYR3rs> {
        FMKEY110_W::new(self, 14)
    }
    ///Bit 15 - Fast master key bit 111 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey111(&mut self) -> FMKEY111_W<'_, FMKEYR3rs> {
        FMKEY111_W::new(self, 15)
    }
    ///Bit 16 - Fast master key bit 112 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey112(&mut self) -> FMKEY112_W<'_, FMKEYR3rs> {
        FMKEY112_W::new(self, 16)
    }
    ///Bit 17 - Fast master key bit 113 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey113(&mut self) -> FMKEY113_W<'_, FMKEYR3rs> {
        FMKEY113_W::new(self, 17)
    }
    ///Bit 18 - Fast master key bit 114 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey114(&mut self) -> FMKEY114_W<'_, FMKEYR3rs> {
        FMKEY114_W::new(self, 18)
    }
    ///Bit 19 - Fast master key bit 115 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey115(&mut self) -> FMKEY115_W<'_, FMKEYR3rs> {
        FMKEY115_W::new(self, 19)
    }
    ///Bit 20 - Fast master key bit 116 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey116(&mut self) -> FMKEY116_W<'_, FMKEYR3rs> {
        FMKEY116_W::new(self, 20)
    }
    ///Bit 21 - Fast master key bit 117 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey117(&mut self) -> FMKEY117_W<'_, FMKEYR3rs> {
        FMKEY117_W::new(self, 21)
    }
    ///Bit 22 - Fast master key bit 118 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey118(&mut self) -> FMKEY118_W<'_, FMKEYR3rs> {
        FMKEY118_W::new(self, 22)
    }
    ///Bit 23 - Fast master key bit 119 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey119(&mut self) -> FMKEY119_W<'_, FMKEYR3rs> {
        FMKEY119_W::new(self, 23)
    }
    ///Bit 24 - Fast master key bit 120 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey120(&mut self) -> FMKEY120_W<'_, FMKEYR3rs> {
        FMKEY120_W::new(self, 24)
    }
    ///Bit 25 - Fast master key bit 121 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey121(&mut self) -> FMKEY121_W<'_, FMKEYR3rs> {
        FMKEY121_W::new(self, 25)
    }
    ///Bit 26 - Fast master key bit 122 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey122(&mut self) -> FMKEY122_W<'_, FMKEYR3rs> {
        FMKEY122_W::new(self, 26)
    }
    ///Bit 27 - Fast master key bit 123 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey123(&mut self) -> FMKEY123_W<'_, FMKEYR3rs> {
        FMKEY123_W::new(self, 27)
    }
    ///Bit 28 - Fast master key bit 124 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey124(&mut self) -> FMKEY124_W<'_, FMKEYR3rs> {
        FMKEY124_W::new(self, 28)
    }
    ///Bit 29 - Fast master key bit 125 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey125(&mut self) -> FMKEY125_W<'_, FMKEYR3rs> {
        FMKEY125_W::new(self, 29)
    }
    ///Bit 30 - Fast master key bit 126 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey126(&mut self) -> FMKEY126_W<'_, FMKEYR3rs> {
        FMKEY126_W::new(self, 30)
    }
    ///Bit 31 - Fast master key bit 127 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey127(&mut self) -> FMKEY127_W<'_, FMKEYR3rs> {
        FMKEY127_W::new(self, 31)
    }
}
/**MCE fast master key 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR3)*/
pub struct FMKEYR3rs;
impl crate::RegisterSpec for FMKEYR3rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmkeyr3::W`](W) writer structure
impl crate::Writable for FMKEYR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMKEYR3 to value 0
impl crate::Resettable for FMKEYR3rs {}
