#[doc = "Reader of register PCROP1SR"]
pub type R = crate::R<u32, super::PCROP1SR>;
#[doc = "Writer for register PCROP1SR"]
pub type W = crate::W<u32, super::PCROP1SR>;
#[doc = "Register PCROP1SR `reset()`'s with value 0xffff_0000"]
impl crate::ResetValue for super::PCROP1SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
#[doc = "Reader of field `PCROP1_STRT`"]
pub type PCROP1_STRT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCROP1_STRT`"]
pub struct PCROP1_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Bank 1 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop1_strt(&self) -> PCROP1_STRT_R {
        PCROP1_STRT_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Bank 1 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop1_strt(&mut self) -> PCROP1_STRT_W {
        PCROP1_STRT_W { w: self }
    }
}
