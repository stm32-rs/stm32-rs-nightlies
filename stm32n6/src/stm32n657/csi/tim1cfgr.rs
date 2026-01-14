///Register `TIM1CFGR` reader
pub type R = crate::R<TIM1CFGRrs>;
///Register `TIM1CFGR` writer
pub type W = crate::W<TIM1CFGRrs>;
///Field `COUNT` reader - Clock cycle counter
pub type COUNT_R = crate::FieldReader<u32>;
///Field `COUNT` writer - Clock cycle counter
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - Clock cycle counter
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1CFGR")
            .field("count", &self.count())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - Clock cycle counter
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W<'_, TIM1CFGRrs> {
        COUNT_W::new(self, 0)
    }
}
/**CSI-2 Host timer 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`tim1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CSI:TIM1CFGR)*/
pub struct TIM1CFGRrs;
impl crate::RegisterSpec for TIM1CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`tim1cfgr::R`](R) reader structure
impl crate::Readable for TIM1CFGRrs {}
///`write(|w| ..)` method takes [`tim1cfgr::W`](W) writer structure
impl crate::Writable for TIM1CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1CFGR to value 0
impl crate::Resettable for TIM1CFGRrs {}
