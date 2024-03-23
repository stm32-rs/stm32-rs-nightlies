#[doc = "Register `TIM14_EGR` writer"]
pub type W = crate::W<TIM14_EGRrs>;
#[doc = "Field `UG` writer - UG"]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - CC1G"]
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - UG"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<TIM14_EGRrs> {
        UG_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1G"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<TIM14_EGRrs> {
        CC1G_W::new(self, 1)
    }
}
#[doc = "TIM14 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim14_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM14_EGRrs;
impl crate::RegisterSpec for TIM14_EGRrs {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`tim14_egr::W`](W) writer structure"]
impl crate::Writable for TIM14_EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM14_EGR to value 0"]
impl crate::Resettable for TIM14_EGRrs {
    const RESET_VALUE: u16 = 0;
}
