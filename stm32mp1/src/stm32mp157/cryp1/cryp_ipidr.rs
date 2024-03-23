#[doc = "Register `CRYP_IPIDR` reader"]
pub type R = crate::R<CRYP_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "CRYP Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_IPIDRrs;
impl crate::RegisterSpec for CRYP_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_ipidr::R`](R) reader structure"]
impl crate::Readable for CRYP_IPIDRrs {}
#[doc = "`reset()` method sets CRYP_IPIDR to value 0x0017_0011"]
impl crate::Resettable for CRYP_IPIDRrs {
    const RESET_VALUE: u32 = 0x0017_0011;
}
