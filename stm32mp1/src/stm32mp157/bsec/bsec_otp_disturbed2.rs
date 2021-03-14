#[doc = "Reader of register BSEC_OTP_DISTURBED2"]
pub type R = crate::R<u32, super::BSEC_OTP_DISTURBED2>;
#[doc = "Reader of field `DIS`"]
pub type DIS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DIS"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
