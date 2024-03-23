#[doc = "Register `DDRPERFM_CNT1` reader"]
pub type R = crate::R<DDRPERFM_CNT1rs>;
#[doc = "Field `CNT` reader - CNT"]
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
#[doc = "DDRPERFM event counter 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrperfm_cnt1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPERFM_CNT1rs;
impl crate::RegisterSpec for DDRPERFM_CNT1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrperfm_cnt1::R`](R) reader structure"]
impl crate::Readable for DDRPERFM_CNT1rs {}
#[doc = "`reset()` method sets DDRPERFM_CNT1 to value 0"]
impl crate::Resettable for DDRPERFM_CNT1rs {
    const RESET_VALUE: u32 = 0;
}
