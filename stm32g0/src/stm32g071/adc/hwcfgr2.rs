#[doc = "Reader of register HWCFGR2"]
pub type R = crate::R<u32, super::HWCFGR2>;
#[doc = "Writer for register HWCFGR2"]
pub type W = crate::W<u32, super::HWCFGR2>;
#[doc = "Register HWCFGR2 `reset()`'s with value 0x0505_0404"]
impl crate::ResetValue for super::HWCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0505_0404
    }
}
#[doc = "Reader of field `CHMAP7`"]
pub type CHMAP7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP7`"]
pub struct CHMAP7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CHMAP6`"]
pub type CHMAP6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP6`"]
pub struct CHMAP6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CHMAP5`"]
pub type CHMAP5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP5`"]
pub struct CHMAP5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CHMAP4`"]
pub type CHMAP4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP4`"]
pub struct CHMAP4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP4_W<'a> {
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
    pub fn chmap7(&self) -> CHMAP7_R {
        CHMAP7_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap6(&self) -> CHMAP6_R {
        CHMAP6_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap5(&self) -> CHMAP5_R {
        CHMAP5_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap4(&self) -> CHMAP4_R {
        CHMAP4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap7(&mut self) -> CHMAP7_W {
        CHMAP7_W { w: self }
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap6(&mut self) -> CHMAP6_W {
        CHMAP6_W { w: self }
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap5(&mut self) -> CHMAP5_W {
        CHMAP5_W { w: self }
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap4(&mut self) -> CHMAP4_W {
        CHMAP4_W { w: self }
    }
}
