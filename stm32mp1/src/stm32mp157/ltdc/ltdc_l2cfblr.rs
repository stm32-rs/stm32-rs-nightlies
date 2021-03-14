#[doc = "Reader of register LTDC_L2CFBLR"]
pub type R = crate::R<u32, super::LTDC_L2CFBLR>;
#[doc = "Writer for register LTDC_L2CFBLR"]
pub type W = crate::W<u32, super::LTDC_L2CFBLR>;
#[doc = "Register LTDC_L2CFBLR `reset()`'s with value 0"]
impl crate::ResetValue for super::LTDC_L2CFBLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFBLL`"]
pub type CFBLL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CFBLL`"]
pub struct CFBLL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `CFBP`"]
pub type CFBP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CFBP`"]
pub struct CFBP_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - CFBLL"]
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - CFBP"]
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - CFBLL"]
    #[inline(always)]
    pub fn cfbll(&mut self) -> CFBLL_W {
        CFBLL_W { w: self }
    }
    #[doc = "Bits 16:29 - CFBP"]
    #[inline(always)]
    pub fn cfbp(&mut self) -> CFBP_W {
        CFBP_W { w: self }
    }
}
