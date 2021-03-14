#[doc = "Reader of register BSEC_OTP_ERROR1"]
pub type R = crate::R<u32, super::BSEC_OTP_ERROR1>;
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ERR"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
