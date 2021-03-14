#[doc = "Reader of register COMP7_CSR"]
pub type R = crate::R<u32, super::COMP7_CSR>;
#[doc = "Writer for register COMP7_CSR"]
pub type W = crate::W<u32, super::COMP7_CSR>;
#[doc = "Register COMP7_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP7_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP7EN`"]
pub type COMP7EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP7EN`"]
pub struct COMP7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7EN_W<'a> {
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
#[doc = "Reader of field `COMP7MODE`"]
pub type COMP7MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP7MODE`"]
pub struct COMP7MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `COMP7INMSEL`"]
pub type COMP7INMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP7INMSEL`"]
pub struct COMP7INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7INMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `COMP7INPSEL`"]
pub type COMP7INPSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP7INPSEL`"]
pub struct COMP7INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7INPSEL_W<'a> {
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
#[doc = "Reader of field `COMP7OUTSEL`"]
pub type COMP7OUTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP7OUTSEL`"]
pub struct COMP7OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7OUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `COMP7POL`"]
pub type COMP7POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP7POL`"]
pub struct COMP7POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7POL_W<'a> {
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
#[doc = "Reader of field `COMP7HYST`"]
pub type COMP7HYST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP7HYST`"]
pub struct COMP7HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `COMP7_BLANKING`"]
pub type COMP7_BLANKING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP7_BLANKING`"]
pub struct COMP7_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `COMP7OUT`"]
pub type COMP7OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP7LOCK`"]
pub type COMP7LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP7LOCK`"]
pub struct COMP7LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP7LOCK_W<'a> {
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
    #[doc = "Bit 0 - Comparator 7 enable"]
    #[inline(always)]
    pub fn comp7en(&self) -> COMP7EN_R {
        COMP7EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 7 mode"]
    #[inline(always)]
    pub fn comp7mode(&self) -> COMP7MODE_R {
        COMP7MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 7 inverting input selection"]
    #[inline(always)]
    pub fn comp7inmsel(&self) -> COMP7INMSEL_R {
        COMP7INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Comparator 7 non inverted input"]
    #[inline(always)]
    pub fn comp7inpsel(&self) -> COMP7INPSEL_R {
        COMP7INPSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 7 output selection"]
    #[inline(always)]
    pub fn comp7outsel(&self) -> COMP7OUTSEL_R {
        COMP7OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 7 output polarity"]
    #[inline(always)]
    pub fn comp7pol(&self) -> COMP7POL_R {
        COMP7POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 7 hysteresis"]
    #[inline(always)]
    pub fn comp7hyst(&self) -> COMP7HYST_R {
        COMP7HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 7 blanking source"]
    #[inline(always)]
    pub fn comp7_blanking(&self) -> COMP7_BLANKING_R {
        COMP7_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 7 output"]
    #[inline(always)]
    pub fn comp7out(&self) -> COMP7OUT_R {
        COMP7OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 7 lock"]
    #[inline(always)]
    pub fn comp7lock(&self) -> COMP7LOCK_R {
        COMP7LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 7 enable"]
    #[inline(always)]
    pub fn comp7en(&mut self) -> COMP7EN_W {
        COMP7EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator 7 mode"]
    #[inline(always)]
    pub fn comp7mode(&mut self) -> COMP7MODE_W {
        COMP7MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 7 inverting input selection"]
    #[inline(always)]
    pub fn comp7inmsel(&mut self) -> COMP7INMSEL_W {
        COMP7INMSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 7 non inverted input"]
    #[inline(always)]
    pub fn comp7inpsel(&mut self) -> COMP7INPSEL_W {
        COMP7INPSEL_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 7 output selection"]
    #[inline(always)]
    pub fn comp7outsel(&mut self) -> COMP7OUTSEL_W {
        COMP7OUTSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 7 output polarity"]
    #[inline(always)]
    pub fn comp7pol(&mut self) -> COMP7POL_W {
        COMP7POL_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 7 hysteresis"]
    #[inline(always)]
    pub fn comp7hyst(&mut self) -> COMP7HYST_W {
        COMP7HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 7 blanking source"]
    #[inline(always)]
    pub fn comp7_blanking(&mut self) -> COMP7_BLANKING_W {
        COMP7_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 7 lock"]
    #[inline(always)]
    pub fn comp7lock(&mut self) -> COMP7LOCK_W {
        COMP7LOCK_W { w: self }
    }
}
