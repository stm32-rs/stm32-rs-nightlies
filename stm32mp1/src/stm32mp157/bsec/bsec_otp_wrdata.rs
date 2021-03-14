#[doc = "Reader of register BSEC_OTP_WRDATA"]
pub type R = crate::R<u32, super::BSEC_OTP_WRDATA>;
#[doc = "Writer for register BSEC_OTP_WRDATA"]
pub type W = crate::W<u32, super::BSEC_OTP_WRDATA>;
#[doc = "Register BSEC_OTP_WRDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::BSEC_OTP_WRDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRDATA`"]
pub type WRDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WRDATA`"]
pub struct WRDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WRDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - WRDATA"]
    #[inline(always)]
    pub fn wrdata(&self) -> WRDATA_R {
        WRDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRDATA"]
    #[inline(always)]
    pub fn wrdata(&mut self) -> WRDATA_W {
        WRDATA_W { w: self }
    }
}
