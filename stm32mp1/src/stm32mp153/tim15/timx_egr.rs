#[doc = "Register `TIMx_EGR` writer"]
pub type W = crate::W<TIMX_EGRrs>;
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - Capture/compare 1 generation"]
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2G` writer - Capture/compare 2 generation"]
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMG` writer - COMG"]
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - Trigger generation"]
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG` writer - BG"]
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<TIMX_EGRrs> {
        UG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<TIMX_EGRrs> {
        CC1G_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CC2G_W<TIMX_EGRrs> {
        CC2G_W::new(self, 2)
    }
    #[doc = "Bit 5 - COMG"]
    #[inline(always)]
    #[must_use]
    pub fn comg(&mut self) -> COMG_W<TIMX_EGRrs> {
        COMG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<TIMX_EGRrs> {
        TG_W::new(self, 6)
    }
    #[doc = "Bit 7 - BG"]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BG_W<TIMX_EGRrs> {
        BG_W::new(self, 7)
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
