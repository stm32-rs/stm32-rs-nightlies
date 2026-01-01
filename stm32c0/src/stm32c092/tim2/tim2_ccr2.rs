///Register `TIM2_CCR2` reader
pub type R = crate::R<TIM2_CCR2rs>;
///Register `TIM2_CCR2` writer
pub type W = crate::W<TIM2_CCR2rs>;
///Field `CCR2` reader - Low Capture/Compare 2 value
pub type CCR2_R = crate::FieldReader<u32>;
///Field `CCR2` writer - Low Capture/Compare 2 value
pub type CCR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Low Capture/Compare 2 value
    #[inline(always)]
    pub fn ccr2(&self) -> CCR2_R {
        CCR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_CCR2")
            .field("ccr2", &self.ccr2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Low Capture/Compare 2 value
    #[inline(always)]
    pub fn ccr2(&mut self) -> CCR2_W<'_, TIM2_CCR2rs> {
        CCR2_W::new(self, 0)
    }
}
/**TIM2 capture/compare register 2

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM2:TIM2_CCR2)*/
pub struct TIM2_CCR2rs;
impl crate::RegisterSpec for TIM2_CCR2rs {
    type Ux = u32;
}
///`read()` method returns [`tim2_ccr2::R`](R) reader structure
impl crate::Readable for TIM2_CCR2rs {}
///`write(|w| ..)` method takes [`tim2_ccr2::W`](W) writer structure
impl crate::Writable for TIM2_CCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_CCR2 to value 0
impl crate::Resettable for TIM2_CCR2rs {}
