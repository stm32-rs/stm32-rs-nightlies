#[doc = "Reader of register BSEC_OTP_LOCK"]
pub type R = crate::R<u32, super::BSEC_OTP_LOCK>;
#[doc = "Writer for register BSEC_OTP_LOCK"]
pub type W = crate::W<u32, super::BSEC_OTP_LOCK>;
#[doc = "Register BSEC_OTP_LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::BSEC_OTP_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OTP`"]
pub type OTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTP`"]
pub struct OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ROMLOCK`"]
pub type ROMLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROMLOCK`"]
pub struct ROMLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ROMLOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DENREG`"]
pub type DENREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DENREG`"]
pub struct DENREG_W<'a> {
    w: &'a mut W,
}
impl<'a> DENREG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `GPLOCK`"]
pub type GPLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPLOCK`"]
pub struct GPLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPLOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OTP"]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ROMLOCK"]
    #[inline(always)]
    pub fn romlock(&self) -> ROMLOCK_R {
        ROMLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DENREG"]
    #[inline(always)]
    pub fn denreg(&self) -> DENREG_R {
        DENREG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPLOCK"]
    #[inline(always)]
    pub fn gplock(&self) -> GPLOCK_R {
        GPLOCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTP"]
    #[inline(always)]
    pub fn otp(&mut self) -> OTP_W {
        OTP_W { w: self }
    }
    #[doc = "Bit 1 - ROMLOCK"]
    #[inline(always)]
    pub fn romlock(&mut self) -> ROMLOCK_W {
        ROMLOCK_W { w: self }
    }
    #[doc = "Bit 2 - DENREG"]
    #[inline(always)]
    pub fn denreg(&mut self) -> DENREG_W {
        DENREG_W { w: self }
    }
    #[doc = "Bit 4 - GPLOCK"]
    #[inline(always)]
    pub fn gplock(&mut self) -> GPLOCK_W {
        GPLOCK_W { w: self }
    }
}
