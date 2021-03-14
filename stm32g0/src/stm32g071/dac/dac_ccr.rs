#[doc = "Reader of register DAC_CCR"]
pub type R = crate::R<u32, super::DAC_CCR>;
#[doc = "Writer for register DAC_CCR"]
pub type W = crate::W<u32, super::DAC_CCR>;
#[doc = "Register DAC_CCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OTRIM1`"]
pub type OTRIM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OTRIM1`"]
pub struct OTRIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> OTRIM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `OTRIM2`"]
pub type OTRIM2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OTRIM2`"]
pub struct OTRIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> OTRIM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DAC Channel 1 offset trimming value"]
    #[inline(always)]
    pub fn otrim1(&self) -> OTRIM1_R {
        OTRIM1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DAC Channel 2 offset trimming value"]
    #[inline(always)]
    pub fn otrim2(&self) -> OTRIM2_R {
        OTRIM2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DAC Channel 1 offset trimming value"]
    #[inline(always)]
    pub fn otrim1(&mut self) -> OTRIM1_W {
        OTRIM1_W { w: self }
    }
    #[doc = "Bits 16:20 - DAC Channel 2 offset trimming value"]
    #[inline(always)]
    pub fn otrim2(&mut self) -> OTRIM2_W {
        OTRIM2_W { w: self }
    }
}
