///Register `TIMx_EGR` writer
pub type W = crate::W<TIMX_EGRrs>;
///Field `UG` writer - Update generation
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TIMX_EGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Update generation
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<TIMX_EGRrs> {
        UG_W::new(self, 0)
    }
}
/**event generation register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timx_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM16:TIMx_EGR)*/
pub struct TIMX_EGRrs;
impl crate::RegisterSpec for TIMX_EGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`timx_egr::W`](W) writer structure
impl crate::Writable for TIMX_EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMx_EGR to value 0
impl crate::Resettable for TIMX_EGRrs {
    const RESET_VALUE: u32 = 0;
}
