#[doc = "Reader of register PCROP1ASR"]
pub type R = crate::R<u32, super::PCROP1ASR>;
#[doc = "Writer for register PCROP1ASR"]
pub type W = crate::W<u32, super::PCROP1ASR>;
#[doc = "Register PCROP1ASR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PCROP1ASR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `PCROP1A_STRT`"]
pub type PCROP1A_STRT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCROP1A_STRT`"]
pub struct PCROP1A_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1A_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PCROP1A area start offset"]
    #[inline(always)]
    pub fn pcrop1a_strt(&self) -> PCROP1A_STRT_R {
        PCROP1A_STRT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PCROP1A area start offset"]
    #[inline(always)]
    pub fn pcrop1a_strt(&mut self) -> PCROP1A_STRT_W {
        PCROP1A_STRT_W { w: self }
    }
}
