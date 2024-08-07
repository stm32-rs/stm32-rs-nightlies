///Register `TIM2_CCR4` reader
pub type R = crate::R<TIM2_CCR4rs>;
///Register `TIM2_CCR4` writer
pub type W = crate::W<TIM2_CCR4rs>;
///Field `CCR4` reader - CCR4
pub type CCR4_R = crate::FieldReader<u16>;
///Field `CCR4` writer - CCR4
pub type CCR4_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - CCR4
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
    ///Bits 0:15 - CCR4
    #[inline(always)]
    #[must_use]
    pub fn ccr4(&mut self) -> CCR4_W<TIM2_CCR4rs> {
        CCR4_W::new(self, 0)
    }
}
/**TIM2 capture/compare register 4

You can [`read`](crate::Reg::read) this register and get [`tim2_ccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM2:TIM2_CCR4)*/
pub struct TIM2_CCR4rs;
impl crate::RegisterSpec for TIM2_CCR4rs {
    type Ux = u16;
}
///`read()` method returns [`tim2_ccr4::R`](R) reader structure
impl crate::Readable for TIM2_CCR4rs {}
///`write(|w| ..)` method takes [`tim2_ccr4::W`](W) writer structure
impl crate::Writable for TIM2_CCR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM2_CCR4 to value 0
impl crate::Resettable for TIM2_CCR4rs {
    const RESET_VALUE: u16 = 0;
}
