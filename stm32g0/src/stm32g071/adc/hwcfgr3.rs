#[doc = "Reader of register HWCFGR3"]
pub type R = crate::R<u32, super::HWCFGR3>;
#[doc = "Writer for register HWCFGR3"]
pub type W = crate::W<u32, super::HWCFGR3>;
#[doc = "Register HWCFGR3 `reset()`'s with value 0x0706_0605"]
impl crate::ResetValue for super::HWCFGR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0706_0605
    }
}
#[doc = "Reader of field `CHMAP11`"]
pub type CHMAP11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP11`"]
pub struct CHMAP11_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CHMAP10`"]
pub type CHMAP10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP10`"]
pub struct CHMAP10_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CHMAP9`"]
pub type CHMAP9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP9`"]
pub struct CHMAP9_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CHMAP8`"]
pub type CHMAP8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHMAP8`"]
pub struct CHMAP8_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP8_W<'a> {
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
    pub fn chmap11(&self) -> CHMAP11_R {
        CHMAP11_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap10(&self) -> CHMAP10_R {
        CHMAP10_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap9(&self) -> CHMAP9_R {
        CHMAP9_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap8(&self) -> CHMAP8_R {
        CHMAP8_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap11(&mut self) -> CHMAP11_W {
        CHMAP11_W { w: self }
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap10(&mut self) -> CHMAP10_W {
        CHMAP10_W { w: self }
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap9(&mut self) -> CHMAP9_W {
        CHMAP9_W { w: self }
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap8(&mut self) -> CHMAP8_W {
        CHMAP8_W { w: self }
    }
}
