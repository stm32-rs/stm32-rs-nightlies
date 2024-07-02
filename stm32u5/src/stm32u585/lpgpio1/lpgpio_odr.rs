///Register `LPGPIO_ODR` reader
pub type R = crate::R<LPGPIO_ODRrs>;
///Register `LPGPIO_ODR` writer
pub type W = crate::W<LPGPIO_ODRrs>;
///Field `ODy0` reader - ODy0
pub type ODY0_R = crate::BitReader;
///Field `ODy0` writer - ODy0
pub type ODY0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy1` reader - ODy1
pub type ODY1_R = crate::BitReader;
///Field `ODy1` writer - ODy1
pub type ODY1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy2` reader - ODy2
pub type ODY2_R = crate::BitReader;
///Field `ODy2` writer - ODy2
pub type ODY2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy3` reader - ODy3
pub type ODY3_R = crate::BitReader;
///Field `ODy3` writer - ODy3
pub type ODY3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy4` reader - ODy4
pub type ODY4_R = crate::BitReader;
///Field `ODy4` writer - ODy4
pub type ODY4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy5` reader - ODy5
pub type ODY5_R = crate::BitReader;
///Field `ODy5` writer - ODy5
pub type ODY5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy6` reader - ODy6
pub type ODY6_R = crate::BitReader;
///Field `ODy6` writer - ODy6
pub type ODY6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy7` reader - ODy7
pub type ODY7_R = crate::BitReader;
///Field `ODy7` writer - ODy7
pub type ODY7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy8` reader - ODy8
pub type ODY8_R = crate::BitReader;
///Field `ODy8` writer - ODy8
pub type ODY8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy9` reader - ODy9
pub type ODY9_R = crate::BitReader;
///Field `ODy9` writer - ODy9
pub type ODY9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy10` reader - ODy10
pub type ODY10_R = crate::BitReader;
///Field `ODy10` writer - ODy10
pub type ODY10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy11` reader - ODy11
pub type ODY11_R = crate::BitReader;
///Field `ODy11` writer - ODy11
pub type ODY11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy12` reader - ODy12
pub type ODY12_R = crate::BitReader;
///Field `ODy12` writer - ODy12
pub type ODY12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy13` reader - ODy13
pub type ODY13_R = crate::BitReader;
///Field `ODy13` writer - ODy13
pub type ODY13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy14` reader - ODy14
pub type ODY14_R = crate::BitReader;
///Field `ODy14` writer - ODy14
pub type ODY14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODy15` reader - ODy15
pub type ODY15_R = crate::BitReader;
///Field `ODy15` writer - ODy15
pub type ODY15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ODy0
    #[inline(always)]
    pub fn ody0(&self) -> ODY0_R {
        ODY0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ODy1
    #[inline(always)]
    pub fn ody1(&self) -> ODY1_R {
        ODY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ODy2
    #[inline(always)]
    pub fn ody2(&self) -> ODY2_R {
        ODY2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ODy3
    #[inline(always)]
    pub fn ody3(&self) -> ODY3_R {
        ODY3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ODy4
    #[inline(always)]
    pub fn ody4(&self) -> ODY4_R {
        ODY4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ODy5
    #[inline(always)]
    pub fn ody5(&self) -> ODY5_R {
        ODY5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ODy6
    #[inline(always)]
    pub fn ody6(&self) -> ODY6_R {
        ODY6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ODy7
    #[inline(always)]
    pub fn ody7(&self) -> ODY7_R {
        ODY7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ODy8
    #[inline(always)]
    pub fn ody8(&self) -> ODY8_R {
        ODY8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ODy9
    #[inline(always)]
    pub fn ody9(&self) -> ODY9_R {
        ODY9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ODy10
    #[inline(always)]
    pub fn ody10(&self) -> ODY10_R {
        ODY10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ODy11
    #[inline(always)]
    pub fn ody11(&self) -> ODY11_R {
        ODY11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ODy12
    #[inline(always)]
    pub fn ody12(&self) -> ODY12_R {
        ODY12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ODy13
    #[inline(always)]
    pub fn ody13(&self) -> ODY13_R {
        ODY13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ODy14
    #[inline(always)]
    pub fn ody14(&self) -> ODY14_R {
        ODY14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ODy15
    #[inline(always)]
    pub fn ody15(&self) -> ODY15_R {
        ODY15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPGPIO_ODR")
            .field("ody15", &self.ody15())
            .field("ody14", &self.ody14())
            .field("ody13", &self.ody13())
            .field("ody12", &self.ody12())
            .field("ody11", &self.ody11())
            .field("ody10", &self.ody10())
            .field("ody9", &self.ody9())
            .field("ody8", &self.ody8())
            .field("ody7", &self.ody7())
            .field("ody6", &self.ody6())
            .field("ody5", &self.ody5())
            .field("ody4", &self.ody4())
            .field("ody3", &self.ody3())
            .field("ody2", &self.ody2())
            .field("ody1", &self.ody1())
            .field("ody0", &self.ody0())
            .finish()
    }
}
impl W {
    ///Bit 0 - ODy0
    #[inline(always)]
    #[must_use]
    pub fn ody0(&mut self) -> ODY0_W<LPGPIO_ODRrs> {
        ODY0_W::new(self, 0)
    }
    ///Bit 1 - ODy1
    #[inline(always)]
    #[must_use]
    pub fn ody1(&mut self) -> ODY1_W<LPGPIO_ODRrs> {
        ODY1_W::new(self, 1)
    }
    ///Bit 2 - ODy2
    #[inline(always)]
    #[must_use]
    pub fn ody2(&mut self) -> ODY2_W<LPGPIO_ODRrs> {
        ODY2_W::new(self, 2)
    }
    ///Bit 3 - ODy3
    #[inline(always)]
    #[must_use]
    pub fn ody3(&mut self) -> ODY3_W<LPGPIO_ODRrs> {
        ODY3_W::new(self, 3)
    }
    ///Bit 4 - ODy4
    #[inline(always)]
    #[must_use]
    pub fn ody4(&mut self) -> ODY4_W<LPGPIO_ODRrs> {
        ODY4_W::new(self, 4)
    }
    ///Bit 5 - ODy5
    #[inline(always)]
    #[must_use]
    pub fn ody5(&mut self) -> ODY5_W<LPGPIO_ODRrs> {
        ODY5_W::new(self, 5)
    }
    ///Bit 6 - ODy6
    #[inline(always)]
    #[must_use]
    pub fn ody6(&mut self) -> ODY6_W<LPGPIO_ODRrs> {
        ODY6_W::new(self, 6)
    }
    ///Bit 7 - ODy7
    #[inline(always)]
    #[must_use]
    pub fn ody7(&mut self) -> ODY7_W<LPGPIO_ODRrs> {
        ODY7_W::new(self, 7)
    }
    ///Bit 8 - ODy8
    #[inline(always)]
    #[must_use]
    pub fn ody8(&mut self) -> ODY8_W<LPGPIO_ODRrs> {
        ODY8_W::new(self, 8)
    }
    ///Bit 9 - ODy9
    #[inline(always)]
    #[must_use]
    pub fn ody9(&mut self) -> ODY9_W<LPGPIO_ODRrs> {
        ODY9_W::new(self, 9)
    }
    ///Bit 10 - ODy10
    #[inline(always)]
    #[must_use]
    pub fn ody10(&mut self) -> ODY10_W<LPGPIO_ODRrs> {
        ODY10_W::new(self, 10)
    }
    ///Bit 11 - ODy11
    #[inline(always)]
    #[must_use]
    pub fn ody11(&mut self) -> ODY11_W<LPGPIO_ODRrs> {
        ODY11_W::new(self, 11)
    }
    ///Bit 12 - ODy12
    #[inline(always)]
    #[must_use]
    pub fn ody12(&mut self) -> ODY12_W<LPGPIO_ODRrs> {
        ODY12_W::new(self, 12)
    }
    ///Bit 13 - ODy13
    #[inline(always)]
    #[must_use]
    pub fn ody13(&mut self) -> ODY13_W<LPGPIO_ODRrs> {
        ODY13_W::new(self, 13)
    }
    ///Bit 14 - ODy14
    #[inline(always)]
    #[must_use]
    pub fn ody14(&mut self) -> ODY14_W<LPGPIO_ODRrs> {
        ODY14_W::new(self, 14)
    }
    ///Bit 15 - ODy15
    #[inline(always)]
    #[must_use]
    pub fn ody15(&mut self) -> ODY15_W<LPGPIO_ODRrs> {
        ODY15_W::new(self, 15)
    }
}
/**LPGPIO port output data register

You can [`read`](crate::Reg::read) this register and get [`lpgpio_odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpgpio_odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#LPGPIO1:LPGPIO_ODR)*/
pub struct LPGPIO_ODRrs;
impl crate::RegisterSpec for LPGPIO_ODRrs {
    type Ux = u32;
}
///`read()` method returns [`lpgpio_odr::R`](R) reader structure
impl crate::Readable for LPGPIO_ODRrs {}
///`write(|w| ..)` method takes [`lpgpio_odr::W`](W) writer structure
impl crate::Writable for LPGPIO_ODRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPGPIO_ODR to value 0
impl crate::Resettable for LPGPIO_ODRrs {
    const RESET_VALUE: u32 = 0;
}
