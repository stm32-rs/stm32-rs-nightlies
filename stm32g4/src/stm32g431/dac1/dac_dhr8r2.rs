#[doc = "Reader of register DAC_DHR8R2"]
pub type R = crate::R<u32, super::DAC_DHR8R2>;
#[doc = "Writer for register DAC_DHR8R2"]
pub type W = crate::W<u32, super::DAC_DHR8R2>;
#[doc = "Register DAC_DHR8R2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_DHR8R2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DACC2DHR`"]
pub type DACC2DHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DACC2DHR`"]
pub struct DACC2DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC2DHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DACC2DHRB`"]
pub type DACC2DHRB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DACC2DHRB`"]
pub struct DACC2DHRB_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC2DHRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhrb(&self) -> DACC2DHRB_R {
        DACC2DHRB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2."]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W {
        DACC2DHR_W { w: self }
    }
    #[doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dacc2dhrb(&mut self) -> DACC2DHRB_W {
        DACC2DHRB_W { w: self }
    }
}
