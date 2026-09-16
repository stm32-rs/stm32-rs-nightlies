///Register `FMKEYR1` writer
pub type W = crate::W<FMKEYR1rs>;
///Field `FMKEY32` writer - Fast master key bit 32 (i = 31 to 0)
pub type FMKEY32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY33` writer - Fast master key bit 33 (i = 31 to 0)
pub type FMKEY33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY34` writer - Fast master key bit 34 (i = 31 to 0)
pub type FMKEY34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY35` writer - Fast master key bit 35 (i = 31 to 0)
pub type FMKEY35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY36` writer - Fast master key bit 36 (i = 31 to 0)
pub type FMKEY36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY37` writer - Fast master key bit 37 (i = 31 to 0)
pub type FMKEY37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY38` writer - Fast master key bit 38 (i = 31 to 0)
pub type FMKEY38_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY39` writer - Fast master key bit 39 (i = 31 to 0)
pub type FMKEY39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY40` writer - Fast master key bit 40 (i = 31 to 0)
pub type FMKEY40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY41` writer - Fast master key bit 41 (i = 31 to 0)
pub type FMKEY41_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY42` writer - Fast master key bit 42 (i = 31 to 0)
pub type FMKEY42_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY43` writer - Fast master key bit 43 (i = 31 to 0)
pub type FMKEY43_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY44` writer - Fast master key bit 44 (i = 31 to 0)
pub type FMKEY44_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY45` writer - Fast master key bit 45 (i = 31 to 0)
pub type FMKEY45_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY46` writer - Fast master key bit 46 (i = 31 to 0)
pub type FMKEY46_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY47` writer - Fast master key bit 47 (i = 31 to 0)
pub type FMKEY47_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY48` writer - Fast master key bit 48 (i = 31 to 0)
pub type FMKEY48_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY49` writer - Fast master key bit 49 (i = 31 to 0)
pub type FMKEY49_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY50` writer - Fast master key bit 50 (i = 31 to 0)
pub type FMKEY50_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY51` writer - Fast master key bit 51 (i = 31 to 0)
pub type FMKEY51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY52` writer - Fast master key bit 52 (i = 31 to 0)
pub type FMKEY52_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY53` writer - Fast master key bit 53 (i = 31 to 0)
pub type FMKEY53_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY54` writer - Fast master key bit 54 (i = 31 to 0)
pub type FMKEY54_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY55` writer - Fast master key bit 55 (i = 31 to 0)
pub type FMKEY55_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY56` writer - Fast master key bit 56 (i = 31 to 0)
pub type FMKEY56_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY57` writer - Fast master key bit 57 (i = 31 to 0)
pub type FMKEY57_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY58` writer - Fast master key bit 58 (i = 31 to 0)
pub type FMKEY58_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY59` writer - Fast master key bit 59 (i = 31 to 0)
pub type FMKEY59_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY60` writer - Fast master key bit 60 (i = 31 to 0)
pub type FMKEY60_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY61` writer - Fast master key bit 61 (i = 31 to 0)
pub type FMKEY61_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY62` writer - Fast master key bit 62 (i = 31 to 0)
pub type FMKEY62_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY63` writer - Fast master key bit 63 (i = 31 to 0)
pub type FMKEY63_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FMKEYR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fast master key bit 32 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey32(&mut self) -> FMKEY32_W<'_, FMKEYR1rs> {
        FMKEY32_W::new(self, 0)
    }
    ///Bit 1 - Fast master key bit 33 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey33(&mut self) -> FMKEY33_W<'_, FMKEYR1rs> {
        FMKEY33_W::new(self, 1)
    }
    ///Bit 2 - Fast master key bit 34 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey34(&mut self) -> FMKEY34_W<'_, FMKEYR1rs> {
        FMKEY34_W::new(self, 2)
    }
    ///Bit 3 - Fast master key bit 35 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey35(&mut self) -> FMKEY35_W<'_, FMKEYR1rs> {
        FMKEY35_W::new(self, 3)
    }
    ///Bit 4 - Fast master key bit 36 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey36(&mut self) -> FMKEY36_W<'_, FMKEYR1rs> {
        FMKEY36_W::new(self, 4)
    }
    ///Bit 5 - Fast master key bit 37 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey37(&mut self) -> FMKEY37_W<'_, FMKEYR1rs> {
        FMKEY37_W::new(self, 5)
    }
    ///Bit 6 - Fast master key bit 38 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey38(&mut self) -> FMKEY38_W<'_, FMKEYR1rs> {
        FMKEY38_W::new(self, 6)
    }
    ///Bit 7 - Fast master key bit 39 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey39(&mut self) -> FMKEY39_W<'_, FMKEYR1rs> {
        FMKEY39_W::new(self, 7)
    }
    ///Bit 8 - Fast master key bit 40 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey40(&mut self) -> FMKEY40_W<'_, FMKEYR1rs> {
        FMKEY40_W::new(self, 8)
    }
    ///Bit 9 - Fast master key bit 41 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey41(&mut self) -> FMKEY41_W<'_, FMKEYR1rs> {
        FMKEY41_W::new(self, 9)
    }
    ///Bit 10 - Fast master key bit 42 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey42(&mut self) -> FMKEY42_W<'_, FMKEYR1rs> {
        FMKEY42_W::new(self, 10)
    }
    ///Bit 11 - Fast master key bit 43 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey43(&mut self) -> FMKEY43_W<'_, FMKEYR1rs> {
        FMKEY43_W::new(self, 11)
    }
    ///Bit 12 - Fast master key bit 44 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey44(&mut self) -> FMKEY44_W<'_, FMKEYR1rs> {
        FMKEY44_W::new(self, 12)
    }
    ///Bit 13 - Fast master key bit 45 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey45(&mut self) -> FMKEY45_W<'_, FMKEYR1rs> {
        FMKEY45_W::new(self, 13)
    }
    ///Bit 14 - Fast master key bit 46 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey46(&mut self) -> FMKEY46_W<'_, FMKEYR1rs> {
        FMKEY46_W::new(self, 14)
    }
    ///Bit 15 - Fast master key bit 47 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey47(&mut self) -> FMKEY47_W<'_, FMKEYR1rs> {
        FMKEY47_W::new(self, 15)
    }
    ///Bit 16 - Fast master key bit 48 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey48(&mut self) -> FMKEY48_W<'_, FMKEYR1rs> {
        FMKEY48_W::new(self, 16)
    }
    ///Bit 17 - Fast master key bit 49 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey49(&mut self) -> FMKEY49_W<'_, FMKEYR1rs> {
        FMKEY49_W::new(self, 17)
    }
    ///Bit 18 - Fast master key bit 50 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey50(&mut self) -> FMKEY50_W<'_, FMKEYR1rs> {
        FMKEY50_W::new(self, 18)
    }
    ///Bit 19 - Fast master key bit 51 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey51(&mut self) -> FMKEY51_W<'_, FMKEYR1rs> {
        FMKEY51_W::new(self, 19)
    }
    ///Bit 20 - Fast master key bit 52 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey52(&mut self) -> FMKEY52_W<'_, FMKEYR1rs> {
        FMKEY52_W::new(self, 20)
    }
    ///Bit 21 - Fast master key bit 53 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey53(&mut self) -> FMKEY53_W<'_, FMKEYR1rs> {
        FMKEY53_W::new(self, 21)
    }
    ///Bit 22 - Fast master key bit 54 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey54(&mut self) -> FMKEY54_W<'_, FMKEYR1rs> {
        FMKEY54_W::new(self, 22)
    }
    ///Bit 23 - Fast master key bit 55 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey55(&mut self) -> FMKEY55_W<'_, FMKEYR1rs> {
        FMKEY55_W::new(self, 23)
    }
    ///Bit 24 - Fast master key bit 56 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey56(&mut self) -> FMKEY56_W<'_, FMKEYR1rs> {
        FMKEY56_W::new(self, 24)
    }
    ///Bit 25 - Fast master key bit 57 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey57(&mut self) -> FMKEY57_W<'_, FMKEYR1rs> {
        FMKEY57_W::new(self, 25)
    }
    ///Bit 26 - Fast master key bit 58 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey58(&mut self) -> FMKEY58_W<'_, FMKEYR1rs> {
        FMKEY58_W::new(self, 26)
    }
    ///Bit 27 - Fast master key bit 59 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey59(&mut self) -> FMKEY59_W<'_, FMKEYR1rs> {
        FMKEY59_W::new(self, 27)
    }
    ///Bit 28 - Fast master key bit 60 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey60(&mut self) -> FMKEY60_W<'_, FMKEYR1rs> {
        FMKEY60_W::new(self, 28)
    }
    ///Bit 29 - Fast master key bit 61 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey61(&mut self) -> FMKEY61_W<'_, FMKEYR1rs> {
        FMKEY61_W::new(self, 29)
    }
    ///Bit 30 - Fast master key bit 62 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey62(&mut self) -> FMKEY62_W<'_, FMKEYR1rs> {
        FMKEY62_W::new(self, 30)
    }
    ///Bit 31 - Fast master key bit 63 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey63(&mut self) -> FMKEY63_W<'_, FMKEYR1rs> {
        FMKEY63_W::new(self, 31)
    }
}
/**MCE fast master key 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR1)*/
pub struct FMKEYR1rs;
impl crate::RegisterSpec for FMKEYR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmkeyr1::W`](W) writer structure
impl crate::Writable for FMKEYR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMKEYR1 to value 0
impl crate::Resettable for FMKEYR1rs {}
