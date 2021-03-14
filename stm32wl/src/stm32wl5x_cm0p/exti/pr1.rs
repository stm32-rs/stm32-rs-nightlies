#[doc = "Reader of register PR1"]
pub type R = crate::R<u32, super::PR1>;
#[doc = "Writer for register PR1"]
pub type W = crate::W<u32, super::PR1>;
#[doc = "Register PR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PIF`"]
pub type PIF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PIF`"]
pub struct PIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "Reader of field `PIF21`"]
pub type PIF21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIF21`"]
pub struct PIF21_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif(&self) -> PIF_R {
        PIF_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bits 21:22 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif(&mut self) -> PIF_W {
        PIF_W { w: self }
    }
    #[doc = "Bits 21:22 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif21(&mut self) -> PIF21_W {
        PIF21_W { w: self }
    }
}
