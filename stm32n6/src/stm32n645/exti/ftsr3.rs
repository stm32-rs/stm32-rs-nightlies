///Register `FTSR3` reader
pub type R = crate::R<FTSR3rs>;
///Register `FTSR3` writer
pub type W = crate::W<FTSR3rs>;
///Field `FT66` reader - Falling trigger event configuration bit of configurable event input 66
pub type FT66_R = crate::BitReader;
///Field `FT66` writer - Falling trigger event configuration bit of configurable event input 66
pub type FT66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT68` reader - Falling trigger event configuration bit of configurable event input x
pub type FT68_R = crate::BitReader;
///Field `FT68` writer - Falling trigger event configuration bit of configurable event input x
pub type FT68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT69` reader - Falling trigger event configuration bit of configurable event input x
pub type FT69_R = crate::BitReader;
///Field `FT69` writer - Falling trigger event configuration bit of configurable event input x
pub type FT69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT70` reader - Falling trigger event configuration bit of configurable event input x
pub type FT70_R = crate::BitReader;
///Field `FT70` writer - Falling trigger event configuration bit of configurable event input x
pub type FT70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT71` reader - Falling trigger event configuration bit of configurable event input x
pub type FT71_R = crate::BitReader;
///Field `FT71` writer - Falling trigger event configuration bit of configurable event input x
pub type FT71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT72` reader - Falling trigger event configuration bit of configurable event input x
pub type FT72_R = crate::BitReader;
///Field `FT72` writer - Falling trigger event configuration bit of configurable event input x
pub type FT72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT73` reader - Falling trigger event configuration bit of configurable event input x
pub type FT73_R = crate::BitReader;
///Field `FT73` writer - Falling trigger event configuration bit of configurable event input x
pub type FT73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT74` reader - Falling trigger event configuration bit of configurable event input x
pub type FT74_R = crate::BitReader;
///Field `FT74` writer - Falling trigger event configuration bit of configurable event input x
pub type FT74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Falling trigger event configuration bit of configurable event input 66
    #[inline(always)]
    pub fn ft66(&self) -> FT66_R {
        FT66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft68(&self) -> FT68_R {
        FT68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft69(&self) -> FT69_R {
        FT69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft70(&self) -> FT70_R {
        FT70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft71(&self) -> FT71_R {
        FT71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft72(&self) -> FT72_R {
        FT72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft73(&self) -> FT73_R {
        FT73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft74(&self) -> FT74_R {
        FT74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR3")
            .field("ft66", &self.ft66())
            .field("ft68", &self.ft68())
            .field("ft69", &self.ft69())
            .field("ft70", &self.ft70())
            .field("ft71", &self.ft71())
            .field("ft72", &self.ft72())
            .field("ft73", &self.ft73())
            .field("ft74", &self.ft74())
            .finish()
    }
}
impl W {
    ///Bit 2 - Falling trigger event configuration bit of configurable event input 66
    #[inline(always)]
    pub fn ft66(&mut self) -> FT66_W<'_, FTSR3rs> {
        FT66_W::new(self, 2)
    }
    ///Bit 4 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft68(&mut self) -> FT68_W<'_, FTSR3rs> {
        FT68_W::new(self, 4)
    }
    ///Bit 5 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft69(&mut self) -> FT69_W<'_, FTSR3rs> {
        FT69_W::new(self, 5)
    }
    ///Bit 6 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft70(&mut self) -> FT70_W<'_, FTSR3rs> {
        FT70_W::new(self, 6)
    }
    ///Bit 7 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft71(&mut self) -> FT71_W<'_, FTSR3rs> {
        FT71_W::new(self, 7)
    }
    ///Bit 8 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft72(&mut self) -> FT72_W<'_, FTSR3rs> {
        FT72_W::new(self, 8)
    }
    ///Bit 9 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft73(&mut self) -> FT73_W<'_, FTSR3rs> {
        FT73_W::new(self, 9)
    }
    ///Bit 10 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft74(&mut self) -> FT74_W<'_, FTSR3rs> {
        FT74_W::new(self, 10)
    }
}
/**EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#EXTI:FTSR3)*/
pub struct FTSR3rs;
impl crate::RegisterSpec for FTSR3rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr3::R`](R) reader structure
impl crate::Readable for FTSR3rs {}
///`write(|w| ..)` method takes [`ftsr3::W`](W) writer structure
impl crate::Writable for FTSR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTSR3 to value 0
impl crate::Resettable for FTSR3rs {}
