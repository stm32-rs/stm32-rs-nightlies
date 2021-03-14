#[doc = "Writer for register DMAMUX_RGCFR"]
pub type W = crate::W<u32, super::DMAMUX_RGCFR>;
#[doc = "Register DMAMUX_RGCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAMUX_RGCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `COF0`"]
pub struct COF0_W<'a> {
    w: &'a mut W,
}
impl<'a> COF0_W<'a> {
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
#[doc = "Write proxy for field `COF1`"]
pub struct COF1_W<'a> {
    w: &'a mut W,
}
impl<'a> COF1_W<'a> {
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
#[doc = "Write proxy for field `COF2`"]
pub struct COF2_W<'a> {
    w: &'a mut W,
}
impl<'a> COF2_W<'a> {
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
#[doc = "Write proxy for field `COF3`"]
pub struct COF3_W<'a> {
    w: &'a mut W,
}
impl<'a> COF3_W<'a> {
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
#[doc = "Write proxy for field `COF4`"]
pub struct COF4_W<'a> {
    w: &'a mut W,
}
impl<'a> COF4_W<'a> {
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
#[doc = "Write proxy for field `COF5`"]
pub struct COF5_W<'a> {
    w: &'a mut W,
}
impl<'a> COF5_W<'a> {
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
#[doc = "Write proxy for field `COF6`"]
pub struct COF6_W<'a> {
    w: &'a mut W,
}
impl<'a> COF6_W<'a> {
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
#[doc = "Write proxy for field `COF7`"]
pub struct COF7_W<'a> {
    w: &'a mut W,
}
impl<'a> COF7_W<'a> {
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
impl W {
    #[doc = "Bit 0 - COF0"]
    #[inline(always)]
    pub fn cof0(&mut self) -> COF0_W {
        COF0_W { w: self }
    }
    #[doc = "Bit 1 - COF1"]
    #[inline(always)]
    pub fn cof1(&mut self) -> COF1_W {
        COF1_W { w: self }
    }
    #[doc = "Bit 2 - COF2"]
    #[inline(always)]
    pub fn cof2(&mut self) -> COF2_W {
        COF2_W { w: self }
    }
    #[doc = "Bit 3 - COF3"]
    #[inline(always)]
    pub fn cof3(&mut self) -> COF3_W {
        COF3_W { w: self }
    }
    #[doc = "Bit 4 - COF4"]
    #[inline(always)]
    pub fn cof4(&mut self) -> COF4_W {
        COF4_W { w: self }
    }
    #[doc = "Bit 5 - COF5"]
    #[inline(always)]
    pub fn cof5(&mut self) -> COF5_W {
        COF5_W { w: self }
    }
    #[doc = "Bit 6 - COF6"]
    #[inline(always)]
    pub fn cof6(&mut self) -> COF6_W {
        COF6_W { w: self }
    }
    #[doc = "Bit 7 - COF7"]
    #[inline(always)]
    pub fn cof7(&mut self) -> COF7_W {
        COF7_W { w: self }
    }
}
