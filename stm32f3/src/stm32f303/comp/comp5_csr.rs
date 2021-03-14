#[doc = "Reader of register COMP5_CSR"]
pub type R = crate::R<u32, super::COMP5_CSR>;
#[doc = "Writer for register COMP5_CSR"]
pub type W = crate::W<u32, super::COMP5_CSR>;
#[doc = "Register COMP5_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP5_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP5EN`"]
pub type COMP5EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP5EN`"]
pub struct COMP5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5EN_W<'a> {
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
#[doc = "Reader of field `COMP5MODE`"]
pub type COMP5MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP5MODE`"]
pub struct COMP5MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `COMP5INMSEL`"]
pub type COMP5INMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP5INMSEL`"]
pub struct COMP5INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5INMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `COMP5INPSEL`"]
pub type COMP5INPSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP5INPSEL`"]
pub struct COMP5INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5INPSEL_W<'a> {
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
#[doc = "Reader of field `COMP5OUTSEL`"]
pub type COMP5OUTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP5OUTSEL`"]
pub struct COMP5OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5OUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `COMP5POL`"]
pub type COMP5POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP5POL`"]
pub struct COMP5POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5POL_W<'a> {
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
#[doc = "Reader of field `COMP5HYST`"]
pub type COMP5HYST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP5HYST`"]
pub struct COMP5HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `COMP5_BLANKING`"]
pub type COMP5_BLANKING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP5_BLANKING`"]
pub struct COMP5_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `COMP5OUT`"]
pub type COMP5OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP5LOCK`"]
pub type COMP5LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP5LOCK`"]
pub struct COMP5LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP5LOCK_W<'a> {
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
    #[doc = "Bit 0 - Comparator 5 enable"]
    #[inline(always)]
    pub fn comp5en(&self) -> COMP5EN_R {
        COMP5EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 5 mode"]
    #[inline(always)]
    pub fn comp5mode(&self) -> COMP5MODE_R {
        COMP5MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 5 inverting input selection"]
    #[inline(always)]
    pub fn comp5inmsel(&self) -> COMP5INMSEL_R {
        COMP5INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Comparator 5 non inverted input"]
    #[inline(always)]
    pub fn comp5inpsel(&self) -> COMP5INPSEL_R {
        COMP5INPSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 5 output selection"]
    #[inline(always)]
    pub fn comp5outsel(&self) -> COMP5OUTSEL_R {
        COMP5OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 5 output polarity"]
    #[inline(always)]
    pub fn comp5pol(&self) -> COMP5POL_R {
        COMP5POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 5 hysteresis"]
    #[inline(always)]
    pub fn comp5hyst(&self) -> COMP5HYST_R {
        COMP5HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 5 blanking source"]
    #[inline(always)]
    pub fn comp5_blanking(&self) -> COMP5_BLANKING_R {
        COMP5_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 5 output"]
    #[inline(always)]
    pub fn comp5out(&self) -> COMP5OUT_R {
        COMP5OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 5 lock"]
    #[inline(always)]
    pub fn comp5lock(&self) -> COMP5LOCK_R {
        COMP5LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 5 enable"]
    #[inline(always)]
    pub fn comp5en(&mut self) -> COMP5EN_W {
        COMP5EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator 5 mode"]
    #[inline(always)]
    pub fn comp5mode(&mut self) -> COMP5MODE_W {
        COMP5MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 5 inverting input selection"]
    #[inline(always)]
    pub fn comp5inmsel(&mut self) -> COMP5INMSEL_W {
        COMP5INMSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 5 non inverted input"]
    #[inline(always)]
    pub fn comp5inpsel(&mut self) -> COMP5INPSEL_W {
        COMP5INPSEL_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 5 output selection"]
    #[inline(always)]
    pub fn comp5outsel(&mut self) -> COMP5OUTSEL_W {
        COMP5OUTSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 5 output polarity"]
    #[inline(always)]
    pub fn comp5pol(&mut self) -> COMP5POL_W {
        COMP5POL_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 5 hysteresis"]
    #[inline(always)]
    pub fn comp5hyst(&mut self) -> COMP5HYST_W {
        COMP5HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 5 blanking source"]
    #[inline(always)]
    pub fn comp5_blanking(&mut self) -> COMP5_BLANKING_W {
        COMP5_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 5 lock"]
    #[inline(always)]
    pub fn comp5lock(&mut self) -> COMP5LOCK_W {
        COMP5LOCK_W { w: self }
    }
}
