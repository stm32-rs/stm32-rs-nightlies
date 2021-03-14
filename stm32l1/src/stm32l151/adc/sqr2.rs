#[doc = "Reader of register SQR2"]
pub type R = crate::R<u32, super::SQR2>;
#[doc = "Writer for register SQR2"]
pub type W = crate::W<u32, super::SQR2>;
#[doc = "Register SQR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SQR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SQ24`"]
pub type SQ24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ24`"]
pub struct SQ24_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
#[doc = "Reader of field `SQ23`"]
pub type SQ23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ23`"]
pub struct SQ23_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `SQ22`"]
pub type SQ22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ22`"]
pub struct SQ22_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `SQ21`"]
pub type SQ21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ21`"]
pub struct SQ21_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `SQ20`"]
pub type SQ20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ20`"]
pub struct SQ20_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `SQ19`"]
pub type SQ19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQ19`"]
pub struct SQ19_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:29 - 24th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq24(&self) -> SQ24_R {
        SQ24_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 23rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq23(&self) -> SQ23_R {
        SQ23_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 22nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq22(&self) -> SQ22_R {
        SQ22_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 21st conversion in regular sequence"]
    #[inline(always)]
    pub fn sq21(&self) -> SQ21_R {
        SQ21_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 20th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq20(&self) -> SQ20_R {
        SQ20_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 19th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq19(&self) -> SQ19_R {
        SQ19_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 24th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq24(&mut self) -> SQ24_W {
        SQ24_W { w: self }
    }
    #[doc = "Bits 20:24 - 23rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq23(&mut self) -> SQ23_W {
        SQ23_W { w: self }
    }
    #[doc = "Bits 15:19 - 22nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq22(&mut self) -> SQ22_W {
        SQ22_W { w: self }
    }
    #[doc = "Bits 10:14 - 21st conversion in regular sequence"]
    #[inline(always)]
    pub fn sq21(&mut self) -> SQ21_W {
        SQ21_W { w: self }
    }
    #[doc = "Bits 5:9 - 20th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq20(&mut self) -> SQ20_W {
        SQ20_W { w: self }
    }
    #[doc = "Bits 0:4 - 19th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq19(&mut self) -> SQ19_W {
        SQ19_W { w: self }
    }
}
