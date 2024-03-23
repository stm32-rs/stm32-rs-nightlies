#[doc = "Register `BCLRFR` writer"]
pub type W = crate::W<BCLRFRrs>;
#[doc = "Field `COVRUDR` writer - Clear overrun / underrun"]
pub type COVRUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUTEDET` writer - Mute detection flag"]
pub type CMUTEDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWCKCFG` writer - Clear wrong clock configuration flag"]
pub type CWCKCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCNRDY` writer - Clear codec not ready flag"]
pub type CCNRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag"]
pub type CAFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLFSDET` writer - Clear late frame synchronization detection flag"]
pub type CLFSDET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    #[must_use]
    pub fn covrudr(&mut self) -> COVRUDR_W<BCLRFRrs> {
        COVRUDR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<BCLRFRrs> {
        CMUTEDET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    #[must_use]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<BCLRFRrs> {
        CWCKCFG_W::new(self, 2)
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<BCLRFRrs> {
        CCNRDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn cafsdet(&mut self) -> CAFSDET_W<BCLRFRrs> {
        CAFSDET_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn clfsdet(&mut self) -> CLFSDET_W<BCLRFRrs> {
        CLFSDET_W::new(self, 6)
    }
}
#[doc = "B Clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bclrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCLRFRrs;
impl crate::RegisterSpec for BCLRFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bclrfr::W`](W) writer structure"]
impl crate::Writable for BCLRFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCLRFR to value 0"]
impl crate::Resettable for BCLRFRrs {
    const RESET_VALUE: u32 = 0;
}
