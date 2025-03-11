///Register `BSRR` writer
pub type W = crate::W<BSRRrs>;
///Field `BSy0` writer - BSy0
pub type BSY0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy1` writer - BSy1
pub type BSY1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy2` writer - BSy2
pub type BSY2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy3` writer - BSy3
pub type BSY3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy4` writer - BSy4
pub type BSY4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy5` writer - BSy5
pub type BSY5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy6` writer - BSy6
pub type BSY6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy7` writer - BSy7
pub type BSY7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy8` writer - BSy8
pub type BSY8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy9` writer - BSy9
pub type BSY9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy10` writer - BSy10
pub type BSY10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy11` writer - BSy11
pub type BSY11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy12` writer - BSy12
pub type BSY12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy13` writer - BSy13
pub type BSY13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy14` writer - BSy14
pub type BSY14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BSy15` writer - BSy15
pub type BSY15_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy16` writer - BRy16
pub type BRY16_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy17` writer - BRy17
pub type BRY17_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy18` writer - BRy18
pub type BRY18_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy19` writer - BRy19
pub type BRY19_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy20` writer - BRy20
pub type BRY20_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy21` writer - BRy21
pub type BRY21_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy22` writer - BRy22
pub type BRY22_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy23` writer - BRy23
pub type BRY23_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy24` writer - BRy24
pub type BRY24_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy25` writer - BRy25
pub type BRY25_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy26` writer - BRy26
pub type BRY26_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy27` writer - BRy27
pub type BRY27_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy28` writer - BRy28
pub type BRY28_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy29` writer - BRy29
pub type BRY29_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy30` writer - BRy30
pub type BRY30_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRy31` writer - BRy31
pub type BRY31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BSRRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - BSy0
    #[inline(always)]
    pub fn bsy0(&mut self) -> BSY0_W<BSRRrs> {
        BSY0_W::new(self, 0)
    }
    ///Bit 1 - BSy1
    #[inline(always)]
    pub fn bsy1(&mut self) -> BSY1_W<BSRRrs> {
        BSY1_W::new(self, 1)
    }
    ///Bit 2 - BSy2
    #[inline(always)]
    pub fn bsy2(&mut self) -> BSY2_W<BSRRrs> {
        BSY2_W::new(self, 2)
    }
    ///Bit 3 - BSy3
    #[inline(always)]
    pub fn bsy3(&mut self) -> BSY3_W<BSRRrs> {
        BSY3_W::new(self, 3)
    }
    ///Bit 4 - BSy4
    #[inline(always)]
    pub fn bsy4(&mut self) -> BSY4_W<BSRRrs> {
        BSY4_W::new(self, 4)
    }
    ///Bit 5 - BSy5
    #[inline(always)]
    pub fn bsy5(&mut self) -> BSY5_W<BSRRrs> {
        BSY5_W::new(self, 5)
    }
    ///Bit 6 - BSy6
    #[inline(always)]
    pub fn bsy6(&mut self) -> BSY6_W<BSRRrs> {
        BSY6_W::new(self, 6)
    }
    ///Bit 7 - BSy7
    #[inline(always)]
    pub fn bsy7(&mut self) -> BSY7_W<BSRRrs> {
        BSY7_W::new(self, 7)
    }
    ///Bit 8 - BSy8
    #[inline(always)]
    pub fn bsy8(&mut self) -> BSY8_W<BSRRrs> {
        BSY8_W::new(self, 8)
    }
    ///Bit 9 - BSy9
    #[inline(always)]
    pub fn bsy9(&mut self) -> BSY9_W<BSRRrs> {
        BSY9_W::new(self, 9)
    }
    ///Bit 10 - BSy10
    #[inline(always)]
    pub fn bsy10(&mut self) -> BSY10_W<BSRRrs> {
        BSY10_W::new(self, 10)
    }
    ///Bit 11 - BSy11
    #[inline(always)]
    pub fn bsy11(&mut self) -> BSY11_W<BSRRrs> {
        BSY11_W::new(self, 11)
    }
    ///Bit 12 - BSy12
    #[inline(always)]
    pub fn bsy12(&mut self) -> BSY12_W<BSRRrs> {
        BSY12_W::new(self, 12)
    }
    ///Bit 13 - BSy13
    #[inline(always)]
    pub fn bsy13(&mut self) -> BSY13_W<BSRRrs> {
        BSY13_W::new(self, 13)
    }
    ///Bit 14 - BSy14
    #[inline(always)]
    pub fn bsy14(&mut self) -> BSY14_W<BSRRrs> {
        BSY14_W::new(self, 14)
    }
    ///Bit 15 - BSy15
    #[inline(always)]
    pub fn bsy15(&mut self) -> BSY15_W<BSRRrs> {
        BSY15_W::new(self, 15)
    }
    ///Bit 16 - BRy16
    #[inline(always)]
    pub fn bry16(&mut self) -> BRY16_W<BSRRrs> {
        BRY16_W::new(self, 16)
    }
    ///Bit 17 - BRy17
    #[inline(always)]
    pub fn bry17(&mut self) -> BRY17_W<BSRRrs> {
        BRY17_W::new(self, 17)
    }
    ///Bit 18 - BRy18
    #[inline(always)]
    pub fn bry18(&mut self) -> BRY18_W<BSRRrs> {
        BRY18_W::new(self, 18)
    }
    ///Bit 19 - BRy19
    #[inline(always)]
    pub fn bry19(&mut self) -> BRY19_W<BSRRrs> {
        BRY19_W::new(self, 19)
    }
    ///Bit 20 - BRy20
    #[inline(always)]
    pub fn bry20(&mut self) -> BRY20_W<BSRRrs> {
        BRY20_W::new(self, 20)
    }
    ///Bit 21 - BRy21
    #[inline(always)]
    pub fn bry21(&mut self) -> BRY21_W<BSRRrs> {
        BRY21_W::new(self, 21)
    }
    ///Bit 22 - BRy22
    #[inline(always)]
    pub fn bry22(&mut self) -> BRY22_W<BSRRrs> {
        BRY22_W::new(self, 22)
    }
    ///Bit 23 - BRy23
    #[inline(always)]
    pub fn bry23(&mut self) -> BRY23_W<BSRRrs> {
        BRY23_W::new(self, 23)
    }
    ///Bit 24 - BRy24
    #[inline(always)]
    pub fn bry24(&mut self) -> BRY24_W<BSRRrs> {
        BRY24_W::new(self, 24)
    }
    ///Bit 25 - BRy25
    #[inline(always)]
    pub fn bry25(&mut self) -> BRY25_W<BSRRrs> {
        BRY25_W::new(self, 25)
    }
    ///Bit 26 - BRy26
    #[inline(always)]
    pub fn bry26(&mut self) -> BRY26_W<BSRRrs> {
        BRY26_W::new(self, 26)
    }
    ///Bit 27 - BRy27
    #[inline(always)]
    pub fn bry27(&mut self) -> BRY27_W<BSRRrs> {
        BRY27_W::new(self, 27)
    }
    ///Bit 28 - BRy28
    #[inline(always)]
    pub fn bry28(&mut self) -> BRY28_W<BSRRrs> {
        BRY28_W::new(self, 28)
    }
    ///Bit 29 - BRy29
    #[inline(always)]
    pub fn bry29(&mut self) -> BRY29_W<BSRRrs> {
        BRY29_W::new(self, 29)
    }
    ///Bit 30 - BRy30
    #[inline(always)]
    pub fn bry30(&mut self) -> BRY30_W<BSRRrs> {
        BRY30_W::new(self, 30)
    }
    ///Bit 31 - BRy31
    #[inline(always)]
    pub fn bry31(&mut self) -> BRY31_W<BSRRrs> {
        BRY31_W::new(self, 31)
    }
}
/**LPGPIO port bit set/reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPGPIO1:BSRR)*/
pub struct BSRRrs;
impl crate::RegisterSpec for BSRRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`bsrr::W`](W) writer structure
impl crate::Writable for BSRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BSRR to value 0
impl crate::Resettable for BSRRrs {}
