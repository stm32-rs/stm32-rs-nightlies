#[doc = "Register `DLYB_SIDR` reader"]
pub type R = crate::R<DLYB_SIDRrs>;
#[doc = "Field `SID` reader - SID"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "DLYB size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlyb_sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLYB_SIDRrs;
impl crate::RegisterSpec for DLYB_SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlyb_sidr::R`](R) reader structure"]
impl crate::Readable for DLYB_SIDRrs {}
#[doc = "`reset()` method sets DLYB_SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for DLYB_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
