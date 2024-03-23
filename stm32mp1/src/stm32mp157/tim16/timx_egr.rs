#[doc = "Register `TIMx_EGR` writer"]
pub type W = crate::W<TIMX_EGRrs>;
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<TIMX_EGRrs> {
        UG_W::new(self, 0)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timx_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMX_EGRrs;
impl crate::RegisterSpec for TIMX_EGRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timx_egr::W`](W) writer structure"]
impl crate::Writable for TIMX_EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMx_EGR to value 0"]
impl crate::Resettable for TIMX_EGRrs {
    const RESET_VALUE: u32 = 0;
}
