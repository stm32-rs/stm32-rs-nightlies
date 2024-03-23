#[doc = "Register `LPGPIO_BSRR` writer"]
pub type W = crate::W<LPGPIO_BSRRrs>;
#[doc = "Field `BSy0` writer - BSy0"]
pub type BSY0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy1` writer - BSy1"]
pub type BSY1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy2` writer - BSy2"]
pub type BSY2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy3` writer - BSy3"]
pub type BSY3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy4` writer - BSy4"]
pub type BSY4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy5` writer - BSy5"]
pub type BSY5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy6` writer - BSy6"]
pub type BSY6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy7` writer - BSy7"]
pub type BSY7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy8` writer - BSy8"]
pub type BSY8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy9` writer - BSy9"]
pub type BSY9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy10` writer - BSy10"]
pub type BSY10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy11` writer - BSy11"]
pub type BSY11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy12` writer - BSy12"]
pub type BSY12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy13` writer - BSy13"]
pub type BSY13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy14` writer - BSy14"]
pub type BSY14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSy15` writer - BSy15"]
pub type BSY15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy16` writer - BRy16"]
pub type BRY16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy17` writer - BRy17"]
pub type BRY17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy18` writer - BRy18"]
pub type BRY18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy19` writer - BRy19"]
pub type BRY19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy20` writer - BRy20"]
pub type BRY20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy21` writer - BRy21"]
pub type BRY21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy22` writer - BRy22"]
pub type BRY22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy23` writer - BRy23"]
pub type BRY23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy24` writer - BRy24"]
pub type BRY24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy25` writer - BRy25"]
pub type BRY25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy26` writer - BRy26"]
pub type BRY26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy27` writer - BRy27"]
pub type BRY27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy28` writer - BRy28"]
pub type BRY28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy29` writer - BRy29"]
pub type BRY29_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy30` writer - BRy30"]
pub type BRY30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRy31` writer - BRy31"]
pub type BRY31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - BSy0"]
    #[inline(always)]
    #[must_use]
    pub fn bsy0(&mut self) -> BSY0_W<LPGPIO_BSRRrs> {
        BSY0_W::new(self, 0)
    }
    #[doc = "Bit 1 - BSy1"]
    #[inline(always)]
    #[must_use]
    pub fn bsy1(&mut self) -> BSY1_W<LPGPIO_BSRRrs> {
        BSY1_W::new(self, 1)
    }
    #[doc = "Bit 2 - BSy2"]
    #[inline(always)]
    #[must_use]
    pub fn bsy2(&mut self) -> BSY2_W<LPGPIO_BSRRrs> {
        BSY2_W::new(self, 2)
    }
    #[doc = "Bit 3 - BSy3"]
    #[inline(always)]
    #[must_use]
    pub fn bsy3(&mut self) -> BSY3_W<LPGPIO_BSRRrs> {
        BSY3_W::new(self, 3)
    }
    #[doc = "Bit 4 - BSy4"]
    #[inline(always)]
    #[must_use]
    pub fn bsy4(&mut self) -> BSY4_W<LPGPIO_BSRRrs> {
        BSY4_W::new(self, 4)
    }
    #[doc = "Bit 5 - BSy5"]
    #[inline(always)]
    #[must_use]
    pub fn bsy5(&mut self) -> BSY5_W<LPGPIO_BSRRrs> {
        BSY5_W::new(self, 5)
    }
    #[doc = "Bit 6 - BSy6"]
    #[inline(always)]
    #[must_use]
    pub fn bsy6(&mut self) -> BSY6_W<LPGPIO_BSRRrs> {
        BSY6_W::new(self, 6)
    }
    #[doc = "Bit 7 - BSy7"]
    #[inline(always)]
    #[must_use]
    pub fn bsy7(&mut self) -> BSY7_W<LPGPIO_BSRRrs> {
        BSY7_W::new(self, 7)
    }
    #[doc = "Bit 8 - BSy8"]
    #[inline(always)]
    #[must_use]
    pub fn bsy8(&mut self) -> BSY8_W<LPGPIO_BSRRrs> {
        BSY8_W::new(self, 8)
    }
    #[doc = "Bit 9 - BSy9"]
    #[inline(always)]
    #[must_use]
    pub fn bsy9(&mut self) -> BSY9_W<LPGPIO_BSRRrs> {
        BSY9_W::new(self, 9)
    }
    #[doc = "Bit 10 - BSy10"]
    #[inline(always)]
    #[must_use]
    pub fn bsy10(&mut self) -> BSY10_W<LPGPIO_BSRRrs> {
        BSY10_W::new(self, 10)
    }
    #[doc = "Bit 11 - BSy11"]
    #[inline(always)]
    #[must_use]
    pub fn bsy11(&mut self) -> BSY11_W<LPGPIO_BSRRrs> {
        BSY11_W::new(self, 11)
    }
    #[doc = "Bit 12 - BSy12"]
    #[inline(always)]
    #[must_use]
    pub fn bsy12(&mut self) -> BSY12_W<LPGPIO_BSRRrs> {
        BSY12_W::new(self, 12)
    }
    #[doc = "Bit 13 - BSy13"]
    #[inline(always)]
    #[must_use]
    pub fn bsy13(&mut self) -> BSY13_W<LPGPIO_BSRRrs> {
        BSY13_W::new(self, 13)
    }
    #[doc = "Bit 14 - BSy14"]
    #[inline(always)]
    #[must_use]
    pub fn bsy14(&mut self) -> BSY14_W<LPGPIO_BSRRrs> {
        BSY14_W::new(self, 14)
    }
    #[doc = "Bit 15 - BSy15"]
    #[inline(always)]
    #[must_use]
    pub fn bsy15(&mut self) -> BSY15_W<LPGPIO_BSRRrs> {
        BSY15_W::new(self, 15)
    }
    #[doc = "Bit 16 - BRy16"]
    #[inline(always)]
    #[must_use]
    pub fn bry16(&mut self) -> BRY16_W<LPGPIO_BSRRrs> {
        BRY16_W::new(self, 16)
    }
    #[doc = "Bit 17 - BRy17"]
    #[inline(always)]
    #[must_use]
    pub fn bry17(&mut self) -> BRY17_W<LPGPIO_BSRRrs> {
        BRY17_W::new(self, 17)
    }
    #[doc = "Bit 18 - BRy18"]
    #[inline(always)]
    #[must_use]
    pub fn bry18(&mut self) -> BRY18_W<LPGPIO_BSRRrs> {
        BRY18_W::new(self, 18)
    }
    #[doc = "Bit 19 - BRy19"]
    #[inline(always)]
    #[must_use]
    pub fn bry19(&mut self) -> BRY19_W<LPGPIO_BSRRrs> {
        BRY19_W::new(self, 19)
    }
    #[doc = "Bit 20 - BRy20"]
    #[inline(always)]
    #[must_use]
    pub fn bry20(&mut self) -> BRY20_W<LPGPIO_BSRRrs> {
        BRY20_W::new(self, 20)
    }
    #[doc = "Bit 21 - BRy21"]
    #[inline(always)]
    #[must_use]
    pub fn bry21(&mut self) -> BRY21_W<LPGPIO_BSRRrs> {
        BRY21_W::new(self, 21)
    }
    #[doc = "Bit 22 - BRy22"]
    #[inline(always)]
    #[must_use]
    pub fn bry22(&mut self) -> BRY22_W<LPGPIO_BSRRrs> {
        BRY22_W::new(self, 22)
    }
    #[doc = "Bit 23 - BRy23"]
    #[inline(always)]
    #[must_use]
    pub fn bry23(&mut self) -> BRY23_W<LPGPIO_BSRRrs> {
        BRY23_W::new(self, 23)
    }
    #[doc = "Bit 24 - BRy24"]
    #[inline(always)]
    #[must_use]
    pub fn bry24(&mut self) -> BRY24_W<LPGPIO_BSRRrs> {
        BRY24_W::new(self, 24)
    }
    #[doc = "Bit 25 - BRy25"]
    #[inline(always)]
    #[must_use]
    pub fn bry25(&mut self) -> BRY25_W<LPGPIO_BSRRrs> {
        BRY25_W::new(self, 25)
    }
    #[doc = "Bit 26 - BRy26"]
    #[inline(always)]
    #[must_use]
    pub fn bry26(&mut self) -> BRY26_W<LPGPIO_BSRRrs> {
        BRY26_W::new(self, 26)
    }
    #[doc = "Bit 27 - BRy27"]
    #[inline(always)]
    #[must_use]
    pub fn bry27(&mut self) -> BRY27_W<LPGPIO_BSRRrs> {
        BRY27_W::new(self, 27)
    }
    #[doc = "Bit 28 - BRy28"]
    #[inline(always)]
    #[must_use]
    pub fn bry28(&mut self) -> BRY28_W<LPGPIO_BSRRrs> {
        BRY28_W::new(self, 28)
    }
    #[doc = "Bit 29 - BRy29"]
    #[inline(always)]
    #[must_use]
    pub fn bry29(&mut self) -> BRY29_W<LPGPIO_BSRRrs> {
        BRY29_W::new(self, 29)
    }
    #[doc = "Bit 30 - BRy30"]
    #[inline(always)]
    #[must_use]
    pub fn bry30(&mut self) -> BRY30_W<LPGPIO_BSRRrs> {
        BRY30_W::new(self, 30)
    }
    #[doc = "Bit 31 - BRy31"]
    #[inline(always)]
    #[must_use]
    pub fn bry31(&mut self) -> BRY31_W<LPGPIO_BSRRrs> {
        BRY31_W::new(self, 31)
    }
}
#[doc = "LPGPIO port bit set/reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpgpio_bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPGPIO_BSRRrs;
impl crate::RegisterSpec for LPGPIO_BSRRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lpgpio_bsrr::W`](W) writer structure"]
impl crate::Writable for LPGPIO_BSRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPGPIO_BSRR to value 0"]
impl crate::Resettable for LPGPIO_BSRRrs {
    const RESET_VALUE: u32 = 0;
}
