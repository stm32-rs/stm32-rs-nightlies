#[doc = "Reader of register FTSR1"]
pub type R = crate::R<u32, super::FTSR1>;
#[doc = "Writer for register FTSR1"]
pub type W = crate::W<u32, super::FTSR1>;
#[doc = "Register FTSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FTSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FT`"]
pub type FT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FT`"]
pub struct FT_W<'a> {
    w: &'a mut W,
}
impl<'a> FT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Reader of field `FT21`"]
pub type FT21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FT21`"]
pub struct FT21_W<'a> {
    w: &'a mut W,
}
impl<'a> FT21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bits 21:22 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft21(&self) -> FT21_R {
        FT21_R::new(((self.bits >> 21) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft(&mut self) -> FT_W {
        FT_W { w: self }
    }
    #[doc = "Bits 21:22 - Falling trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn ft21(&mut self) -> FT21_W {
        FT21_W { w: self }
    }
}
