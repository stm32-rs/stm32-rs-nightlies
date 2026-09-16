///Register `RTSR2` reader
pub type R = crate::R<RTSR2rs>;
///Register `RTSR2` writer
pub type W = crate::W<RTSR2rs>;
///Field `RT39` reader - Rising trigger event configuration bit of configurable event input x
pub type RT39_R = crate::BitReader;
///Field `RT39` writer - Rising trigger event configuration bit of configurable event input x
pub type RT39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT40` reader - Rising trigger event configuration bit of configurable event input x
pub type RT40_R = crate::BitReader;
///Field `RT40` writer - Rising trigger event configuration bit of configurable event input x
pub type RT40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT51` reader - Rising trigger event configuration bit of configurable event input 51
pub type RT51_R = crate::BitReader;
///Field `RT51` writer - Rising trigger event configuration bit of configurable event input 51
pub type RT51_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT54` reader - Rising trigger event configuration bit of configurable event input 54
pub type RT54_R = crate::BitReader;
///Field `RT54` writer - Rising trigger event configuration bit of configurable event input 54
pub type RT54_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RT56` reader - Rising trigger event configuration bit of configurable event input 56
pub type RT56_R = crate::BitReader;
///Field `RT56` writer - Rising trigger event configuration bit of configurable event input 56
pub type RT56_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt39(&self) -> RT39_R {
        RT39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt40(&self) -> RT40_R {
        RT40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 19 - Rising trigger event configuration bit of configurable event input 51
    #[inline(always)]
    pub fn rt51(&self) -> RT51_R {
        RT51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 22 - Rising trigger event configuration bit of configurable event input 54
    #[inline(always)]
    pub fn rt54(&self) -> RT54_R {
        RT54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Rising trigger event configuration bit of configurable event input 56
    #[inline(always)]
    pub fn rt56(&self) -> RT56_R {
        RT56_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTSR2")
            .field("rt39", &self.rt39())
            .field("rt40", &self.rt40())
            .field("rt51", &self.rt51())
            .field("rt54", &self.rt54())
            .field("rt56", &self.rt56())
            .finish()
    }
}
impl W {
    ///Bit 7 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt39(&mut self) -> RT39_W<'_, RTSR2rs> {
        RT39_W::new(self, 7)
    }
    ///Bit 8 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt40(&mut self) -> RT40_W<'_, RTSR2rs> {
        RT40_W::new(self, 8)
    }
    ///Bit 19 - Rising trigger event configuration bit of configurable event input 51
    #[inline(always)]
    pub fn rt51(&mut self) -> RT51_W<'_, RTSR2rs> {
        RT51_W::new(self, 19)
    }
    ///Bit 22 - Rising trigger event configuration bit of configurable event input 54
    #[inline(always)]
    pub fn rt54(&mut self) -> RT54_W<'_, RTSR2rs> {
        RT54_W::new(self, 22)
    }
    ///Bit 24 - Rising trigger event configuration bit of configurable event input 56
    #[inline(always)]
    pub fn rt56(&mut self) -> RT56_W<'_, RTSR2rs> {
        RT56_W::new(self, 24)
    }
}
/**EXTI rising trigger selection register

You can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#EXTI:RTSR2)*/
pub struct RTSR2rs;
impl crate::RegisterSpec for RTSR2rs {
    type Ux = u32;
}
///`read()` method returns [`rtsr2::R`](R) reader structure
impl crate::Readable for RTSR2rs {}
///`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure
impl crate::Writable for RTSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTSR2 to value 0
impl crate::Resettable for RTSR2rs {}
