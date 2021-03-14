#[doc = "Reader of register STGENR_CNTCVU"]
pub type R = crate::R<u32, super::STGENR_CNTCVU>;
#[doc = "Reader of field `CNTCVU_U_32`"]
pub type CNTCVU_U_32_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CNTCVU_U_32"]
    #[inline(always)]
    pub fn cntcvu_u_32(&self) -> CNTCVU_U_32_R {
        CNTCVU_U_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
