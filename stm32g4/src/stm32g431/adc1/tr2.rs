#[doc = "Reader of register TR2"]
pub type R = crate::R<u32, super::TR2>;
#[doc = "Writer for register TR2"]
pub type W = crate::W<u32, super::TR2>;
#[doc = "Register TR2 `reset()`'s with value 0x00ff_0000"]
impl crate::ResetValue for super::TR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_0000
    }
}
#[doc = "Reader of field `HT2`"]
pub type HT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HT2`"]
pub struct HT2_W<'a> {
    w: &'a mut W,
}
impl<'a> HT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LT2`"]
pub type LT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LT2`"]
pub struct LT2_W<'a> {
    w: &'a mut W,
}
impl<'a> LT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Analog watchdog 2 higher threshold"]
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Analog watchdog 2 higher threshold"]
    #[inline(always)]
    pub fn ht2(&mut self) -> HT2_W {
        HT2_W { w: self }
    }
    #[doc = "Bits 0:7 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    pub fn lt2(&mut self) -> LT2_W {
        LT2_W { w: self }
    }
}
