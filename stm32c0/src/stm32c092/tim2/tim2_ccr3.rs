///Register `TIM2_CCR3` reader
pub type R = crate::R<TIM2_CCR3rs>;
///Register `TIM2_CCR3` writer
pub type W = crate::W<TIM2_CCR3rs>;
///Field `CCR3` reader - Low Capture/Compare value
pub type CCR3_R = crate::FieldReader<u32>;
///Field `CCR3` writer - Low Capture/Compare value
pub type CCR3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Low Capture/Compare value
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_CCR3")
            .field("ccr3", &self.ccr3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Low Capture/Compare value
    #[inline(always)]
    pub fn ccr3(&mut self) -> CCR3_W<'_, TIM2_CCR3rs> {
        CCR3_W::new(self, 0)
    }
}
/**TIM2 capture/compare register 3

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM2:TIM2_CCR3)*/
pub struct TIM2_CCR3rs;
impl crate::RegisterSpec for TIM2_CCR3rs {
    type Ux = u32;
}
///`read()` method returns [`tim2_ccr3::R`](R) reader structure
impl crate::Readable for TIM2_CCR3rs {}
///`write(|w| ..)` method takes [`tim2_ccr3::W`](W) writer structure
impl crate::Writable for TIM2_CCR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_CCR3 to value 0
impl crate::Resettable for TIM2_CCR3rs {}
