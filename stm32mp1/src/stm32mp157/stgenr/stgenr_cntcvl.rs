#[doc = "Reader of register STGENR_CNTCVL"]
pub type R = crate::R<u32, super::STGENR_CNTCVL>;
#[doc = "Reader of field `CNTCVL_L_32`"]
pub type CNTCVL_L_32_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CNTCVL_L_32"]
    #[inline(always)]
    pub fn cntcvl_l_32(&self) -> CNTCVL_L_32_R {
        CNTCVL_L_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
