#[doc = "Reader of register COMP2_CSR"]
pub type R = crate::R<u32, super::COMP2_CSR>;
#[doc = "Writer for register COMP2_CSR"]
pub type W = crate::W<u32, super::COMP2_CSR>;
#[doc = "Register COMP2_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP2_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP2_EN`"]
pub type COMP2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP2_EN`"]
pub struct COMP2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_EN_W<'a> {
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
#[doc = "Reader of field `COMP2_PWRMODE`"]
pub type COMP2_PWRMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP2_PWRMODE`"]
pub struct COMP2_PWRMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_PWRMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `COMP2_INMSEL`"]
pub type COMP2_INMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP2_INMSEL`"]
pub struct COMP2_INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_INMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `COMP2_INPSEL`"]
pub type COMP2_INPSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP2_INPSEL`"]
pub struct COMP2_INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_INPSEL_W<'a> {
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
#[doc = "Reader of field `COMP2_WINMODE`"]
pub type COMP2_WINMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP2_WINMODE`"]
pub struct COMP2_WINMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_WINMODE_W<'a> {
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
#[doc = "Reader of field `COMP2_POLARITY`"]
pub type COMP2_POLARITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP2_POLARITY`"]
pub struct COMP2_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_POLARITY_W<'a> {
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
#[doc = "Reader of field `COMP2_HYST`"]
pub type COMP2_HYST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP2_HYST`"]
pub struct COMP2_HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `COMP2_BLANKING`"]
pub type COMP2_BLANKING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP2_BLANKING`"]
pub struct COMP2_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `COMP2_BRGEN`"]
pub type COMP2_BRGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP2_BRGEN`"]
pub struct COMP2_BRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_BRGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `COMP2_SCALEN`"]
pub type COMP2_SCALEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP2_SCALEN`"]
pub struct COMP2_SCALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_SCALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `COMP2_VALUE`"]
pub type COMP2_VALUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP2_LOCK`"]
pub struct COMP2_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2_en(&self) -> COMP2_EN_R {
        COMP2_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn comp2_pwrmode(&self) -> COMP2_PWRMODE_R {
        COMP2_PWRMODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inmsel(&self) -> COMP2_INMSEL_R {
        COMP2_INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inpsel(&self) -> COMP2_INPSEL_R {
        COMP2_INPSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn comp2_winmode(&self) -> COMP2_WINMODE_R {
        COMP2_WINMODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2_polarity(&self) -> COMP2_POLARITY_R {
        COMP2_POLARITY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn comp2_hyst(&self) -> COMP2_HYST_R {
        COMP2_HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn comp2_brgen(&self) -> COMP2_BRGEN_R {
        COMP2_BRGEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn comp2_scalen(&self) -> COMP2_SCALEN_R {
        COMP2_SCALEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn comp2_value(&self) -> COMP2_VALUE_R {
        COMP2_VALUE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2_en(&mut self) -> COMP2_EN_W {
        COMP2_EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn comp2_pwrmode(&mut self) -> COMP2_PWRMODE_W {
        COMP2_PWRMODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inmsel(&mut self) -> COMP2_INMSEL_W {
        COMP2_INMSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2_inpsel(&mut self) -> COMP2_INPSEL_W {
        COMP2_INPSEL_W { w: self }
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn comp2_winmode(&mut self) -> COMP2_WINMODE_W {
        COMP2_WINMODE_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2_polarity(&mut self) -> COMP2_POLARITY_W {
        COMP2_POLARITY_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn comp2_hyst(&mut self) -> COMP2_HYST_W {
        COMP2_HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W {
        COMP2_BLANKING_W { w: self }
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn comp2_brgen(&mut self) -> COMP2_BRGEN_W {
        COMP2_BRGEN_W { w: self }
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn comp2_scalen(&mut self) -> COMP2_SCALEN_W {
        COMP2_SCALEN_W { w: self }
    }
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    pub fn comp2_lock(&mut self) -> COMP2_LOCK_W {
        COMP2_LOCK_W { w: self }
    }
}
