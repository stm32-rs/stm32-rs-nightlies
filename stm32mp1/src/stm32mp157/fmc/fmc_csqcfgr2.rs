#[doc = "Reader of register FMC_CSQCFGR2"]
pub type R = crate::R<u32, super::FMC_CSQCFGR2>;
#[doc = "Writer for register FMC_CSQCFGR2"]
pub type W = crate::W<u32, super::FMC_CSQCFGR2>;
#[doc = "Register FMC_CSQCFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC_CSQCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SQSDTEN`"]
pub type SQSDTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SQSDTEN`"]
pub struct SQSDTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SQSDTEN_W<'a> {
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
#[doc = "Reader of field `RCMD2EN`"]
pub type RCMD2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCMD2EN`"]
pub struct RCMD2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMD2EN_W<'a> {
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
#[doc = "Reader of field `DMASEN`"]
pub type DMASEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMASEN`"]
pub struct DMASEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASEN_W<'a> {
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
#[doc = "Reader of field `RCMD1`"]
pub type RCMD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCMD1`"]
pub struct RCMD1_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RCMD2`"]
pub type RCMD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCMD2`"]
pub struct RCMD2_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RCMD1T`"]
pub type RCMD1T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCMD1T`"]
pub struct RCMD1T_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMD1T_W<'a> {
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
#[doc = "Reader of field `RCMD2T`"]
pub type RCMD2T_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCMD2T`"]
pub struct RCMD2T_W<'a> {
    w: &'a mut W,
}
impl<'a> RCMD2T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SQSDTEN"]
    #[inline(always)]
    pub fn sqsdten(&self) -> SQSDTEN_R {
        SQSDTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RCMD2EN"]
    #[inline(always)]
    pub fn rcmd2en(&self) -> RCMD2EN_R {
        RCMD2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMASEN"]
    #[inline(always)]
    pub fn dmasen(&self) -> DMASEN_R {
        DMASEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - RCMD1"]
    #[inline(always)]
    pub fn rcmd1(&self) -> RCMD1_R {
        RCMD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RCMD2"]
    #[inline(always)]
    pub fn rcmd2(&self) -> RCMD2_R {
        RCMD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - RCMD1T"]
    #[inline(always)]
    pub fn rcmd1t(&self) -> RCMD1T_R {
        RCMD1T_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RCMD2T"]
    #[inline(always)]
    pub fn rcmd2t(&self) -> RCMD2T_R {
        RCMD2T_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SQSDTEN"]
    #[inline(always)]
    pub fn sqsdten(&mut self) -> SQSDTEN_W {
        SQSDTEN_W { w: self }
    }
    #[doc = "Bit 1 - RCMD2EN"]
    #[inline(always)]
    pub fn rcmd2en(&mut self) -> RCMD2EN_W {
        RCMD2EN_W { w: self }
    }
    #[doc = "Bit 2 - DMASEN"]
    #[inline(always)]
    pub fn dmasen(&mut self) -> DMASEN_W {
        DMASEN_W { w: self }
    }
    #[doc = "Bits 8:15 - RCMD1"]
    #[inline(always)]
    pub fn rcmd1(&mut self) -> RCMD1_W {
        RCMD1_W { w: self }
    }
    #[doc = "Bits 16:23 - RCMD2"]
    #[inline(always)]
    pub fn rcmd2(&mut self) -> RCMD2_W {
        RCMD2_W { w: self }
    }
    #[doc = "Bit 24 - RCMD1T"]
    #[inline(always)]
    pub fn rcmd1t(&mut self) -> RCMD1T_W {
        RCMD1T_W { w: self }
    }
    #[doc = "Bit 25 - RCMD2T"]
    #[inline(always)]
    pub fn rcmd2t(&mut self) -> RCMD2T_W {
        RCMD2T_W { w: self }
    }
}
