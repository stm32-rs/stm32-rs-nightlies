#[doc = "Register `MDMA_C20IFCR` writer"]
pub type W = crate::W<MDMA_C20IFCRrs>;
#[doc = "Field `CTEIF` writer - CTEIF"]
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF` writer - CCTCIF"]
pub type CCTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF` writer - CBRTIF"]
pub type CBRTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF` writer - CBTIF"]
pub type CBTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF` writer - CLTCIF"]
pub type CLTCIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CTEIF"]
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<MDMA_C20IFCRrs> {
        CTEIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CCTCIF"]
    #[inline(always)]
    #[must_use]
    pub fn cctcif(&mut self) -> CCTCIF_W<MDMA_C20IFCRrs> {
        CCTCIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - CBRTIF"]
    #[inline(always)]
    #[must_use]
    pub fn cbrtif(&mut self) -> CBRTIF_W<MDMA_C20IFCRrs> {
        CBRTIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - CBTIF"]
    #[inline(always)]
    #[must_use]
    pub fn cbtif(&mut self) -> CBTIF_W<MDMA_C20IFCRrs> {
        CBTIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - CLTCIF"]
    #[inline(always)]
    #[must_use]
    pub fn cltcif(&mut self) -> CLTCIF_W<MDMA_C20IFCRrs> {
        CLTCIF_W::new(self, 4)
    }
}
#[doc = "MDMA channel 20 interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c20ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C20IFCRrs;
impl crate::RegisterSpec for MDMA_C20IFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c20ifcr::W`](W) writer structure"]
impl crate::Writable for MDMA_C20IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C20IFCR to value 0"]
impl crate::Resettable for MDMA_C20IFCRrs {
    const RESET_VALUE: u32 = 0;
}
