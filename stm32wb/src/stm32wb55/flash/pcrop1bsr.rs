#[doc = "Reader of register PCROP1BSR"]
pub type R = crate::R<u32, super::PCROP1BSR>;
#[doc = "Writer for register PCROP1BSR"]
pub type W = crate::W<u32, super::PCROP1BSR>;
#[doc = "Register PCROP1BSR `reset()`'s with value 0xffff_fe00"]
impl crate::ResetValue for super::PCROP1BSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_fe00
    }
}
#[doc = "Reader of field `PCROP1B_STRT`"]
pub type PCROP1B_STRT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCROP1B_STRT`"]
pub struct PCROP1B_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1B_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Bank 1 PCROP area B start offset"]
    #[inline(always)]
    pub fn pcrop1b_strt(&self) -> PCROP1B_STRT_R {
        PCROP1B_STRT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Bank 1 PCROP area B start offset"]
    #[inline(always)]
    pub fn pcrop1b_strt(&mut self) -> PCROP1B_STRT_W {
        PCROP1B_STRT_W { w: self }
    }
}
