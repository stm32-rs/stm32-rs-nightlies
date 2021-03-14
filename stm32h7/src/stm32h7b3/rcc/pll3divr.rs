#[doc = "Reader of register PLL3DIVR"]
pub type R = crate::R<u32, super::PLL3DIVR>;
#[doc = "Writer for register PLL3DIVR"]
pub type W = crate::W<u32, super::PLL3DIVR>;
#[doc = "Register PLL3DIVR `reset()`'s with value 0x0101_0280"]
impl crate::ResetValue for super::PLL3DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0101_0280
    }
}
#[doc = "Reader of field `DIVN3`"]
pub type DIVN3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVN3`"]
pub struct DIVN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `DIVP3`"]
pub type DIVP3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVP3`"]
pub struct DIVP3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `DIVQ3`"]
pub type DIVQ3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVQ3`"]
pub struct DIVQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIVR3`"]
pub type DIVR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVR3`"]
pub struct DIVR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR3_W<'a> {
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
    pub fn divn3(&self) -> DIVN3_R {
        DIVN3_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL DIVP division factor"]
    #[inline(always)]
    pub fn divp3(&self) -> DIVP3_R {
        DIVP3_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL DIVQ division factor"]
    #[inline(always)]
    pub fn divq3(&self) -> DIVQ3_R {
        DIVQ3_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL DIVR division factor"]
    #[inline(always)]
    pub fn divr3(&self) -> DIVR3_R {
        DIVR3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn3(&mut self) -> DIVN3_W {
        DIVN3_W { w: self }
    }
    #[doc = "Bits 9:15 - PLL DIVP division factor"]
    #[inline(always)]
    pub fn divp3(&mut self) -> DIVP3_W {
        DIVP3_W { w: self }
    }
    #[doc = "Bits 16:22 - PLL DIVQ division factor"]
    #[inline(always)]
    pub fn divq3(&mut self) -> DIVQ3_W {
        DIVQ3_W { w: self }
    }
    #[doc = "Bits 24:30 - PLL DIVR division factor"]
    #[inline(always)]
    pub fn divr3(&mut self) -> DIVR3_W {
        DIVR3_W { w: self }
    }
}
