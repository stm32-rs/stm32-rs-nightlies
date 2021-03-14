#[doc = "Reader of register PCROP1ASR"]
pub type R = crate::R<u32, super::PCROP1ASR>;
#[doc = "Writer for register PCROP1ASR"]
pub type W = crate::W<u32, super::PCROP1ASR>;
#[doc = "Register PCROP1ASR `reset()`'s with value 0xffff_fe00"]
impl crate::ResetValue for super::PCROP1ASR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_fe00
    }
}
#[doc = "Reader of field `PCROP1A_STRT`"]
pub type PCROP1A_STRT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCROP1A_STRT`"]
pub struct PCROP1A_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1A_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Bank 1 PCROPQ area start offset"]
    #[inline(always)]
    pub fn pcrop1a_strt(&self) -> PCROP1A_STRT_R {
        PCROP1A_STRT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Bank 1 PCROPQ area start offset"]
    #[inline(always)]
    pub fn pcrop1a_strt(&mut self) -> PCROP1A_STRT_W {
        PCROP1A_STRT_W { w: self }
    }
}
