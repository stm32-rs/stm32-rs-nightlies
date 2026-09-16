///Register `FMKEYR0` writer
pub type W = crate::W<FMKEYR0rs>;
///Field `FMKEY0` writer - Fast master key bit 0 (i = 31 to 0)
pub type FMKEY0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY1` writer - Fast master key bit 1 (i = 31 to 0)
pub type FMKEY1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY2` writer - Fast master key bit 2 (i = 31 to 0)
pub type FMKEY2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY3` writer - Fast master key bit 3 (i = 31 to 0)
pub type FMKEY3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY4` writer - Fast master key bit 4 (i = 31 to 0)
pub type FMKEY4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY5` writer - Fast master key bit 5 (i = 31 to 0)
pub type FMKEY5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY6` writer - Fast master key bit 6 (i = 31 to 0)
pub type FMKEY6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY7` writer - Fast master key bit 7 (i = 31 to 0)
pub type FMKEY7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY8` writer - Fast master key bit 8 (i = 31 to 0)
pub type FMKEY8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY9` writer - Fast master key bit 9 (i = 31 to 0)
pub type FMKEY9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY10` writer - Fast master key bit 10 (i = 31 to 0)
pub type FMKEY10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY11` writer - Fast master key bit 11 (i = 31 to 0)
pub type FMKEY11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY12` writer - Fast master key bit 12 (i = 31 to 0)
pub type FMKEY12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY13` writer - Fast master key bit 13 (i = 31 to 0)
pub type FMKEY13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY14` writer - Fast master key bit 14 (i = 31 to 0)
pub type FMKEY14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY15` writer - Fast master key bit 15 (i = 31 to 0)
pub type FMKEY15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY16` writer - Fast master key bit 16 (i = 31 to 0)
pub type FMKEY16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY17` writer - Fast master key bit 17 (i = 31 to 0)
pub type FMKEY17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY18` writer - Fast master key bit 18 (i = 31 to 0)
pub type FMKEY18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY19` writer - Fast master key bit 19 (i = 31 to 0)
pub type FMKEY19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY20` writer - Fast master key bit 20 (i = 31 to 0)
pub type FMKEY20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY21` writer - Fast master key bit 21 (i = 31 to 0)
pub type FMKEY21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY22` writer - Fast master key bit 22 (i = 31 to 0)
pub type FMKEY22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY23` writer - Fast master key bit 23 (i = 31 to 0)
pub type FMKEY23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY24` writer - Fast master key bit 24 (i = 31 to 0)
pub type FMKEY24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY25` writer - Fast master key bit 25 (i = 31 to 0)
pub type FMKEY25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY26` writer - Fast master key bit 26 (i = 31 to 0)
pub type FMKEY26_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY27` writer - Fast master key bit 27 (i = 31 to 0)
pub type FMKEY27_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY28` writer - Fast master key bit 28 (i = 31 to 0)
pub type FMKEY28_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY29` writer - Fast master key bit 29 (i = 31 to 0)
pub type FMKEY29_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY30` writer - Fast master key bit 30 (i = 31 to 0)
pub type FMKEY30_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMKEY31` writer - Fast master key bit 31 (i = 31 to 0)
pub type FMKEY31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FMKEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Fast master key bit 0 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey0(&mut self) -> FMKEY0_W<'_, FMKEYR0rs> {
        FMKEY0_W::new(self, 0)
    }
    ///Bit 1 - Fast master key bit 1 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey1(&mut self) -> FMKEY1_W<'_, FMKEYR0rs> {
        FMKEY1_W::new(self, 1)
    }
    ///Bit 2 - Fast master key bit 2 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey2(&mut self) -> FMKEY2_W<'_, FMKEYR0rs> {
        FMKEY2_W::new(self, 2)
    }
    ///Bit 3 - Fast master key bit 3 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey3(&mut self) -> FMKEY3_W<'_, FMKEYR0rs> {
        FMKEY3_W::new(self, 3)
    }
    ///Bit 4 - Fast master key bit 4 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey4(&mut self) -> FMKEY4_W<'_, FMKEYR0rs> {
        FMKEY4_W::new(self, 4)
    }
    ///Bit 5 - Fast master key bit 5 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey5(&mut self) -> FMKEY5_W<'_, FMKEYR0rs> {
        FMKEY5_W::new(self, 5)
    }
    ///Bit 6 - Fast master key bit 6 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey6(&mut self) -> FMKEY6_W<'_, FMKEYR0rs> {
        FMKEY6_W::new(self, 6)
    }
    ///Bit 7 - Fast master key bit 7 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey7(&mut self) -> FMKEY7_W<'_, FMKEYR0rs> {
        FMKEY7_W::new(self, 7)
    }
    ///Bit 8 - Fast master key bit 8 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey8(&mut self) -> FMKEY8_W<'_, FMKEYR0rs> {
        FMKEY8_W::new(self, 8)
    }
    ///Bit 9 - Fast master key bit 9 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey9(&mut self) -> FMKEY9_W<'_, FMKEYR0rs> {
        FMKEY9_W::new(self, 9)
    }
    ///Bit 10 - Fast master key bit 10 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey10(&mut self) -> FMKEY10_W<'_, FMKEYR0rs> {
        FMKEY10_W::new(self, 10)
    }
    ///Bit 11 - Fast master key bit 11 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey11(&mut self) -> FMKEY11_W<'_, FMKEYR0rs> {
        FMKEY11_W::new(self, 11)
    }
    ///Bit 12 - Fast master key bit 12 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey12(&mut self) -> FMKEY12_W<'_, FMKEYR0rs> {
        FMKEY12_W::new(self, 12)
    }
    ///Bit 13 - Fast master key bit 13 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey13(&mut self) -> FMKEY13_W<'_, FMKEYR0rs> {
        FMKEY13_W::new(self, 13)
    }
    ///Bit 14 - Fast master key bit 14 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey14(&mut self) -> FMKEY14_W<'_, FMKEYR0rs> {
        FMKEY14_W::new(self, 14)
    }
    ///Bit 15 - Fast master key bit 15 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey15(&mut self) -> FMKEY15_W<'_, FMKEYR0rs> {
        FMKEY15_W::new(self, 15)
    }
    ///Bit 16 - Fast master key bit 16 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey16(&mut self) -> FMKEY16_W<'_, FMKEYR0rs> {
        FMKEY16_W::new(self, 16)
    }
    ///Bit 17 - Fast master key bit 17 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey17(&mut self) -> FMKEY17_W<'_, FMKEYR0rs> {
        FMKEY17_W::new(self, 17)
    }
    ///Bit 18 - Fast master key bit 18 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey18(&mut self) -> FMKEY18_W<'_, FMKEYR0rs> {
        FMKEY18_W::new(self, 18)
    }
    ///Bit 19 - Fast master key bit 19 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey19(&mut self) -> FMKEY19_W<'_, FMKEYR0rs> {
        FMKEY19_W::new(self, 19)
    }
    ///Bit 20 - Fast master key bit 20 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey20(&mut self) -> FMKEY20_W<'_, FMKEYR0rs> {
        FMKEY20_W::new(self, 20)
    }
    ///Bit 21 - Fast master key bit 21 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey21(&mut self) -> FMKEY21_W<'_, FMKEYR0rs> {
        FMKEY21_W::new(self, 21)
    }
    ///Bit 22 - Fast master key bit 22 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey22(&mut self) -> FMKEY22_W<'_, FMKEYR0rs> {
        FMKEY22_W::new(self, 22)
    }
    ///Bit 23 - Fast master key bit 23 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey23(&mut self) -> FMKEY23_W<'_, FMKEYR0rs> {
        FMKEY23_W::new(self, 23)
    }
    ///Bit 24 - Fast master key bit 24 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey24(&mut self) -> FMKEY24_W<'_, FMKEYR0rs> {
        FMKEY24_W::new(self, 24)
    }
    ///Bit 25 - Fast master key bit 25 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey25(&mut self) -> FMKEY25_W<'_, FMKEYR0rs> {
        FMKEY25_W::new(self, 25)
    }
    ///Bit 26 - Fast master key bit 26 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey26(&mut self) -> FMKEY26_W<'_, FMKEYR0rs> {
        FMKEY26_W::new(self, 26)
    }
    ///Bit 27 - Fast master key bit 27 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey27(&mut self) -> FMKEY27_W<'_, FMKEYR0rs> {
        FMKEY27_W::new(self, 27)
    }
    ///Bit 28 - Fast master key bit 28 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey28(&mut self) -> FMKEY28_W<'_, FMKEYR0rs> {
        FMKEY28_W::new(self, 28)
    }
    ///Bit 29 - Fast master key bit 29 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey29(&mut self) -> FMKEY29_W<'_, FMKEYR0rs> {
        FMKEY29_W::new(self, 29)
    }
    ///Bit 30 - Fast master key bit 30 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey30(&mut self) -> FMKEY30_W<'_, FMKEYR0rs> {
        FMKEY30_W::new(self, 30)
    }
    ///Bit 31 - Fast master key bit 31 (i = 31 to 0)
    #[inline(always)]
    pub fn fmkey31(&mut self) -> FMKEY31_W<'_, FMKEYR0rs> {
        FMKEY31_W::new(self, 31)
    }
}
/**MCE fast master key 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmkeyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#MCE1:FMKEYR0)*/
pub struct FMKEYR0rs;
impl crate::RegisterSpec for FMKEYR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`fmkeyr0::W`](W) writer structure
impl crate::Writable for FMKEYR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FMKEYR0 to value 0
impl crate::Resettable for FMKEYR0rs {}
