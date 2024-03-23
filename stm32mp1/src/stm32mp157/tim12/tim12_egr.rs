#[doc = "Register `TIM12_EGR` writer"]
pub type W = crate::W<TIM12_EGRrs>;
#[doc = "Field `UG` writer - UG"]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - CC1G"]
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2G` writer - CC2G"]
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - TG"]
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - UG"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<TIM12_EGRrs> {
        UG_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1G"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<TIM12_EGRrs> {
        CC1G_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC2G"]
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CC2G_W<TIM12_EGRrs> {
        CC2G_W::new(self, 2)
    }
    #[doc = "Bit 6 - TG"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<TIM12_EGRrs> {
        TG_W::new(self, 6)
    }
}
#[doc = "TIM12 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim12_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM12_EGRrs;
impl crate::RegisterSpec for TIM12_EGRrs {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`tim12_egr::W`](W) writer structure"]
impl crate::Writable for TIM12_EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM12_EGR to value 0"]
impl crate::Resettable for TIM12_EGRrs {
    const RESET_VALUE: u16 = 0;
}
