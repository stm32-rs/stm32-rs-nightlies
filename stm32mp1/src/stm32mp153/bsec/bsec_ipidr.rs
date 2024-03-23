#[doc = "Register `BSEC_IPIDR` reader"]
pub type R = crate::R<BSEC_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "BSEC identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_IPIDRrs;
impl crate::RegisterSpec for BSEC_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_ipidr::R`](R) reader structure"]
impl crate::Readable for BSEC_IPIDRrs {}
#[doc = "`reset()` method sets BSEC_IPIDR to value 0x0010_0032"]
impl crate::Resettable for BSEC_IPIDRrs {
    const RESET_VALUE: u32 = 0x0010_0032;
}
