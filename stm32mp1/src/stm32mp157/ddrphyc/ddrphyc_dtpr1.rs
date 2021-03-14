#[doc = "Reader of register DDRPHYC_DTPR1"]
pub type R = crate::R<u32, super::DDRPHYC_DTPR1>;
#[doc = "Writer for register DDRPHYC_DTPR1"]
pub type W = crate::W<u32, super::DDRPHYC_DTPR1>;
#[doc = "Register DDRPHYC_DTPR1 `reset()`'s with value 0x0a03_0090"]
impl crate::ResetValue for super::DDRPHYC_DTPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a03_0090
    }
}
#[doc = "Reader of field `TAOND`"]
pub type TAOND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAOND`"]
pub struct TAOND_W<'a> {
    w: &'a mut W,
}
impl<'a> TAOND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TRTW`"]
pub type TRTW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRTW`"]
pub struct TRTW_W<'a> {
    w: &'a mut W,
}
impl<'a> TRTW_W<'a> {
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
#[doc = "Reader of field `TFAW`"]
pub type TFAW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TFAW`"]
pub struct TFAW_W<'a> {
    w: &'a mut W,
}
impl<'a> TFAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 3)) | (((value as u32) & 0x3f) << 3);
        self.w
    }
}
#[doc = "Reader of field `TMOD`"]
pub type TMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMOD`"]
pub struct TMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `TRTODT`"]
pub type TRTODT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRTODT`"]
pub struct TRTODT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRTODT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TRFC`"]
pub type TRFC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRFC`"]
pub struct TRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TDQSCKMIN`"]
pub type TDQSCKMIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDQSCKMIN`"]
pub struct TDQSCKMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDQSCKMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `TDQSCKMAX`"]
pub type TDQSCKMAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDQSCKMAX`"]
pub struct TDQSCKMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> TDQSCKMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TAOND"]
    #[inline(always)]
    pub fn taond(&self) -> TAOND_R {
        TAOND_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - TRTW"]
    #[inline(always)]
    pub fn trtw(&self) -> TRTW_R {
        TRTW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:8 - TFAW"]
    #[inline(always)]
    pub fn tfaw(&self) -> TFAW_R {
        TFAW_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bits 9:10 - TMOD"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - TRTODT"]
    #[inline(always)]
    pub fn trtodt(&self) -> TRTODT_R {
        TRTODT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - TRFC"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - TDQSCKMIN"]
    #[inline(always)]
    pub fn tdqsckmin(&self) -> TDQSCKMIN_R {
        TDQSCKMIN_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:29 - TDQSCKMAX"]
    #[inline(always)]
    pub fn tdqsckmax(&self) -> TDQSCKMAX_R {
        TDQSCKMAX_R::new(((self.bits >> 27) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TAOND"]
    #[inline(always)]
    pub fn taond(&mut self) -> TAOND_W {
        TAOND_W { w: self }
    }
    #[doc = "Bit 2 - TRTW"]
    #[inline(always)]
    pub fn trtw(&mut self) -> TRTW_W {
        TRTW_W { w: self }
    }
    #[doc = "Bits 3:8 - TFAW"]
    #[inline(always)]
    pub fn tfaw(&mut self) -> TFAW_W {
        TFAW_W { w: self }
    }
    #[doc = "Bits 9:10 - TMOD"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W {
        TMOD_W { w: self }
    }
    #[doc = "Bit 11 - TRTODT"]
    #[inline(always)]
    pub fn trtodt(&mut self) -> TRTODT_W {
        TRTODT_W { w: self }
    }
    #[doc = "Bits 16:23 - TRFC"]
    #[inline(always)]
    pub fn trfc(&mut self) -> TRFC_W {
        TRFC_W { w: self }
    }
    #[doc = "Bits 24:26 - TDQSCKMIN"]
    #[inline(always)]
    pub fn tdqsckmin(&mut self) -> TDQSCKMIN_W {
        TDQSCKMIN_W { w: self }
    }
    #[doc = "Bits 27:29 - TDQSCKMAX"]
    #[inline(always)]
    pub fn tdqsckmax(&mut self) -> TDQSCKMAX_W {
        TDQSCKMAX_W { w: self }
    }
}
