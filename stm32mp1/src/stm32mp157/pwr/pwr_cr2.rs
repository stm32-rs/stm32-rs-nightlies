#[doc = "Reader of register PWR_CR2"]
pub type R = crate::R<u32, super::PWR_CR2>;
#[doc = "Writer for register PWR_CR2"]
pub type W = crate::W<u32, super::PWR_CR2>;
#[doc = "Register PWR_CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR_CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BREN`"]
pub type BREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BREN`"]
pub struct BREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BREN_W<'a> {
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
#[doc = "Reader of field `RREN`"]
pub type RREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RREN`"]
pub struct RREN_W<'a> {
    w: &'a mut W,
}
impl<'a> RREN_W<'a> {
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
#[doc = "Reader of field `MONEN`"]
pub type MONEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONEN`"]
pub struct MONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MONEN_W<'a> {
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
#[doc = "Reader of field `BRRDY`"]
pub type BRRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RRRDY`"]
pub type RRRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBATL`"]
pub type VBATL_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBATH`"]
pub type VBATH_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMPL`"]
pub type TEMPL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TEMPH`"]
pub type TEMPH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - BREN"]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RREN"]
    #[inline(always)]
    pub fn rren(&self) -> RREN_R {
        RREN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MONEN"]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BRRDY"]
    #[inline(always)]
    pub fn brrdy(&self) -> BRRDY_R {
        BRRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RRRDY"]
    #[inline(always)]
    pub fn rrrdy(&self) -> RRRDY_R {
        RRRDY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - VBATL"]
    #[inline(always)]
    pub fn vbatl(&self) -> VBATL_R {
        VBATL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VBATH"]
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TEMPL"]
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TEMPH"]
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BREN"]
    #[inline(always)]
    pub fn bren(&mut self) -> BREN_W {
        BREN_W { w: self }
    }
    #[doc = "Bit 1 - RREN"]
    #[inline(always)]
    pub fn rren(&mut self) -> RREN_W {
        RREN_W { w: self }
    }
    #[doc = "Bit 4 - MONEN"]
    #[inline(always)]
    pub fn monen(&mut self) -> MONEN_W {
        MONEN_W { w: self }
    }
}
