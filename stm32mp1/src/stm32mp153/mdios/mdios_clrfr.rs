#[doc = "Register `MDIOS_CLRFR` reader"]
pub type R = crate::R<MDIOS_CLRFRrs>;
#[doc = "Register `MDIOS_CLRFR` writer"]
pub type W = crate::W<MDIOS_CLRFRrs>;
#[doc = "Field `CPERF` reader - CPERF"]
pub type CPERF_R = crate::BitReader;
#[doc = "Field `CPERF` writer - CPERF"]
pub type CPERF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSERF` reader - CSERF"]
pub type CSERF_R = crate::BitReader;
#[doc = "Field `CSERF` writer - CSERF"]
pub type CSERF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTERF` reader - CTERF"]
pub type CTERF_R = crate::BitReader;
#[doc = "Field `CTERF` writer - CTERF"]
pub type CTERF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPERF"]
    #[inline(always)]
    pub fn cperf(&self) -> CPERF_R {
        CPERF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSERF"]
    #[inline(always)]
    pub fn cserf(&self) -> CSERF_R {
        CSERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTERF"]
    #[inline(always)]
    pub fn cterf(&self) -> CTERF_R {
        CTERF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPERF"]
    #[inline(always)]
    #[must_use]
    pub fn cperf(&mut self) -> CPERF_W<MDIOS_CLRFRrs> {
        CPERF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CSERF"]
    #[inline(always)]
    #[must_use]
    pub fn cserf(&mut self) -> CSERF_W<MDIOS_CLRFRrs> {
        CSERF_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTERF"]
    #[inline(always)]
    #[must_use]
    pub fn cterf(&mut self) -> CTERF_W<MDIOS_CLRFRrs> {
        CTERF_W::new(self, 2)
    }
}
#[doc = "MDIOS clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_clrfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdios_clrfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_CLRFRrs;
impl crate::RegisterSpec for MDIOS_CLRFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_clrfr::R`](R) reader structure"]
impl crate::Readable for MDIOS_CLRFRrs {}
#[doc = "`write(|w| ..)` method takes [`mdios_clrfr::W`](W) writer structure"]
impl crate::Writable for MDIOS_CLRFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDIOS_CLRFR to value 0"]
impl crate::Resettable for MDIOS_CLRFRrs {
    const RESET_VALUE: u32 = 0;
}
