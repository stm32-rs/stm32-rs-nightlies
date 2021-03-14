#[doc = "Reader of register CRRCR"]
pub type R = crate::R<u32, super::CRRCR>;
#[doc = "Writer for register CRRCR"]
pub type W = crate::W<u32, super::CRRCR>;
#[doc = "Register CRRCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CRRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSI48CAL`"]
pub type HSI48CAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `HSI48RDY`"]
pub type HSI48RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSI48ON`"]
pub type HSI48ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSI48ON`"]
pub struct HSI48ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48ON_W<'a> {
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
#[doc = "Reader of field `HSI48DIV6EN`"]
pub type HSI48DIV6EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSI48DIV6EN`"]
pub struct HSI48DIV6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48DIV6EN_W<'a> {
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
impl R {
    #[doc = "Bits 8:15 - 48 MHz HSI clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 1 - 48MHz HSI clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 48MHz HSI clock enable bit"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - 48 MHz HSI clock divided by 6 output enable"]
    #[inline(always)]
    pub fn hsi48div6en(&self) -> HSI48DIV6EN_R {
        HSI48DIV6EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 48MHz HSI clock enable bit"]
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W {
        HSI48ON_W { w: self }
    }
    #[doc = "Bit 2 - 48 MHz HSI clock divided by 6 output enable"]
    #[inline(always)]
    pub fn hsi48div6en(&mut self) -> HSI48DIV6EN_W {
        HSI48DIV6EN_W { w: self }
    }
}
