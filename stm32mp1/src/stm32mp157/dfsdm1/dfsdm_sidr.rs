#[doc = "Register `DFSDM_SIDR` reader"]
pub type R = crate::R<DFSDM_SIDRrs>;
#[doc = "Field `SID` reader - SID"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "This register specifies the size allocated to DFSDM registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_SIDRrs;
impl crate::RegisterSpec for DFSDM_SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_sidr::R`](R) reader structure"]
impl crate::Readable for DFSDM_SIDRrs {}
#[doc = "`reset()` method sets DFSDM_SIDR to value 0xa3c5_dd02"]
impl crate::Resettable for DFSDM_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd02;
}
