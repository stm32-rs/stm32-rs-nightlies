#[doc = "Reader of register HWCFGR6"]
pub type R = crate::R<u32, super::HWCFGR6>;
#[doc = "Writer for register HWCFGR6"]
pub type W = crate::W<u32, super::HWCFGR6>;
#[doc = "Register HWCFGR6 `reset()`'s with value 0x1f1f_1f1f"]
impl crate::ResetValue for super::HWCFGR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f1f_1f1f
    }
}
#[doc = "Reader of field `CHMAP20`"]
pub type CHMAP20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP20`"]
pub struct CHMAP20_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CHMAP21`"]
pub type CHMAP21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP21`"]
pub struct CHMAP21_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CHMAP22`"]
pub type CHMAP22_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP22`"]
pub struct CHMAP22_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CHMAP23`"]
pub type CHMAP23_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP23`"]
pub struct CHMAP23_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP23_W<'a> {
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
    pub fn chmap20(&self) -> CHMAP20_R {
        CHMAP20_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap21(&self) -> CHMAP21_R {
        CHMAP21_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap22(&self) -> CHMAP22_R {
        CHMAP22_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap23(&self) -> CHMAP23_R {
        CHMAP23_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap20(&mut self) -> CHMAP20_W {
        CHMAP20_W { w: self }
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap21(&mut self) -> CHMAP21_W {
        CHMAP21_W { w: self }
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap22(&mut self) -> CHMAP22_W {
        CHMAP22_W { w: self }
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap23(&mut self) -> CHMAP23_W {
        CHMAP23_W { w: self }
    }
}
