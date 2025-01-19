///Register `TIM7_SR` reader
pub type R = crate::R<TIM7_SRrs>;
///Register `TIM7_SR` writer
pub type W = crate::W<TIM7_SRrs>;
///Field `UIF` reader - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value and if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS1=10 and UDIS1=10 in the TIMx_CR1 register.
pub type UIF_R = crate::BitReader;
///Field `UIF` writer - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value and if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS1=10 and UDIS1=10 in the TIMx_CR1 register.
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value and if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS1=10 and UDIS1=10 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM7_SR").field("uif", &self.uif()).finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value and if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS1=10 and UDIS1=10 in the TIMx_CR1 register.
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<TIM7_SRrs> {
        UIF_W::new(self, 0)
    }
}
/**TIM7 status register

You can [`read`](crate::Reg::read) this register and get [`tim7_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim7_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM7:TIM7_SR)*/
pub struct TIM7_SRrs;
impl crate::RegisterSpec for TIM7_SRrs {
    type Ux = u16;
}
///`read()` method returns [`tim7_sr::R`](R) reader structure
impl crate::Readable for TIM7_SRrs {}
///`write(|w| ..)` method takes [`tim7_sr::W`](W) writer structure
impl crate::Writable for TIM7_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM7_SR to value 0
impl crate::Resettable for TIM7_SRrs {
    const RESET_VALUE: u16 = 0;
}
