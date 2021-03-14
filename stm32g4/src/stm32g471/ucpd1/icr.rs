#[doc = "Reader of register ICR"]
pub type R = crate::R<u32, super::ICR>;
#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXMSGDISCCF`"]
pub type TXMSGDISCCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMSGDISCCF`"]
pub struct TXMSGDISCCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGDISCCF_W<'a> {
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
#[doc = "Reader of field `TXMSGSENTCF`"]
pub type TXMSGSENTCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMSGSENTCF`"]
pub struct TXMSGSENTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGSENTCF_W<'a> {
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
#[doc = "Reader of field `TXMSGABTCF`"]
pub type TXMSGABTCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMSGABTCF`"]
pub struct TXMSGABTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMSGABTCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `HRSTDISCCF`"]
pub type HRSTDISCCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HRSTDISCCF`"]
pub struct HRSTDISCCF_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTDISCCF_W<'a> {
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
#[doc = "Reader of field `HRSTSENTCF`"]
pub type HRSTSENTCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HRSTSENTCF`"]
pub struct HRSTSENTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSTSENTCF_W<'a> {
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
#[doc = "Reader of field `TXUNDCF`"]
pub type TXUNDCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUNDCF`"]
pub struct TXUNDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RXORDDETCF`"]
pub type RXORDDETCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXORDDETCF`"]
pub struct RXORDDETCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORDDETCF_W<'a> {
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
#[doc = "Reader of field `RXHRSTDETCF`"]
pub type RXHRSTDETCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXHRSTDETCF`"]
pub struct RXHRSTDETCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXHRSTDETCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `RXOVRCF`"]
pub type RXOVRCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOVRCF`"]
pub struct RXOVRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVRCF_W<'a> {
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
#[doc = "Reader of field `RXMSGENDCF`"]
pub type RXMSGENDCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXMSGENDCF`"]
pub struct RXMSGENDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMSGENDCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TYPECEVT1CF`"]
pub type TYPECEVT1CF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TYPECEVT1CF`"]
pub struct TYPECEVT1CF_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT1CF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TYPECEVT2CF`"]
pub type TYPECEVT2CF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TYPECEVT2CF`"]
pub struct TYPECEVT2CF_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPECEVT2CF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `FRSEVTCF`"]
pub type FRSEVTCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRSEVTCF`"]
pub struct FRSEVTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSEVTCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - TXMSGDISCCF"]
    #[inline(always)]
    pub fn txmsgdisccf(&self) -> TXMSGDISCCF_R {
        TXMSGDISCCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENTCF"]
    #[inline(always)]
    pub fn txmsgsentcf(&self) -> TXMSGSENTCF_R {
        TXMSGSENTCF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TXMSGABTCF"]
    #[inline(always)]
    pub fn txmsgabtcf(&self) -> TXMSGABTCF_R {
        TXMSGABTCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HRSTDISCCF"]
    #[inline(always)]
    pub fn hrstdisccf(&self) -> HRSTDISCCF_R {
        HRSTDISCCF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HRSTSENTCF"]
    #[inline(always)]
    pub fn hrstsentcf(&self) -> HRSTSENTCF_R {
        HRSTSENTCF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TXUNDCF"]
    #[inline(always)]
    pub fn txundcf(&self) -> TXUNDCF_R {
        TXUNDCF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXORDDETCF"]
    #[inline(always)]
    pub fn rxorddetcf(&self) -> RXORDDETCF_R {
        RXORDDETCF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDETCF"]
    #[inline(always)]
    pub fn rxhrstdetcf(&self) -> RXHRSTDETCF_R {
        RXHRSTDETCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RXOVRCF"]
    #[inline(always)]
    pub fn rxovrcf(&self) -> RXOVRCF_R {
        RXOVRCF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RXMSGENDCF"]
    #[inline(always)]
    pub fn rxmsgendcf(&self) -> RXMSGENDCF_R {
        RXMSGENDCF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1CF"]
    #[inline(always)]
    pub fn typecevt1cf(&self) -> TYPECEVT1CF_R {
        TYPECEVT1CF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2CF"]
    #[inline(always)]
    pub fn typecevt2cf(&self) -> TYPECEVT2CF_R {
        TYPECEVT2CF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 20 - FRSEVTCF"]
    #[inline(always)]
    pub fn frsevtcf(&self) -> FRSEVTCF_R {
        FRSEVTCF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TXMSGDISCCF"]
    #[inline(always)]
    pub fn txmsgdisccf(&mut self) -> TXMSGDISCCF_W {
        TXMSGDISCCF_W { w: self }
    }
    #[doc = "Bit 2 - TXMSGSENTCF"]
    #[inline(always)]
    pub fn txmsgsentcf(&mut self) -> TXMSGSENTCF_W {
        TXMSGSENTCF_W { w: self }
    }
    #[doc = "Bit 3 - TXMSGABTCF"]
    #[inline(always)]
    pub fn txmsgabtcf(&mut self) -> TXMSGABTCF_W {
        TXMSGABTCF_W { w: self }
    }
    #[doc = "Bit 4 - HRSTDISCCF"]
    #[inline(always)]
    pub fn hrstdisccf(&mut self) -> HRSTDISCCF_W {
        HRSTDISCCF_W { w: self }
    }
    #[doc = "Bit 5 - HRSTSENTCF"]
    #[inline(always)]
    pub fn hrstsentcf(&mut self) -> HRSTSENTCF_W {
        HRSTSENTCF_W { w: self }
    }
    #[doc = "Bit 6 - TXUNDCF"]
    #[inline(always)]
    pub fn txundcf(&mut self) -> TXUNDCF_W {
        TXUNDCF_W { w: self }
    }
    #[doc = "Bit 9 - RXORDDETCF"]
    #[inline(always)]
    pub fn rxorddetcf(&mut self) -> RXORDDETCF_W {
        RXORDDETCF_W { w: self }
    }
    #[doc = "Bit 10 - RXHRSTDETCF"]
    #[inline(always)]
    pub fn rxhrstdetcf(&mut self) -> RXHRSTDETCF_W {
        RXHRSTDETCF_W { w: self }
    }
    #[doc = "Bit 11 - RXOVRCF"]
    #[inline(always)]
    pub fn rxovrcf(&mut self) -> RXOVRCF_W {
        RXOVRCF_W { w: self }
    }
    #[doc = "Bit 12 - RXMSGENDCF"]
    #[inline(always)]
    pub fn rxmsgendcf(&mut self) -> RXMSGENDCF_W {
        RXMSGENDCF_W { w: self }
    }
    #[doc = "Bit 14 - TYPECEVT1CF"]
    #[inline(always)]
    pub fn typecevt1cf(&mut self) -> TYPECEVT1CF_W {
        TYPECEVT1CF_W { w: self }
    }
    #[doc = "Bit 15 - TYPECEVT2CF"]
    #[inline(always)]
    pub fn typecevt2cf(&mut self) -> TYPECEVT2CF_W {
        TYPECEVT2CF_W { w: self }
    }
    #[doc = "Bit 20 - FRSEVTCF"]
    #[inline(always)]
    pub fn frsevtcf(&mut self) -> FRSEVTCF_W {
        FRSEVTCF_W { w: self }
    }
}
