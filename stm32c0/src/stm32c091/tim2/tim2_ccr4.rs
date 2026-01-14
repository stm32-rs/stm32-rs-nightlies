///Register `TIM2_CCR4` reader
pub type R = crate::R<TIM2_CCR4rs>;
///Register `TIM2_CCR4` writer
pub type W = crate::W<TIM2_CCR4rs>;
///Field `CCR4` reader - Low Capture/Compare value
pub type CCR4_R = crate::FieldReader<u32>;
///Field `CCR4` writer - Low Capture/Compare value
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Low Capture/Compare value
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_CCR4")
            .field("ccr4", &self.ccr4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Low Capture/Compare value
    #[inline(always)]
    pub fn ccr4(&mut self) -> CCR4_W<'_, TIM2_CCR4rs> {
        CCR4_W::new(self, 0)
    }
}
/**TIM2 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM2:TIM2_CCR4)*/
pub struct TIM2_CCR4rs;
impl crate::RegisterSpec for TIM2_CCR4rs {
    type Ux = u32;
}
///`read()` method returns [`tim2_ccr4::R`](R) reader structure
impl crate::Readable for TIM2_CCR4rs {}
///`write(|w| ..)` method takes [`tim2_ccr4::W`](W) writer structure
impl crate::Writable for TIM2_CCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_CCR4 to value 0
impl crate::Resettable for TIM2_CCR4rs {}
