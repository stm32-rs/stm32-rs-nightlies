#[doc = "Register `DDRPERFM_TCNT` reader"]
pub type R = crate::R<DDRPERFM_TCNTrs>;
#[doc = "Field `CNT` reader - CNT"]
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
#[doc = "DDRPERFM time counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_tcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_TCNTrs;
impl crate::RegisterSpec for DDRPERFM_TCNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrperfm_tcnt::R`](R) reader structure"]
impl crate::Readable for DDRPERFM_TCNTrs {}
#[doc = "`reset()` method sets DDRPERFM_TCNT to value 0"]
impl crate::Resettable for DDRPERFM_TCNTrs {
    const RESET_VALUE: u32 = 0;
}
