#[doc = "Reader of register DAC_STMODR"]
pub type R = crate::R<u32, super::DAC_STMODR>;
#[doc = "Writer for register DAC_STMODR"]
pub type W = crate::W<u32, super::DAC_STMODR>;
#[doc = "Register DAC_STMODR `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_STMODR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STRSTTRIGSEL1`"]
pub type STRSTTRIGSEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STRSTTRIGSEL1`"]
pub struct STRSTTRIGSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> STRSTTRIGSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `STINCTRIGSEL1`"]
pub type STINCTRIGSEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STINCTRIGSEL1`"]
pub struct STINCTRIGSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> STINCTRIGSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `STRSTTRIGSEL2`"]
pub type STRSTTRIGSEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STRSTTRIGSEL2`"]
pub struct STRSTTRIGSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> STRSTTRIGSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `STINCTRIGSEL2`"]
pub type STINCTRIGSEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STINCTRIGSEL2`"]
pub struct STINCTRIGSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> STINCTRIGSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel1(&self) -> STRSTTRIGSEL1_R {
        STRSTTRIGSEL1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel1(&self) -> STINCTRIGSEL1_R {
        STINCTRIGSEL1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel2(&self) -> STRSTTRIGSEL2_R {
        STRSTTRIGSEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel2(&self) -> STINCTRIGSEL2_R {
        STINCTRIGSEL2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel1(&mut self) -> STRSTTRIGSEL1_W {
        STRSTTRIGSEL1_W { w: self }
    }
    #[doc = "Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel1(&mut self) -> STINCTRIGSEL1_W {
        STINCTRIGSEL1_W { w: self }
    }
    #[doc = "Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel2(&mut self) -> STRSTTRIGSEL2_W {
        STRSTTRIGSEL2_W { w: self }
    }
    #[doc = "Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel2(&mut self) -> STINCTRIGSEL2_W {
        STINCTRIGSEL2_W { w: self }
    }
}
