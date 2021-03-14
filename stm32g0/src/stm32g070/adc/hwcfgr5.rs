#[doc = "Reader of register HWCFGR5"]
pub type R = crate::R<u32, super::HWCFGR5>;
#[doc = "Writer for register HWCFGR5"]
pub type W = crate::W<u32, super::HWCFGR5>;
#[doc = "Register HWCFGR5 `reset()`'s with value 0x1f08_0807"]
impl crate::ResetValue for super::HWCFGR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f08_0807
    }
}
#[doc = "Reader of field `CHMAP19`"]
pub type CHMAP19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP19`"]
pub struct CHMAP19_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CHMAP18`"]
pub type CHMAP18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP18`"]
pub struct CHMAP18_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CHMAP17`"]
pub type CHMAP17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP17`"]
pub struct CHMAP17_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CHMAP16`"]
pub type CHMAP16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP16`"]
pub struct CHMAP16_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap19(&self) -> CHMAP19_R {
        CHMAP19_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap18(&self) -> CHMAP18_R {
        CHMAP18_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap17(&self) -> CHMAP17_R {
        CHMAP17_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap16(&self) -> CHMAP16_R {
        CHMAP16_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap19(&mut self) -> CHMAP19_W {
        CHMAP19_W { w: self }
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap18(&mut self) -> CHMAP18_W {
        CHMAP18_W { w: self }
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap17(&mut self) -> CHMAP17_W {
        CHMAP17_W { w: self }
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap16(&mut self) -> CHMAP16_W {
        CHMAP16_W { w: self }
    }
}
