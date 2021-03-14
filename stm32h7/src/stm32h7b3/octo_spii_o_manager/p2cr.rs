#[doc = "Reader of register P2CR"]
pub type R = crate::R<u32, super::P2CR>;
#[doc = "Writer for register P2CR"]
pub type W = crate::W<u32, super::P2CR>;
#[doc = "Register P2CR `reset()`'s with value 0x0705_0333"]
impl crate::ResetValue for super::P2CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0705_0333
    }
}
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
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
#[doc = "Reader of field `CLKSRC`"]
pub type CLKSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKSRC`"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
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
#[doc = "Reader of field `DQSEN`"]
pub type DQSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQSEN`"]
pub struct DQSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSEN_W<'a> {
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
#[doc = "Reader of field `DQSSRC`"]
pub type DQSSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQSSRC`"]
pub struct DQSSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSSRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `NCSEN`"]
pub type NCSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NCSEN`"]
pub struct NCSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NCSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `NCSSRC`"]
pub type NCSSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NCSSRC`"]
pub struct NCSSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> NCSSRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `IOLEN`"]
pub type IOLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOLEN`"]
pub struct IOLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `IOLSRC`"]
pub type IOLSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOLSRC`"]
pub struct IOLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `IOHEN`"]
pub type IOHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOHEN`"]
pub struct IOHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `IOHSRC`"]
pub type IOHSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOHSRC`"]
pub struct IOHSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CLKEN"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CLKSRC"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DQSEN"]
    #[inline(always)]
    pub fn dqsen(&self) -> DQSEN_R {
        DQSEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DQSSRC"]
    #[inline(always)]
    pub fn dqssrc(&self) -> DQSSRC_R {
        DQSSRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NCSEN"]
    #[inline(always)]
    pub fn ncsen(&self) -> NCSEN_R {
        NCSEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - NCSSRC"]
    #[inline(always)]
    pub fn ncssrc(&self) -> NCSSRC_R {
        NCSSRC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IOLEN"]
    #[inline(always)]
    pub fn iolen(&self) -> IOLEN_R {
        IOLEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - IOLSRC"]
    #[inline(always)]
    pub fn iolsrc(&self) -> IOLSRC_R {
        IOLSRC_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 24 - IOHEN"]
    #[inline(always)]
    pub fn iohen(&self) -> IOHEN_R {
        IOHEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - IOHSRC"]
    #[inline(always)]
    pub fn iohsrc(&self) -> IOHSRC_R {
        IOHSRC_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CLKEN"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 1 - CLKSRC"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
    #[doc = "Bit 4 - DQSEN"]
    #[inline(always)]
    pub fn dqsen(&mut self) -> DQSEN_W {
        DQSEN_W { w: self }
    }
    #[doc = "Bit 5 - DQSSRC"]
    #[inline(always)]
    pub fn dqssrc(&mut self) -> DQSSRC_W {
        DQSSRC_W { w: self }
    }
    #[doc = "Bit 8 - NCSEN"]
    #[inline(always)]
    pub fn ncsen(&mut self) -> NCSEN_W {
        NCSEN_W { w: self }
    }
    #[doc = "Bit 9 - NCSSRC"]
    #[inline(always)]
    pub fn ncssrc(&mut self) -> NCSSRC_W {
        NCSSRC_W { w: self }
    }
    #[doc = "Bit 16 - IOLEN"]
    #[inline(always)]
    pub fn iolen(&mut self) -> IOLEN_W {
        IOLEN_W { w: self }
    }
    #[doc = "Bits 17:18 - IOLSRC"]
    #[inline(always)]
    pub fn iolsrc(&mut self) -> IOLSRC_W {
        IOLSRC_W { w: self }
    }
    #[doc = "Bit 24 - IOHEN"]
    #[inline(always)]
    pub fn iohen(&mut self) -> IOHEN_W {
        IOHEN_W { w: self }
    }
    #[doc = "Bits 25:26 - IOHSRC"]
    #[inline(always)]
    pub fn iohsrc(&mut self) -> IOHSRC_W {
        IOHSRC_W { w: self }
    }
}
