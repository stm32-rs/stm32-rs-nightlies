#[doc = "Reader of register BSEC_OTP_WRLOCK2"]
pub type R = crate::R<u32, super::BSEC_OTP_WRLOCK2>;
#[doc = "Reader of field `WRLOCK`"]
pub type WRLOCK_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - WRLOCK"]
    #[inline(always)]
    pub fn wrlock(&self) -> WRLOCK_R {
        WRLOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
