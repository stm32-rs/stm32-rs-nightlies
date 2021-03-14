#[doc = "Reader of register RCC_MCO2CFGR"]
pub type R = crate::R<u32, super::RCC_MCO2CFGR>;
#[doc = "Writer for register RCC_MCO2CFGR"]
pub type W = crate::W<u32, super::RCC_MCO2CFGR>;
#[doc = "Register RCC_MCO2CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MCO2CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCO2SEL`"]
pub type MCO2SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCO2SEL`"]
pub struct MCO2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `MCO2DIV`"]
pub type MCO2DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCO2DIV`"]
pub struct MCO2DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MCO2ON`"]
pub type MCO2ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCO2ON`"]
pub struct MCO2ON_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2ON_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - MCO2SEL"]
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - MCO2DIV"]
    #[inline(always)]
    pub fn mco2div(&self) -> MCO2DIV_R {
        MCO2DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MCO2ON"]
    #[inline(always)]
    pub fn mco2on(&self) -> MCO2ON_R {
        MCO2ON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCO2SEL"]
    #[inline(always)]
    pub fn mco2sel(&mut self) -> MCO2SEL_W {
        MCO2SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - MCO2DIV"]
    #[inline(always)]
    pub fn mco2div(&mut self) -> MCO2DIV_W {
        MCO2DIV_W { w: self }
    }
    #[doc = "Bit 12 - MCO2ON"]
    #[inline(always)]
    pub fn mco2on(&mut self) -> MCO2ON_W {
        MCO2ON_W { w: self }
    }
}
