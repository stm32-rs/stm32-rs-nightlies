#[doc = "Reader of register SYSCFG_ICNR"]
pub type R = crate::R<u32, super::SYSCFG_ICNR>;
#[doc = "Writer for register SYSCFG_ICNR"]
pub type W = crate::W<u32, super::SYSCFG_ICNR>;
#[doc = "Register SYSCFG_ICNR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG_ICNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AXI_M0`"]
pub type AXI_M0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_M0`"]
pub struct AXI_M0_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_M0_W<'a> {
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
#[doc = "Reader of field `AXI_M1`"]
pub type AXI_M1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_M1`"]
pub struct AXI_M1_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_M1_W<'a> {
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
#[doc = "Reader of field `AXI_M2`"]
pub type AXI_M2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_M2`"]
pub struct AXI_M2_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_M2_W<'a> {
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
#[doc = "Reader of field `AXI_M3`"]
pub type AXI_M3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_M3`"]
pub struct AXI_M3_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_M3_W<'a> {
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
#[doc = "Reader of field `AXI_M5`"]
pub type AXI_M5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_M5`"]
pub struct AXI_M5_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_M5_W<'a> {
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
#[doc = "Reader of field `AXI_M6`"]
pub type AXI_M6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_M6`"]
pub struct AXI_M6_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_M6_W<'a> {
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
#[doc = "Reader of field `AXI_M7`"]
pub type AXI_M7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_M7`"]
pub struct AXI_M7_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_M7_W<'a> {
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
#[doc = "Reader of field `AXI_M8`"]
pub type AXI_M8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_M8`"]
pub struct AXI_M8_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_M8_W<'a> {
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
#[doc = "Reader of field `AXI_M9`"]
pub type AXI_M9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_M9`"]
pub struct AXI_M9_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_M9_W<'a> {
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
#[doc = "Reader of field `AXI_M10`"]
pub type AXI_M10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXI_M10`"]
pub struct AXI_M10_W<'a> {
    w: &'a mut W,
}
impl<'a> AXI_M10_W<'a> {
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
impl R {
    #[doc = "Bit 0 - AXI_M0"]
    #[inline(always)]
    pub fn axi_m0(&self) -> AXI_M0_R {
        AXI_M0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AXI_M1"]
    #[inline(always)]
    pub fn axi_m1(&self) -> AXI_M1_R {
        AXI_M1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AXI_M2"]
    #[inline(always)]
    pub fn axi_m2(&self) -> AXI_M2_R {
        AXI_M2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AXI_M3"]
    #[inline(always)]
    pub fn axi_m3(&self) -> AXI_M3_R {
        AXI_M3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AXI_M5"]
    #[inline(always)]
    pub fn axi_m5(&self) -> AXI_M5_R {
        AXI_M5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AXI_M6"]
    #[inline(always)]
    pub fn axi_m6(&self) -> AXI_M6_R {
        AXI_M6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AXI_M7"]
    #[inline(always)]
    pub fn axi_m7(&self) -> AXI_M7_R {
        AXI_M7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AXI_M8"]
    #[inline(always)]
    pub fn axi_m8(&self) -> AXI_M8_R {
        AXI_M8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AXI_M9"]
    #[inline(always)]
    pub fn axi_m9(&self) -> AXI_M9_R {
        AXI_M9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AXI_M10"]
    #[inline(always)]
    pub fn axi_m10(&self) -> AXI_M10_R {
        AXI_M10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AXI_M0"]
    #[inline(always)]
    pub fn axi_m0(&mut self) -> AXI_M0_W {
        AXI_M0_W { w: self }
    }
    #[doc = "Bit 1 - AXI_M1"]
    #[inline(always)]
    pub fn axi_m1(&mut self) -> AXI_M1_W {
        AXI_M1_W { w: self }
    }
    #[doc = "Bit 2 - AXI_M2"]
    #[inline(always)]
    pub fn axi_m2(&mut self) -> AXI_M2_W {
        AXI_M2_W { w: self }
    }
    #[doc = "Bit 3 - AXI_M3"]
    #[inline(always)]
    pub fn axi_m3(&mut self) -> AXI_M3_W {
        AXI_M3_W { w: self }
    }
    #[doc = "Bit 5 - AXI_M5"]
    #[inline(always)]
    pub fn axi_m5(&mut self) -> AXI_M5_W {
        AXI_M5_W { w: self }
    }
    #[doc = "Bit 6 - AXI_M6"]
    #[inline(always)]
    pub fn axi_m6(&mut self) -> AXI_M6_W {
        AXI_M6_W { w: self }
    }
    #[doc = "Bit 7 - AXI_M7"]
    #[inline(always)]
    pub fn axi_m7(&mut self) -> AXI_M7_W {
        AXI_M7_W { w: self }
    }
    #[doc = "Bit 8 - AXI_M8"]
    #[inline(always)]
    pub fn axi_m8(&mut self) -> AXI_M8_W {
        AXI_M8_W { w: self }
    }
    #[doc = "Bit 9 - AXI_M9"]
    #[inline(always)]
    pub fn axi_m9(&mut self) -> AXI_M9_W {
        AXI_M9_W { w: self }
    }
    #[doc = "Bit 10 - AXI_M10"]
    #[inline(always)]
    pub fn axi_m10(&mut self) -> AXI_M10_W {
        AXI_M10_W { w: self }
    }
}
