#[doc = "Reader of register HWCFGR4"]
pub type R = crate::R<u32, super::HWCFGR4>;
#[doc = "Writer for register HWCFGR4"]
pub type W = crate::W<u32, super::HWCFGR4>;
#[doc = "Register HWCFGR4 `reset()`'s with value 0x070b_0a09"]
impl crate::ResetValue for super::HWCFGR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x070b_0a09
    }
}
#[doc = "Reader of field `CHMAP15`"]
pub type CHMAP15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP15`"]
pub struct CHMAP15_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CHMAP14`"]
pub type CHMAP14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP14`"]
pub struct CHMAP14_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CHMAP13`"]
pub type CHMAP13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP13`"]
pub struct CHMAP13_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CHMAP12`"]
pub type CHMAP12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP12`"]
pub struct CHMAP12_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP12_W<'a> {
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
    pub fn chmap15(&self) -> CHMAP15_R {
        CHMAP15_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap14(&self) -> CHMAP14_R {
        CHMAP14_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap13(&self) -> CHMAP13_R {
        CHMAP13_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap12(&self) -> CHMAP12_R {
        CHMAP12_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap15(&mut self) -> CHMAP15_W {
        CHMAP15_W { w: self }
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap14(&mut self) -> CHMAP14_W {
        CHMAP14_W { w: self }
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap13(&mut self) -> CHMAP13_W {
        CHMAP13_W { w: self }
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap12(&mut self) -> CHMAP12_W {
        CHMAP12_W { w: self }
    }
}
