#[doc = "Register `BSEC_SIDR` reader"]
pub type R = crate::R<BSEC_SIDRrs>;
#[doc = "Field `SID` reader - SID"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "BSEC size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_SIDRrs;
impl crate::RegisterSpec for BSEC_SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_sidr::R`](R) reader structure"]
impl crate::Readable for BSEC_SIDRrs {}
#[doc = "`reset()` method sets BSEC_SIDR to value 0xa3c5_dd04"]
impl crate::Resettable for BSEC_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd04;
}
