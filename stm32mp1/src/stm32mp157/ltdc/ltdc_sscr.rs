#[doc = "Reader of register LTDC_SSCR"]
pub type R = crate::R<u32, super::LTDC_SSCR>;
#[doc = "Writer for register LTDC_SSCR"]
pub type W = crate::W<u32, super::LTDC_SSCR>;
#[doc = "Register LTDC_SSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LTDC_SSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VSH`"]
pub type VSH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VSH`"]
pub struct VSH_W<'a> {
    w: &'a mut W,
}
impl<'a> VSH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `HSW`"]
pub type HSW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HSW`"]
pub struct HSW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - VSH"]
    #[inline(always)]
    pub fn vsh(&self) -> VSH_R {
        VSH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - HSW"]
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - VSH"]
    #[inline(always)]
    pub fn vsh(&mut self) -> VSH_W {
        VSH_W { w: self }
    }
    #[doc = "Bits 16:27 - HSW"]
    #[inline(always)]
    pub fn hsw(&mut self) -> HSW_W {
        HSW_W { w: self }
    }
}
