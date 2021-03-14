#[doc = "Reader of register ADC_PCSEL"]
pub type R = crate::R<u32, super::ADC_PCSEL>;
#[doc = "Writer for register ADC_PCSEL"]
pub type W = crate::W<u32, super::ADC_PCSEL>;
#[doc = "Register ADC_PCSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_PCSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCSEL0`"]
pub type PCSEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL0`"]
pub struct PCSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL0_W<'a> {
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
#[doc = "Reader of field `PCSEL1`"]
pub type PCSEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL1`"]
pub struct PCSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL1_W<'a> {
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
#[doc = "Reader of field `PCSEL2`"]
pub type PCSEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL2`"]
pub struct PCSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL2_W<'a> {
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
#[doc = "Reader of field `PCSEL3`"]
pub type PCSEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL3`"]
pub struct PCSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL3_W<'a> {
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
#[doc = "Reader of field `PCSEL4`"]
pub type PCSEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL4`"]
pub struct PCSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL4_W<'a> {
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
#[doc = "Reader of field `PCSEL5`"]
pub type PCSEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL5`"]
pub struct PCSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL5_W<'a> {
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
#[doc = "Reader of field `PCSEL6`"]
pub type PCSEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL6`"]
pub struct PCSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL6_W<'a> {
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
#[doc = "Reader of field `PCSEL7`"]
pub type PCSEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL7`"]
pub struct PCSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL7_W<'a> {
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
#[doc = "Reader of field `PCSEL8`"]
pub type PCSEL8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL8`"]
pub struct PCSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PCSEL9`"]
pub type PCSEL9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL9`"]
pub struct PCSEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL9_W<'a> {
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
#[doc = "Reader of field `PCSEL10`"]
pub type PCSEL10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL10`"]
pub struct PCSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PCSEL11`"]
pub type PCSEL11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL11`"]
pub struct PCSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PCSEL12`"]
pub type PCSEL12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL12`"]
pub struct PCSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PCSEL13`"]
pub type PCSEL13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL13`"]
pub struct PCSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PCSEL14`"]
pub type PCSEL14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL14`"]
pub struct PCSEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PCSEL15`"]
pub type PCSEL15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL15`"]
pub struct PCSEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL15_W<'a> {
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
#[doc = "Reader of field `PCSEL16`"]
pub type PCSEL16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL16`"]
pub struct PCSEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PCSEL17`"]
pub type PCSEL17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL17`"]
pub struct PCSEL17_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PCSEL18`"]
pub type PCSEL18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL18`"]
pub struct PCSEL18_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `PCSEL19`"]
pub type PCSEL19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCSEL19`"]
pub struct PCSEL19_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSEL19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PCSEL0"]
    #[inline(always)]
    pub fn pcsel0(&self) -> PCSEL0_R {
        PCSEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PCSEL1"]
    #[inline(always)]
    pub fn pcsel1(&self) -> PCSEL1_R {
        PCSEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PCSEL2"]
    #[inline(always)]
    pub fn pcsel2(&self) -> PCSEL2_R {
        PCSEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PCSEL3"]
    #[inline(always)]
    pub fn pcsel3(&self) -> PCSEL3_R {
        PCSEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PCSEL4"]
    #[inline(always)]
    pub fn pcsel4(&self) -> PCSEL4_R {
        PCSEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PCSEL5"]
    #[inline(always)]
    pub fn pcsel5(&self) -> PCSEL5_R {
        PCSEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PCSEL6"]
    #[inline(always)]
    pub fn pcsel6(&self) -> PCSEL6_R {
        PCSEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PCSEL7"]
    #[inline(always)]
    pub fn pcsel7(&self) -> PCSEL7_R {
        PCSEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PCSEL8"]
    #[inline(always)]
    pub fn pcsel8(&self) -> PCSEL8_R {
        PCSEL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PCSEL9"]
    #[inline(always)]
    pub fn pcsel9(&self) -> PCSEL9_R {
        PCSEL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PCSEL10"]
    #[inline(always)]
    pub fn pcsel10(&self) -> PCSEL10_R {
        PCSEL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PCSEL11"]
    #[inline(always)]
    pub fn pcsel11(&self) -> PCSEL11_R {
        PCSEL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PCSEL12"]
    #[inline(always)]
    pub fn pcsel12(&self) -> PCSEL12_R {
        PCSEL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PCSEL13"]
    #[inline(always)]
    pub fn pcsel13(&self) -> PCSEL13_R {
        PCSEL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PCSEL14"]
    #[inline(always)]
    pub fn pcsel14(&self) -> PCSEL14_R {
        PCSEL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PCSEL15"]
    #[inline(always)]
    pub fn pcsel15(&self) -> PCSEL15_R {
        PCSEL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PCSEL16"]
    #[inline(always)]
    pub fn pcsel16(&self) -> PCSEL16_R {
        PCSEL16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PCSEL17"]
    #[inline(always)]
    pub fn pcsel17(&self) -> PCSEL17_R {
        PCSEL17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PCSEL18"]
    #[inline(always)]
    pub fn pcsel18(&self) -> PCSEL18_R {
        PCSEL18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PCSEL19"]
    #[inline(always)]
    pub fn pcsel19(&self) -> PCSEL19_R {
        PCSEL19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCSEL0"]
    #[inline(always)]
    pub fn pcsel0(&mut self) -> PCSEL0_W {
        PCSEL0_W { w: self }
    }
    #[doc = "Bit 1 - PCSEL1"]
    #[inline(always)]
    pub fn pcsel1(&mut self) -> PCSEL1_W {
        PCSEL1_W { w: self }
    }
    #[doc = "Bit 2 - PCSEL2"]
    #[inline(always)]
    pub fn pcsel2(&mut self) -> PCSEL2_W {
        PCSEL2_W { w: self }
    }
    #[doc = "Bit 3 - PCSEL3"]
    #[inline(always)]
    pub fn pcsel3(&mut self) -> PCSEL3_W {
        PCSEL3_W { w: self }
    }
    #[doc = "Bit 4 - PCSEL4"]
    #[inline(always)]
    pub fn pcsel4(&mut self) -> PCSEL4_W {
        PCSEL4_W { w: self }
    }
    #[doc = "Bit 5 - PCSEL5"]
    #[inline(always)]
    pub fn pcsel5(&mut self) -> PCSEL5_W {
        PCSEL5_W { w: self }
    }
    #[doc = "Bit 6 - PCSEL6"]
    #[inline(always)]
    pub fn pcsel6(&mut self) -> PCSEL6_W {
        PCSEL6_W { w: self }
    }
    #[doc = "Bit 7 - PCSEL7"]
    #[inline(always)]
    pub fn pcsel7(&mut self) -> PCSEL7_W {
        PCSEL7_W { w: self }
    }
    #[doc = "Bit 8 - PCSEL8"]
    #[inline(always)]
    pub fn pcsel8(&mut self) -> PCSEL8_W {
        PCSEL8_W { w: self }
    }
    #[doc = "Bit 9 - PCSEL9"]
    #[inline(always)]
    pub fn pcsel9(&mut self) -> PCSEL9_W {
        PCSEL9_W { w: self }
    }
    #[doc = "Bit 10 - PCSEL10"]
    #[inline(always)]
    pub fn pcsel10(&mut self) -> PCSEL10_W {
        PCSEL10_W { w: self }
    }
    #[doc = "Bit 11 - PCSEL11"]
    #[inline(always)]
    pub fn pcsel11(&mut self) -> PCSEL11_W {
        PCSEL11_W { w: self }
    }
    #[doc = "Bit 12 - PCSEL12"]
    #[inline(always)]
    pub fn pcsel12(&mut self) -> PCSEL12_W {
        PCSEL12_W { w: self }
    }
    #[doc = "Bit 13 - PCSEL13"]
    #[inline(always)]
    pub fn pcsel13(&mut self) -> PCSEL13_W {
        PCSEL13_W { w: self }
    }
    #[doc = "Bit 14 - PCSEL14"]
    #[inline(always)]
    pub fn pcsel14(&mut self) -> PCSEL14_W {
        PCSEL14_W { w: self }
    }
    #[doc = "Bit 15 - PCSEL15"]
    #[inline(always)]
    pub fn pcsel15(&mut self) -> PCSEL15_W {
        PCSEL15_W { w: self }
    }
    #[doc = "Bit 16 - PCSEL16"]
    #[inline(always)]
    pub fn pcsel16(&mut self) -> PCSEL16_W {
        PCSEL16_W { w: self }
    }
    #[doc = "Bit 17 - PCSEL17"]
    #[inline(always)]
    pub fn pcsel17(&mut self) -> PCSEL17_W {
        PCSEL17_W { w: self }
    }
    #[doc = "Bit 18 - PCSEL18"]
    #[inline(always)]
    pub fn pcsel18(&mut self) -> PCSEL18_W {
        PCSEL18_W { w: self }
    }
    #[doc = "Bit 19 - PCSEL19"]
    #[inline(always)]
    pub fn pcsel19(&mut self) -> PCSEL19_W {
        PCSEL19_W { w: self }
    }
}
