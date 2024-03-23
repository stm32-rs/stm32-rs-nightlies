#[doc = "Register `STGENR_CNTCVU` reader"]
pub type R = crate::R<STGENR_CNTCVUrs>;
#[doc = "Field `CNTCVU_U_32` reader - CNTCVU_U_32"]
pub type CNTCVU_U_32_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CNTCVU_U_32"]
    #[inline(always)]
    pub fn cntcvu_u_32(&self) -> CNTCVU_U_32_R {
        CNTCVU_U_32_R::new(self.bits)
    }
}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cntcvu::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENR_CNTCVUrs;
impl crate::RegisterSpec for STGENR_CNTCVUrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenr_cntcvu::R`](R) reader structure"]
impl crate::Readable for STGENR_CNTCVUrs {}
#[doc = "`reset()` method sets STGENR_CNTCVU to value 0"]
impl crate::Resettable for STGENR_CNTCVUrs {
    const RESET_VALUE: u32 = 0;
}
