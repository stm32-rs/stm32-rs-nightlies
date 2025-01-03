///Register `TIM6_EGR` writer
pub type W = crate::W<TIM6_EGRrs>;
///Field `UG` writer - Update generation This bit can be set by software, it is automatically cleared by hardware.
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TIM6_EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation This bit can be set by software, it is automatically cleared by hardware.
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<TIM6_EGRrs> {
        UG_W::new(self, 0)
    }
}
/**TIM6 event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim6_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM6:TIM6_EGR)*/
pub struct TIM6_EGRrs;
impl crate::RegisterSpec for TIM6_EGRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`tim6_egr::W`](W) writer structure
impl crate::Writable for TIM6_EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM6_EGR to value 0
impl crate::Resettable for TIM6_EGRrs {
    const RESET_VALUE: u16 = 0;
}
