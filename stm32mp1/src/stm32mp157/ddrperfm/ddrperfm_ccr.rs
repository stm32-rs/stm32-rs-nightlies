#[doc = "Register `DDRPERFM_CCR` writer"]
pub type W = crate::W<DDRPERFM_CCRrs>;
#[doc = "Field `CCLR` writer - CCLR"]
pub type CCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCLR` writer - TCLR"]
pub type TCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:3 - CCLR"]
    #[inline(always)]
    #[must_use]
    pub fn cclr(&mut self) -> CCLR_W<DDRPERFM_CCRrs> {
        CCLR_W::new(self, 0)
    }
    #[doc = "Bit 31 - TCLR"]
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<DDRPERFM_CCRrs> {
        TCLR_W::new(self, 31)
    }
}
#[doc = "Write-only register. A read request returns all zeros\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrperfm_ccr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_CCRrs;
impl crate::RegisterSpec for DDRPERFM_CCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ddrperfm_ccr::W`](W) writer structure"]
impl crate::Writable for DDRPERFM_CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPERFM_CCR to value 0"]
impl crate::Resettable for DDRPERFM_CCRrs {
    const RESET_VALUE: u32 = 0;
}
