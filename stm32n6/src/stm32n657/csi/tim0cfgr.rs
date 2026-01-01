///Register `TIM0CFGR` reader
pub type R = crate::R<TIM0CFGRrs>;
///Register `TIM0CFGR` writer
pub type W = crate::W<TIM0CFGRrs>;
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
        f.debug_struct("TIM0CFGR")
            .field("count", &self.count())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - Clock cycle counter
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W<'_, TIM0CFGRrs> {
        COUNT_W::new(self, 0)
    }
}
/**CSI-2 Host timer 0 configuration register

You can [`read`](crate::Reg::read) this register and get [`tim0cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim0cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CSI:TIM0CFGR)*/
pub struct TIM0CFGRrs;
impl crate::RegisterSpec for TIM0CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`tim0cfgr::R`](R) reader structure
impl crate::Readable for TIM0CFGRrs {}
///`write(|w| ..)` method takes [`tim0cfgr::W`](W) writer structure
impl crate::Writable for TIM0CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM0CFGR to value 0
impl crate::Resettable for TIM0CFGRrs {}
