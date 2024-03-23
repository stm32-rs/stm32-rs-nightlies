#[doc = "Register `EGR` writer"]
pub type W = crate::W<EGRrs>;
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - Capture/compare 1 generation"]
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2G` writer - Capture/compare 2 generation"]
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3G` writer - Capture/compare 3 generation"]
pub type CC3G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4G` writer - Capture/compare 4 generation"]
pub type CC4G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - Trigger generation"]
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<EGRrs> {
        UG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<EGRrs> {
        CC1G_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CC2G_W<EGRrs> {
        CC2G_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc3g(&mut self) -> CC3G_W<EGRrs> {
        CC3G_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline(always)]
    #[must_use]
    pub fn cc4g(&mut self) -> CC4G_W<EGRrs> {
        CC4G_W::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<EGRrs> {
        TG_W::new(self, 6)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EGRrs;
impl crate::RegisterSpec for EGRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EGRrs {
    const RESET_VALUE: u32 = 0;
}
