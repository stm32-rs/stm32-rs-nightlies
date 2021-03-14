#[doc = "Reader of register SWIER1"]
pub type R = crate::R<u32, super::SWIER1>;
#[doc = "Writer for register SWIER1"]
pub type W = crate::W<u32, super::SWIER1>;
#[doc = "Register SWIER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWIER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWI`"]
pub type SWI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SWI`"]
pub struct SWI_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Reader of field `SWI21`"]
pub type SWI21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWI21`"]
pub struct SWI21_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi(&self) -> SWI_R {
        SWI_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bits 21:22 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi21(&self) -> SWI21_R {
        SWI21_R::new(((self.bits >> 21) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi(&mut self) -> SWI_W {
        SWI_W { w: self }
    }
    #[doc = "Bits 21:22 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi21(&mut self) -> SWI21_W {
        SWI21_W { w: self }
    }
}
