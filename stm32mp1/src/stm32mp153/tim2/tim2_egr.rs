#[doc = "Register `TIM2_EGR` writer"]
pub type W = crate::W<TIM2_EGRrs>;
#[doc = "Field `UG` writer - UG"]
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - CC1G"]
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2G` writer - CC2G"]
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3G` writer - CC3G"]
pub type CC3G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4G` writer - CC4G"]
pub type CC4G_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMG` writer - COMG"]
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - TG"]
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BG` writer - BG"]
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2G` writer - B2G"]
pub type B2G_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - UG"]
    #[inline(always)]
    #[must_use]
    pub fn ug(&mut self) -> UG_W<TIM2_EGRrs> {
        UG_W::new(self, 0)
    }
    #[doc = "Bit 1 - CC1G"]
    #[inline(always)]
    #[must_use]
    pub fn cc1g(&mut self) -> CC1G_W<TIM2_EGRrs> {
        CC1G_W::new(self, 1)
    }
    #[doc = "Bit 2 - CC2G"]
    #[inline(always)]
    #[must_use]
    pub fn cc2g(&mut self) -> CC2G_W<TIM2_EGRrs> {
        CC2G_W::new(self, 2)
    }
    #[doc = "Bit 3 - CC3G"]
    #[inline(always)]
    #[must_use]
    pub fn cc3g(&mut self) -> CC3G_W<TIM2_EGRrs> {
        CC3G_W::new(self, 3)
    }
    #[doc = "Bit 4 - CC4G"]
    #[inline(always)]
    #[must_use]
    pub fn cc4g(&mut self) -> CC4G_W<TIM2_EGRrs> {
        CC4G_W::new(self, 4)
    }
    #[doc = "Bit 5 - COMG"]
    #[inline(always)]
    #[must_use]
    pub fn comg(&mut self) -> COMG_W<TIM2_EGRrs> {
        COMG_W::new(self, 5)
    }
    #[doc = "Bit 6 - TG"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<TIM2_EGRrs> {
        TG_W::new(self, 6)
    }
    #[doc = "Bit 7 - BG"]
    #[inline(always)]
    #[must_use]
    pub fn bg(&mut self) -> BG_W<TIM2_EGRrs> {
        BG_W::new(self, 7)
    }
    #[doc = "Bit 8 - B2G"]
    #[inline(always)]
    #[must_use]
    pub fn b2g(&mut self) -> B2G_W<TIM2_EGRrs> {
        B2G_W::new(self, 8)
    }
}
#[doc = "TIM2 event generation register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim2_egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM2_EGRrs;
impl crate::RegisterSpec for TIM2_EGRrs {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`tim2_egr::W`](W) writer structure"]
impl crate::Writable for TIM2_EGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets TIM2_EGR to value 0"]
impl crate::Resettable for TIM2_EGRrs {
    const RESET_VALUE: u16 = 0;
}
