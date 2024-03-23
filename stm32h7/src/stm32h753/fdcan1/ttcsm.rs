#[doc = "Register `TTCSM` reader"]
pub type R = crate::R<TTCSMrs>;
#[doc = "Field `CSM` reader - Cycle Sync Mark"]
pub type CSM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Cycle Sync Mark"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT Cycle Sync Mark Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttcsm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTCSMrs;
impl crate::RegisterSpec for TTCSMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttcsm::R`](R) reader structure"]
impl crate::Readable for TTCSMrs {}
#[doc = "`reset()` method sets TTCSM to value 0"]
impl crate::Resettable for TTCSMrs {
    const RESET_VALUE: u32 = 0;
}
