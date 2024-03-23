#[doc = "Register `FDCAN_TTCSM` reader"]
pub type R = crate::R<FDCAN_TTCSMrs>;
#[doc = "Field `CSM` reader - CSM"]
pub type CSM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CSM"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT cycle sync mark register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttcsm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTCSMrs;
impl crate::RegisterSpec for FDCAN_TTCSMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttcsm::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTCSMrs {}
#[doc = "`reset()` method sets FDCAN_TTCSM to value 0"]
impl crate::Resettable for FDCAN_TTCSMrs {
    const RESET_VALUE: u32 = 0;
}
