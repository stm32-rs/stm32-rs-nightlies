#[doc = "Reader of register RCC_MCO1CFGR"]
pub type R = crate::R<u32, super::RCC_MCO1CFGR>;
#[doc = "Writer for register RCC_MCO1CFGR"]
pub type W = crate::W<u32, super::RCC_MCO1CFGR>;
#[doc = "Register RCC_MCO1CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MCO1CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCO1SEL`"]
pub type MCO1SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCO1SEL`"]
pub struct MCO1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `MCO1DIV`"]
pub type MCO1DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCO1DIV`"]
pub struct MCO1DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MCO1ON`"]
pub type MCO1ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCO1ON`"]
pub struct MCO1ON_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1ON_W<'a> {
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
    #[doc = "Bits 0:2 - MCO1SEL"]
    #[inline(always)]
    pub fn mco1sel(&self) -> MCO1SEL_R {
        MCO1SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - MCO1DIV"]
    #[inline(always)]
    pub fn mco1div(&self) -> MCO1DIV_R {
        MCO1DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MCO1ON"]
    #[inline(always)]
    pub fn mco1on(&self) -> MCO1ON_R {
        MCO1ON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCO1SEL"]
    #[inline(always)]
    pub fn mco1sel(&mut self) -> MCO1SEL_W {
        MCO1SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - MCO1DIV"]
    #[inline(always)]
    pub fn mco1div(&mut self) -> MCO1DIV_W {
        MCO1DIV_W { w: self }
    }
    #[doc = "Bit 12 - MCO1ON"]
    #[inline(always)]
    pub fn mco1on(&mut self) -> MCO1ON_W {
        MCO1ON_W { w: self }
    }
}
