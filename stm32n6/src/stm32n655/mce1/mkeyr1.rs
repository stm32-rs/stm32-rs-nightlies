///Register `MKEYR1` writer
pub type W = crate::W<MKEYR1rs>;
///Field `MKEY32` writer - Master key bit 32 (i = 31 to 0)
pub type MKEY32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY33` writer - Master key bit 33 (i = 31 to 0)
pub type MKEY33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY34` writer - Master key bit 34 (i = 31 to 0)
pub type MKEY34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY35` writer - Master key bit 35 (i = 31 to 0)
pub type MKEY35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY36` writer - Master key bit 36 (i = 31 to 0)
pub type MKEY36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY37` writer - Master key bit 37 (i = 31 to 0)
pub type MKEY37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY38` writer - Master key bit 38 (i = 31 to 0)
pub type MKEY38_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY39` writer - Master key bit 39 (i = 31 to 0)
pub type MKEY39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY40` writer - Master key bit 40 (i = 31 to 0)
pub type MKEY40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY41` writer - Master key bit 41 (i = 31 to 0)
pub type MKEY41_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY42` writer - Master key bit 42 (i = 31 to 0)
pub type MKEY42_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY43` writer - Master key bit 43 (i = 31 to 0)
pub type MKEY43_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY44` writer - Master key bit 44 (i = 31 to 0)
pub type MKEY44_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY45` writer - Master key bit 45 (i = 31 to 0)
pub type MKEY45_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY46` writer - Master key bit 46 (i = 31 to 0)
pub type MKEY46_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY47` writer - Master key bit 47 (i = 31 to 0)
pub type MKEY47_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY48` writer - Master key bit 48 (i = 31 to 0)
pub type MKEY48_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY49` writer - Master key bit 49 (i = 31 to 0)
pub type MKEY49_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY50` writer - Master key bit 50 (i = 31 to 0)
pub type MKEY50_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY51` writer - Master key bit 51 (i = 31 to 0)
pub type MKEY51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY52` writer - Master key bit 52 (i = 31 to 0)
pub type MKEY52_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY53` writer - Master key bit 53 (i = 31 to 0)
pub type MKEY53_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY54` writer - Master key bit 54 (i = 31 to 0)
pub type MKEY54_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY55` writer - Master key bit 55 (i = 31 to 0)
pub type MKEY55_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY56` writer - Master key bit 56 (i = 31 to 0)
pub type MKEY56_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY57` writer - Master key bit 57 (i = 31 to 0)
pub type MKEY57_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY58` writer - Master key bit 58 (i = 31 to 0)
pub type MKEY58_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY59` writer - Master key bit 59 (i = 31 to 0)
pub type MKEY59_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY60` writer - Master key bit 60 (i = 31 to 0)
pub type MKEY60_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY61` writer - Master key bit 61 (i = 31 to 0)
pub type MKEY61_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY62` writer - Master key bit 62 (i = 31 to 0)
pub type MKEY62_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY63` writer - Master key bit 63 (i = 31 to 0)
pub type MKEY63_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MKEYR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Master key bit 32 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey32(&mut self) -> MKEY32_W<'_, MKEYR1rs> {
        MKEY32_W::new(self, 0)
    }
    ///Bit 1 - Master key bit 33 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey33(&mut self) -> MKEY33_W<'_, MKEYR1rs> {
        MKEY33_W::new(self, 1)
    }
    ///Bit 2 - Master key bit 34 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey34(&mut self) -> MKEY34_W<'_, MKEYR1rs> {
        MKEY34_W::new(self, 2)
    }
    ///Bit 3 - Master key bit 35 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey35(&mut self) -> MKEY35_W<'_, MKEYR1rs> {
        MKEY35_W::new(self, 3)
    }
    ///Bit 4 - Master key bit 36 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey36(&mut self) -> MKEY36_W<'_, MKEYR1rs> {
        MKEY36_W::new(self, 4)
    }
    ///Bit 5 - Master key bit 37 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey37(&mut self) -> MKEY37_W<'_, MKEYR1rs> {
        MKEY37_W::new(self, 5)
    }
    ///Bit 6 - Master key bit 38 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey38(&mut self) -> MKEY38_W<'_, MKEYR1rs> {
        MKEY38_W::new(self, 6)
    }
    ///Bit 7 - Master key bit 39 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey39(&mut self) -> MKEY39_W<'_, MKEYR1rs> {
        MKEY39_W::new(self, 7)
    }
    ///Bit 8 - Master key bit 40 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey40(&mut self) -> MKEY40_W<'_, MKEYR1rs> {
        MKEY40_W::new(self, 8)
    }
    ///Bit 9 - Master key bit 41 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey41(&mut self) -> MKEY41_W<'_, MKEYR1rs> {
        MKEY41_W::new(self, 9)
    }
    ///Bit 10 - Master key bit 42 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey42(&mut self) -> MKEY42_W<'_, MKEYR1rs> {
        MKEY42_W::new(self, 10)
    }
    ///Bit 11 - Master key bit 43 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey43(&mut self) -> MKEY43_W<'_, MKEYR1rs> {
        MKEY43_W::new(self, 11)
    }
    ///Bit 12 - Master key bit 44 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey44(&mut self) -> MKEY44_W<'_, MKEYR1rs> {
        MKEY44_W::new(self, 12)
    }
    ///Bit 13 - Master key bit 45 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey45(&mut self) -> MKEY45_W<'_, MKEYR1rs> {
        MKEY45_W::new(self, 13)
    }
    ///Bit 14 - Master key bit 46 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey46(&mut self) -> MKEY46_W<'_, MKEYR1rs> {
        MKEY46_W::new(self, 14)
    }
    ///Bit 15 - Master key bit 47 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey47(&mut self) -> MKEY47_W<'_, MKEYR1rs> {
        MKEY47_W::new(self, 15)
    }
    ///Bit 16 - Master key bit 48 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey48(&mut self) -> MKEY48_W<'_, MKEYR1rs> {
        MKEY48_W::new(self, 16)
    }
    ///Bit 17 - Master key bit 49 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey49(&mut self) -> MKEY49_W<'_, MKEYR1rs> {
        MKEY49_W::new(self, 17)
    }
    ///Bit 18 - Master key bit 50 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey50(&mut self) -> MKEY50_W<'_, MKEYR1rs> {
        MKEY50_W::new(self, 18)
    }
    ///Bit 19 - Master key bit 51 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey51(&mut self) -> MKEY51_W<'_, MKEYR1rs> {
        MKEY51_W::new(self, 19)
    }
    ///Bit 20 - Master key bit 52 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey52(&mut self) -> MKEY52_W<'_, MKEYR1rs> {
        MKEY52_W::new(self, 20)
    }
    ///Bit 21 - Master key bit 53 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey53(&mut self) -> MKEY53_W<'_, MKEYR1rs> {
        MKEY53_W::new(self, 21)
    }
    ///Bit 22 - Master key bit 54 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey54(&mut self) -> MKEY54_W<'_, MKEYR1rs> {
        MKEY54_W::new(self, 22)
    }
    ///Bit 23 - Master key bit 55 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey55(&mut self) -> MKEY55_W<'_, MKEYR1rs> {
        MKEY55_W::new(self, 23)
    }
    ///Bit 24 - Master key bit 56 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey56(&mut self) -> MKEY56_W<'_, MKEYR1rs> {
        MKEY56_W::new(self, 24)
    }
    ///Bit 25 - Master key bit 57 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey57(&mut self) -> MKEY57_W<'_, MKEYR1rs> {
        MKEY57_W::new(self, 25)
    }
    ///Bit 26 - Master key bit 58 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey58(&mut self) -> MKEY58_W<'_, MKEYR1rs> {
        MKEY58_W::new(self, 26)
    }
    ///Bit 27 - Master key bit 59 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey59(&mut self) -> MKEY59_W<'_, MKEYR1rs> {
        MKEY59_W::new(self, 27)
    }
    ///Bit 28 - Master key bit 60 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey60(&mut self) -> MKEY60_W<'_, MKEYR1rs> {
        MKEY60_W::new(self, 28)
    }
    ///Bit 29 - Master key bit 61 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey61(&mut self) -> MKEY61_W<'_, MKEYR1rs> {
        MKEY61_W::new(self, 29)
    }
    ///Bit 30 - Master key bit 62 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey62(&mut self) -> MKEY62_W<'_, MKEYR1rs> {
        MKEY62_W::new(self, 30)
    }
    ///Bit 31 - Master key bit 63 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey63(&mut self) -> MKEY63_W<'_, MKEYR1rs> {
        MKEY63_W::new(self, 31)
    }
}
/**.MCE master key 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:MKEYR1)*/
pub struct MKEYR1rs;
impl crate::RegisterSpec for MKEYR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`mkeyr1::W`](W) writer structure
impl crate::Writable for MKEYR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MKEYR1 to value 0
impl crate::Resettable for MKEYR1rs {}
