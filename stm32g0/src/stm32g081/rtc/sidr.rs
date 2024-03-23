#[doc = "Register `SIDR` reader"]
pub type R = crate::R<SIDRrs>;
#[doc = "Field `SID` reader - Size Identification"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Size Identification"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "EXTI Size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIDRrs;
impl crate::RegisterSpec for SIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidr::R`](R) reader structure"]
impl crate::Readable for SIDRrs {}
#[doc = "`reset()` method sets SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
