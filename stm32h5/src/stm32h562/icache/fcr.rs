#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCRrs>;
#[doc = "Field `CBSYENDF` writer - clear busy end flag Set by software."]
pub type CBSYENDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERRF` writer - clear cache error flag Set by software."]
pub type CERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - clear busy end flag Set by software."]
    #[inline(always)]
    #[must_use]
    pub fn cbsyendf(&mut self) -> CBSYENDF_W<FCRrs> {
        CBSYENDF_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear cache error flag Set by software."]
    #[inline(always)]
    #[must_use]
    pub fn cerrf(&mut self) -> CERRF_W<FCRrs> {
        CERRF_W::new(self, 2)
    }
}
#[doc = "ICACHE flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCRrs {
    const RESET_VALUE: u32 = 0;
}
