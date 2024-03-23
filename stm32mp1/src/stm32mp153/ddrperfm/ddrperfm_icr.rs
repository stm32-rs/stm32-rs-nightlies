#[doc = "Register `DDRPERFM_ICR` writer"]
pub type W = crate::W<DDRPERFM_ICRrs>;
#[doc = "Field `OVF` writer - OVF"]
pub type OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - OVF"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<DDRPERFM_ICRrs> {
        OVF_W::new(self, 0)
    }
}
#[doc = "Write-only register. A read request returns all zeros\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrperfm_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_ICRrs;
impl crate::RegisterSpec for DDRPERFM_ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ddrperfm_icr::W`](W) writer structure"]
impl crate::Writable for DDRPERFM_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPERFM_ICR to value 0"]
impl crate::Resettable for DDRPERFM_ICRrs {
    const RESET_VALUE: u32 = 0;
}
