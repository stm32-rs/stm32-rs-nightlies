///Register `FTSR2` reader
pub type R = crate::R<FTSR2rs>;
///Register `FTSR2` writer
pub type W = crate::W<FTSR2rs>;
///Field `FT39` reader - Falling trigger event configuration bit of configurable event input x
pub type FT39_R = crate::BitReader;
///Field `FT39` writer - Falling trigger event configuration bit of configurable event input x
pub type FT39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT40` reader - Falling trigger event configuration bit of configurable event input x
pub type FT40_R = crate::BitReader;
///Field `FT40` writer - Falling trigger event configuration bit of configurable event input x
pub type FT40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT51` reader - Falling trigger event configuration bit of configurable event input 51
pub type FT51_R = crate::BitReader;
///Field `FT51` writer - Falling trigger event configuration bit of configurable event input 51
pub type FT51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT54` reader - Falling trigger event configuration bit of configurable event input 54
pub type FT54_R = crate::BitReader;
///Field `FT54` writer - Falling trigger event configuration bit of configurable event input 54
pub type FT54_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FT56` reader - Falling trigger event configuration bit of configurable event input 56
pub type FT56_R = crate::BitReader;
///Field `FT56` writer - Falling trigger event configuration bit of configurable event input 56
pub type FT56_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft39(&self) -> FT39_R {
        FT39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft40(&self) -> FT40_R {
        FT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 19 - Falling trigger event configuration bit of configurable event input 51
    #[inline(always)]
    pub fn ft51(&self) -> FT51_R {
        FT51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Falling trigger event configuration bit of configurable event input 54
    #[inline(always)]
    pub fn ft54(&self) -> FT54_R {
        FT54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Falling trigger event configuration bit of configurable event input 56
    #[inline(always)]
    pub fn ft56(&self) -> FT56_R {
        FT56_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FTSR2")
            .field("ft39", &self.ft39())
            .field("ft40", &self.ft40())
            .field("ft51", &self.ft51())
            .field("ft54", &self.ft54())
            .field("ft56", &self.ft56())
            .finish()
    }
}
impl W {
    ///Bit 7 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft39(&mut self) -> FT39_W<'_, FTSR2rs> {
        FT39_W::new(self, 7)
    }
    ///Bit 8 - Falling trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn ft40(&mut self) -> FT40_W<'_, FTSR2rs> {
        FT40_W::new(self, 8)
    }
    ///Bit 19 - Falling trigger event configuration bit of configurable event input 51
    #[inline(always)]
    pub fn ft51(&mut self) -> FT51_W<'_, FTSR2rs> {
        FT51_W::new(self, 19)
    }
    ///Bit 22 - Falling trigger event configuration bit of configurable event input 54
    #[inline(always)]
    pub fn ft54(&mut self) -> FT54_W<'_, FTSR2rs> {
        FT54_W::new(self, 22)
    }
    ///Bit 24 - Falling trigger event configuration bit of configurable event input 56
    #[inline(always)]
    pub fn ft56(&mut self) -> FT56_W<'_, FTSR2rs> {
        FT56_W::new(self, 24)
    }
}
/**EXTI falling trigger selection register

You can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#EXTI:FTSR2)*/
pub struct FTSR2rs;
impl crate::RegisterSpec for FTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`ftsr2::R`](R) reader structure
impl crate::Readable for FTSR2rs {}
///`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure
impl crate::Writable for FTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FTSR2 to value 0
impl crate::Resettable for FTSR2rs {}
