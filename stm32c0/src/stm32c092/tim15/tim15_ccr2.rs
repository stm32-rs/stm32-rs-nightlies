///Register `TIM15_CCR2` reader
pub type R = crate::R<TIM15_CCR2rs>;
///Register `TIM15_CCR2` writer
pub type W = crate::W<TIM15_CCR2rs>;
///Field `CCR2` reader - Capture/Compare 2 value
pub type CCR2_R = crate::FieldReader<u16>;
///Field `CCR2` writer - Capture/Compare 2 value
pub type CCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Capture/Compare 2 value
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_CCR2")
            .field("ccr2", &self.ccr2())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare 2 value
    #[inline(always)]
    pub fn ccr2(&mut self) -> CCR2_W<'_, TIM15_CCR2rs> {
        CCR2_W::new(self, 0)
    }
}
/**TIM15 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim15_ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM15:TIM15_CCR2)*/
pub struct TIM15_CCR2rs;
impl crate::RegisterSpec for TIM15_CCR2rs {
    type Ux = u16;
}
///`read()` method returns [`tim15_ccr2::R`](R) reader structure
impl crate::Readable for TIM15_CCR2rs {}
///`write(|w| ..)` method takes [`tim15_ccr2::W`](W) writer structure
impl crate::Writable for TIM15_CCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_CCR2 to value 0
impl crate::Resettable for TIM15_CCR2rs {}
