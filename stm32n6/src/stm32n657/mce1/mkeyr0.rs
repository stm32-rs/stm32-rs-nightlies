///Register `MKEYR0` writer
pub type W = crate::W<MKEYR0rs>;
///Field `MKEY0` writer - Master key bit 0 (i = 31 to 0)
pub type MKEY0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY1` writer - Master key bit 1 (i = 31 to 0)
pub type MKEY1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY2` writer - Master key bit 2 (i = 31 to 0)
pub type MKEY2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY3` writer - Master key bit 3 (i = 31 to 0)
pub type MKEY3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY4` writer - Master key bit 4 (i = 31 to 0)
pub type MKEY4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY5` writer - Master key bit 5 (i = 31 to 0)
pub type MKEY5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY6` writer - Master key bit 6 (i = 31 to 0)
pub type MKEY6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY7` writer - Master key bit 7 (i = 31 to 0)
pub type MKEY7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY8` writer - Master key bit 8 (i = 31 to 0)
pub type MKEY8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY9` writer - Master key bit 9 (i = 31 to 0)
pub type MKEY9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY10` writer - Master key bit 10 (i = 31 to 0)
pub type MKEY10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY11` writer - Master key bit 11 (i = 31 to 0)
pub type MKEY11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY12` writer - Master key bit 12 (i = 31 to 0)
pub type MKEY12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY13` writer - Master key bit 13 (i = 31 to 0)
pub type MKEY13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY14` writer - Master key bit 14 (i = 31 to 0)
pub type MKEY14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY15` writer - Master key bit 15 (i = 31 to 0)
pub type MKEY15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY16` writer - Master key bit 16 (i = 31 to 0)
pub type MKEY16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY17` writer - Master key bit 17 (i = 31 to 0)
pub type MKEY17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY18` writer - Master key bit 18 (i = 31 to 0)
pub type MKEY18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY19` writer - Master key bit 19 (i = 31 to 0)
pub type MKEY19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY20` writer - Master key bit 20 (i = 31 to 0)
pub type MKEY20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY21` writer - Master key bit 21 (i = 31 to 0)
pub type MKEY21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY22` writer - Master key bit 22 (i = 31 to 0)
pub type MKEY22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY23` writer - Master key bit 23 (i = 31 to 0)
pub type MKEY23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY24` writer - Master key bit 24 (i = 31 to 0)
pub type MKEY24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY25` writer - Master key bit 25 (i = 31 to 0)
pub type MKEY25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY26` writer - Master key bit 26 (i = 31 to 0)
pub type MKEY26_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY27` writer - Master key bit 27 (i = 31 to 0)
pub type MKEY27_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY28` writer - Master key bit 28 (i = 31 to 0)
pub type MKEY28_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY29` writer - Master key bit 29 (i = 31 to 0)
pub type MKEY29_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY30` writer - Master key bit 30 (i = 31 to 0)
pub type MKEY30_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MKEY31` writer - Master key bit 31 (i = 31 to 0)
pub type MKEY31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<MKEYR0rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Master key bit 0 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey0(&mut self) -> MKEY0_W<'_, MKEYR0rs> {
        MKEY0_W::new(self, 0)
    }
    ///Bit 1 - Master key bit 1 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey1(&mut self) -> MKEY1_W<'_, MKEYR0rs> {
        MKEY1_W::new(self, 1)
    }
    ///Bit 2 - Master key bit 2 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey2(&mut self) -> MKEY2_W<'_, MKEYR0rs> {
        MKEY2_W::new(self, 2)
    }
    ///Bit 3 - Master key bit 3 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey3(&mut self) -> MKEY3_W<'_, MKEYR0rs> {
        MKEY3_W::new(self, 3)
    }
    ///Bit 4 - Master key bit 4 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey4(&mut self) -> MKEY4_W<'_, MKEYR0rs> {
        MKEY4_W::new(self, 4)
    }
    ///Bit 5 - Master key bit 5 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey5(&mut self) -> MKEY5_W<'_, MKEYR0rs> {
        MKEY5_W::new(self, 5)
    }
    ///Bit 6 - Master key bit 6 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey6(&mut self) -> MKEY6_W<'_, MKEYR0rs> {
        MKEY6_W::new(self, 6)
    }
    ///Bit 7 - Master key bit 7 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey7(&mut self) -> MKEY7_W<'_, MKEYR0rs> {
        MKEY7_W::new(self, 7)
    }
    ///Bit 8 - Master key bit 8 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey8(&mut self) -> MKEY8_W<'_, MKEYR0rs> {
        MKEY8_W::new(self, 8)
    }
    ///Bit 9 - Master key bit 9 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey9(&mut self) -> MKEY9_W<'_, MKEYR0rs> {
        MKEY9_W::new(self, 9)
    }
    ///Bit 10 - Master key bit 10 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey10(&mut self) -> MKEY10_W<'_, MKEYR0rs> {
        MKEY10_W::new(self, 10)
    }
    ///Bit 11 - Master key bit 11 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey11(&mut self) -> MKEY11_W<'_, MKEYR0rs> {
        MKEY11_W::new(self, 11)
    }
    ///Bit 12 - Master key bit 12 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey12(&mut self) -> MKEY12_W<'_, MKEYR0rs> {
        MKEY12_W::new(self, 12)
    }
    ///Bit 13 - Master key bit 13 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey13(&mut self) -> MKEY13_W<'_, MKEYR0rs> {
        MKEY13_W::new(self, 13)
    }
    ///Bit 14 - Master key bit 14 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey14(&mut self) -> MKEY14_W<'_, MKEYR0rs> {
        MKEY14_W::new(self, 14)
    }
    ///Bit 15 - Master key bit 15 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey15(&mut self) -> MKEY15_W<'_, MKEYR0rs> {
        MKEY15_W::new(self, 15)
    }
    ///Bit 16 - Master key bit 16 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey16(&mut self) -> MKEY16_W<'_, MKEYR0rs> {
        MKEY16_W::new(self, 16)
    }
    ///Bit 17 - Master key bit 17 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey17(&mut self) -> MKEY17_W<'_, MKEYR0rs> {
        MKEY17_W::new(self, 17)
    }
    ///Bit 18 - Master key bit 18 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey18(&mut self) -> MKEY18_W<'_, MKEYR0rs> {
        MKEY18_W::new(self, 18)
    }
    ///Bit 19 - Master key bit 19 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey19(&mut self) -> MKEY19_W<'_, MKEYR0rs> {
        MKEY19_W::new(self, 19)
    }
    ///Bit 20 - Master key bit 20 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey20(&mut self) -> MKEY20_W<'_, MKEYR0rs> {
        MKEY20_W::new(self, 20)
    }
    ///Bit 21 - Master key bit 21 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey21(&mut self) -> MKEY21_W<'_, MKEYR0rs> {
        MKEY21_W::new(self, 21)
    }
    ///Bit 22 - Master key bit 22 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey22(&mut self) -> MKEY22_W<'_, MKEYR0rs> {
        MKEY22_W::new(self, 22)
    }
    ///Bit 23 - Master key bit 23 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey23(&mut self) -> MKEY23_W<'_, MKEYR0rs> {
        MKEY23_W::new(self, 23)
    }
    ///Bit 24 - Master key bit 24 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey24(&mut self) -> MKEY24_W<'_, MKEYR0rs> {
        MKEY24_W::new(self, 24)
    }
    ///Bit 25 - Master key bit 25 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey25(&mut self) -> MKEY25_W<'_, MKEYR0rs> {
        MKEY25_W::new(self, 25)
    }
    ///Bit 26 - Master key bit 26 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey26(&mut self) -> MKEY26_W<'_, MKEYR0rs> {
        MKEY26_W::new(self, 26)
    }
    ///Bit 27 - Master key bit 27 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey27(&mut self) -> MKEY27_W<'_, MKEYR0rs> {
        MKEY27_W::new(self, 27)
    }
    ///Bit 28 - Master key bit 28 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey28(&mut self) -> MKEY28_W<'_, MKEYR0rs> {
        MKEY28_W::new(self, 28)
    }
    ///Bit 29 - Master key bit 29 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey29(&mut self) -> MKEY29_W<'_, MKEYR0rs> {
        MKEY29_W::new(self, 29)
    }
    ///Bit 30 - Master key bit 30 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey30(&mut self) -> MKEY30_W<'_, MKEYR0rs> {
        MKEY30_W::new(self, 30)
    }
    ///Bit 31 - Master key bit 31 (i = 31 to 0)
    #[inline(always)]
    pub fn mkey31(&mut self) -> MKEY31_W<'_, MKEYR0rs> {
        MKEY31_W::new(self, 31)
    }
}
/**.MCE master key 0

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkeyr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:MKEYR0)*/
pub struct MKEYR0rs;
impl crate::RegisterSpec for MKEYR0rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`mkeyr0::W`](W) writer structure
impl crate::Writable for MKEYR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MKEYR0 to value 0
impl crate::Resettable for MKEYR0rs {}
