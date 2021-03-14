#[doc = "Reader of register SQR3"]
pub type R = crate::R<u32, super::SQR3>;
#[doc = "Writer for register SQR3"]
pub type W = crate::W<u32, super::SQR3>;
#[doc = "Register SQR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SQR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SQ18`"]
pub type SQ18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ18`"]
pub struct SQ18_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
#[doc = "Reader of field `SQ17`"]
pub type SQ17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ17`"]
pub struct SQ17_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `SQ16`"]
pub type SQ16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ16`"]
pub struct SQ16_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `SQ15`"]
pub type SQ15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ15`"]
pub struct SQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SQ14`"]
pub type SQ14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ14`"]
pub struct SQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `SQ13`"]
pub type SQ13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ13`"]
pub struct SQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:29 - 18th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq18(&self) -> SQ18_R {
        SQ18_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 17th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq17(&self) -> SQ17_R {
        SQ17_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 18th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq18(&mut self) -> SQ18_W {
        SQ18_W { w: self }
    }
    #[doc = "Bits 20:24 - 17th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq17(&mut self) -> SQ17_W {
        SQ17_W { w: self }
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq16(&mut self) -> SQ16_W {
        SQ16_W { w: self }
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq15(&mut self) -> SQ15_W {
        SQ15_W { w: self }
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq14(&mut self) -> SQ14_W {
        SQ14_W { w: self }
    }
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq13(&mut self) -> SQ13_W {
        SQ13_W { w: self }
    }
}
