#[doc = "Register `CLRFR` reader"]
pub type R = crate::R<CLRFRrs>;
#[doc = "Register `CLRFR` writer"]
pub type W = crate::W<CLRFRrs>;
#[doc = "Field `CPERF` reader - Clear the preamble error flag"]
pub type CPERF_R = crate::BitReader;
#[doc = "Field `CPERF` writer - Clear the preamble error flag"]
pub type CPERF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSERF` reader - Clear the start error flag"]
pub type CSERF_R = crate::BitReader;
#[doc = "Field `CSERF` writer - Clear the start error flag"]
pub type CSERF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTERF` reader - Clear the turnaround error flag"]
pub type CTERF_R = crate::BitReader;
#[doc = "Field `CTERF` writer - Clear the turnaround error flag"]
pub type CTERF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    pub fn cperf(&self) -> CPERF_R {
        CPERF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    pub fn cserf(&self) -> CSERF_R {
        CSERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    pub fn cterf(&self) -> CTERF_R {
        CTERF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cperf(&mut self) -> CPERF_W<CLRFRrs> {
        CPERF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cserf(&mut self) -> CSERF_W<CLRFRrs> {
        CSERF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    #[must_use]
    pub fn cterf(&mut self) -> CTERF_W<CLRFRrs> {
        CTERF_W::new(self, 2)
    }
}
#[doc = "MDIOS clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clrfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLRFRrs;
impl crate::RegisterSpec for CLRFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clrfr::R`](R) reader structure"]
impl crate::Readable for CLRFRrs {}
#[doc = "`write(|w| ..)` method takes [`clrfr::W`](W) writer structure"]
impl crate::Writable for CLRFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFRrs {
    const RESET_VALUE: u32 = 0;
}
