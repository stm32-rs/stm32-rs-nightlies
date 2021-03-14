#[doc = "Reader of register RCC_MP_CIFR"]
pub type R = crate::R<u32, super::RCC_MP_CIFR>;
#[doc = "Writer for register RCC_MP_CIFR"]
pub type W = crate::W<u32, super::RCC_MP_CIFR>;
#[doc = "Register RCC_MP_CIFR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_CIFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSIRDYF`"]
pub type LSIRDYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSIRDYF`"]
pub struct LSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYF_W<'a> {
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
#[doc = "Reader of field `LSERDYF`"]
pub type LSERDYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSERDYF`"]
pub struct LSERDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYF_W<'a> {
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
#[doc = "Reader of field `HSIRDYF`"]
pub type HSIRDYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIRDYF`"]
pub struct HSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYF_W<'a> {
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
#[doc = "Reader of field `HSERDYF`"]
pub type HSERDYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSERDYF`"]
pub struct HSERDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYF_W<'a> {
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
#[doc = "Reader of field `CSIRDYF`"]
pub type CSIRDYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSIRDYF`"]
pub struct CSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIRDYF_W<'a> {
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
#[doc = "Reader of field `PLL1DYF`"]
pub type PLL1DYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL1DYF`"]
pub struct PLL1DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1DYF_W<'a> {
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
#[doc = "Reader of field `PLL2DYF`"]
pub type PLL2DYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL2DYF`"]
pub struct PLL2DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2DYF_W<'a> {
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
#[doc = "Reader of field `PLL3DYF`"]
pub type PLL3DYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL3DYF`"]
pub struct PLL3DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3DYF_W<'a> {
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
#[doc = "Reader of field `PLL4DYF`"]
pub type PLL4DYF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL4DYF`"]
pub struct PLL4DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL4DYF_W<'a> {
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
#[doc = "Reader of field `LSECSSF`"]
pub type LSECSSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSECSSF`"]
pub struct LSECSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSF_W<'a> {
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
#[doc = "Reader of field `WKUPF`"]
pub type WKUPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPF`"]
pub struct WKUPF_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSIRDYF"]
    #[inline(always)]
    pub fn csirdyf(&self) -> CSIRDYF_R {
        CSIRDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL1DYF"]
    #[inline(always)]
    pub fn pll1dyf(&self) -> PLL1DYF_R {
        PLL1DYF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLL2DYF"]
    #[inline(always)]
    pub fn pll2dyf(&self) -> PLL2DYF_R {
        PLL2DYF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PLL3DYF"]
    #[inline(always)]
    pub fn pll3dyf(&self) -> PLL3DYF_R {
        PLL3DYF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PLL4DYF"]
    #[inline(always)]
    pub fn pll4dyf(&self) -> PLL4DYF_R {
        PLL4DYF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LSECSSF"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WKUPF"]
    #[inline(always)]
    pub fn wkupf(&self) -> WKUPF_R {
        WKUPF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W {
        LSIRDYF_W { w: self }
    }
    #[doc = "Bit 1 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&mut self) -> LSERDYF_W {
        LSERDYF_W { w: self }
    }
    #[doc = "Bit 2 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W {
        HSIRDYF_W { w: self }
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&mut self) -> HSERDYF_W {
        HSERDYF_W { w: self }
    }
    #[doc = "Bit 4 - CSIRDYF"]
    #[inline(always)]
    pub fn csirdyf(&mut self) -> CSIRDYF_W {
        CSIRDYF_W { w: self }
    }
    #[doc = "Bit 8 - PLL1DYF"]
    #[inline(always)]
    pub fn pll1dyf(&mut self) -> PLL1DYF_W {
        PLL1DYF_W { w: self }
    }
    #[doc = "Bit 9 - PLL2DYF"]
    #[inline(always)]
    pub fn pll2dyf(&mut self) -> PLL2DYF_W {
        PLL2DYF_W { w: self }
    }
    #[doc = "Bit 10 - PLL3DYF"]
    #[inline(always)]
    pub fn pll3dyf(&mut self) -> PLL3DYF_W {
        PLL3DYF_W { w: self }
    }
    #[doc = "Bit 11 - PLL4DYF"]
    #[inline(always)]
    pub fn pll4dyf(&mut self) -> PLL4DYF_W {
        PLL4DYF_W { w: self }
    }
    #[doc = "Bit 16 - LSECSSF"]
    #[inline(always)]
    pub fn lsecssf(&mut self) -> LSECSSF_W {
        LSECSSF_W { w: self }
    }
    #[doc = "Bit 20 - WKUPF"]
    #[inline(always)]
    pub fn wkupf(&mut self) -> WKUPF_W {
        WKUPF_W { w: self }
    }
}
