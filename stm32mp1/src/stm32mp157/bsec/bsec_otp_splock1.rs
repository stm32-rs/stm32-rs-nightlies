#[doc = "Reader of register BSEC_OTP_SPLOCK1"]
pub type R = crate::R<u32, super::BSEC_OTP_SPLOCK1>;
#[doc = "Writer for register BSEC_OTP_SPLOCK1"]
pub type W = crate::W<u32, super::BSEC_OTP_SPLOCK1>;
#[doc = "Register BSEC_OTP_SPLOCK1 `reset()`'s with value 0"]
impl crate::ResetValue for super::BSEC_OTP_SPLOCK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPLOCK`"]
pub type SPLOCK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPLOCK`"]
pub struct SPLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SPLOCK"]
    #[inline(always)]
    pub fn splock(&self) -> SPLOCK_R {
        SPLOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPLOCK"]
    #[inline(always)]
    pub fn splock(&mut self) -> SPLOCK_W {
        SPLOCK_W { w: self }
    }
}
