#[doc = "Reader of register IER3"]
pub type R = crate::R<u32, super::IER3>;
#[doc = "Writer for register IER3"]
pub type W = crate::W<u32, super::IER3>;
#[doc = "Register IER3 `reset()`'s with value 0"]
impl crate::ResetValue for super::IER3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TZSCIE`"]
pub type TZSCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZSCIE`"]
pub struct TZSCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZSCIE_W<'a> {
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
#[doc = "Reader of field `TZICIE`"]
pub type TZICIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZICIE`"]
pub struct TZICIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZICIE_W<'a> {
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
#[doc = "Reader of field `MPCWM1IE`"]
pub type MPCWM1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCWM1IE`"]
pub struct MPCWM1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM1IE_W<'a> {
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
#[doc = "Reader of field `MPCWM2IE`"]
pub type MPCWM2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCWM2IE`"]
pub struct MPCWM2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCWM2IE_W<'a> {
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
#[doc = "Reader of field `MPCBB1IE`"]
pub type MPCBB1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB1IE`"]
pub struct MPCBB1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1IE_W<'a> {
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
#[doc = "Reader of field `MPCBB1_REGIE`"]
pub type MPCBB1_REGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB1_REGIE`"]
pub struct MPCBB1_REGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB1_REGIE_W<'a> {
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
#[doc = "Reader of field `MPCBB2IE`"]
pub type MPCBB2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB2IE`"]
pub struct MPCBB2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2IE_W<'a> {
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
#[doc = "Reader of field `MPCBB2_REGIE`"]
pub type MPCBB2_REGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPCBB2_REGIE`"]
pub struct MPCBB2_REGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MPCBB2_REGIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MPCWM1IE"]
    #[inline(always)]
    pub fn mpcwm1ie(&self) -> MPCWM1IE_R {
        MPCWM1IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MPCWM2IE"]
    #[inline(always)]
    pub fn mpcwm2ie(&self) -> MPCWM2IE_R {
        MPCWM2IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MPCBB1IE"]
    #[inline(always)]
    pub fn mpcbb1ie(&self) -> MPCBB1IE_R {
        MPCBB1IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MPCBB1_REGIE"]
    #[inline(always)]
    pub fn mpcbb1_regie(&self) -> MPCBB1_REGIE_R {
        MPCBB1_REGIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPCBB2IE"]
    #[inline(always)]
    pub fn mpcbb2ie(&self) -> MPCBB2IE_R {
        MPCBB2IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MPCBB2_REGIE"]
    #[inline(always)]
    pub fn mpcbb2_regie(&self) -> MPCBB2_REGIE_R {
        MPCBB2_REGIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&mut self) -> TZSCIE_W {
        TZSCIE_W { w: self }
    }
    #[doc = "Bit 1 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&mut self) -> TZICIE_W {
        TZICIE_W { w: self }
    }
    #[doc = "Bit 2 - MPCWM1IE"]
    #[inline(always)]
    pub fn mpcwm1ie(&mut self) -> MPCWM1IE_W {
        MPCWM1IE_W { w: self }
    }
    #[doc = "Bit 3 - MPCWM2IE"]
    #[inline(always)]
    pub fn mpcwm2ie(&mut self) -> MPCWM2IE_W {
        MPCWM2IE_W { w: self }
    }
    #[doc = "Bit 4 - MPCBB1IE"]
    #[inline(always)]
    pub fn mpcbb1ie(&mut self) -> MPCBB1IE_W {
        MPCBB1IE_W { w: self }
    }
    #[doc = "Bit 5 - MPCBB1_REGIE"]
    #[inline(always)]
    pub fn mpcbb1_regie(&mut self) -> MPCBB1_REGIE_W {
        MPCBB1_REGIE_W { w: self }
    }
    #[doc = "Bit 6 - MPCBB2IE"]
    #[inline(always)]
    pub fn mpcbb2ie(&mut self) -> MPCBB2IE_W {
        MPCBB2IE_W { w: self }
    }
    #[doc = "Bit 7 - MPCBB2_REGIE"]
    #[inline(always)]
    pub fn mpcbb2_regie(&mut self) -> MPCBB2_REGIE_W {
        MPCBB2_REGIE_W { w: self }
    }
}
