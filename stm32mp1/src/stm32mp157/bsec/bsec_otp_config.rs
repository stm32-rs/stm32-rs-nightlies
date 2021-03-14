#[doc = "Reader of register BSEC_OTP_CONFIG"]
pub type R = crate::R<u32, super::BSEC_OTP_CONFIG>;
#[doc = "Writer for register BSEC_OTP_CONFIG"]
pub type W = crate::W<u32, super::BSEC_OTP_CONFIG>;
#[doc = "Register BSEC_OTP_CONFIG `reset()`'s with value 0x0e"]
impl crate::ResetValue for super::BSEC_OTP_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0e
    }
}
#[doc = "Reader of field `PWRUP`"]
pub type PWRUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRUP`"]
pub struct PWRUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUP_W<'a> {
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
#[doc = "Reader of field `FRC`"]
pub type FRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRC`"]
pub struct FRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `PRGWIDTH`"]
pub type PRGWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRGWIDTH`"]
pub struct PRGWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `TREAD`"]
pub type TREAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TREAD`"]
pub struct TREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TREAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWRUP"]
    #[inline(always)]
    pub fn pwrup(&self) -> PWRUP_R {
        PWRUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - FRC"]
    #[inline(always)]
    pub fn frc(&self) -> FRC_R {
        FRC_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:6 - PRGWIDTH"]
    #[inline(always)]
    pub fn prgwidth(&self) -> PRGWIDTH_R {
        PRGWIDTH_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:8 - TREAD"]
    #[inline(always)]
    pub fn tread(&self) -> TREAD_R {
        TREAD_R::new(((self.bits >> 7) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PWRUP"]
    #[inline(always)]
    pub fn pwrup(&mut self) -> PWRUP_W {
        PWRUP_W { w: self }
    }
    #[doc = "Bits 1:2 - FRC"]
    #[inline(always)]
    pub fn frc(&mut self) -> FRC_W {
        FRC_W { w: self }
    }
    #[doc = "Bits 3:6 - PRGWIDTH"]
    #[inline(always)]
    pub fn prgwidth(&mut self) -> PRGWIDTH_W {
        PRGWIDTH_W { w: self }
    }
    #[doc = "Bits 7:8 - TREAD"]
    #[inline(always)]
    pub fn tread(&mut self) -> TREAD_W {
        TREAD_W { w: self }
    }
}
