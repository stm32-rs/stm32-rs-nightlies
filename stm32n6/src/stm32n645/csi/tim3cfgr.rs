///Register `TIM3CFGR` reader
pub type R = crate::R<TIM3CFGRrs>;
///Register `TIM3CFGR` writer
pub type W = crate::W<TIM3CFGRrs>;
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
        f.debug_struct("TIM3CFGR")
            .field("count", &self.count())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - Clock cycle counter
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W<'_, TIM3CFGRrs> {
        COUNT_W::new(self, 0)
    }
}
/**CSI-2 Host timer 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`tim3cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#CSI:TIM3CFGR)*/
pub struct TIM3CFGRrs;
impl crate::RegisterSpec for TIM3CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`tim3cfgr::R`](R) reader structure
impl crate::Readable for TIM3CFGRrs {}
///`write(|w| ..)` method takes [`tim3cfgr::W`](W) writer structure
impl crate::Writable for TIM3CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM3CFGR to value 0
impl crate::Resettable for TIM3CFGRrs {}
