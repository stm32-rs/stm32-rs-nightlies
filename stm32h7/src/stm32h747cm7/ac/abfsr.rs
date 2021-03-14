#[doc = "Reader of register ABFSR"]
pub type R = crate::R<u32, super::ABFSR>;
#[doc = "Writer for register ABFSR"]
pub type W = crate::W<u32, super::ABFSR>;
#[doc = "Register ABFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ABFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ITCM`"]
pub type ITCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITCM`"]
pub struct ITCM_W<'a> {
    w: &'a mut W,
}
impl<'a> ITCM_W<'a> {
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
#[doc = "Reader of field `DTCM`"]
pub type DTCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTCM`"]
pub struct DTCM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM_W<'a> {
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
#[doc = "Reader of field `AHBP`"]
pub type AHBP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBP`"]
pub struct AHBP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBP_W<'a> {
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
#[doc = "Reader of field `AXIM`"]
pub type AXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXIM`"]
pub struct AXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIM_W<'a> {
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
#[doc = "Reader of field `EPPB`"]
pub type EPPB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPPB`"]
pub struct EPPB_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPB_W<'a> {
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
#[doc = "Reader of field `AXIMTYPE`"]
pub type AXIMTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AXIMTYPE`"]
pub struct AXIMTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIMTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ITCM"]
    #[inline(always)]
    pub fn itcm(&self) -> ITCM_R {
        ITCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DTCM"]
    #[inline(always)]
    pub fn dtcm(&self) -> DTCM_R {
        DTCM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHBP"]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AXIM"]
    #[inline(always)]
    pub fn axim(&self) -> AXIM_R {
        AXIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EPPB"]
    #[inline(always)]
    pub fn eppb(&self) -> EPPB_R {
        EPPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - AXIMTYPE"]
    #[inline(always)]
    pub fn aximtype(&self) -> AXIMTYPE_R {
        AXIMTYPE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ITCM"]
    #[inline(always)]
    pub fn itcm(&mut self) -> ITCM_W {
        ITCM_W { w: self }
    }
    #[doc = "Bit 1 - DTCM"]
    #[inline(always)]
    pub fn dtcm(&mut self) -> DTCM_W {
        DTCM_W { w: self }
    }
    #[doc = "Bit 2 - AHBP"]
    #[inline(always)]
    pub fn ahbp(&mut self) -> AHBP_W {
        AHBP_W { w: self }
    }
    #[doc = "Bit 3 - AXIM"]
    #[inline(always)]
    pub fn axim(&mut self) -> AXIM_W {
        AXIM_W { w: self }
    }
    #[doc = "Bit 4 - EPPB"]
    #[inline(always)]
    pub fn eppb(&mut self) -> EPPB_W {
        EPPB_W { w: self }
    }
    #[doc = "Bits 8:9 - AXIMTYPE"]
    #[inline(always)]
    pub fn aximtype(&mut self) -> AXIMTYPE_W {
        AXIMTYPE_W { w: self }
    }
}
