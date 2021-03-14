#[doc = "Reader of register LTDC_BPCR"]
pub type R = crate::R<u32, super::LTDC_BPCR>;
#[doc = "Writer for register LTDC_BPCR"]
pub type W = crate::W<u32, super::LTDC_BPCR>;
#[doc = "Register LTDC_BPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LTDC_BPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AVBP`"]
pub type AVBP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AVBP`"]
pub struct AVBP_W<'a> {
    w: &'a mut W,
}
impl<'a> AVBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `AHBP`"]
pub type AHBP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AHBP`"]
pub struct AHBP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - AVBP"]
    #[inline(always)]
    pub fn avbp(&self) -> AVBP_R {
        AVBP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - AHBP"]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - AVBP"]
    #[inline(always)]
    pub fn avbp(&mut self) -> AVBP_W {
        AVBP_W { w: self }
    }
    #[doc = "Bits 16:27 - AHBP"]
    #[inline(always)]
    pub fn ahbp(&mut self) -> AHBP_W {
        AHBP_W { w: self }
    }
}
