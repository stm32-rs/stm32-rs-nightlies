#[doc = "Reader of register AWD3TR"]
pub type R = crate::R<u32, super::AWD3TR>;
#[doc = "Writer for register AWD3TR"]
pub type W = crate::W<u32, super::AWD3TR>;
#[doc = "Register AWD3TR `reset()`'s with value 0x0fff_0000"]
impl crate::ResetValue for super::AWD3TR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
#[doc = "Reader of field `HT3`"]
pub type HT3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HT3`"]
pub struct HT3_W<'a> {
    w: &'a mut W,
}
impl<'a> HT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LT3`"]
pub type LT3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LT3`"]
pub struct LT3_W<'a> {
    w: &'a mut W,
}
impl<'a> LT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    pub fn ht3(&mut self) -> HT3_W {
        HT3_W { w: self }
    }
    #[doc = "Bits 0:11 - ADC analog watchdog 3 threshold high"]
    #[inline(always)]
    pub fn lt3(&mut self) -> LT3_W {
        LT3_W { w: self }
    }
}
