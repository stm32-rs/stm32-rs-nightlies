#[doc = "Reader of register PLL2DIVR"]
pub type R = crate::R<u32, super::PLL2DIVR>;
#[doc = "Writer for register PLL2DIVR"]
pub type W = crate::W<u32, super::PLL2DIVR>;
#[doc = "Register PLL2DIVR `reset()`'s with value 0x0101_0280"]
impl crate::ResetValue for super::PLL2DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0101_0280
    }
}
#[doc = "Reader of field `DIVN2`"]
pub type DIVN2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVN2`"]
pub struct DIVN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `DIVP2`"]
pub type DIVP2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVP2`"]
pub struct DIVP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `DIVQ2`"]
pub type DIVQ2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVQ2`"]
pub struct DIVQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIVR2`"]
pub type DIVR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVR2`"]
pub struct DIVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn2(&self) -> DIVN2_R {
        DIVN2_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp2(&self) -> DIVP2_R {
        DIVP2_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq2(&self) -> DIVQ2_R {
        DIVQ2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr2(&self) -> DIVR2_R {
        DIVR2_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn2(&mut self) -> DIVN2_W {
        DIVN2_W { w: self }
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp2(&mut self) -> DIVP2_W {
        DIVP2_W { w: self }
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq2(&mut self) -> DIVQ2_W {
        DIVQ2_W { w: self }
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr2(&mut self) -> DIVR2_W {
        DIVR2_W { w: self }
    }
}
