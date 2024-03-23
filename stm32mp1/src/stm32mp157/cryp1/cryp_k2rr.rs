#[doc = "Register `CRYP_K2RR` writer"]
pub type W = crate::W<CRYP_K2RRrs>;
#[doc = "Field `K` writer - K"]
pub type K_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - K"]
    #[inline(always)]
    #[must_use]
    pub fn k(&mut self) -> K_W<CRYP_K2RRrs> {
        K_W::new(self, 0)
    }
}
#[doc = "Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_k2rr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_K2RRrs;
impl crate::RegisterSpec for CRYP_K2RRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cryp_k2rr::W`](W) writer structure"]
impl crate::Writable for CRYP_K2RRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYP_K2RR to value 0"]
impl crate::Resettable for CRYP_K2RRrs {
    const RESET_VALUE: u32 = 0;
}
