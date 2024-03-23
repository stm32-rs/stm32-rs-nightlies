#[doc = "Register `CRYP_MISR` reader"]
pub type R = crate::R<CRYP_MISRrs>;
#[doc = "Field `INMIS` reader - INMIS"]
pub type INMIS_R = crate::BitReader;
#[doc = "Field `OUTMIS` reader - OUTMIS"]
pub type OUTMIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - INMIS"]
    #[inline(always)]
    pub fn inmis(&self) -> INMIS_R {
        INMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUTMIS"]
    #[inline(always)]
    pub fn outmis(&self) -> OUTMIS_R {
        OUTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "The CRYP_MISR register is the masked interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current masked status of the corresponding interrupt, i.e. the interrupt information taking CRYP_IMSCR mask into account. Write operations have no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_MISRrs;
impl crate::RegisterSpec for CRYP_MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_misr::R`](R) reader structure"]
impl crate::Readable for CRYP_MISRrs {}
#[doc = "`reset()` method sets CRYP_MISR to value 0"]
impl crate::Resettable for CRYP_MISRrs {
    const RESET_VALUE: u32 = 0;
}
