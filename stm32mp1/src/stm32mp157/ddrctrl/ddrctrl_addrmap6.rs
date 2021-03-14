#[doc = "Reader of register DDRCTRL_ADDRMAP6"]
pub type R = crate::R<u32, super::DDRCTRL_ADDRMAP6>;
#[doc = "Writer for register DDRCTRL_ADDRMAP6"]
pub type W = crate::W<u32, super::DDRCTRL_ADDRMAP6>;
#[doc = "Register DDRCTRL_ADDRMAP6 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_ADDRMAP6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B12`"]
pub type ADDRMAP_ROW_B12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B12`"]
pub struct ADDRMAP_ROW_B12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B13`"]
pub type ADDRMAP_ROW_B13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B13`"]
pub struct ADDRMAP_ROW_B13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B14`"]
pub type ADDRMAP_ROW_B14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B14`"]
pub struct ADDRMAP_ROW_B14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADDRMAP_ROW_B15`"]
pub type ADDRMAP_ROW_B15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRMAP_ROW_B15`"]
pub struct ADDRMAP_ROW_B15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMAP_ROW_B15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `LPDDR3_6GB_12GB`"]
pub type LPDDR3_6GB_12GB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPDDR3_6GB_12GB`"]
pub struct LPDDR3_6GB_12GB_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDDR3_6GB_12GB_W<'a> {
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
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B12"]
    #[inline(always)]
    pub fn addrmap_row_b12(&self) -> ADDRMAP_ROW_B12_R {
        ADDRMAP_ROW_B12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B13"]
    #[inline(always)]
    pub fn addrmap_row_b13(&self) -> ADDRMAP_ROW_B13_R {
        ADDRMAP_ROW_B13_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B14"]
    #[inline(always)]
    pub fn addrmap_row_b14(&self) -> ADDRMAP_ROW_B14_R {
        ADDRMAP_ROW_B14_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B15"]
    #[inline(always)]
    pub fn addrmap_row_b15(&self) -> ADDRMAP_ROW_B15_R {
        ADDRMAP_ROW_B15_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LPDDR3_6GB_12GB"]
    #[inline(always)]
    pub fn lpddr3_6gb_12gb(&self) -> LPDDR3_6GB_12GB_R {
        LPDDR3_6GB_12GB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B12"]
    #[inline(always)]
    pub fn addrmap_row_b12(&mut self) -> ADDRMAP_ROW_B12_W {
        ADDRMAP_ROW_B12_W { w: self }
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B13"]
    #[inline(always)]
    pub fn addrmap_row_b13(&mut self) -> ADDRMAP_ROW_B13_W {
        ADDRMAP_ROW_B13_W { w: self }
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B14"]
    #[inline(always)]
    pub fn addrmap_row_b14(&mut self) -> ADDRMAP_ROW_B14_W {
        ADDRMAP_ROW_B14_W { w: self }
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B15"]
    #[inline(always)]
    pub fn addrmap_row_b15(&mut self) -> ADDRMAP_ROW_B15_W {
        ADDRMAP_ROW_B15_W { w: self }
    }
    #[doc = "Bit 31 - LPDDR3_6GB_12GB"]
    #[inline(always)]
    pub fn lpddr3_6gb_12gb(&mut self) -> LPDDR3_6GB_12GB_W {
        LPDDR3_6GB_12GB_W { w: self }
    }
}
