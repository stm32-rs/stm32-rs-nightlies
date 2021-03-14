#[doc = "Reader of register COMP3_CSR"]
pub type R = crate::R<u32, super::COMP3_CSR>;
#[doc = "Writer for register COMP3_CSR"]
pub type W = crate::W<u32, super::COMP3_CSR>;
#[doc = "Register COMP3_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP3_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP3EN`"]
pub type COMP3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP3EN`"]
pub struct COMP3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3EN_W<'a> {
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
#[doc = "Reader of field `COMP3MODE`"]
pub type COMP3MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP3MODE`"]
pub struct COMP3MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `COMP3INMSEL`"]
pub type COMP3INMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP3INMSEL`"]
pub struct COMP3INMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3INMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `COMP3INPSEL`"]
pub type COMP3INPSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP3INPSEL`"]
pub struct COMP3INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3INPSEL_W<'a> {
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
#[doc = "Reader of field `COMP3OUTSEL`"]
pub type COMP3OUTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP3OUTSEL`"]
pub struct COMP3OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3OUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `COMP3POL`"]
pub type COMP3POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP3POL`"]
pub struct COMP3POL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3POL_W<'a> {
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
#[doc = "Reader of field `COMP3HYST`"]
pub type COMP3HYST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP3HYST`"]
pub struct COMP3HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `COMP3_BLANKING`"]
pub type COMP3_BLANKING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP3_BLANKING`"]
pub struct COMP3_BLANKING_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3_BLANKING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `COMP3OUT`"]
pub type COMP3OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP3LOCK`"]
pub type COMP3LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP3LOCK`"]
pub struct COMP3LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3LOCK_W<'a> {
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
    #[doc = "Bit 0 - Comparator 3 enable"]
    #[inline(always)]
    pub fn comp3en(&self) -> COMP3EN_R {
        COMP3EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 3 mode"]
    #[inline(always)]
    pub fn comp3mode(&self) -> COMP3MODE_R {
        COMP3MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 3 inverting input selection"]
    #[inline(always)]
    pub fn comp3inmsel(&self) -> COMP3INMSEL_R {
        COMP3INMSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Comparator 3 non inverted input"]
    #[inline(always)]
    pub fn comp3inpsel(&self) -> COMP3INPSEL_R {
        COMP3INPSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 3 output selection"]
    #[inline(always)]
    pub fn comp3outsel(&self) -> COMP3OUTSEL_R {
        COMP3OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 3 output polarity"]
    #[inline(always)]
    pub fn comp3pol(&self) -> COMP3POL_R {
        COMP3POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 3 hysteresis"]
    #[inline(always)]
    pub fn comp3hyst(&self) -> COMP3HYST_R {
        COMP3HYST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 3 blanking source"]
    #[inline(always)]
    pub fn comp3_blanking(&self) -> COMP3_BLANKING_R {
        COMP3_BLANKING_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Comparator 3 output"]
    #[inline(always)]
    pub fn comp3out(&self) -> COMP3OUT_R {
        COMP3OUT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Comparator 3 lock"]
    #[inline(always)]
    pub fn comp3lock(&self) -> COMP3LOCK_R {
        COMP3LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 3 enable"]
    #[inline(always)]
    pub fn comp3en(&mut self) -> COMP3EN_W {
        COMP3EN_W { w: self }
    }
    #[doc = "Bits 2:3 - Comparator 3 mode"]
    #[inline(always)]
    pub fn comp3mode(&mut self) -> COMP3MODE_W {
        COMP3MODE_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 3 inverting input selection"]
    #[inline(always)]
    pub fn comp3inmsel(&mut self) -> COMP3INMSEL_W {
        COMP3INMSEL_W { w: self }
    }
    #[doc = "Bit 7 - Comparator 3 non inverted input"]
    #[inline(always)]
    pub fn comp3inpsel(&mut self) -> COMP3INPSEL_W {
        COMP3INPSEL_W { w: self }
    }
    #[doc = "Bits 10:13 - Comparator 3 output selection"]
    #[inline(always)]
    pub fn comp3outsel(&mut self) -> COMP3OUTSEL_W {
        COMP3OUTSEL_W { w: self }
    }
    #[doc = "Bit 15 - Comparator 3 output polarity"]
    #[inline(always)]
    pub fn comp3pol(&mut self) -> COMP3POL_W {
        COMP3POL_W { w: self }
    }
    #[doc = "Bits 16:17 - Comparator 3 hysteresis"]
    #[inline(always)]
    pub fn comp3hyst(&mut self) -> COMP3HYST_W {
        COMP3HYST_W { w: self }
    }
    #[doc = "Bits 18:20 - Comparator 3 blanking source"]
    #[inline(always)]
    pub fn comp3_blanking(&mut self) -> COMP3_BLANKING_W {
        COMP3_BLANKING_W { w: self }
    }
    #[doc = "Bit 31 - Comparator 3 lock"]
    #[inline(always)]
    pub fn comp3lock(&mut self) -> COMP3LOCK_W {
        COMP3LOCK_W { w: self }
    }
}
