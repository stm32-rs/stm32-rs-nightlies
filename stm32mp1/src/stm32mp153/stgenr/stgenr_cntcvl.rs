#[doc = "Register `STGENR_CNTCVL` reader"]
pub type R = crate::R<STGENR_CNTCVLrs>;
#[doc = "Field `CNTCVL_L_32` reader - CNTCVL_L_32"]
pub type CNTCVL_L_32_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CNTCVL_L_32"]
    #[inline(always)]
    pub fn cntcvl_l_32(&self) -> CNTCVL_L_32_R {
        CNTCVL_L_32_R::new(self.bits)
    }
}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stgenr_cntcvl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STGENR_CNTCVLrs;
impl crate::RegisterSpec for STGENR_CNTCVLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stgenr_cntcvl::R`](R) reader structure"]
impl crate::Readable for STGENR_CNTCVLrs {}
#[doc = "`reset()` method sets STGENR_CNTCVL to value 0"]
impl crate::Resettable for STGENR_CNTCVLrs {
    const RESET_VALUE: u32 = 0;
}
