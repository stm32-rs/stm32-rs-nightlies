#[doc = "Reader of register TR3"]
pub type R = crate::R<u32, super::TR3>;
#[doc = "Writer for register TR3"]
pub type W = crate::W<u32, super::TR3>;
#[doc = "Register TR3 `reset()`'s with value 0x00ff_0000"]
impl crate::ResetValue for super::TR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_0000
    }
}
#[doc = "Reader of field `HT3`"]
pub type HT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HT3`"]
pub struct HT3_W<'a> {
    w: &'a mut W,
}
impl<'a> HT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LT3`"]
pub type LT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LT3`"]
pub struct LT3_W<'a> {
    w: &'a mut W,
}
impl<'a> LT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Analog watchdog 3 higher threshold"]
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Analog watchdog 3 lower threshold"]
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Analog watchdog 3 higher threshold"]
    #[inline(always)]
    pub fn ht3(&mut self) -> HT3_W {
        HT3_W { w: self }
    }
    #[doc = "Bits 0:7 - Analog watchdog 3 lower threshold"]
    #[inline(always)]
    pub fn lt3(&mut self) -> LT3_W {
        LT3_W { w: self }
    }
}
