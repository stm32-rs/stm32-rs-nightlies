#[doc = "Reader of register LTDC_L1WVPCR"]
pub type R = crate::R<u32, super::LTDC_L1WVPCR>;
#[doc = "Writer for register LTDC_L1WVPCR"]
pub type W = crate::W<u32, super::LTDC_L1WVPCR>;
#[doc = "Register LTDC_L1WVPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LTDC_L1WVPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WVSTPOS`"]
pub type WVSTPOS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WVSTPOS`"]
pub struct WVSTPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WVSTPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `WVSPPOS`"]
pub type WVSPPOS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WVSPPOS`"]
pub struct WVSPPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WVSPPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - WVSTPOS"]
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - WVSPPOS"]
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - WVSTPOS"]
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WVSTPOS_W {
        WVSTPOS_W { w: self }
    }
    #[doc = "Bits 16:27 - WVSPPOS"]
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WVSPPOS_W {
        WVSPPOS_W { w: self }
    }
}
