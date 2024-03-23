#[doc = "Register `DDRPERFM_SID` reader"]
pub type R = crate::R<DDRPERFM_SIDrs>;
#[doc = "Field `SID` reader - SID"]
pub type SID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SID"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
#[doc = "DDRPERFM magic ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_sid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_SIDrs;
impl crate::RegisterSpec for DDRPERFM_SIDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrperfm_sid::R`](R) reader structure"]
impl crate::Readable for DDRPERFM_SIDrs {}
#[doc = "`reset()` method sets DDRPERFM_SID to value 0xa3c5_dd01"]
impl crate::Resettable for DDRPERFM_SIDrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
