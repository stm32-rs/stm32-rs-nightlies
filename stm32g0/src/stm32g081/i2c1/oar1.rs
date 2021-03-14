#[doc = "Reader of register OAR1"]
pub type R = crate::R<u32, super::OAR1>;
#[doc = "Writer for register OAR1"]
pub type W = crate::W<u32, super::OAR1>;
#[doc = "Register OAR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OAR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OA1_0`"]
pub type OA1_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1_0`"]
pub struct OA1_0_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_0_W<'a> {
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
#[doc = "Reader of field `OA1_7_1`"]
pub type OA1_7_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OA1_7_1`"]
pub struct OA1_7_1_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_7_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `OA1_8_9`"]
pub type OA1_8_9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OA1_8_9`"]
pub struct OA1_8_9_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_8_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `OA1MODE`"]
pub type OA1MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1MODE`"]
pub struct OA1MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1MODE_W<'a> {
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
#[doc = "Reader of field `OA1EN`"]
pub type OA1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1EN`"]
pub struct OA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn oa1_0(&self) -> OA1_0_R {
        OA1_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn oa1_7_1(&self) -> OA1_7_1_R {
        OA1_7_1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn oa1_8_9(&self) -> OA1_8_9_R {
        OA1_8_9_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode"]
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn oa1_0(&mut self) -> OA1_0_W {
        OA1_0_W { w: self }
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn oa1_7_1(&mut self) -> OA1_7_1_W {
        OA1_7_1_W { w: self }
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn oa1_8_9(&mut self) -> OA1_8_9_W {
        OA1_8_9_W { w: self }
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode"]
    #[inline(always)]
    pub fn oa1mode(&mut self) -> OA1MODE_W {
        OA1MODE_W { w: self }
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&mut self) -> OA1EN_W {
        OA1EN_W { w: self }
    }
}
