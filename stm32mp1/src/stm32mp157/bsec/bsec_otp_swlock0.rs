#[doc = "Reader of register BSEC_OTP_SWLOCK0"]
pub type R = crate::R<u32, super::BSEC_OTP_SWLOCK0>;
#[doc = "Writer for register BSEC_OTP_SWLOCK0"]
pub type W = crate::W<u32, super::BSEC_OTP_SWLOCK0>;
#[doc = "Register BSEC_OTP_SWLOCK0 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::BSEC_OTP_SWLOCK0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `SWLOCK`"]
pub type SWLOCK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SWLOCK`"]
pub struct SWLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SWLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SWLOCK"]
    #[inline(always)]
    pub fn swlock(&self) -> SWLOCK_R {
        SWLOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SWLOCK"]
    #[inline(always)]
    pub fn swlock(&mut self) -> SWLOCK_W {
        SWLOCK_W { w: self }
    }
}
