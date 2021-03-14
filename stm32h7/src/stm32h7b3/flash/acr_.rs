#[doc = "Reader of register ACR_"]
pub type R = crate::R<u32, super::ACR_>;
#[doc = "Writer for register ACR_"]
pub type W = crate::W<u32, super::ACR_>;
#[doc = "Register ACR_ `reset()`'s with value 0"]
impl crate::ResetValue for super::ACR_ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LATENCY`"]
pub type LATENCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LATENCY`"]
pub struct LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> LATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `WRHIGHFREQ`"]
pub type WRHIGHFREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRHIGHFREQ`"]
pub struct WRHIGHFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WRHIGHFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Read latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5 - Flash signal delay"]
    #[inline(always)]
    pub fn wrhighfreq(&self) -> WRHIGHFREQ_R {
        WRHIGHFREQ_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read latency"]
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W {
        LATENCY_W { w: self }
    }
    #[doc = "Bits 4:5 - Flash signal delay"]
    #[inline(always)]
    pub fn wrhighfreq(&mut self) -> WRHIGHFREQ_W {
        WRHIGHFREQ_W { w: self }
    }
}
